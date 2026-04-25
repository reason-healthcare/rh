use std::env;
use std::path::{Path, PathBuf};

use crate::FoundationError;

use super::{LoaderError, LoaderResult};

pub(super) fn default_packages_dir() -> LoaderResult<PathBuf> {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| {
            LoaderError::Foundation(FoundationError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            )))
        })?;

    Ok(PathBuf::from(home_dir).join(".fhir").join("packages"))
}

pub(super) fn package_directory(base_path: &Path, package_name: &str, version: &str) -> PathBuf {
    base_path.join(format!("{package_name}#{version}"))
}
