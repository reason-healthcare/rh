use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum PublishCommands {
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
        PublishCommands::Build(args) => {
            let output_dir = args.out.unwrap_or_else(|| args.dir.join("output"));
            rh_publisher::build(&args.dir, &output_dir)?;
            Ok(())
        }
        PublishCommands::Lock(args) => {
            let output_dir = args.dir.join("output");
            rh_publisher::lock_package(&args.dir, &output_dir)?;
            Ok(())
        }
        PublishCommands::Check(args) => {
            rh_publisher::check(&args.dir)?;
            Ok(())
        }
        PublishCommands::Pack(args) => {
            let tgz = rh_publisher::pack_dir(&args.dir)?;
            println!("Packed: {}", tgz.display());
            Ok(())
        }
    }
}
