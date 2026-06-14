use anyhow::{anyhow, Result};
use clap::Subcommand;
use rh_foundation::cli;
use rh_vcl::{parse_vcl, VclExplainer, VclExpression, VclTranslator};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;
use tracing::{error, info};

use crate::output::{Format, OutputContext};

#[derive(Subcommand)]
pub enum VclCommands {
    /// Parse a VCL expression and show the AST
    Parse {
        /// VCL expression to parse
        expression: String,

        /// Output format: pretty, json, debug
        #[clap(short, long, default_value = "pretty")]
        format: String,
    },
    /// Translate a VCL expression to FHIR ValueSet.compose
    Translate {
        /// VCL expression to translate
        expression: String,

        /// Output format: json, pretty
        #[clap(short, long, default_value = "json")]
        format: String,

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

        /// Output format: pretty, json
        #[clap(short, long, default_value = "pretty")]
        format: String,

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
        VclCommands::Parse { expression, format } => {
            parse_expression(&expression, &format, ctx)?;
        }
        VclCommands::Translate {
            expression,
            format,
            output,
            default_system,
        } => {
            translate_expression(&expression, &format, output.as_deref(), default_system, ctx)?;
        }
        VclCommands::Explain {
            expression,
            format,
            output,
            default_system,
        } => {
            explain_expression(&expression, &format, output.as_deref(), default_system, ctx)?;
        }
        VclCommands::Repl {
            translate,
            explain,
            default_system,
        } => {
            run_repl(translate, explain, default_system)?;
        }
    }
    Ok(())
}

fn parse_expression(expression: &str, format: &str, ctx: &OutputContext) -> Result<()> {
    match parse_vcl(expression) {
        Ok(ast) => {
            let output = match ctx.format {
                Format::Json => {
                    let _json = serde_json::to_string_pretty(&ast)?;
                    let envelope = crate::output::OutputEnvelope::success(
                        serde_json::json!({"ast": ast}),
                        "vcl parse",
                    );
                    serde_json::to_string_pretty(&envelope)?
                }
                Format::Ndjson => {
                    let envelope = crate::output::OutputEnvelope::success(
                        serde_json::json!({"ast": ast}),
                        "vcl parse",
                    );
                    serde_json::to_string(&envelope)?
                }
                Format::Human => match format {
                    "pretty" => {
                        let sym = crate::output::symbol_success(ctx);
                        eprintln!("{sym} VCL Expression parsed successfully");
                        let mut result = String::new();
                        result.push_str("AST:\n");
                        result.push_str(&indent_json(&serde_json::to_string_pretty(&ast)?, 1));
                        result
                    }
                    "json" => serde_json::to_string_pretty(&ast)?,
                    "debug" => format!("{ast:#?}"),
                    _ => {
                        return Err(anyhow!(
                            "Invalid format: {format}. Use 'pretty', 'json', or 'debug'"
                        ))
                    }
                },
            };
            println!("{output}");
        }
        Err(e) => {
            error!("Failed to parse VCL expression: {}", e);
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
    let ast = parse_vcl(expression).map_err(|e| anyhow!("Failed to parse VCL expression: {e}"))?;

    let translator = if let Some(system) = &default_system {
        VclTranslator::with_default_system(system.clone())
    } else {
        VclTranslator::new()
    };
    let fhir_compose = translator
        .translate(&ast)
        .map_err(|e| anyhow!("Failed to translate to FHIR: {e}"))?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let envelope = crate::output::OutputEnvelope::success(
                serde_json::json!({"translation": fhir_compose, "expression": expression}),
                "vcl translate",
            );
            let json = if ctx.pretty {
                serde_json::to_string_pretty(&envelope)?
            } else {
                serde_json::to_string(&envelope)?
            };
            if let Some(output_path) = output {
                cli::write_output(Some(output_path), &json)?;
                info!("Translation written to: {}", output_path.display());
            } else {
                println!("{json}");
            }
        }
        Format::Human => {
            let output_content = match format {
                "json" => serde_json::to_string_pretty(&fhir_compose)?,
                "pretty" => {
                    let sym = crate::output::symbol_success(ctx);
                    let mut result = String::new();
                    result.push_str(&format!("{sym} VCL Translation successful:\n\n"));
                    result.push_str("Original VCL:\n");
                    result.push_str(&format!("  {expression}\n\n"));
                    result.push_str("FHIR ValueSet.compose:\n");
                    result.push_str(&serde_json::to_string_pretty(&fhir_compose)?);
                    result
                }
                _ => return Err(anyhow!("Invalid format: {format}. Use 'json' or 'pretty'")),
            };
            if let Some(output_path) = output {
                cli::write_output(Some(output_path), &output_content)?;
                info!("Translation written to: {}", output_path.display());
            } else {
                println!("{output_content}");
            }
        }
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
    let ast = parse_vcl(expression).map_err(|e| anyhow!("Failed to parse VCL expression: {e}"))?;

    let explainer = VclExplainer::new();
    let explanation_result = explainer
        .explain_with_text(&ast, expression)
        .map_err(|e| anyhow!("Failed to explain VCL expression: {e}"))?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let envelope = crate::output::OutputEnvelope::success(
                serde_json::to_string_pretty(&explanation_result).map(serde_json::Value::String)?,
                "vcl explain",
            );
            let json = if ctx.pretty {
                serde_json::to_string_pretty(&envelope)?
            } else {
                serde_json::to_string(&envelope)?
            };
            if let Some(output_path) = output {
                cli::write_output(Some(output_path), &json)?;
                info!("Explanation written to: {}", output_path.display());
            } else {
                println!("{json}");
            }
        }
        Format::Human => {
            let output_content = match format {
                "json" => serde_json::to_string_pretty(&explanation_result)?,
                "pretty" => {
                    let sym = crate::output::symbol_success(ctx);
                    let mut result = String::new();
                    result.push_str(&format!("{sym} VCL Expression Explanation:\n\n"));
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
                            let indent = "  ".repeat(component.nesting_level + 1);
                            result.push_str(&format!(
                                "{indent}\u{2022} {} ({}): {}\n",
                                component.component, component.component_type, component.meaning
                            ));
                        }
                    }

                    result
                }
                _ => return Err(anyhow!("Invalid format: {format}. Use 'json' or 'pretty'")),
            };
            if let Some(output_path) = output {
                cli::write_output(Some(output_path), &output_content)?;
                info!("Explanation written to: {}", output_path.display());
            } else {
                println!("{output_content}");
            }
        }
    }

    Ok(())
}

fn run_repl(
    translate_mode: bool,
    explain_mode: bool,
    default_system: Option<String>,
) -> Result<()> {
    println!("VCL Interactive REPL");
    let mode_description = match (translate_mode, explain_mode) {
        (true, true) => ", translate, and explain",
        (true, false) => " and translate",
        (false, true) => " and explain",
        (false, false) => "",
    };
    println!("Type VCL expressions to parse{mode_description}.");
    if let Some(ref system) = default_system {
        println!("Default code system: {system}");
    }
    println!("Commands: .help, .exit, .quit");
    println!();

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("vcl> ");
        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                if line.starts_with('.') {
                    match line {
                        ".help" => {
                            println!("VCL REPL Commands:");
                            println!("  .help     - Show this help");
                            println!("  .exit     - Exit the REPL");
                            println!("  .quit     - Exit the REPL");
                            println!();
                        }
                        ".exit" | ".quit" => {
                            println!("Goodbye!");
                            break;
                        }
                        _ => {
                            println!(
                                "Unknown command: {line}. Type '.help' for available commands."
                            );
                        }
                    }
                    continue;
                }

                if line == "exit" || line == "quit" {
                    println!("Goodbye!");
                    break;
                }

                rl.add_history_entry(line)?;

                match parse_vcl(line) {
                    Ok(ast) => {
                        println!("Parsed successfully:");
                        print_ast_pretty(&ast, 1);

                        if translate_mode {
                            println!();
                            let translator = if let Some(ref default_sys) = default_system {
                                VclTranslator::with_default_system(default_sys.clone())
                            } else {
                                VclTranslator::new()
                            };
                            match translator.translate(&ast) {
                                Ok(fhir_compose) => {
                                    println!("FHIR Translation:");
                                    let json = serde_json::to_string_pretty(&fhir_compose)?;
                                    println!("{}", indent_json(&json, 1));
                                }
                                Err(e) => {
                                    println!("Translation error: {e}");
                                }
                            }
                        }

                        if explain_mode {
                            println!();
                            let explainer = VclExplainer::new();
                            match explainer.explain_with_text(&ast, line) {
                                Ok(explanation_result) => {
                                    println!("Explanation:");
                                    println!("  {}", explanation_result.explanation);
                                    println!("  Type: {:?}", explanation_result.expression_type);
                                    println!(
                                        "  Translatable to FHIR: {}",
                                        explanation_result.translatable_to_fhir
                                    );

                                    if !explanation_result.components.is_empty() {
                                        println!("  Components:");
                                        for component in &explanation_result.components {
                                            println!(
                                                "    - {} ({}): {}",
                                                component.component,
                                                component.component_type,
                                                component.meaning
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Explanation error: {e}");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Parse error: {e}");
                    }
                }
                println!();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn print_ast_pretty(expr: &VclExpression, indent: usize) {
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
