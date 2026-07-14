//! FSH export — convert tank entities to FHIR JSON resources

pub mod code_system;
pub mod instance;
pub mod mapping;
pub mod structure_def;
pub mod value_set;

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::schema::CompiledSchema;
use crate::tank::FshTank;
use crate::{
    build_definition_index, load_dependency_structure_definitions, DependencyDefinitionSet,
};
use rayon::prelude::*;
use std::sync::Arc;

/// A compiled FHIR package — the output of FSH compilation
pub struct FhirPackage {
    pub resources: Vec<serde_json::Value>,
    pub errors: Vec<FshError>,
}

/// FSH exporter — converts a tank into a FHIR package
pub struct FshExporter;

impl FshExporter {
    pub fn export(tank: &FshTank, defs: Arc<FhirDefs>, config: &crate::FshConfig) -> FhirPackage {
        let mut resources = Vec::new();
        let mut errors = Vec::new();
        let dependencies = match load_dependency_structure_definitions(config) {
            Ok(dependencies) => dependencies,
            Err(err) => {
                errors.push(err);
                DependencyDefinitionSet::default()
            }
        };
        let definition_index = build_definition_index(tank, config, &dependencies);
        let schema = Arc::new(CompiledSchema::compile(
            tank,
            defs.as_ref(),
            &definition_index,
        ));
        errors.extend(definition_index.dependency_warnings.iter().map(|message| {
            FshError::Export {
                message: message.clone(),
            }
        }));

        export_par(
            tank.profiles.values(),
            |p| structure_def::export_profile(p, defs.clone(), config, tank, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.extensions.values(),
            |e| structure_def::export_extension(e, defs.clone(), config, tank, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.logicals.values(),
            |l| structure_def::export_logical(l, defs.clone(), config, tank, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.resources.values(),
            |r| {
                structure_def::export_resource_def(r, defs.clone(), config, tank, &definition_index)
            },
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.instances.values().filter(|instance| {
                !matches!(
                    instance.metadata.usage.as_deref(),
                    Some("#inline" | "inline")
                )
            }),
            |i| {
                instance::export_instance(
                    i,
                    defs.as_ref(),
                    config,
                    tank,
                    &definition_index,
                    schema.as_ref(),
                )
            },
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.value_sets.values(),
            |vs| value_set::export_value_set(vs, config, tank),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.code_systems.values(),
            |cs| code_system::export_code_system(cs, config),
            &mut resources,
            &mut errors,
        );
        if let Some(implementation_guide) =
            export_implementation_guide(config, &resources, tank, &definition_index)
        {
            resources.push(implementation_guide);
        }

        resources.sort_by_key(resource_sort_key);

        // Mappings do NOT produce standalone FHIR resources (H6)

        FhirPackage { resources, errors }
    }
}

fn resource_sort_key(resource: &serde_json::Value) -> (String, String, String) {
    (
        resource
            .get("resourceType")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        resource
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        serde_json::to_string(resource).unwrap_or_default(),
    )
}

fn export_implementation_guide(
    config: &crate::FshConfig,
    resources: &[serde_json::Value],
    tank: &FshTank,
    definition_index: &crate::DefinitionIndex,
) -> Option<serde_json::Value> {
    let id = config.id.as_ref()?;
    let canonical = config.canonical.as_ref()?;
    let self_reference = format!("ImplementationGuide/{id}");
    let mut definition_resources: Vec<serde_json::Value> = resources
        .iter()
        .filter_map(|resource| resource_reference(resource).map(|reference| (resource, reference)))
        .filter(|(_, reference)| reference != &self_reference)
        .map(|(source, reference)| {
            let mut entry = serde_json::json!({
                "reference": {
                    "reference": &reference,
                },
            });
            apply_derived_resource_metadata(&mut entry, source);
            apply_instance_resource_metadata(&mut entry, &reference, tank, definition_index);
            if let Some(metadata) = config.resources.get(&reference) {
                apply_resource_metadata(&mut entry, metadata);
            }
            if entry.get("groupingId").is_none() {
                if let Some(group) = config
                    .groups
                    .iter()
                    .find(|group| group.resources.iter().any(|item| item == &reference))
                {
                    entry["groupingId"] = serde_json::Value::String(group.id.clone());
                }
            }
            entry
        })
        .collect();
    definition_resources.sort_by_key(definition_resource_sort_key);
    if definition_resources.is_empty() {
        definition_resources.push(serde_json::json!({
            "reference": {
                "reference": self_reference,
            },
        }));
    }

    let mut resource = serde_json::json!({
        "resourceType": "ImplementationGuide",
        "id": id,
        "url": format!("{}/ImplementationGuide/{}", canonical.trim_end_matches('/'), id),
        "name": config.name.as_deref().unwrap_or(id),
        "status": config.status.as_deref().unwrap_or("active"),
        "packageId": id,
        "fhirVersion": [config.fhir_version.as_deref().unwrap_or("4.0.1")],
        "definition": {
            "resource": definition_resources
        }
    });

    if let Some(version) = &config.version {
        resource["version"] = serde_json::Value::String(version.clone());
    }
    for (key, value) in [
        ("title", config.title.as_ref()),
        ("description", config.description.as_ref()),
        ("license", config.license.as_ref()),
    ] {
        if let Some(value) = value {
            resource[key] = serde_json::Value::String(value.clone());
        }
    }
    if let Some(experimental) = config.experimental {
        resource["experimental"] = serde_json::Value::Bool(experimental);
    }
    if !config.extensions.is_empty() {
        resource["extension"] = serde_json::Value::Array(config.extensions.clone());
    }
    if let Some(jurisdiction) = &config.jurisdiction {
        let mut coding = serde_json::json!({
            "system": jurisdiction.system,
            "code": jurisdiction.code,
        });
        if let Some(display) = &jurisdiction.display {
            coding["display"] = serde_json::Value::String(display.clone());
        }
        resource["jurisdiction"] = serde_json::json!([{ "coding": [coding] }]);
    }
    apply_definition_metadata(&mut resource, config);
    apply_config_metadata(&mut resource, config);
    if !config.dependencies.is_empty() {
        resource["dependsOn"] = serde_json::Value::Array(
            config
                .dependencies
                .iter()
                .map(|dependency| {
                    let mut item = serde_json::json!({
                        "packageId": dependency.package_id,
                        "version": dependency.version,
                    });
                    if let Some(id) = &dependency.id {
                        item["id"] = serde_json::Value::String(id.clone());
                    }
                    if let Some(uri) = &dependency.uri {
                        item["uri"] = serde_json::Value::String(uri.clone());
                    }
                    item
                })
                .collect(),
        );
    }

    Some(resource)
}

fn apply_instance_resource_metadata(
    entry: &mut serde_json::Value,
    reference: &str,
    tank: &FshTank,
    definition_index: &crate::DefinitionIndex,
) {
    let Some(id) = reference.split_once('/').map(|(_, id)| id) else {
        return;
    };
    let mut matches = tank
        .instances
        .values()
        .filter(|instance| instance::instance_id(instance) == id);
    let Some(instance) = matches.next() else {
        return;
    };
    if matches.next().is_some() {
        return;
    }
    if let Some(title) = &instance.metadata.title {
        entry["name"] = serde_json::Value::String(title.clone());
    }
    if let Some(description) = &instance.metadata.description {
        entry["description"] = serde_json::Value::String(description.clone());
    }
    if let Some(canonical) = definition_index
        .lookup(&instance.metadata.instance_of)
        .and_then(|definition| definition.url.as_ref())
    {
        if let Some(object) = entry.as_object_mut() {
            object.remove("exampleBoolean");
        }
        entry["exampleCanonical"] = serde_json::Value::String(canonical.clone());
    }
}

fn apply_derived_resource_metadata(entry: &mut serde_json::Value, resource: &serde_json::Value) {
    if let Some(name) = ["title", "name", "id"]
        .into_iter()
        .find_map(|key| resource.get(key).and_then(|value| value.as_str()))
    {
        entry["name"] = serde_json::Value::String(name.to_string());
    }
    if let Some(description) = resource.get("description").and_then(|value| value.as_str()) {
        entry["description"] = serde_json::Value::String(description.to_string());
    }
    let resource_type = resource
        .get("resourceType")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    if matches!(
        resource_type,
        "StructureDefinition" | "ValueSet" | "CodeSystem"
    ) {
        entry["exampleBoolean"] = serde_json::Value::Bool(false);
    } else if let Some(profile) = resource
        .pointer("/meta/profile/0")
        .and_then(|value| value.as_str())
    {
        entry["exampleCanonical"] = serde_json::Value::String(profile.to_string());
    } else {
        entry["exampleBoolean"] = serde_json::Value::Bool(true);
    }
}

fn definition_resource_sort_key(resource: &serde_json::Value) -> (String, String) {
    (
        resource
            .get("name")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_ascii_lowercase(),
        resource
            .pointer("/reference/reference")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
    )
}

fn apply_definition_metadata(resource: &mut serde_json::Value, config: &crate::FshConfig) {
    if !config.pages.is_empty() {
        let pages = config.pages.iter().map(export_page).collect::<Vec<_>>();
        resource["definition"]["page"] = serde_json::json!({
            "nameUrl": "toc.html",
            "title": "Table of Contents",
            "generation": "html",
            "page": pages,
        });
    }
    if !config.groups.is_empty() {
        resource["definition"]["grouping"] = serde_json::Value::Array(
            config
                .groups
                .iter()
                .map(|group| {
                    let mut value = serde_json::json!({ "id": group.id, "name": group.name });
                    if let Some(description) = &group.description {
                        value["description"] = serde_json::Value::String(description.clone());
                    }
                    value
                })
                .collect(),
        );
    }
    if !config.parameters.is_empty() {
        resource["definition"]["parameter"] = serde_json::Value::Array(
            config
                .parameters
                .iter()
                .map(|(code, value)| serde_json::json!({ "code": code, "value": value }))
                .collect(),
        );
    }
}

fn export_page(page: &crate::FshPage) -> serde_json::Value {
    let is_markdown = page.source.ends_with(".md");
    let mut value = serde_json::json!({
        "nameUrl": if is_markdown {
            format!("{}.html", page.source.trim_end_matches(".md"))
        } else {
            page.source.clone()
        },
        "title": page.title,
        "generation": if is_markdown { "markdown" } else { "html" },
    });
    if !page.extensions.is_empty() {
        value["extension"] = serde_json::Value::Array(page.extensions.clone());
    }
    if !page.pages.is_empty() {
        value["page"] = serde_json::Value::Array(page.pages.iter().map(export_page).collect());
    }
    value
}

fn apply_resource_metadata(entry: &mut serde_json::Value, metadata: &crate::FshResourceMetadata) {
    for (key, value) in [
        ("name", metadata.name.as_ref()),
        ("description", metadata.description.as_ref()),
        ("exampleCanonical", metadata.example_canonical.as_ref()),
        ("groupingId", metadata.grouping_id.as_ref()),
    ] {
        if let Some(value) = value {
            entry[key] = serde_json::Value::String(value.clone());
        }
    }
    if let Some(value) = metadata.example_boolean {
        entry["exampleBoolean"] = serde_json::Value::Bool(value);
    }
}

pub(crate) fn apply_config_metadata(resource: &mut serde_json::Value, config: &crate::FshConfig) {
    if let Some(publisher) = &config.publisher {
        resource["publisher"] = serde_json::Value::String(publisher.clone());
    }
    if config.contacts.is_empty() {
        return;
    }
    resource["contact"] = serde_json::Value::Array(
        config
            .contacts
            .iter()
            .map(|contact| {
                let mut value = serde_json::Map::new();
                if let Some(name) = &contact.name {
                    value.insert("name".to_string(), serde_json::Value::String(name.clone()));
                }
                if !contact.telecom.is_empty() {
                    value.insert(
                        "telecom".to_string(),
                        serde_json::Value::Array(
                            contact
                                .telecom
                                .iter()
                                .map(|telecom| {
                                    serde_json::json!({
                                        "system": telecom.system,
                                        "value": telecom.value,
                                    })
                                })
                                .collect(),
                        ),
                    );
                }
                serde_json::Value::Object(value)
            })
            .collect(),
    );
}

fn resource_reference(resource: &serde_json::Value) -> Option<String> {
    let resource_type = resource.get("resourceType")?.as_str()?;
    let id = resource.get("id").and_then(|v| v.as_str())?;
    Some(format!("{resource_type}/{id}"))
}

/// Collect results, separating values from errors.
fn collect_results(
    results: Vec<Result<serde_json::Value, FshError>>,
    resources: &mut Vec<serde_json::Value>,
    errors: &mut Vec<FshError>,
) {
    for result in results {
        match result {
            Ok(v) => resources.push(v),
            Err(e) => errors.push(e),
        }
    }
}

/// Export a collection of entities in parallel, accumulating results and errors.
fn export_par<'a, T, F, I>(
    iter: I,
    f: F,
    resources: &mut Vec<serde_json::Value>,
    errors: &mut Vec<FshError>,
) where
    T: Sync + 'a,
    F: Fn(&T) -> Result<serde_json::Value, FshError> + Send + Sync,
    I: Iterator<Item = &'a T>,
{
    let items: Vec<&T> = iter.collect();
    let results: Vec<_> = items.into_par_iter().map(f).collect();
    collect_results(results, resources, errors);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerOptions, FshCompiler, FshConfig, FshContact, FshDependency, FshTelecom};

    #[test]
    fn emits_implementation_guide_from_project_config() {
        let mut package = FshExporter::export(
            &FshTank::new(),
            FhirDefs::r4(),
            &FshConfig {
                id: Some("example.fhir".to_string()),
                canonical: Some("http://example.org/fhir".to_string()),
                name: Some("ExampleIG".to_string()),
                status: Some("draft".to_string()),
                publisher: Some("Example Publisher".to_string()),
                contacts: vec![FshContact {
                    name: Some("Example Publisher".to_string()),
                    telecom: vec![FshTelecom {
                        system: "url".to_string(),
                        value: "http://example.org/publisher".to_string(),
                    }],
                }],
                version: Some("1.2.3".to_string()),
                fhir_version: Some("4.0.1".to_string()),
                dependencies: vec![FshDependency {
                    package_id: "hl7.fhir.us.core".to_string(),
                    version: "6.1.0".to_string(),
                    id: Some("uscore".to_string()),
                    uri: Some(
                        "http://hl7.org/fhir/us/core/ImplementationGuide/hl7.fhir.us.core"
                            .to_string(),
                    ),
                }],
                ..Default::default()
            },
        );

        let ig = package
            .resources
            .drain(..)
            .find(|resource| {
                resource.get("resourceType").and_then(|v| v.as_str()) == Some("ImplementationGuide")
            })
            .expect("ImplementationGuide exists");
        assert_eq!(ig["id"], "example.fhir");
        assert_eq!(
            ig["url"],
            "http://example.org/fhir/ImplementationGuide/example.fhir"
        );
        assert_eq!(ig["packageId"], "example.fhir");
        assert_eq!(ig["dependsOn"][0]["packageId"], "hl7.fhir.us.core");
        assert_eq!(ig["contact"][0]["telecom"][0]["system"], "url");
    }

    #[test]
    fn emits_implementation_guide_definition_metadata() {
        let config = crate::parse_sushi_config(
            r#"
id: example.fhir
canonical: http://example.org/fhir
pages:
  index.md:
    title: Home
    details.md:
      title: Details
groups:
  examples:
    name: Examples
    resources: [Patient/example]
parameters:
  shownav: true
resources:
  Patient/example:
    name: Example Patient
    description: A patient example.
    exampleCanonical: http://example.org/fhir/StructureDefinition/patient
"#,
        )
        .expect("config parses");
        let package = FshCompiler::new(CompilerOptions {
            config,
            ..Default::default()
        })
        .compile(
            r#"
Profile: ExampleProfile
Parent: Patient
Id: example-profile

Instance: profiled-example
InstanceOf: ExampleProfile
Title: "A Profiled Patient"
Description: "A patient conforming to the example profile."
Usage: #example

Instance: example
InstanceOf: Patient
Usage: #example
"#,
            "example.fsh",
        )
        .expect("FSH compiles");
        let ig = package
            .resources
            .iter()
            .find(|resource| resource["resourceType"] == "ImplementationGuide")
            .expect("ImplementationGuide exists");

        assert_eq!(ig["definition"]["page"]["page"][0]["nameUrl"], "index.html");
        assert_eq!(
            ig["definition"]["page"]["page"][0]["page"][0]["nameUrl"],
            "details.html"
        );
        assert_eq!(ig["definition"]["grouping"][0]["id"], "examples");
        assert_eq!(ig["definition"]["parameter"][0]["value"], "true");
        assert_eq!(
            ig["definition"]["resource"][0]["reference"]["reference"],
            "Patient/profiled-example"
        );
        assert_eq!(
            ig["definition"]["resource"][0]["description"],
            "A patient conforming to the example profile."
        );
        assert_eq!(
            ig["definition"]["resource"][0]["exampleCanonical"],
            "http://example.org/fhir/StructureDefinition/example-profile"
        );
        let patient = ig["definition"]["resource"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["reference"]["reference"] == "Patient/example")
            .unwrap();
        assert_eq!(patient["name"], "Example Patient");
        assert_eq!(patient["groupingId"], "examples");
        assert_eq!(patient["exampleBoolean"], true);
        assert_eq!(
            patient["exampleCanonical"],
            "http://example.org/fhir/StructureDefinition/patient"
        );
    }

    #[test]
    fn embeds_inline_instances_without_emitting_them_at_top_level() {
        let package = FshCompiler::new(CompilerOptions::default())
            .compile(
                r#"
Instance: example-bundle
InstanceOf: Bundle
Usage: #example
* type = #collection
* entry[+].resource = inline-patient

Instance: inline-patient
InstanceOf: Patient
Usage: #inline
* active = true
"#,
                "inline.fsh",
            )
            .expect("FSH compiles");

        assert_eq!(package.resources.len(), 1);
        assert_eq!(package.resources[0]["resourceType"], "Bundle");
        assert_eq!(
            package.resources[0]["entry"][0]["resource"]["resourceType"],
            "Patient"
        );
        assert_eq!(
            package.resources[0]["entry"][0]["resource"]["id"],
            "inline-patient"
        );
    }

    #[test]
    fn embeds_inline_datatypes_without_resource_identity_fields() {
        let package = FshCompiler::new(CompilerOptions::default())
            .compile(
                r#"
Instance: identifier-pattern
InstanceOf: Identifier
Usage: #inline
* system = "http://example.org/identifier"

Profile: IdentifiedOrganization
Parent: Organization
* identifier = identifier-pattern

Instance: organization-example
InstanceOf: IdentifiedOrganization
"#,
                "inline-datatype.fsh",
            )
            .expect("FSH compiles");

        let identifier = &package.resources[0]["identifier"][0];
        assert_eq!(identifier["system"], "http://example.org/identifier");
        assert!(identifier.get("resourceType").is_none());
        assert!(identifier.get("id").is_none());
    }
}
