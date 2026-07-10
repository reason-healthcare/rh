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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyFixedValue {
    pub path: String,
    pub slice_name: Option<String>,
    pub value: serde_json::Value,
    pub required: bool,
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
        )?;
    }

    definitions.structure_definitions.sort_by(|a, b| {
        (&a.package_id, &a.version, &a.url, &a.id).cmp(&(&b.package_id, &b.version, &b.url, &b.id))
    });

    Ok(definitions)
}

fn load_package_structure_definitions(
    package_id: &str,
    version: &str,
    dir: &Path,
    definitions: &mut DependencyDefinitionSet,
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
            load_package_structure_definitions(package_id, version, &path, definitions)?;
            continue;
        }

        if !is_structure_definition_json_file(&path) {
            continue;
        }

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
            continue;
        }
        let raw: RawStructureDefinition =
            serde_json::from_value(resource).map_err(|e| FshError::Export {
                message: format!(
                    "failed to parse dependency StructureDefinition {}: {e}",
                    path.display()
                ),
            })?;

        let fixed_values = raw
            .differential
            .map(|differential| {
                let required_paths = differential
                    .element
                    .iter()
                    .filter(|element| element.min.is_some_and(|min| min > 0))
                    .filter_map(|element| element.id.clone())
                    .collect::<std::collections::HashSet<_>>();
                differential
                    .element
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
            })
            .unwrap_or_default();

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
            });
    }

    Ok(())
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
