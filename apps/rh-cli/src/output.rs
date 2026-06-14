//! Output framework for the rh CLI.
//!
//! Implements the design contract from the WS3 refactor plan:
//!
//! - [`ExitCode`] — canonical exit codes (0/1/3/4; clap owns 2)
//! - [`OutputFormat`] — format negotiation (`human` / `json` / `ndjson`)
//! - [`Envelope`] — stable JSON output envelope
//! - [`OutputContext`] — resolved output settings for a command run

// Items in this module are part of the output contract API; not all are wired
// up yet — they will be used as 3.5/3.6 land.
#![allow(dead_code)]
//! - [`ColorMode`] — color / TTY policy

use is_terminal::IsTerminal;
use serde::{Deserialize, Serialize};
use std::io;

// ---------------------------------------------------------------------------
// ExitCode
// ---------------------------------------------------------------------------

/// Canonical exit codes for rh commands.
///
/// Exit code `2` is reserved for clap usage errors (argument parsing).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    /// Command succeeded.
    Success = 0,
    /// Operational error: I/O failure, network error, missing file, etc.
    OperationalError = 1,
    /// Validation / conformance failure: resource is invalid, tests failed.
    ValidationFailure = 3,
    /// Parse error of user input: FHIRPath / CQL / FSH / VCL syntax error.
    ParseError = 4,
}

impl ExitCode {
    /// Exit the process with this code.
    pub fn exit(self) -> ! {
        std::process::exit(self as i32);
    }
}

// ---------------------------------------------------------------------------
// OutputFormat
// ---------------------------------------------------------------------------

/// Output format requested by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Human-readable text (default).
    #[default]
    Human,
    /// Stable JSON envelope (`{"ok":…,"result":…,"errors":[],"meta":{…}}`).
    Json,
    /// Newline-delimited JSON — one envelope per result line.
    Ndjson,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" | "text" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            "ndjson" => Ok(Self::Ndjson),
            _ => Err(format!(
                "unknown format '{s}'. Valid values: human, json, ndjson"
            )),
        }
    }
}

// ---------------------------------------------------------------------------
// ColorMode
// ---------------------------------------------------------------------------

/// Color / ANSI output policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    /// Enable color only when stdout is a TTY and `NO_COLOR` is not set.
    #[default]
    Auto,
    /// Always emit ANSI color sequences.
    Always,
    /// Never emit ANSI color sequences.
    Never,
}

impl std::str::FromStr for ColorMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(format!(
                "unknown color mode '{s}'. Valid values: auto, always, never"
            )),
        }
    }
}

impl ColorMode {
    /// Returns `true` if ANSI color should be emitted given current env.
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => {
                // Respect https://no-color.org
                if std::env::var_os("NO_COLOR").is_some() {
                    return false;
                }
                io::stdout().is_terminal()
            }
        }
    }
}

// ---------------------------------------------------------------------------
// OutputContext
// ---------------------------------------------------------------------------

/// Resolved output settings for a single command run.
///
/// Built from parsed global CLI flags and used by subcommands to format their
/// output consistently.
#[derive(Debug, Clone)]
pub struct OutputContext {
    pub format: OutputFormat,
    pub quiet: bool,
    pub color: ColorMode,
}

impl OutputContext {
    pub fn new(format: OutputFormat, quiet: bool, color: ColorMode) -> Self {
        Self {
            format,
            quiet,
            color,
        }
    }

    /// Returns `true` if ANSI color output is enabled.
    pub fn color_enabled(&self) -> bool {
        self.color.is_enabled()
    }

    /// Returns `true` when the format is JSON or NDJSON.
    pub fn is_json(&self) -> bool {
        matches!(self.format, OutputFormat::Json | OutputFormat::Ndjson)
    }
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

/// A single error entry in the JSON output envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<String>,
}

impl EnvelopeError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span: None,
        }
    }

    pub fn with_span(mut self, span: impl Into<String>) -> Self {
        self.span = Some(span.into());
        self
    }
}

/// Metadata attached to every JSON output envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeMeta {
    pub version: String,
    pub command: String,
}

/// Stable JSON output envelope emitted when `--format json` is active.
///
/// Schema: `{"ok": bool, "result": T|null, "errors": [...], "meta": {...}}`
///
/// # Example
///
/// ```
/// use rh_cli_output::{Envelope, EnvelopeError};
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T: Serialize> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    pub errors: Vec<EnvelopeError>,
    pub meta: EnvelopeMeta,
}

impl<T: Serialize> Envelope<T> {
    /// Create a successful envelope wrapping `result`.
    pub fn ok(result: T, command: impl Into<String>) -> Self {
        Self {
            ok: true,
            result: Some(result),
            errors: vec![],
            meta: EnvelopeMeta {
                version: env!("CARGO_PKG_VERSION").to_string(),
                command: command.into(),
            },
        }
    }
}

/// Create an error envelope (no result value) with the given errors.
///
/// Using a free function avoids having to annotate the unused `T` when calling
/// `Envelope::<T>::err(…)`.
pub fn error_envelope(
    errors: Vec<EnvelopeError>,
    command: impl Into<String>,
) -> Envelope<serde_json::Value> {
    Envelope {
        ok: false,
        result: None,
        errors,
        meta: EnvelopeMeta {
            version: env!("CARGO_PKG_VERSION").to_string(),
            command: command.into(),
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn envelope_ok_serializes_correctly() {
        let env = Envelope::ok(42i32, "cql compile");
        let json: Value = serde_json::to_value(&env).unwrap();
        assert_eq!(json["ok"], true);
        assert_eq!(json["result"], 42);
        assert!(json["errors"].as_array().unwrap().is_empty());
        assert_eq!(json["meta"]["command"], "cql compile");
        assert!(!json["meta"]["version"].as_str().unwrap().is_empty());
    }

    #[test]
    fn envelope_err_serializes_correctly() {
        let errors = vec![EnvelopeError::new("parse_error", "unexpected token")];
        let env = error_envelope(errors, "fhirpath eval");
        let json: Value = serde_json::to_value(&env).unwrap();
        assert_eq!(json["ok"], false);
        // result must be absent (skip_serializing_if = None)
        assert!(json.get("result").is_none());
        assert_eq!(json["errors"][0]["code"], "parse_error");
        assert_eq!(json["errors"][0]["message"], "unexpected token");
    }

    #[test]
    fn envelope_error_span_omitted_when_absent() {
        let e = EnvelopeError::new("syntax", "bad char");
        let json: Value = serde_json::to_value(&e).unwrap();
        assert!(json.get("span").is_none());
    }

    #[test]
    fn envelope_error_span_present_when_set() {
        let e = EnvelopeError::new("syntax", "bad char").with_span("1:5");
        let json: Value = serde_json::to_value(&e).unwrap();
        assert_eq!(json["span"], "1:5");
    }

    #[test]
    fn output_format_parsing() {
        assert_eq!(
            "human".parse::<OutputFormat>().unwrap(),
            OutputFormat::Human
        );
        assert_eq!("text".parse::<OutputFormat>().unwrap(), OutputFormat::Human);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!(
            "ndjson".parse::<OutputFormat>().unwrap(),
            OutputFormat::Ndjson
        );
        assert!("xml".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn color_mode_parsing() {
        assert_eq!("auto".parse::<ColorMode>().unwrap(), ColorMode::Auto);
        assert_eq!("always".parse::<ColorMode>().unwrap(), ColorMode::Always);
        assert_eq!("never".parse::<ColorMode>().unwrap(), ColorMode::Never);
        assert!("yes".parse::<ColorMode>().is_err());
    }

    #[test]
    fn color_never_is_disabled() {
        assert!(!ColorMode::Never.is_enabled());
    }

    #[test]
    fn color_always_is_enabled() {
        assert!(ColorMode::Always.is_enabled());
    }

    #[test]
    fn exit_code_values() {
        assert_eq!(ExitCode::Success as i32, 0);
        assert_eq!(ExitCode::OperationalError as i32, 1);
        assert_eq!(ExitCode::ValidationFailure as i32, 3);
        assert_eq!(ExitCode::ParseError as i32, 4);
    }

    #[test]
    fn output_context_is_json() {
        let ctx = OutputContext::new(OutputFormat::Json, false, ColorMode::Never);
        assert!(ctx.is_json());
        let ctx2 = OutputContext::new(OutputFormat::Human, false, ColorMode::Never);
        assert!(!ctx2.is_json());
    }
}
