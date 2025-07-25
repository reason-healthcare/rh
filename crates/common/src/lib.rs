//! Common utilities and types shared across the workspace.
//!
//! This crate provides foundational functionality that can be used
//! by other crates and applications in the workspace.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Common error type used throughout the workspace
#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, CommonError>;

/// Configuration trait that all configuration types should implement
pub trait Config: for<'de> Deserialize<'de> + Serialize + fmt::Debug {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Common utilities module
pub mod utils {
    use super::*;

    /// Load configuration from a JSON file
    pub fn load_config_from_file<T: Config>(path: &str) -> Result<T> {
        let content = std::fs::read_to_string(path)?;
        let config: T = serde_json::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn save_config_to_file<T: Config>(config: &T, path: &str) -> Result<()> {
        config.validate()?;
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestConfig {
        name: String,
        value: u32,
    }

    impl Config for TestConfig {}

    #[test]
    fn test_config_serialization() {
        let config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TestConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.value, deserialized.value);
    }
}
