//! Output directory assembly and `.tgz` tarball creation.

use crate::{context::PublishContext, index::build_index, Result};
use flate2::{write::GzEncoder, Compression};
use serde_json::Value;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tar::Builder;
use tracing::info;

/// Write the assembled package to `ctx.output_dir/package/`.
///
/// Layout:
/// ```text
/// <output_dir>/package/
///   package.json
///   ImplementationGuide.json
///   StructureDefinition-foo.json   (with .text populated by narrative module)
///   ...
///   other/
///     overview.md                  (standalone markdown files)
///   .index.json
/// ```
pub fn write_output_dir(ctx: &PublishContext) -> Result<PathBuf> {
    let pkg_dir = ctx.output_dir.join("package");
    fs::create_dir_all(&pkg_dir)?;

    // Write FHIR resources.
    for (stem, value) in &ctx.resources {
        let out_path = pkg_dir.join(format!("{stem}.json"));
        write_json(&out_path, value)?;
    }

    // Write package.json manifest.
    let pkg_json_value = serde_json::to_value(&ctx.package_json)?;
    write_json(&pkg_dir.join("package.json"), &pkg_json_value)?;

    // Copy standalone markdown to package/other/.
    if !ctx.standalone_markdown.is_empty() {
        let other_dir = pkg_dir.join("other");
        fs::create_dir_all(&other_dir)?;
        for md_path in &ctx.standalone_markdown {
            if let Some(filename) = md_path.file_name() {
                let dest = other_dir.join(filename);
                fs::copy(md_path, &dest)?;
            }
        }
    }

    // Write .index.json.
    let index = build_index(ctx)?;
    let index_value = serde_json::to_value(&index)?;
    write_json(&pkg_dir.join(".index.json"), &index_value)?;

    info!("Output written to {}", pkg_dir.display());
    Ok(pkg_dir)
}

/// Pack the `package/` directory under `output_dir` into a `.tgz` tarball.
///
/// All entries in the tarball will have the `package/` prefix as required by the
/// FHIR Package Specification.
///
/// Returns the path to the written `.tgz` file.
pub fn create_tarball(ctx: &PublishContext, pkg_dir: &Path, out_path: Option<PathBuf>) -> Result<PathBuf> {
    let name = &ctx.package_json.name;
    let version = &ctx.package_json.version;
    let tgz_name = format!("{name}-{version}.tgz");

    let tgz_path = out_path.unwrap_or_else(|| ctx.output_dir.join(&tgz_name));

    let tgz_file = fs::File::create(&tgz_path)?;
    let encoder = GzEncoder::new(tgz_file, Compression::default());
    let mut archive = Builder::new(encoder);

    append_dir_all(&mut archive, pkg_dir, "package")?;

    archive.finish()?;

    info!("Tarball written to {}", tgz_path.display());
    Ok(tgz_path)
}

/// Recursively append all files under `dir` into the archive, prefixed with
/// `archive_prefix`.
fn append_dir_all<W: Write>(
    archive: &mut Builder<W>,
    dir: &Path,
    archive_prefix: &str,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let archive_path = format!("{archive_prefix}/{file_name}");

        if path.is_dir() {
            append_dir_all(archive, &path, &archive_path)?;
        } else {
            let mut file = fs::File::open(&path)?;
            archive.append_file(&archive_path, &mut file)?;
        }
    }
    Ok(())
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let content = serde_json::to_string_pretty(value)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, context::PublishContext, manifest::PackageJson,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(tmp: &TempDir) -> PublishContext {
        let pkg = PackageJson {
            name: "test.fhir.pkg".to_string(),
            version: "1.0.0".to_string(),
            fhir_versions: vec!["4.0.1".to_string()],
            dependencies: HashMap::new(),
            url: Some("http://example.org/fhir".to_string()),
            description: None,
            author: None,
            license: None,
            extra: HashMap::new(),
        };
        let mut resources = HashMap::new();
        resources.insert(
            "ImplementationGuide".to_string(),
            json!({"resourceType":"ImplementationGuide","id":"ig","packageId":"test.fhir.pkg","version":"1.0.0","url":"http://example.org/fhir","fhirVersion":["4.0.1"],"status":"draft"}),
        );
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","id":"foo","url":"http://example.org/fhir/StructureDefinition/foo","version":"1.0.0"}),
        );

        let standalone_md = vec![{
            let p = tmp.path().join("overview.md");
            fs::write(&p, "# Overview").unwrap();
            p
        }];

        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: pkg,
            resources,
            config: PublisherConfig::default(),
            standalone_markdown: standalone_md,
        }
    }

    #[test]
    fn write_output_dir_creates_expected_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(&tmp);
        let pkg_dir = write_output_dir(&ctx).unwrap();

        assert!(pkg_dir.join("package.json").exists());
        assert!(pkg_dir.join("ImplementationGuide.json").exists());
        assert!(pkg_dir.join("StructureDefinition-foo.json").exists());
        assert!(pkg_dir.join(".index.json").exists());
        assert!(pkg_dir.join("other").join("overview.md").exists());
    }

    #[test]
    fn index_json_has_correct_version() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(&tmp);
        let pkg_dir = write_output_dir(&ctx).unwrap();

        let index_text = fs::read_to_string(pkg_dir.join(".index.json")).unwrap();
        let index: serde_json::Value = serde_json::from_str(&index_text).unwrap();
        assert_eq!(index["index-version"], 2);
    }

    #[test]
    fn create_tarball_uses_package_prefix() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(&tmp);
        let pkg_dir = write_output_dir(&ctx).unwrap();

        let tgz_path = create_tarball(&ctx, &pkg_dir, None).unwrap();
        assert!(tgz_path.exists());
        assert!(tgz_path.to_str().unwrap().ends_with(".tgz"));

        // Verify all entries start with "package/"
        let tgz_file = fs::File::open(&tgz_path).unwrap();
        let decoder = flate2::read::GzDecoder::new(tgz_file);
        let mut archive = tar::Archive::new(decoder);
        for entry in archive.entries().unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path().unwrap();
            assert!(
                entry_path.starts_with("package"),
                "Entry does not start with 'package/': {}",
                entry_path.display()
            );
        }
    }

    #[test]
    fn tarball_name_includes_package_name_and_version() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(&tmp);
        let pkg_dir = write_output_dir(&ctx).unwrap();

        let tgz_path = create_tarball(&ctx, &pkg_dir, None).unwrap();
        let filename = tgz_path.file_name().unwrap().to_str().unwrap();
        assert!(filename.contains("test.fhir.pkg"));
        assert!(filename.contains("1.0.0"));
    }
}
