//! CLI utilities for command-line applications.
//!
//! This module provides common patterns for CLI applications including:
//! - Reading input from files, stdin, or command-line arguments
//! - Writing output to files or stdout
//! - Formatting and printing results with different output formats
//! - Error handling and exit codes

use crate::error::{io_error_with_path, FoundationError, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Reads input from a file, inline string, or stdin.
///
/// Priority order:
/// 1. If `file` is Some, reads from that file path
/// 2. If `inline` is Some, returns that string
/// 3. Otherwise, reads from stdin
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::read_input;
/// use std::path::PathBuf;
///
/// # fn example() -> anyhow::Result<()> {
/// // Read from file
/// let content = read_input(Some("input.txt"), None)?;
///
/// // Read from PathBuf
/// let path = PathBuf::from("input.txt");
/// let content = read_input(Some(&path), None)?;
///
/// // Read from inline argument
/// let content = read_input::<&str>(None, Some("inline data".to_string()))?;
///
/// // Read from stdin
/// let content = read_input::<&str>(None, None)?;
/// # Ok(())
/// # }
/// ```
pub fn read_input<P: AsRef<Path>>(file: Option<P>, inline: Option<String>) -> Result<String> {
    if let Some(filename) = file {
        let path = filename.as_ref();
        fs::read_to_string(path).map_err(|e| io_error_with_path(e, path, "read file"))
    } else if let Some(content) = inline {
        Ok(content)
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| FoundationError::Io(e).with_context("Failed to read from stdin"))?;
        Ok(buffer)
    }
}

/// Reads and parses JSON from a file.
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::read_json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     name: String,
///     value: i32,
/// }
///
/// # fn example() -> anyhow::Result<()> {
/// let config: Config = read_json("config.json")?;
/// # Ok(())
/// # }
/// ```
pub fn read_json<T: DeserializeOwned>(path_str: &str) -> Result<T> {
    let path = Path::new(path_str);
    let content = fs::read_to_string(path).map_err(|e| io_error_with_path(e, path, "read"))?;

    serde_json::from_str(&content).map_err(|e| {
        FoundationError::Serialization(e)
            .with_context(&format!("Failed to parse JSON from {path_str}"))
    })
}

/// Writes content to a file or stdout.
///
/// If `path` is None, writes to stdout. Otherwise writes to the specified file.
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::write_output;
///
/// # fn example() -> anyhow::Result<()> {
/// use std::path::Path;
///
/// // Write to file
/// write_output(Some(Path::new("output.txt")), "Hello, world!")?;
///
/// // Write to stdout
/// write_output(None, "Hello, world!")?;
/// # Ok(())
/// # }
/// ```
pub fn write_output(path: Option<&Path>, content: &str) -> Result<()> {
    match path {
        Some(file_path) => {
            fs::write(file_path, content).map_err(|e| io_error_with_path(e, file_path, "write to"))
        }
        None => io::stdout()
            .write_all(content.as_bytes())
            .map_err(|e| FoundationError::Io(e).with_context("Failed to write to stdout")),
    }
}

/// Output format for CLI results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable text output
    Text,
    /// JSON output (pretty-printed)
    Json,
    /// Compact JSON output (single line)
    JsonCompact,
    /// Debug format (Rust Debug trait)
    Debug,
    /// Pretty debug format (Rust Debug trait with pretty-printing)
    DebugPretty,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "json-compact" | "compact" => Ok(OutputFormat::JsonCompact),
            "debug" => Ok(OutputFormat::Debug),
            "debug-pretty" | "pretty" => Ok(OutputFormat::DebugPretty),
            _ => Err(format!(
                "Invalid output format: {s}. Valid options: text, json, compact, debug, pretty"
            )),
        }
    }
}

/// Prints a value with the specified output format.
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::{print_with_format, OutputFormat};
/// use serde::Serialize;
///
/// #[derive(Debug, Serialize)]
/// struct Data {
///     name: String,
///     value: i32,
/// }
///
/// # fn example() -> anyhow::Result<()> {
/// let data = Data {
///     name: "test".to_string(),
///     value: 42,
/// };
///
/// // Print as JSON
/// print_with_format(&data, OutputFormat::Json)?;
///
/// // Print as debug
/// print_with_format(&data, OutputFormat::Debug)?;
/// # Ok(())
/// # }
/// ```
pub fn print_with_format<T>(value: &T, format: OutputFormat) -> Result<()>
where
    T: std::fmt::Debug + Serialize,
{
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(value).map_err(|e| {
                FoundationError::Serialization(e).with_context("Failed to serialize JSON")
            })?;
            println!("{json}");
        }
        OutputFormat::JsonCompact => {
            let json = serde_json::to_string(value).map_err(|e| {
                FoundationError::Serialization(e).with_context("Failed to serialize JSON")
            })?;
            println!("{json}");
        }
        OutputFormat::Debug => {
            println!("{value:?}");
        }
        OutputFormat::DebugPretty => {
            println!("{value:#?}");
        }
        OutputFormat::Text => {
            println!("{value:?}");
        }
    }
    Ok(())
}

/// Prints a result, handling errors with proper formatting.
///
/// If the result is an error, it will be printed to stderr and return false.
/// If the result is Ok, the value will be printed and return true.
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::print_result;
///
/// # fn example() -> anyhow::Result<()> {
/// let result: Result<i32, String> = Ok(42);
/// let success = print_result(result);
/// assert!(success);
/// # Ok(())
/// # }
/// ```
pub fn print_result<T, E>(result: std::result::Result<T, E>) -> bool
where
    T: std::fmt::Display,
    E: std::fmt::Display,
{
    match result {
        Ok(value) => {
            println!("{value}");
            true
        }
        Err(e) => {
            eprintln!("Error: {e}");
            false
        }
    }
}

/// Exits the process with an error code if condition is true.
///
/// This is useful for implementing `--strict` flags in CLI applications.
///
/// # Examples
///
/// ```no_run
/// use rh_foundation::cli::exit_if;
///
/// # fn example() {
/// let has_errors = true;
/// let strict = true;
///
/// // Exit with code 1 if there are errors and strict mode is enabled
/// exit_if(has_errors && strict, 1);
/// # }
/// ```
pub fn exit_if(condition: bool, code: i32) {
    if condition {
        std::process::exit(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parsing() {
        assert_eq!("text".parse::<OutputFormat>().unwrap(), OutputFormat::Text);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!(
            "compact".parse::<OutputFormat>().unwrap(),
            OutputFormat::JsonCompact
        );
        assert_eq!(
            "debug".parse::<OutputFormat>().unwrap(),
            OutputFormat::Debug
        );
        assert_eq!(
            "pretty".parse::<OutputFormat>().unwrap(),
            OutputFormat::DebugPretty
        );

        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_read_json() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_cli_json.json");

        let test_data = r#"{"name": "test", "value": 42}"#;
        fs::write(&test_file, test_data).unwrap();

        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct TestData {
            name: String,
            value: i32,
        }

        let result: TestData = read_json(test_file.to_str().unwrap()).unwrap();
        assert_eq!(result.name, "test");
        assert_eq!(result.value, 42);

        fs::remove_file(test_file).unwrap();
    }
}
