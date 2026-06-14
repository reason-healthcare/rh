use anyhow::{anyhow, Result};
use clap::Subcommand;
use rh_foundation::cli;
use rh_vcl::{parse_vcl, VclExplainer, VclTranslator};
use serde::Serialize;
use std::path::PathBuf;
use tracing::error;

use crate::output::{Envelope, OutputContext, OutputFormat};

fn print_envelope<T: Serialize>(ctx: &OutputContext, envelope: &Envelope<T>) -> Result<()> {
    let json = if matches!(ctx.format, OutputFormat::Json) {
        serde_json::to_string_pretty(envelope)?
    } else {
        serde_json::to_string(envelope)?
    };
    println!("{json}");
    Ok(())
}

#[derive(Subcommand)]
pub enum VclCommands {
    /// Parse a VCL expression and show the AST
    Parse {
        /// VCL expression to parse
        expression: String,

        /// Display format: pretty, json, debug
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,
    },
    /// Translate a VCL expression to FHIR ValueSet.compose
    Translate {
        /// VCL expression to translate
        expression: String,

        /// Display format: json, pretty
        #[clap(long = "display-format", default_value = "json")]
        display_format: String,

        /// Output file path (optional)
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Default code system URI to use for codes without explicit system
        #[clap(short = 's', long)]
        default_system: Option<String>,
    },
    /// Explain a VCL expression in plain English
    Explain {
        /// VCL expression to explain
        expression: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Output file path (optional)
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Default code system URI to use for codes without explicit system
        #[clap(short = 's', long)]
        default_system: Option<String>,
    },
    /// Interactive REPL for VCL expressions
    Repl {
        /// Enable translation mode (shows both AST and FHIR translation)
        #[clap(short, long)]
        translate: bool,

        /// Enable explanation mode (shows plain English explanations)
        #[clap(short, long)]
        explain: bool,

        /// Default code system URI to use for codes without explicit system
        #[clap(short = 's', long)]
        default_system: Option<String>,
    },
}

pub async fn handle_command(cmd: VclCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        VclCommands::Parse {
            expression,
            display_format,
        } => {
            parse_expression(&expression, &display_format, ctx)?;
        }
        VclCommands::Translate {
            expression,
            display_format,
            output,
            default_system,
        } => {
            translate_expression(
                &expression,
                &display_format,
                output.as_deref(),
                default_system,
                ctx,
            )?;
        }
        VclCommands::Explain {
            expression,
            display_format,
            output,
            default_system,
        } => {
            explain_expression(
                &expression,
                &display_format,
                output.as_deref(),
                default_system,
                ctx,
            )?;
        }
        VclCommands::Repl {
            translate,
            explain,
            default_system,
        } => {
            rh_vcl::repl::run_repl(translate, explain, default_system)?;
        }
    }
    Ok(())
}

fn parse_expression(expression: &str, format: &str, ctx: &OutputContext) -> Result<()> {
    match parse_vcl(expression) {
        Ok(ast) => {
            if ctx.is_json() {
                return print_envelope(ctx, &Envelope::ok(ast, "vcl parse"));
            }
            match format {
                "pretty" => {
                    println!("✅ VCL Expression parsed successfully:");
                    print_ast_pretty(&ast, 0);
                }
                "json" => {
                    let json = serde_json::to_string_pretty(&ast)?;
                    println!("{json}");
                }
                "debug" => {
                    println!("{ast:#?}");
                }
                _ => {
                    return Err(anyhow!(
                        "Invalid format: {format}. Use 'pretty', 'json', or 'debug'"
                    ));
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to parse VCL expression: {}", e);
            return Err(anyhow!("Parse error: {e}"));
        }
    }
    Ok(())
}

fn translate_expression(
    expression: &str,
    format: &str,
    output: Option<&std::path::Path>,
    default_system: Option<String>,
    ctx: &OutputContext,
) -> Result<()> {
    // First parse the expression
    let ast = parse_vcl(expression).map_err(|e| anyhow!("Failed to parse VCL expression: {e}"))?;

    // Then translate to FHIR
    // Use default system if provided - translator will handle mixed expressions correctly
    let translator = if let Some(system) = default_system {
        VclTranslator::with_default_system(system)
    } else {
        VclTranslator::new()
    };
    let fhir_compose = translator
        .translate(&ast)
        .map_err(|e| anyhow!("Failed to translate to FHIR: {e}"))?;

    if ctx.is_json() {
        if let Some(output_path) = output {
            cli::write_output(
                Some(output_path),
                &serde_json::to_string_pretty(&fhir_compose)?,
            )?;
        }
        return print_envelope(ctx, &Envelope::ok(fhir_compose, "vcl translate"));
    }

    let output_content = match format {
        "json" => serde_json::to_string_pretty(&fhir_compose)?,
        "pretty" => {
            let mut result = String::new();
            result.push_str("✅ VCL Translation successful:\n\n");
            result.push_str("Original VCL:\n");
            result.push_str(&format!("  {expression}\n\n"));
            result.push_str("FHIR ValueSet.compose:\n");
            result.push_str(&serde_json::to_string_pretty(&fhir_compose)?);
            result
        }
        _ => {
            return Err(anyhow!("Invalid format: {format}. Use 'json' or 'pretty'"));
        }
    };

    if let Some(output_path) = output {
        cli::write_output(Some(output_path), &output_content)?;
        println!("✅ Translation written to: {}", output_path.display());
    } else {
        println!("{output_content}");
    }

    Ok(())
}

fn explain_expression(
    expression: &str,
    format: &str,
    output: Option<&std::path::Path>,
    _default_system: Option<String>,
    ctx: &OutputContext,
) -> Result<()> {
    // First parse the expression
    let ast = parse_vcl(expression).map_err(|e| anyhow!("Failed to parse VCL expression: {e}"))?;

    // Create explainer and explain the expression
    let explainer = VclExplainer::new();
    let explanation_result = explainer
        .explain_with_text(&ast, expression)
        .map_err(|e| anyhow!("Failed to explain VCL expression: {e}"))?;

    if ctx.is_json() {
        if let Some(output_path) = output {
            cli::write_output(
                Some(output_path),
                &serde_json::to_string_pretty(&explanation_result)?,
            )?;
        }
        return print_envelope(ctx, &Envelope::ok(explanation_result, "vcl explain"));
    }

    let output_content = match format {
        "json" => serde_json::to_string_pretty(&explanation_result)?,
        "pretty" => {
            let mut result = String::new();
            result.push_str("✅ VCL Expression Explanation:\n\n");
            result.push_str("Original VCL:\n");
            result.push_str(&format!("  {expression}\n\n"));
            result.push_str("Explanation:\n");
            result.push_str(&format!("  {}\n\n", explanation_result.explanation));
            result.push_str("Expression Type:\n");
            result.push_str(&format!("  {:?}\n\n", explanation_result.expression_type));
            result.push_str("Translatable to FHIR:\n");
            result.push_str(&format!("  {}\n", explanation_result.translatable_to_fhir));

            if !explanation_result.components.is_empty() {
                result.push_str("\nComponents:\n");
                for component in &explanation_result.components {
                    // Create indentation based on nesting level
                    let indent = "  ".repeat(component.nesting_level + 1);
                    result.push_str(&format!(
                        "{}• {} ({}): {}\n",
                        indent, component.component, component.component_type, component.meaning
                    ));
                }
            }

            result
        }
        _ => {
            return Err(anyhow!("Invalid format: {format}. Use 'json' or 'pretty'"));
        }
    };

    if let Some(output_path) = output {
        cli::write_output(Some(output_path), &output_content)?;
        println!("✅ Explanation written to: {}", output_path.display());
    } else {
        println!("{output_content}");
    }

    Ok(())
}

// (REPL moved to rh-vcl::repl; see crates/rh-vcl/src/repl.rs)

fn print_ast_pretty(expr: &rh_vcl::VclExpression, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{indent_str}AST:");
    let json = serde_json::to_string_pretty(expr).unwrap_or_else(|_| format!("{expr:#?}"));
    println!("{}", indent_json(&json, indent + 1));
}

fn indent_json(json: &str, indent_level: usize) -> String {
    let indent_str = "  ".repeat(indent_level);
    json.lines()
        .map(|line| format!("{indent_str}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
