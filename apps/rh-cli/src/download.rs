use anyhow::Result;
use clap::Subcommand;
use rh_foundation::loader::{LoaderConfig, PackageLoader};
use std::path::{Path, PathBuf};
use tracing::info;

use crate::output::{Format, OutputContext};

/// Download FHIR packages from npm-style registries
///
/// Authentication is handled via the RH_REGISTRY_TOKEN environment variable.
/// Set this variable to your bearer token for private registries.
#[derive(Subcommand)]
pub enum DownloadCommands {
    /// Download a FHIR package from registry
    Package {
        /// Package name (e.g., "hl7.fhir.r4.core")
        package: String,

        /// Package version (e.g., "4.0.1")
        version: String,

        /// Output directory for downloaded package (defaults to ~/.fhir/packages)
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Registry URL
        #[clap(long, default_value = "https://packages.fhir.org")]
        registry: String,

        /// Overwrite package if it already exists
        #[clap(long)]
        overwrite: bool,
    },
    /// List available versions for a package
    List {
        /// Package name (e.g., "hl7.fhir.r4.core")
        package: String,

        /// Registry URL
        #[clap(long, default_value = "https://packages.fhir.org")]
        registry: String,

        /// Show only the latest version
        #[clap(long)]
        latest: bool,
    },
}

pub async fn handle_command(cmd: DownloadCommands, ctx: &OutputContext) -> Result<()> {
    let token = std::env::var("RH_REGISTRY_TOKEN").ok();

    match cmd {
        DownloadCommands::Package {
            package,
            version,
            output,
            registry,
            overwrite,
        } => {
            let output_dir = match output {
                Some(dir) => dir,
                None => PackageLoader::get_default_packages_dir().map_err(|e| {
                    anyhow::anyhow!("Failed to get default packages directory: {e}")
                })?,
            };
            download_package(
                &package,
                &version,
                &output_dir,
                &registry,
                token.as_deref(),
                overwrite,
                ctx,
            )
            .await?;
        }
        DownloadCommands::List {
            package,
            registry,
            latest,
        } => {
            list_package_versions(&package, &registry, token.as_deref(), latest, ctx).await?;
        }
    }

    Ok(())
}

async fn download_package(
    package: &str,
    version: &str,
    output: &Path,
    registry: &str,
    token: Option<&str>,
    overwrite: bool,
    ctx: &OutputContext,
) -> Result<()> {
    info!(
        "Downloading package {}@{} from {}",
        package, version, registry
    );

    let loader_config = LoaderConfig {
        registry_url: registry.to_string(),
        auth_token: token.map(|t| t.to_string()),
        timeout_seconds: 30,
        max_retries: 3,
        verify_checksums: false,
        overwrite_existing: overwrite,
    };

    let loader = PackageLoader::new(loader_config)?;
    let manifest = loader.download_package(package, version, output).await?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "name": manifest.name,
                "version": manifest.version,
                "output": output.display().to_string(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            info!(
                "Successfully downloaded {} v{} to {}",
                manifest.name,
                manifest.version,
                output.display()
            );
            println!(
                "Successfully downloaded {} v{} to {}",
                manifest.name,
                manifest.version,
                output.display()
            );
        }
    }

    Ok(())
}

async fn list_package_versions(
    package: &str,
    registry: &str,
    token: Option<&str>,
    latest_only: bool,
    ctx: &OutputContext,
) -> Result<()> {
    let loader_config = LoaderConfig {
        registry_url: registry.to_string(),
        auth_token: token.map(|t| t.to_string()),
        timeout_seconds: 30,
        max_retries: 3,
        verify_checksums: false,
        overwrite_existing: false,
    };

    let loader = PackageLoader::new(loader_config)?;

    if latest_only {
        let latest = loader.get_latest_version(package).await?;
        match ctx.format {
            Format::Json | Format::Ndjson => {
                let result = serde_json::json!({
                    "package": package,
                    "latest_version": latest,
                });
                ctx.write_success(result)?;
            }
            Format::Human => {
                println!("{latest}");
            }
        }
    } else {
        let versions = loader.list_versions(package).await?;
        match ctx.format {
            Format::Json | Format::Ndjson => {
                let result = serde_json::json!({
                    "package": package,
                    "versions": versions,
                });
                ctx.write_success(result)?;
            }
            Format::Human => {
                if versions.is_empty() {
                    println!("No versions found for package: {package}");
                } else {
                    println!("Available versions for {package}:");
                    for version in &versions {
                        println!("  {version}");
                    }
                }
            }
        }
    }

    Ok(())
}
