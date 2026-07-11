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
            |p| structure_def::export_profile(p, defs.clone(), config, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.extensions.values(),
            |e| structure_def::export_extension(e, defs.clone(), config, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.logicals.values(),
            |l| structure_def::export_logical(l, defs.clone(), config, &definition_index),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.resources.values(),
            |r| structure_def::export_resource_def(r, defs.clone(), config, &definition_index),
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
            |vs| value_set::export_value_set(vs, config),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.code_systems.values(),
            |cs| code_system::export_code_system(cs, config),
            &mut resources,
            &mut errors,
        );
        if let Some(implementation_guide) = export_implementation_guide(config, &resources) {
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
) -> Option<serde_json::Value> {
    let id = config.id.as_ref()?;
    let canonical = config.canonical.as_ref()?;
    let self_reference = format!("ImplementationGuide/{id}");
    let mut definition_resources: Vec<serde_json::Value> = resources
        .iter()
        .filter_map(resource_reference)
        .filter(|reference| reference != &self_reference)
        .map(|reference| {
            serde_json::json!({
                "reference": {
                    "reference": reference,
                },
            })
        })
        .collect();
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
