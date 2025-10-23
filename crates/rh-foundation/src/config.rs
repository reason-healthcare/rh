//! Configuration traits and utilities.
//!
//! This module provides a common configuration trait that can be
//! implemented by configuration types across the workspace.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Configuration trait that all configuration types should implement.
///
/// This trait provides a common interface for configuration objects,
/// ensuring they can be serialized/deserialized and validated.
///
/// # Example
/// ```
/// use rh_foundation::Config;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct MyConfig {
///     name: String,
///     port: u16,
/// }
///
/// impl Config for MyConfig {
///     fn validate(&self) -> rh_foundation::Result<()> {
///         if self.port == 0 {
///             return Err(rh_foundation::FoundationError::InvalidInput(
///                 "Port cannot be 0".to_string()
///             ));
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Config: for<'de> Deserialize<'de> + Serialize + fmt::Debug {
    /// Validate the configuration.
    ///
    /// Returns `Ok(())` if the configuration is valid, or an error
    /// describing what is invalid.
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
