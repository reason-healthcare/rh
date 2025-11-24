use crate::snapshot::error::{SnapshotError, SnapshotResult};
use crate::snapshot::types::StructureDefinition;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

pub struct StructureDefinitionLoader;

impl StructureDefinitionLoader {
    pub fn load_from_file(path: &Path) -> SnapshotResult<StructureDefinition> {
        debug!("Loading StructureDefinition from file: {}", path.display());

        let content = fs::read_to_string(path).map_err(|e| {
            SnapshotError::Other(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        let sd: StructureDefinition =
            serde_json::from_str(&content).map_err(SnapshotError::SerializationError)?;

        Self::validate_structure_definition(&sd)?;

        info!("Loaded StructureDefinition: {} ({})", sd.name, sd.url);

        Ok(sd)
    }

    pub fn load_from_directory(dir: &Path) -> SnapshotResult<Vec<StructureDefinition>> {
        info!(
            "Loading StructureDefinitions from directory: {}",
            dir.display()
        );

        if !dir.exists() {
            return Err(SnapshotError::Other(format!(
                "Directory does not exist: {}",
                dir.display()
            )));
        }

        if !dir.is_dir() {
            return Err(SnapshotError::Other(format!(
                "Path is not a directory: {}",
                dir.display()
            )));
        }

        let mut structure_definitions = Vec::new();
        let entries = fs::read_dir(dir).map_err(|e| {
            SnapshotError::Other(format!("Failed to read directory {}: {}", dir.display(), e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                SnapshotError::Other(format!("Failed to read directory entry: {e}"))
            })?;

            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "json" {
                        match Self::try_load_structure_definition(&path) {
                            Ok(Some(sd)) => {
                                structure_definitions.push(sd);
                            }
                            Ok(None) => {
                                debug!("Skipping non-StructureDefinition file: {}", path.display());
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to load StructureDefinition from {}: {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        info!(
            "Loaded {} StructureDefinitions from {}",
            structure_definitions.len(),
            dir.display()
        );

        Ok(structure_definitions)
    }

    pub fn load_from_package(
        package_name: &str,
        version: &str,
        packages_dir: &Path,
    ) -> SnapshotResult<Vec<StructureDefinition>> {
        info!(
            "Loading StructureDefinitions from package {}@{}",
            package_name, version
        );

        let package_dir = Self::get_package_directory(packages_dir, package_name, version);

        if !package_dir.exists() {
            return Err(SnapshotError::Other(format!(
                "Package not found: {}@{} at {}",
                package_name,
                version,
                package_dir.display()
            )));
        }

        let package_subdir = package_dir.join("package");
        let dir_to_scan = if package_subdir.exists() && package_subdir.is_dir() {
            package_subdir
        } else {
            package_dir
        };

        Self::load_from_directory(&dir_to_scan)
    }

    fn try_load_structure_definition(path: &Path) -> SnapshotResult<Option<StructureDefinition>> {
        let content = fs::read_to_string(path).map_err(|e| {
            SnapshotError::Other(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        let json: serde_json::Value =
            serde_json::from_str(&content).map_err(SnapshotError::SerializationError)?;

        if json.get("resourceType")
            == Some(&serde_json::Value::String(
                "StructureDefinition".to_string(),
            ))
        {
            let sd: StructureDefinition =
                serde_json::from_value(json).map_err(SnapshotError::SerializationError)?;

            Self::validate_structure_definition(&sd)?;
            Ok(Some(sd))
        } else {
            Ok(None)
        }
    }

    fn validate_structure_definition(sd: &StructureDefinition) -> SnapshotResult<()> {
        if sd.url.is_empty() {
            return Err(SnapshotError::InvalidStructureDefinition(
                "StructureDefinition.url is required".to_string(),
            ));
        }

        if sd.name.is_empty() {
            return Err(SnapshotError::InvalidStructureDefinition(
                "StructureDefinition.name is required".to_string(),
            ));
        }

        if sd.type_.is_empty() {
            return Err(SnapshotError::InvalidStructureDefinition(
                "StructureDefinition.type is required".to_string(),
            ));
        }

        Ok(())
    }

    fn get_package_directory(packages_dir: &Path, package_name: &str, version: &str) -> PathBuf {
        packages_dir.join(format!("{package_name}#{version}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_structure_definition() -> String {
        r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/StructureDefinition/test-patient",
            "name": "TestPatient",
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "differential": {
                "element": [
                    {
                        "path": "Patient",
                        "min": 0,
                        "max": "*"
                    }
                ]
            }
        }"#
        .to_string()
    }

    fn create_invalid_structure_definition() -> String {
        r#"{
            "resourceType": "StructureDefinition",
            "url": "",
            "name": "TestPatient",
            "type": "Patient"
        }"#
        .to_string()
    }

    fn create_non_structure_definition() -> String {
        r#"{
            "resourceType": "Patient",
            "id": "example"
        }"#
        .to_string()
    }

    #[test]
    fn test_load_from_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(create_test_structure_definition().as_bytes())
            .unwrap();

        let result = StructureDefinitionLoader::load_from_file(&file_path);
        assert!(result.is_ok());

        let sd = result.unwrap();
        assert_eq!(sd.name, "TestPatient");
        assert_eq!(
            sd.url,
            "http://example.org/StructureDefinition/test-patient"
        );
        assert_eq!(sd.type_, "Patient");
    }

    #[test]
    fn test_load_from_file_missing_url() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(create_invalid_structure_definition().as_bytes())
            .unwrap();

        let result = StructureDefinitionLoader::load_from_file(&file_path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SnapshotError::InvalidStructureDefinition(_)
        ));
    }

    #[test]
    fn test_load_from_directory() {
        let temp_dir = TempDir::new().unwrap();

        let file1 = temp_dir.path().join("sd1.json");
        fs::write(&file1, create_test_structure_definition()).unwrap();

        let file2 = temp_dir.path().join("patient.json");
        fs::write(&file2, create_non_structure_definition()).unwrap();

        let file3 = temp_dir.path().join("readme.txt");
        fs::write(&file3, "not json").unwrap();

        let result = StructureDefinitionLoader::load_from_directory(temp_dir.path());
        assert!(result.is_ok());

        let sds = result.unwrap();
        assert_eq!(sds.len(), 1);
        assert_eq!(sds[0].name, "TestPatient");
    }

    #[test]
    fn test_validate_structure_definition() {
        let valid_sd = StructureDefinition {
            url: "http://example.org/test".to_string(),
            name: "Test".to_string(),
            type_: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: None,
        };

        assert!(StructureDefinitionLoader::validate_structure_definition(&valid_sd).is_ok());

        let invalid_sd = StructureDefinition {
            url: "".to_string(),
            name: "Test".to_string(),
            type_: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: None,
        };

        assert!(StructureDefinitionLoader::validate_structure_definition(&invalid_sd).is_err());
    }
}
