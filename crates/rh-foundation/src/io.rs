//! File I/O utilities.
//!
//! This module provides common file operations used across the workspace.

use crate::config::Config;
use crate::error::{FoundationError, Result};
use std::path::Path;

/// Load configuration from a JSON file.
///
/// # Example
/// ```no_run
/// use rh_foundation::{Config, io};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct MyConfig {
///     name: String,
/// }
///
/// impl Config for MyConfig {}
///
/// let config: MyConfig = io::load_config_from_file("config.json").unwrap();
/// ```
pub fn load_config_from_file<T: Config>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).map_err(|e| {
        FoundationError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read config from {}: {}", path.display(), e),
        ))
    })?;
    let config: T = serde_json::from_str(&content)?;
    config.validate()?;
    Ok(config)
}

/// Save configuration to a JSON file.
///
/// # Example
/// ```no_run
/// use rh_foundation::{Config, io};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct MyConfig {
///     name: String,
/// }
///
/// impl Config for MyConfig {}
///
/// let config = MyConfig { name: "test".to_string() };
/// io::save_config_to_file(&config, "config.json").unwrap();
/// ```
pub fn save_config_to_file<T: Config>(config: &T, path: impl AsRef<Path>) -> Result<()> {
    config.validate()?;
    let content = serde_json::to_string_pretty(config)?;
    let path = path.as_ref();
    std::fs::write(path, content).map_err(|e| {
        FoundationError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to write config to {}: {}", path.display(), e),
        ))
    })?;
    Ok(())
}

/// Read a JSON file and deserialize it.
pub fn read_json<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).map_err(|e| {
        FoundationError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read JSON from {}: {}", path.display(), e),
        ))
    })?;
    serde_json::from_str(&content).map_err(Into::into)
}

/// Write a value as JSON to a file.
pub fn write_json<T>(path: impl AsRef<Path>, value: &T, pretty: bool) -> Result<()>
where
    T: serde::Serialize,
{
    let path = path.as_ref();
    let content = if pretty {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    };
    std::fs::write(path, content).map_err(|e| {
        FoundationError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to write JSON to {}: {}", path.display(), e),
        ))
    })?;
    Ok(())
}

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Get the canonical (absolute) path.
pub fn canonical_path(path: impl AsRef<Path>) -> Result<std::path::PathBuf> {
    path.as_ref().canonicalize().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: u32,
    }

    impl Config for TestConfig {}

    #[test]
    fn test_config_roundtrip() {
        let config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TestConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }
}
