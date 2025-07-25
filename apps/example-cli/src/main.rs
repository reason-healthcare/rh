use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, warn};
use common::{utils, Config};
use serde::{Deserialize, Serialize};

/// Example CLI application demonstrating workspace usage
#[derive(Parser)]
#[clap(name = "example-cli")]
#[clap(about = "An example CLI application from our Rust monorepo")]
#[clap(version)]
struct Cli {
    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration file
    Init {
        /// Path to the configuration file
        #[clap(short, long, default_value = "config.json")]
        config: String,
    },
    /// Run the main application logic
    Run {
        /// Path to the configuration file
        #[clap(short, long, default_value = "config.json")]
        config: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    version: String,
    debug: bool,
}

impl Config for AppConfig {
    fn validate(&self) -> common::Result<()> {
        if self.name.is_empty() {
            return Err(common::CommonError::Config {
                message: "Name cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "example-app".to_string(),
            version: "0.1.0".to_string(),
            debug: false,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let subscriber = if cli.verbose {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish()
    } else {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).finish()
    };
    tracing::subscriber::set_global_default(subscriber)?;
    
    match cli.command {
        Commands::Init { config } => {
            info!("Initializing configuration at: {}", config);
            
            let default_config = AppConfig::default();
            utils::save_config_to_file(&default_config, &config)
                .map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))?;
            
            info!("Configuration initialized successfully!");
        }
        Commands::Run { config } => {
            info!("Loading configuration from: {}", config);
            
            let app_config: AppConfig = utils::load_config_from_file(&config)
                .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
            
            info!("Running application with config: {:?}", app_config);
            
            if app_config.debug {
                warn!("Debug mode is enabled");
            }
            
            // Your application logic here
            info!("Application {} v{} completed successfully!", app_config.name, app_config.version);
        }
    }
    
    Ok(())
}
