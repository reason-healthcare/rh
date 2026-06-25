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

        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        if !path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("StructureDefinition"))
        {
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
            });
    }

    Ok(())
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
              "derivation": "constraint"
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
