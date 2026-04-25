use std::fs;
use std::path::{Path, PathBuf};

use crate::snapshot::error::{SnapshotError, SnapshotResult};
use crate::snapshot::types::StructureDefinition;

pub(super) fn read_file(path: &Path) -> SnapshotResult<String> {
    fs::read_to_string(path).map_err(|error| {
        SnapshotError::Other(format!("Failed to read file {}: {}", path.display(), error))
    })
}

pub(super) fn ensure_directory(dir: &Path) -> SnapshotResult<()> {
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

    Ok(())
}

pub(super) fn json_files_in_directory(dir: &Path) -> SnapshotResult<Vec<PathBuf>> {
    let entries = fs::read_dir(dir).map_err(|error| {
        SnapshotError::Other(format!(
            "Failed to read directory {}: {}",
            dir.display(),
            error
        ))
    })?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            SnapshotError::Other(format!("Failed to read directory entry: {error}"))
        })?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }

    Ok(files)
}

pub(super) fn try_parse_structure_definition(
    content: &str,
) -> SnapshotResult<Option<StructureDefinition>> {
    let json: serde_json::Value =
        serde_json::from_str(content).map_err(SnapshotError::SerializationError)?;

    if json.get("resourceType")
        != Some(&serde_json::Value::String(
            "StructureDefinition".to_string(),
        ))
    {
        return Ok(None);
    }

    let structure_definition =
        serde_json::from_value(json).map_err(SnapshotError::SerializationError)?;
    validate_structure_definition(&structure_definition)?;
    Ok(Some(structure_definition))
}

pub(super) fn validate_structure_definition(sd: &StructureDefinition) -> SnapshotResult<()> {
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

pub(super) fn package_directory(packages_dir: &Path, package_name: &str, version: &str) -> PathBuf {
    packages_dir.join(format!("{package_name}#{version}"))
}
