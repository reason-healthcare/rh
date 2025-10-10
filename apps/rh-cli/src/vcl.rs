use anyhow::{anyhow, Result};
use clap::Subcommand;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::path::PathBuf;
use tracing::error;

use rh_vcl::{parse_vcl, VclExpression, VclTranslator};

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
    },
    /// Interactive REPL for VCL expressions
    Repl {
        /// Enable translation mode (shows both AST and FHIR translation)
        #[clap(short, long)]
        translate: bool,
    },
}

pub async fn handle_command(cmd: VclCommands) -> Result<()> {
    match cmd {
        VclCommands::Parse { expression, format } => {
            parse_expression(&expression, &format)?;
        }
        VclCommands::Translate {
            expression,
            format,
            output,
        } => {
            translate_expression(&expression, &format, output.as_deref())?;
        }
        VclCommands::Repl { translate } => {
            run_repl(translate)?;
        }
    }
    Ok(())
}

fn parse_expression(expression: &str, format: &str) -> Result<()> {
    match parse_vcl(expression) {
        Ok(ast) => match format {
            "pretty" => {
                println!("✅ VCL Expression parsed successfully:");
                print_ast_pretty(&ast, 0);
            }
            "json" => {
                let json = serde_json::to_string_pretty(&ast)?;
                println!("{}", json);
            }
            "debug" => {
                println!("{:#?}", ast);
            }
            _ => {
                return Err(anyhow!(
                    "Invalid format: {}. Use 'pretty', 'json', or 'debug'",
                    format
                ));
            }
        },
        Err(e) => {
            error!("❌ Failed to parse VCL expression: {}", e);
            return Err(anyhow!("Parse error: {}", e));
        }
    }
    Ok(())
}

fn translate_expression(
    expression: &str,
    format: &str,
    output: Option<&std::path::Path>,
) -> Result<()> {
    // First parse the expression
    let ast =
        parse_vcl(expression).map_err(|e| anyhow!("Failed to parse VCL expression: {}", e))?;

    // Then translate to FHIR
    let translator = VclTranslator::new();
    let fhir_compose = translator
        .translate(&ast)
        .map_err(|e| anyhow!("Failed to translate to FHIR: {}", e))?;

    let output_content = match format {
        "json" => serde_json::to_string_pretty(&fhir_compose)?,
        "pretty" => {
            let mut result = String::new();
            result.push_str("✅ VCL Translation successful:\n\n");
            result.push_str("Original VCL:\n");
            result.push_str(&format!("  {}\n\n", expression));
            result.push_str("FHIR ValueSet.compose:\n");
            result.push_str(&serde_json::to_string_pretty(&fhir_compose)?);
            result
        }
        _ => {
            return Err(anyhow!(
                "Invalid format: {}. Use 'json' or 'pretty'",
                format
            ));
        }
    };

    if let Some(output_path) = output {
        fs::write(output_path, &output_content)?;
        println!("✅ Translation written to: {}", output_path.display());
    } else {
        println!("{}", output_content);
    }

    Ok(())
}

fn run_repl(translate_mode: bool) -> Result<()> {
    println!("🚀 VCL Interactive REPL");
    println!(
        "Type VCL expressions to parse{}. Type 'exit' or 'quit' to exit.",
        if translate_mode { " and translate" } else { "" }
    );
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

                // Handle REPL commands
                if line.starts_with('.') {
                    match line {
                        ".help" => {
                            println!("VCL REPL Commands:");
                            println!("  .help     - Show this help");
                            println!("  .exit     - Exit the REPL");
                            println!("  .quit     - Exit the REPL");
                            println!();
                            println!("VCL Syntax Examples:");
                            println!("  http://snomed.info/sct");
                            println!("  http://loinc.org");
                            println!("  http://snomed.info/sct#123456007");
                            println!("  http://snomed.info/sct is-a #123456007");
                            println!("  http://snomed.info/sct descendant-of #123456007");
                            println!(
                                "  http://snomed.info/sct#123456007 OR http://loinc.org#LA1234-5"
                            );
                            println!("  http://snomed.info/sct#123456007 AND NOT http://snomed.info/sct#789012003");
                        }
                        ".exit" | ".quit" => {
                            println!("Goodbye! 👋");
                            break;
                        }
                        _ => {
                            println!(
                                "Unknown command: {}. Type '.help' for available commands.",
                                line
                            );
                        }
                    }
                    continue;
                }

                if line == "exit" || line == "quit" {
                    println!("Goodbye! 👋");
                    break;
                }

                rl.add_history_entry(line)?;

                // Parse the VCL expression
                match parse_vcl(line) {
                    Ok(ast) => {
                        println!("✅ Parsed successfully:");
                        print_ast_pretty(&ast, 1);

                        if translate_mode {
                            println!();
                            let translator = VclTranslator::new();
                            match translator.translate(&ast) {
                                Ok(fhir_compose) => {
                                    println!("🔄 FHIR Translation:");
                                    let json = serde_json::to_string_pretty(&fhir_compose)?;
                                    println!("{}", indent_json(&json, 1));
                                }
                                Err(e) => {
                                    println!("❌ Translation error: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Parse error: {}", e);
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
    println!("{}AST:", indent_str);
    let json = serde_json::to_string_pretty(expr).unwrap_or_else(|_| format!("{:#?}", expr));
    println!("{}", indent_json(&json, indent + 1));
}

fn indent_json(json: &str, indent_level: usize) -> String {
    let indent_str = "  ".repeat(indent_level);
    json.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<_>>()
        .join("\n")
}
