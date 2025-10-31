use anyhow::{Context, Result};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

const FHIR_TEST_CASES_URL: &str =
    "https://github.com/FHIR/fhir-test-cases/archive/refs/heads/master.zip";
const VERSION_FILE: &str = "VERSION";
const EXPECTED_VERSION: &str = "master-snapshot";

#[cfg(feature = "fhir-test-cases")]
pub fn ensure_test_cases() -> Result<PathBuf> {
    let cache_dir = get_cache_dir()?;

    if !is_cache_valid(&cache_dir)? {
        println!("Downloading FHIR test cases from GitHub...");
        println!("URL: {FHIR_TEST_CASES_URL}");
        println!("Cache: {}", cache_dir.display());

        download_and_extract(&cache_dir)
            .context("Failed to download and extract FHIR test cases")?;

        mark_cache_version(&cache_dir)?;

        println!("✓ FHIR test cases downloaded and cached successfully");
    } else {
        println!("Using cached FHIR test cases from {}", cache_dir.display());
    }

    Ok(cache_dir)
}

fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("target"))
        .join("rh")
        .join("fhir-test-cases");

    Ok(cache_dir)
}

fn is_cache_valid(cache_dir: &Path) -> Result<bool> {
    if !cache_dir.exists() {
        return Ok(false);
    }

    let version_file = cache_dir.join(VERSION_FILE);
    if !version_file.exists() {
        return Ok(false);
    }

    let cached_version = fs::read_to_string(&version_file)?;
    Ok(cached_version.trim() == EXPECTED_VERSION)
}

fn mark_cache_version(cache_dir: &Path) -> Result<()> {
    let version_file = cache_dir.join(VERSION_FILE);
    fs::write(version_file, EXPECTED_VERSION)?;
    Ok(())
}

#[cfg(feature = "fhir-test-cases")]
fn download_and_extract(cache_dir: &Path) -> Result<()> {
    use reqwest::blocking::Client;
    use zip::ZipArchive;

    if cache_dir.exists() {
        fs::remove_dir_all(cache_dir).context("Failed to remove existing cache directory")?;
    }
    fs::create_dir_all(cache_dir).context("Failed to create cache directory")?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    println!("  Fetching ZIP archive...");
    let response = client
        .get(FHIR_TEST_CASES_URL)
        .send()
        .context("Failed to send HTTP request")?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    let bytes = response.bytes().context("Failed to read response bytes")?;

    println!("  Downloaded {} bytes", bytes.len());
    println!("  Extracting archive...");

    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).context("Failed to open ZIP archive")?;

    let mut extracted_files = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file
            .enclosed_name()
            .context("Invalid path in ZIP archive")?;

        let outpath = if let Ok(stripped) = outpath.strip_prefix("fhir-test-cases-master") {
            cache_dir.join(stripped)
        } else {
            cache_dir.join(outpath)
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
            extracted_files += 1;
        }
    }

    println!("  Extracted {extracted_files} files");

    Ok(())
}

pub fn find_r4_test_cases(cache_dir: &Path) -> Result<PathBuf> {
    let r4_dir = cache_dir.join("r4");

    if !r4_dir.exists() {
        anyhow::bail!(
            "R4 test cases directory not found at {}. Cache may be corrupted.",
            r4_dir.display()
        );
    }

    Ok(r4_dir)
}

#[cfg(not(feature = "fhir-test-cases"))]
pub fn ensure_test_cases() -> Result<PathBuf> {
    anyhow::bail!(
        "FHIR test cases feature is not enabled. \
         Run tests with: cargo test --features fhir-test-cases"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_cache_dir_creation() {
        let cache_dir = get_cache_dir().unwrap();
        assert!(cache_dir.to_string_lossy().contains("fhir-test-cases"));
    }

    #[test]
    #[cfg(feature = "fhir-test-cases")]
    fn test_ensure_test_cases() {
        let result = ensure_test_cases();
        assert!(
            result.is_ok(),
            "Failed to download test cases: {:?}",
            result.err()
        );

        let cache_dir = result.unwrap();
        assert!(cache_dir.exists());

        let r4_dir = find_r4_test_cases(&cache_dir);
        assert!(r4_dir.is_ok(), "R4 directory not found");
    }
}
