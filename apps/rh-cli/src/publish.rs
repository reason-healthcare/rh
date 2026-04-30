use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum PublishCommands {
    /// Scaffold a new FHIR Package source directory
    Init(InitArgs),

    /// Build a FHIR Package from a source directory
    Build(BuildArgs),

    /// Resolve canonical references and write/update fhir-lock.json
    Lock(LockArgs),

    /// Validate source before building (sync check, lock check)
    Check(CheckArgs),

    /// Pack an expanded output directory into a .tgz tarball
    Pack(PackArgs),
}

#[derive(Args)]
pub struct InitArgs {
    /// Target directory for the new package (defaults to current directory)
    pub dir: Option<PathBuf>,

    /// Package name in reverse-DNS NPM style (e.g. com.example.fhir)
    #[clap(long, short = 'n')]
    pub name: String,

    /// Canonical URL base for resources (e.g. https://example.org/fhir)
    #[clap(long, short = 'c')]
    pub canonical: String,

    /// Package version [default: 0.1.0]
    #[clap(long, default_value = "0.1.0")]
    pub version: String,

    /// FHIR version [default: 4.0.1]
    #[clap(long, default_value = "4.0.1")]
    pub fhir_version: String,

    /// Human-readable title (defaults to PascalCase of name)
    #[clap(long)]
    pub title: Option<String>,

    /// Package description
    #[clap(long)]
    pub description: Option<String>,

    /// Author or publisher name
    #[clap(long)]
    pub author: Option<String>,

    /// SPDX license identifier [default: CC0-1.0]
    #[clap(long, default_value = "CC0-1.0")]
    pub license: String,

    /// IG resource status (draft|active|retired) [default: draft]
    #[clap(long, default_value = "draft")]
    pub status: String,
}

#[derive(Args)]
pub struct BuildArgs {
    /// Path to the source directory containing package.json and FHIR resources
    pub dir: PathBuf,

    /// Output directory for the expanded package (default: <dir>/output)
    #[clap(short, long)]
    pub out: Option<PathBuf>,
}

#[derive(Args)]
pub struct LockArgs {
    /// Path to the source directory
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Path to the source directory
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct PackArgs {
    /// Path to the expanded output directory to pack
    pub dir: PathBuf,

    /// Output path for the resulting .tgz file (default: <dir>/../<name>-<version>.tgz)
    #[clap(short, long)]
    pub out: Option<PathBuf>,
}

pub async fn handle_command(cmd: PublishCommands) -> Result<()> {
    match cmd {
        PublishCommands::Init(args) => {
            let dir = args
                .dir
                .unwrap_or_else(|| std::env::current_dir().expect("cannot determine current dir"));
            let opts = rh_packager::InitOptions {
                name: args.name,
                canonical: args.canonical,
                version: args.version,
                fhir_version: args.fhir_version,
                title: args.title,
                description: args.description,
                author: args.author,
                license: args.license,
                status: args.status,
            };
            let created = rh_packager::init_package(&dir, opts)?;
            for path in &created {
                println!("  Created: {}", path.display());
            }
            println!("\nPackage initialised at {}", dir.display());
            println!("Next: edit package.json, then run `rh publish build {}`", dir.display());
            Ok(())
        }
        PublishCommands::Build(args) => {
            let output_dir = args.out.unwrap_or_else(|| args.dir.join("output"));
            rh_packager::build(&args.dir, &output_dir)?;
            Ok(())
        }
        PublishCommands::Lock(args) => {
            let output_dir = args.dir.join("output");
            rh_packager::lock_package(&args.dir, &output_dir)?;
            Ok(())
        }
        PublishCommands::Check(args) => {
            rh_packager::check(&args.dir)?;
            Ok(())
        }
        PublishCommands::Pack(args) => {
            let tgz = rh_packager::pack_dir(&args.dir)?;
            println!("Packed: {}", tgz.display());
            Ok(())
        }
    }
}
