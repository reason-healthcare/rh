use std::fmt;
use std::io::{self, Write};

use clap::ValueEnum;
use serde::Serialize;

#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[allow(dead_code)]
pub const GIT_SHA: &str = match option_env!("VERGEN_GIT_SHA") {
    Some(s) => s,
    None => "unknown",
};

#[allow(dead_code)]
pub fn version_string() -> String {
    format!("{} (git: {})", VERSION, GIT_SHA)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum Format {
    #[default]
    Human,
    Json,
    Ndjson,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Human => write!(f, "human"),
            Format::Json => write!(f, "json"),
            Format::Ndjson => write!(f, "ndjson"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorOpt {
    #[default]
    Auto,
    Always,
    Never,
}

impl fmt::Display for ColorOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorOpt::Auto => write!(f, "auto"),
            ColorOpt::Always => write!(f, "always"),
            ColorOpt::Never => write!(f, "never"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    OperationalError = 1,
    UsageError = 2,
    ValidationError = 3,
    ParseError = 4,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> i32 {
        code as i32
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExitCode::Success => write!(f, "0 (success)"),
            ExitCode::OperationalError => write!(f, "1 (operational error)"),
            ExitCode::UsageError => write!(f, "2 (usage error)"),
            ExitCode::ValidationError => write!(f, "3 (validation/conformance failure)"),
            ExitCode::ParseError => write!(f, "4 (parse error)"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Meta {
    pub version: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutputEnvelope<T: Serialize> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorInfo>>,
    pub meta: Meta,
}

impl<T: Serialize> OutputEnvelope<T> {
    pub fn success(result: T, command: &str) -> Self {
        Self {
            ok: true,
            result: Some(result),
            errors: None,
            meta: Meta {
                version: VERSION.to_string(),
                command: command.to_string(),
            },
        }
    }

    #[allow(dead_code)]
    pub fn failure(errors: Vec<ErrorInfo>, command: &str) -> OutputEnvelope<serde_json::Value> {
        OutputEnvelope {
            ok: false,
            result: None,
            errors: Some(errors),
            meta: Meta {
                version: VERSION.to_string(),
                command: command.to_string(),
            },
        }
    }
}

impl OutputEnvelope<serde_json::Value> {
    pub fn failure_val(errors: Vec<ErrorInfo>, command: &str) -> Self {
        Self {
            ok: false,
            result: None,
            errors: Some(errors),
            meta: Meta {
                version: VERSION.to_string(),
                command: command.to_string(),
            },
        }
    }
}

#[derive(Debug)]
pub struct OutputContext {
    pub format: Format,
    #[allow(dead_code)]
    pub quiet: bool,
    #[allow(dead_code)]
    pub verbose: u8,
    pub color: ColorOpt,
    pub pretty: bool,
    command: String,
}

impl OutputContext {
    pub fn new(format: Format, quiet: bool, verbose: u8, color: ColorOpt, command: &str) -> Self {
        Self {
            format,
            quiet,
            verbose,
            color,
            pretty: false,
            command: command.to_string(),
        }
    }

    pub fn is_tty(&self) -> bool {
        is_terminal::IsTerminal::is_terminal(&io::stdout())
    }

    pub fn use_color(&self) -> bool {
        match self.color {
            ColorOpt::Always => true,
            ColorOpt::Never => false,
            ColorOpt::Auto => std::env::var("NO_COLOR").is_err() && self.is_tty(),
        }
    }

    pub fn should_use_symbols(&self) -> bool {
        self.use_color() || self.is_tty()
    }

    pub fn write_success(&self, result: serde_json::Value) -> anyhow::Result<()> {
        match self.format {
            Format::Human => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                self.write_human_result(&mut handle, &result)?;
            }
            Format::Json => {
                let envelope = OutputEnvelope::success(result, &self.command);
                let json = if self.pretty {
                    serde_json::to_string_pretty(&envelope)?
                } else {
                    serde_json::to_string(&envelope)?
                };
                println!("{json}");
            }
            Format::Ndjson => {
                let envelope = OutputEnvelope::success(result, &self.command);
                let json = serde_json::to_string(&envelope)?;
                println!("{json}");
            }
        }
        Ok(())
    }

    pub fn write_error(&self, code: ExitCode, message: &str) -> anyhow::Result<()> {
        match self.format {
            Format::Human => {
                eprintln!("error: {message}");
            }
            Format::Json | Format::Ndjson => {
                let errors = vec![ErrorInfo {
                    code: format!("{code:?}"),
                    message: message.to_string(),
                    span: None,
                }];
                let envelope =
                    OutputEnvelope::<serde_json::Value>::failure_val(errors, &self.command);
                let json = if self.pretty {
                    serde_json::to_string_pretty(&envelope)?
                } else {
                    serde_json::to_string(&envelope)?
                };
                eprintln!("{json}");
            }
        }
        Ok(())
    }

    pub fn write_errors(&self, errors: Vec<ErrorInfo>) -> anyhow::Result<()> {
        match self.format {
            Format::Human => {
                for err in &errors {
                    let span_msg = err
                        .span
                        .as_ref()
                        .map(|s| format!(" ({s})"))
                        .unwrap_or_default();
                    eprintln!("error: {}{}", err.message, span_msg);
                }
            }
            Format::Json | Format::Ndjson => {
                let envelope =
                    OutputEnvelope::<serde_json::Value>::failure_val(errors, &self.command);
                let json = if self.pretty {
                    serde_json::to_string_pretty(&envelope)?
                } else {
                    serde_json::to_string(&envelope)?
                };
                eprintln!("{json}");
            }
        }
        Ok(())
    }

    fn write_human_result(
        &self,
        handle: &mut io::StdoutLock<'_>,
        result: &serde_json::Value,
    ) -> io::Result<()> {
        match result {
            serde_json::Value::String(s) => writeln!(handle, "{s}"),
            serde_json::Value::Number(n) => writeln!(handle, "{n}"),
            serde_json::Value::Bool(b) => writeln!(handle, "{b}"),
            serde_json::Value::Null => Ok(()),
            other => {
                let json = if self.pretty {
                    serde_json::to_string_pretty(other)?
                } else {
                    serde_json::to_string(other)?
                };
                writeln!(handle, "{json}")
            }
        }
    }

    #[allow(dead_code)]
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::Success
    }
}

pub fn classify_error(err: &anyhow::Error) -> ExitCode {
    let msg = err.to_string().to_lowercase();

    if msg.contains("parse error")
        || msg.contains("failed to parse")
        || msg.contains("syntax error")
        || msg.contains("invalid fhirpath")
        || msg.contains("invalid cql")
        || msg.contains("invalid fsh")
        || msg.contains("invalid vcl")
    {
        ExitCode::ParseError
    } else if msg.contains("validation failed")
        || msg.contains("invalid")
        || msg.contains("conformance")
    {
        ExitCode::ValidationError
    } else {
        ExitCode::OperationalError
    }
}

pub fn symbol_success(ctx: &OutputContext) -> &'static str {
    if ctx.should_use_symbols() {
        "\u{2705}"
    } else {
        "OK"
    }
}

#[allow(dead_code)]
pub fn symbol_fail(ctx: &OutputContext) -> &'static str {
    if ctx.should_use_symbols() {
        "\u{274C}"
    } else {
        "FAIL"
    }
}

pub fn symbol_warn(ctx: &OutputContext) -> &'static str {
    if ctx.should_use_symbols() {
        "\u{26A0}\u{FE0F}"
    } else {
        "WARN"
    }
}

#[allow(dead_code)]
pub fn symbol_info(ctx: &OutputContext) -> &'static str {
    if ctx.should_use_symbols() {
        "\u{2139}\u{FE0F}"
    } else {
        "INFO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_variants() {
        assert_eq!(Format::Human.to_string(), "human");
        assert_eq!(Format::Json.to_string(), "json");
        assert_eq!(Format::Ndjson.to_string(), "ndjson");
    }

    #[test]
    fn test_color_opt_variants() {
        assert_eq!(ColorOpt::Auto.to_string(), "auto");
        assert_eq!(ColorOpt::Always.to_string(), "always");
        assert_eq!(ColorOpt::Never.to_string(), "never");
    }

    #[test]
    fn test_exit_code_values() {
        assert_eq!(i32::from(ExitCode::Success), 0);
        assert_eq!(i32::from(ExitCode::OperationalError), 1);
        assert_eq!(i32::from(ExitCode::UsageError), 2);
        assert_eq!(i32::from(ExitCode::ValidationError), 3);
        assert_eq!(i32::from(ExitCode::ParseError), 4);
    }

    #[test]
    fn test_envelope_success_serialization() {
        let envelope = OutputEnvelope::success(
            serde_json::json!({"resourceType": "Patient", "id": "1"}),
            "validate",
        );
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"ok\":true"));
        assert!(json.contains("\"result\""));
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"command\":\"validate\""));
    }

    #[test]
    fn test_envelope_failure_serialization() {
        let envelope = OutputEnvelope::<serde_json::Value>::failure_val(
            vec![ErrorInfo {
                code: "ParseError".to_string(),
                message: "invalid syntax".to_string(),
                span: None,
            }],
            "fhirpath",
        );
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"ok\":false"));
        assert!(json.contains("\"errors\""));
        assert!(json.contains("\"ParseError\""));
    }

    #[test]
    fn test_classify_error() {
        let err = anyhow::anyhow!("failed to parse CQL expression");
        assert_eq!(classify_error(&err), ExitCode::ParseError);

        let err = anyhow::anyhow!("validation failed: missing required field");
        assert_eq!(classify_error(&err), ExitCode::ValidationError);

        let err = anyhow::anyhow!("file not found");
        assert_eq!(classify_error(&err), ExitCode::OperationalError);
    }

    #[test]
    fn test_output_context_no_color_env() {
        std::env::set_var("NO_COLOR", "1");
        let ctx = OutputContext::new(Format::Human, false, 0, ColorOpt::Auto, "test");
        assert!(!ctx.use_color());
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_output_context_always_color() {
        let ctx = OutputContext::new(Format::Human, false, 0, ColorOpt::Always, "test");
        assert!(ctx.use_color());
    }

    #[test]
    fn test_output_context_never_color() {
        let ctx = OutputContext::new(Format::Human, false, 0, ColorOpt::Never, "test");
        assert!(!ctx.use_color());
    }
}
