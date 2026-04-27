//! High-level pipeline orchestration for `rh publish` subcommands.

use crate::{
    context::PublishContext,
    hooks::{build_registry, run_stage},
    ig_sync::check_ig_sync,
    loader::load_source_dir,
    lock::{apply_pinning, generate_lock, load_lock},
    narrative::process_narrative,
    pack::{create_tarball, write_output_dir},
    Result,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::info;

fn default_packages_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".fhir").join("packages")
}

/// Run the full `rh publish build` pipeline.
///
/// Steps:
/// 1. Load source directory
/// 2. Validate IG ↔ package.json sync
/// 3. Run `before_build` hooks
/// 4. Process narrative (embed markdown into resource `.text`)
/// 5. Apply canonical pinning from `fhir-lock.json` (if present)
/// 6. Run `after_build` hooks
/// 7. Write output directory + tarball
pub fn build(source_dir: &Path, output_dir: &Path) -> Result<PathBuf> {
    let mut ctx = load_source_dir(source_dir, output_dir.to_path_buf())?;
    check_ig_sync(&ctx)?;

    let registry = build_registry();

    let before = ctx.config.hooks.before_build.clone();
    run_stage(&registry, &before, &mut ctx)?;

    process_narrative(&mut ctx)?;

    if let Some(lock) = load_lock(&ctx)? {
        let pin_map: HashMap<String, String> = lock
            .canonicals
            .iter()
            .map(|c| (c.url.clone(), c.resolved_version.clone()))
            .collect();
        apply_pinning(&mut ctx, &pin_map);
    }

    let after = ctx.config.hooks.after_build.clone();
    run_stage(&registry, &after, &mut ctx)?;

    let pkg_dir = write_output_dir(&ctx)?;
    let tgz = create_tarball(&ctx, &pkg_dir, None)?;
    info!("Package written to {}", tgz.display());
    Ok(tgz)
}

/// Run `rh publish lock` — scan source resources and write/update `fhir-lock.json`.
pub fn lock(source_dir: &Path, output_dir: &Path) -> Result<()> {
    let ctx = load_source_dir(source_dir, output_dir.to_path_buf())?;
    let packages_dir = ctx
        .config
        .validate
        .packages_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(default_packages_dir);
    generate_lock(&ctx, &packages_dir)?;
    let lock_path = source_dir.join("fhir-lock.json");
    info!("fhir-lock.json written to {}", lock_path.display());
    Ok(())
}

/// Run `rh publish check` — validate source without writing any output.
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
pub fn pack_dir(output_dir: &Path) -> Result<PathBuf> {
    // Re-read the output package.json to get name/version for the tarball filename.
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
