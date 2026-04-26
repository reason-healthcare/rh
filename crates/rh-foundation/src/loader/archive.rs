use std::fs;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

use super::{LoaderError, LoaderResult, PackageDist};

pub(super) fn verify_tarball_checksum(
    _tarball_data: &[u8],
    _dist: &PackageDist,
) -> LoaderResult<()> {
    tracing::debug!("Checksum verification not yet implemented");
    Ok(())
}

pub(super) fn extract_tarball(tarball_data: &[u8], extract_to: &Path) -> LoaderResult<()> {
    tracing::debug!("Extracting tarball to: {}", extract_to.display());

    fs::create_dir_all(extract_to)?;

    let tar_decoder = GzDecoder::new(tarball_data);
    let mut archive = Archive::new(tar_decoder);

    archive
        .unpack(extract_to)
        .map_err(|e| LoaderError::ArchiveError {
            message: format!("Failed to extract tarball: {e}"),
        })?;

    Ok(())
}
