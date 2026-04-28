//! High-level pipeline orchestration for `rh publish` subcommands.

use crate::{
    context::PublishContext,
    hooks::{build_registry, run_stage},
    ig_sync::check_ig_sync,
    loader::load_source_dir,
    lock::{apply_pinning, generate_lock, load_lock},
    narrative::process_narrative,
    pack::{create_tarball, write_output_dir},
    utils::resolve_packages_dir,
    Result,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::{info, warn};

/// Run the full `rh publish build` pipeline.
///
/// Steps:
/// 1. Load source directory
/// 2. Validate IG ↔ package.json sync
/// 3. Run `before_build` hooks
/// 4. Process narrative (embed markdown into resource `.text`)
/// 5. Apply canonical pinning from `fhir-lock.json` (if present)
/// 6. Run `after_build` hooks
/// 7. Write output directory
/// 8. Run `before_pack` hooks
/// 9. Create tarball
/// 10. Run `after_pack` hooks
///
/// # Examples
///
/// ```no_run
/// use rh_packager::build;
/// use std::path::Path;
///
/// let tgz = build(Path::new("my-package"), Path::new("output")).unwrap();
/// println!("Package written to {}", tgz.display());
/// ```
pub fn build(source_dir: &Path, output_dir: &Path) -> Result<PathBuf> {
    let mut ctx = load_source_dir(source_dir, output_dir.to_path_buf())?;
    check_ig_sync(&ctx)?;

    let registry = build_registry();

    let before = ctx.config.hooks.before_build.clone();
    run_stage(&registry, &before, &mut ctx)?;

    process_narrative(&mut ctx)?;

    match load_lock(&ctx)? {
        Some(lock) => {
            let pin_map: HashMap<String, String> = lock
                .canonicals
                .iter()
                .map(|c| (c.url.clone(), c.resolved_version.clone()))
                .collect();
            apply_pinning(&mut ctx, &pin_map);
        }
        None => {
            warn!(
                "No fhir-lock.json found; canonical references will not be pinned. \
                 Run `rh publish lock` to generate a lock file."
            );
        }
    }

    let after = ctx.config.hooks.after_build.clone();
    run_stage(&registry, &after, &mut ctx)?;

    let pkg_dir = write_output_dir(&ctx)?;

    let before_pack = ctx.config.hooks.before_pack.clone();
    run_stage(&registry, &before_pack, &mut ctx)?;

    let tgz = create_tarball(&ctx, &pkg_dir, None)?;
    info!("Package written to {}", tgz.display());

    let after_pack = ctx.config.hooks.after_pack.clone();
    run_stage(&registry, &after_pack, &mut ctx)?;

    Ok(tgz)
}

/// Run `rh publish lock` — scan source resources and write/update `fhir-lock.json`.
///
/// Reads dependency packages from the configured packages cache and resolves all
/// canonical URLs found in source resources against those packages.
///
/// # Examples
///
/// ```no_run
/// use rh_packager::lock_package;
/// use std::path::Path;
///
/// lock_package(Path::new("my-package"), Path::new("output")).unwrap();
/// ```
pub fn lock(source_dir: &Path, output_dir: &Path) -> Result<()> {
    let ctx = load_source_dir(source_dir, output_dir.to_path_buf())?;
    let packages_dir = resolve_packages_dir(
        ctx.config.validate.packages_dir.as_deref(),
        ctx.config.packages_dir.as_deref(),
    );
    generate_lock(&ctx, &packages_dir)?;
    let lock_path = source_dir.join("fhir-lock.json");
    info!("fhir-lock.json written to {}", lock_path.display());
    Ok(())
}

/// Run `rh publish check` — validate source without writing any output.
///
/// Runs IG sync validation and all `before_build` hook processors. Succeeds only if
/// all checks pass. No files are written.
///
/// # Examples
///
/// ```no_run
/// use rh_packager::check;
/// use std::path::Path;
///
/// check(Path::new("my-package")).unwrap();
/// ```
pub fn check(source_dir: &Path) -> Result<()> {
    let output_dir = source_dir.join("output");
    let mut ctx = load_source_dir(source_dir, output_dir)?;
    check_ig_sync(&ctx)?;

    let registry = build_registry();
    let before = ctx.config.hooks.before_build.clone();
    run_stage(&registry, &before, &mut ctx)?;

    info!("Source directory check passed");
    Ok(())
}

/// Run `rh publish pack` — pack an already-built expanded output directory into a tarball.
///
/// Reads `package.json` from the given output directory to determine the package name and
/// version for the tarball filename.
///
/// **Note:** This function does not run `before_pack`/`after_pack` lifecycle hooks because
/// it operates on a pre-built output directory without access to the source `publisher.toml`.
/// Hook support for `before_pack`/`after_pack` is provided by the full [`build`] pipeline.
///
/// # Examples
///
/// ```no_run
/// use rh_packager::pack_dir;
/// use std::path::Path;
///
/// let tgz = pack_dir(Path::new("output")).unwrap();
/// println!("Tarball written to {}", tgz.display());
/// ```
pub fn pack_dir(output_dir: &Path) -> Result<PathBuf> {
    let pkg_dir = output_dir.join("package");
    let pkg_json_path = pkg_dir.join("package.json");
    let raw = std::fs::read_to_string(&pkg_json_path)?;
    let package_json: crate::manifest::PackageJson = serde_json::from_str(&raw)?;

    let ctx = PublishContext {
        source_dir: output_dir.to_path_buf(),
        output_dir: output_dir.to_path_buf(),
        package_json,
        resources: Default::default(),
        config: Default::default(),
        standalone_markdown: Vec::new(),
    };

    let tgz = create_tarball(&ctx, &pkg_dir, None)?;
    info!("Tarball written to {}", tgz.display());
    Ok(tgz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PublisherError;
    use std::fs;
    use tempfile::TempDir;

    fn write_file(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).unwrap();
    }

    fn setup_minimal_package(dir: &Path) {
        write_file(
            dir,
            "package.json",
            r#"{"name":"test.fhir.pkg","version":"1.0.0","fhirVersions":["4.0.1"]}"#,
        );
        write_file(
            dir,
            "ImplementationGuide.json",
            r#"{"resourceType":"ImplementationGuide","id":"test-ig","packageId":"test.fhir.pkg","version":"1.0.0","url":"http://example.org/fhir","fhirVersion":["4.0.1"],"status":"draft"}"#,
        );
    }

    #[test]
    fn build_produces_tarball() {
        let tmp = TempDir::new().unwrap();
        setup_minimal_package(tmp.path());
        let out = tmp.path().join("out");

        let tgz = build(tmp.path(), &out).unwrap();

        assert!(tgz.exists());
        assert!(tgz.to_str().unwrap().ends_with(".tgz"));
    }

    #[test]
    fn check_does_not_write_output() {
        let tmp = TempDir::new().unwrap();
        setup_minimal_package(tmp.path());

        check(tmp.path()).unwrap();

        // check() must not create any output directory.
        assert!(!tmp.path().join("output").exists());
    }

    #[test]
    fn check_fails_on_ig_sync_mismatch() {
        let tmp = TempDir::new().unwrap();
        write_file(
            tmp.path(),
            "package.json",
            r#"{"name":"test.fhir.pkg","version":"2.0.0","fhirVersions":["4.0.1"]}"#,
        );
        write_file(
            tmp.path(),
            "ImplementationGuide.json",
            r#"{"resourceType":"ImplementationGuide","id":"test-ig","packageId":"test.fhir.pkg","version":"1.0.0","url":"http://example.org/fhir","fhirVersion":["4.0.1"],"status":"draft"}"#,
        );

        let err = check(tmp.path()).unwrap_err();
        assert!(
            matches!(err, PublisherError::IgSync(_)),
            "Expected IgSync error, got: {err:?}"
        );
    }

    #[test]
    fn lock_writes_fhir_lock_json() {
        let tmp = TempDir::new().unwrap();
        setup_minimal_package(tmp.path());
        let out = tmp.path().join("out");

        lock(tmp.path(), &out).unwrap();

        let lock_path = tmp.path().join("fhir-lock.json");
        assert!(lock_path.exists());

        let content = fs::read_to_string(&lock_path).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(value["pinMode"], "pin-all");
        assert!(value["generated"].is_string());
    }

    #[test]
    fn pack_dir_creates_tarball_from_existing_output() {
        let tmp = TempDir::new().unwrap();
        setup_minimal_package(tmp.path());
        let out = tmp.path().join("out");

        // Build first to produce the expanded output.
        let _ = build(tmp.path(), &out).unwrap();

        // Remove the tarball so we can re-pack.
        for entry in fs::read_dir(&out).unwrap() {
            let entry = entry.unwrap();
            if entry.path().extension().is_some_and(|e| e == "tgz") {
                fs::remove_file(entry.path()).unwrap();
            }
        }

        let tgz = pack_dir(&out).unwrap();
        assert!(tgz.exists());
        assert!(tgz.to_str().unwrap().ends_with(".tgz"));
    }

    #[test]
    fn check_fails_when_unknown_hook_processor_configured() {
        let tmp = TempDir::new().unwrap();
        setup_minimal_package(tmp.path());
        write_file(
            tmp.path(),
            "publisher.toml",
            r#"[hooks]
before_build = ["nonexistent-processor"]"#,
        );

        let err = check(tmp.path()).unwrap_err();
        assert!(
            matches!(err, PublisherError::UnknownProcessor(_)),
            "Expected UnknownProcessor error, got: {err:?}"
        );
    }
}
