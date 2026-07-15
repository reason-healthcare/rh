//! Loading of FHIR package dependency definitions declared in `sushi-config.yaml`.

use crate::{FshConfig, FshError};
use rh_foundation::loader::PackageLoader;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// StructureDefinition metadata loaded from a dependency package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyStructureDefinition {
    pub package_id: String,
    pub version: String,
    pub path: PathBuf,
    pub id: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub kind: Option<String>,
    pub type_: Option<String>,
    pub base_definition: Option<String>,
    pub derivation: Option<String>,
    pub fixed_values: Vec<DependencyFixedValue>,
    pub extension_slices: Vec<DependencyExtensionSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyFixedValue {
    pub path: String,
    pub slice_name: Option<String>,
    pub value: serde_json::Value,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// One child slice declared by a dependency-backed Extension definition.
pub struct DependencyExtensionSlice {
    pub name: String,
    pub url: String,
    pub min: u32,
}

/// Dependency definitions available to the compiler.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DependencyDefinitionSet {
    pub structure_definitions: Vec<DependencyStructureDefinition>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawStructureDefinition {
    id: Option<String>,
    url: Option<String>,
    name: Option<String>,
    title: Option<String>,
    kind: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    base_definition: Option<String>,
    derivation: Option<String>,
    differential: Option<RawDifferential>,
}

#[derive(Debug, Deserialize)]
struct RawDifferential {
    #[serde(default)]
    element: Vec<RawElementDefinition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawElementDefinition {
    id: Option<String>,
    path: String,
    slice_name: Option<String>,
    min: Option<u32>,
    #[serde(flatten)]
    properties: serde_json::Map<String, serde_json::Value>,
}

/// Load StructureDefinition metadata for dependency packages from the default FHIR cache.
///
/// Missing packages are reported as warnings rather than hard errors so callers can continue
/// compiling local FSH while surfacing dependency-cache diagnostics separately.
pub fn load_dependency_structure_definitions(
    config: &FshConfig,
) -> Result<DependencyDefinitionSet, FshError> {
    let packages_dir = PackageLoader::get_default_packages_dir().map_err(|e| FshError::Export {
        message: format!("failed to locate default FHIR package cache: {e}"),
    })?;
    load_dependency_structure_definitions_from_dir(config, &packages_dir)
}

/// Load StructureDefinition metadata for dependency packages from a specific cache root.
pub fn load_dependency_structure_definitions_from_dir(
    config: &FshConfig,
    packages_dir: &Path,
) -> Result<DependencyDefinitionSet, FshError> {
    let mut definitions = DependencyDefinitionSet::default();

    let core_package = packages_dir.join("hl7.fhir.r4.core#4.0.1/package");
    if core_package.is_dir() {
        // Core extensions are valid `contains` targets even when they are not
        // declared in sushi-config.yaml. Index their metadata so a named slice
        // resolves to its canonical URL rather than its local alias.
        load_package_structure_definitions(
            "hl7.fhir.r4.core",
            "4.0.1",
            &core_package,
            &mut definitions,
            true,
        )?;
        for name in ["vitalsigns", "bodyheight", "bodyweight", "bodytemp"] {
            let path = core_package.join(format!("StructureDefinition-{name}.json"));
            if path.is_file() {
                load_structure_definition_file(
                    "hl7.fhir.r4.core",
                    "4.0.1",
                    path,
                    &mut definitions,
                    false,
                )?;
            }
        }
    }

    for dependency in &config.dependencies {
        let package_dir = packages_dir
            .join(format!("{}#{}", dependency.package_id, dependency.version))
            .join("package");
        if !package_dir.is_dir() {
            definitions.warnings.push(format!(
                "dependency package {}#{} was not found in {}",
                dependency.package_id,
                dependency.version,
                packages_dir.display()
            ));
            continue;
        }

        load_package_structure_definitions(
            &dependency.package_id,
            &dependency.version,
            &package_dir,
            &mut definitions,
            false,
        )?;
    }

    definitions.structure_definitions.sort_by(|a, b| {
        // The built-in core package is a fallback. A declared IG dependency can
        // legitimately reuse a core definition name (for example, mCODE's
        // `GenomicVariant` Observation profile versus the core extension of
        // that name), so it must be indexed first.
        (
            a.package_id == "hl7.fhir.r4.core",
            &a.package_id,
            &a.version,
            &a.url,
            &a.id,
        )
            .cmp(&(
                b.package_id == "hl7.fhir.r4.core",
                &b.package_id,
                &b.version,
                &b.url,
                &b.id,
            ))
    });

    Ok(definitions)
}

fn load_package_structure_definitions(
    package_id: &str,
    version: &str,
    dir: &Path,
    definitions: &mut DependencyDefinitionSet,
    extensions_only: bool,
) -> Result<(), FshError> {
    for entry in fs::read_dir(dir).map_err(|e| FshError::Export {
        message: format!("failed to read dependency package {}: {e}", dir.display()),
    })? {
        let entry = entry.map_err(|e| FshError::Export {
            message: format!(
                "failed to read dependency package entry in {}: {e}",
                dir.display()
            ),
        })?;
        let path = entry.path();

        if path.is_dir() {
            load_package_structure_definitions(
                package_id,
                version,
                &path,
                definitions,
                extensions_only,
            )?;
            continue;
        }

        if !is_structure_definition_json_file(&path) {
            continue;
        }

        load_structure_definition_file(package_id, version, path, definitions, extensions_only)?;
    }

    Ok(())
}

fn load_structure_definition_file(
    package_id: &str,
    version: &str,
    path: PathBuf,
    definitions: &mut DependencyDefinitionSet,
    extensions_only: bool,
) -> Result<(), FshError> {
    let text = fs::read_to_string(&path).map_err(|e| FshError::Export {
        message: format!("failed to read dependency resource {}: {e}", path.display()),
    })?;
    let resource: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| FshError::Export {
            message: format!(
                "failed to parse dependency resource {}: {e}",
                path.display()
            ),
        })?;

    if resource.get("resourceType").and_then(|v| v.as_str()) != Some("StructureDefinition") {
        return Ok(());
    }
    let raw: RawStructureDefinition =
        serde_json::from_value(resource).map_err(|e| FshError::Export {
            message: format!(
                "failed to parse dependency StructureDefinition {}: {e}",
                path.display()
            ),
        })?;

    if extensions_only && raw.type_.as_deref() != Some("Extension") {
        return Ok(());
    }

    let differential_elements = raw
        .differential
        .map(|differential| differential.element)
        .unwrap_or_default();
    let extension_slices = extract_extension_slices(&differential_elements);
    let fixed_values = {
        let required_paths = differential_elements
            .iter()
            .filter(|element| element.min.is_some_and(|min| min > 0))
            .filter_map(|element| element.id.clone())
            .collect::<std::collections::HashSet<_>>();
        differential_elements
            .into_iter()
            .filter_map(|element| {
                element
                    .properties
                    .into_iter()
                    .find(|(key, _)| key.starts_with("fixed") || key.starts_with("pattern"))
                    .map(|(_, value)| {
                        let path = element.id.unwrap_or(element.path);
                        let parts = path.split('.').collect::<Vec<_>>();
                        let required_path = parts
                            .iter()
                            .position(|part| part.contains(':'))
                            .map(|index| parts[..=index].join("."))
                            .or_else(|| (parts.len() > 2).then(|| parts[..2].join(".")));
                        DependencyFixedValue {
                            path,
                            slice_name: element.slice_name,
                            value,
                            required: required_path
                                .is_none_or(|path| required_paths.contains(&path)),
                        }
                    })
            })
            .collect()
    };

    definitions
        .structure_definitions
        .push(DependencyStructureDefinition {
            package_id: package_id.to_string(),
            version: version.to_string(),
            path,
            id: raw.id,
            url: raw.url,
            name: raw.name,
            title: raw.title,
            kind: raw.kind,
            type_: raw.type_,
            base_definition: raw.base_definition,
            derivation: raw.derivation,
            fixed_values,
            extension_slices,
        });

    Ok(())
}

fn extract_extension_slices(elements: &[RawElementDefinition]) -> Vec<DependencyExtensionSlice> {
    elements
        .iter()
        .filter(|element| element.path == "Extension.extension")
        .filter_map(|element| {
            let name = element.slice_name.clone()?;
            let profile = element
                .properties
                .get("type")
                .and_then(serde_json::Value::as_array)
                .and_then(|types| {
                    types.iter().find_map(|type_| {
                        type_
                            .get("profile")
                            .and_then(serde_json::Value::as_array)
                            .and_then(|profiles| profiles.first())
                            .and_then(serde_json::Value::as_str)
                    })
                });
            let fixed_url = element.id.as_deref().and_then(|id| {
                let url_id = format!("{id}.url");
                elements
                    .iter()
                    .find(|candidate| candidate.id.as_deref() == Some(url_id.as_str()))
                    .and_then(|candidate| {
                        candidate
                            .properties
                            .iter()
                            .find(|(key, value)| {
                                (key.starts_with("fixed") || key.starts_with("pattern"))
                                    && value.is_string()
                            })
                            .and_then(|(_, value)| value.as_str())
                    })
            });
            Some(DependencyExtensionSlice {
                url: profile.or(fixed_url).unwrap_or(&name).to_string(),
                name,
                min: element.min.unwrap_or(0),
            })
        })
        .collect()
}

fn is_structure_definition_json_file(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("json")
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("StructureDefinition"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FshDependency;

    #[test]
    fn extracts_dependency_extension_slice_urls_and_cardinality() {
        let elements = serde_json::from_value::<Vec<RawElementDefinition>>(serde_json::json!([
            {
                "id": "Extension.extension:provider",
                "path": "Extension.extension",
                "sliceName": "provider",
                "min": 0
            },
            {
                "id": "Extension.extension:provider.url",
                "path": "Extension.extension.url",
                "fixedUri": "provider"
            },
            {
                "id": "Extension.extension:providerType",
                "path": "Extension.extension",
                "sliceName": "providerType",
                "min": 1,
                "type": [{
                    "code": "Extension",
                    "profile": ["http://example.org/StructureDefinition/provider-type"]
                }]
            }
        ]))
        .expect("elements deserialize");

        let slices = extract_extension_slices(&elements);

        assert_eq!(slices.len(), 2);
        assert_eq!(slices[0].url, "provider");
        assert_eq!(slices[0].min, 0);
        assert_eq!(
            slices[1].url,
            "http://example.org/StructureDefinition/provider-type"
        );
        assert_eq!(slices[1].min, 1);
    }

    fn temp_package_root(test_name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "rh_fsh_dependency_test_{}_{}",
            test_name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("temp package root");
        path
    }

    #[test]
    fn loads_structure_definitions_from_dependency_package_cache() {
        let packages_dir = temp_package_root("loads");
        let package_dir = packages_dir.join("hl7.fhir.us.core#6.1.0/package");
        fs::create_dir_all(&package_dir).expect("package dir");
        fs::write(
            package_dir.join("StructureDefinition-us-core-patient.json"),
            r#"{
              "resourceType": "StructureDefinition",
              "id": "us-core-patient",
              "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
              "name": "USCorePatientProfile",
              "title": "US Core Patient Profile",
              "kind": "resource",
              "type": "Patient",
              "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
              "derivation": "constraint",
              "differential": {
                "element": [
                  {
                    "id": "Patient.communication:required",
                    "path": "Patient.communication",
                    "sliceName": "required",
                    "min": 1
                  },
                  {
                    "id": "Patient.communication:required.language",
                    "path": "Patient.communication.language",
                    "patternCodeableConcept": {
                      "coding": [{"system": "urn:ietf:bcp:47", "code": "en"}]
                    }
                  }
                ]
              }
            }"#,
        )
        .expect("write structure definition");
        fs::write(
            package_dir.join("ValueSet-not-a-structure-definition.json"),
            r#"{"resourceType":"ValueSet","id":"ignored"}"#,
        )
        .expect("write non-SD resource");
        fs::write(
            package_dir.join("CapabilityStatement-array-type.json"),
            r#"{"resourceType":"CapabilityStatement","id":"ignored","type":["server"]}"#,
        )
        .expect("write non-SD resource with array type");

        let config = FshConfig {
            dependencies: vec![FshDependency {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "6.1.0".to_string(),
                id: None,
                uri: None,
            }],
            ..Default::default()
        };

        let definitions =
            load_dependency_structure_definitions_from_dir(&config, &packages_dir).unwrap();

        assert!(definitions.warnings.is_empty());
        assert_eq!(definitions.structure_definitions.len(), 1);
        let definition = &definitions.structure_definitions[0];
        assert_eq!(definition.id.as_deref(), Some("us-core-patient"));
        assert_eq!(definition.name.as_deref(), Some("USCorePatientProfile"));
        assert_eq!(definition.type_.as_deref(), Some("Patient"));
        assert_eq!(
            definition.base_definition.as_deref(),
            Some("http://hl7.org/fhir/StructureDefinition/Patient")
        );
        assert_eq!(definition.fixed_values.len(), 1);
        assert_eq!(
            definition.fixed_values[0].path,
            "Patient.communication:required.language"
        );
        assert_eq!(definition.fixed_values[0].value["coding"][0]["code"], "en");

        fs::remove_dir_all(packages_dir).ok();
    }

    #[test]
    fn loads_core_extension_definitions_for_slice_url_resolution() {
        let packages_dir = temp_package_root("core-extension");
        let core_dir = packages_dir.join("hl7.fhir.r4.core#4.0.1/package");
        fs::create_dir_all(&core_dir).expect("core package dir");
        fs::write(
            core_dir.join("StructureDefinition-condition-related.json"),
            r#"{
              "resourceType": "StructureDefinition",
              "id": "condition-related",
              "url": "http://hl7.org/fhir/StructureDefinition/condition-related",
              "name": "ConditionRelated",
              "kind": "complex-type",
              "type": "Extension",
              "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Extension",
              "derivation": "constraint"
            }"#,
        )
        .expect("write core extension");

        let definitions =
            load_dependency_structure_definitions_from_dir(&FshConfig::default(), &packages_dir)
                .expect("core package loads");

        assert_eq!(definitions.warnings, Vec::<String>::new());
        assert_eq!(definitions.structure_definitions.len(), 1);
        let definition = &definitions.structure_definitions[0];
        assert_eq!(definition.package_id, "hl7.fhir.r4.core");
        assert_eq!(definition.id.as_deref(), Some("condition-related"));
        assert_eq!(
            definition.url.as_deref(),
            Some("http://hl7.org/fhir/StructureDefinition/condition-related")
        );

        fs::remove_dir_all(packages_dir).ok();
    }

    #[test]
    fn indexes_declared_dependency_definitions_before_core_name_collisions() {
        let packages_dir = temp_package_root("core-collision");
        let core_dir = packages_dir.join("hl7.fhir.r4.core#4.0.1/package");
        let dependency_dir = packages_dir.join("example.ig#1.0.0/package");
        fs::create_dir_all(&core_dir).expect("core package dir");
        fs::create_dir_all(&dependency_dir).expect("dependency package dir");
        fs::write(
            core_dir.join("StructureDefinition-observation-geneticsVariant.json"),
            r#"{
              "resourceType": "StructureDefinition",
              "id": "observation-geneticsVariant",
              "url": "http://hl7.org/fhir/StructureDefinition/observation-geneticsVariant",
              "name": "GenomicVariant",
              "kind": "complex-type",
              "type": "Extension",
              "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Extension",
              "derivation": "constraint"
            }"#,
        )
        .expect("write core extension");
        fs::write(
            dependency_dir.join("StructureDefinition-example-genomic-variant.json"),
            r#"{
              "resourceType": "StructureDefinition",
              "id": "example-genomic-variant",
              "url": "http://example.org/StructureDefinition/example-genomic-variant",
              "name": "GenomicVariant",
              "kind": "resource",
              "type": "Observation",
              "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Observation",
              "derivation": "constraint"
            }"#,
        )
        .expect("write dependency profile");
        let config = FshConfig {
            dependencies: vec![FshDependency {
                package_id: "example.ig".to_string(),
                version: "1.0.0".to_string(),
                id: None,
                uri: None,
            }],
            ..Default::default()
        };

        let definitions = load_dependency_structure_definitions_from_dir(&config, &packages_dir)
            .expect("packages load");

        assert_eq!(definitions.structure_definitions.len(), 2);
        assert_eq!(
            definitions.structure_definitions[0].package_id,
            "example.ig"
        );
        assert_eq!(
            definitions.structure_definitions[1].package_id,
            "hl7.fhir.r4.core"
        );

        fs::remove_dir_all(packages_dir).ok();
    }

    #[test]
    fn records_warning_for_missing_dependency_package() {
        let packages_dir = temp_package_root("missing");
        let config = FshConfig {
            dependencies: vec![FshDependency {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "6.1.0".to_string(),
                id: None,
                uri: None,
            }],
            ..Default::default()
        };

        let definitions =
            load_dependency_structure_definitions_from_dir(&config, &packages_dir).unwrap();

        assert!(definitions.structure_definitions.is_empty());
        assert_eq!(definitions.warnings.len(), 1);
        assert!(definitions.warnings[0].contains("hl7.fhir.us.core#6.1.0"));

        fs::remove_dir_all(packages_dir).ok();
    }
}
