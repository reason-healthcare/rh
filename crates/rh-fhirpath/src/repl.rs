//! Interactive REPL for FHIRPath expressions.
//!
//! Enabled via the `repl` crate feature.

use anyhow::Result;
use rh_foundation::cli::repl::{ReplInput, ReplSession};
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::{EvaluationContext, FhirPathEvaluator, FhirPathParser};

/// Run an interactive REPL for FHIRPath expressions.
pub fn run_repl(data_file: Option<&Path>) -> Result<()> {
    println!("🔍 FHIRPath Interactive REPL");
    println!("Type FHIRPath expressions to evaluate them.");
    println!("Commands: .help, .data, .quit");
    println!("Use ↑/↓ arrow keys to navigate command history (persistent across sessions).");
    println!();

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let mut data = if let Some(path) = data_file {
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Value>(&content)?
    } else {
        Value::Null
    };

    let history_file = std::env::temp_dir().join("fhirpath_repl_history.txt");
    let mut session = ReplSession::with_history(history_file)?;

    loop {
        match session.readline("fhirpath> ")? {
            ReplInput::Line(line) => {
                let input = line.trim().to_owned();

                if input.is_empty() {
                    continue;
                }

                session.add_history_entry(&input)?;

                match input.as_str() {
                    ".quit" | ".exit" => {
                        println!("Goodbye!");
                        break;
                    }
                    ".help" => {
                        print_help();
                        continue;
                    }
                    ".data" => {
                        if data == Value::Null {
                            println!("No data loaded. Use .load <file> to load FHIR data.");
                        } else {
                            println!("Data: {}", serde_json::to_string_pretty(&data)?);
                        }
                        continue;
                    }
                    cmd if cmd.starts_with(".load ") => {
                        let path = cmd.strip_prefix(".load ").unwrap().trim();
                        match load_data_file(path) {
                            Ok(new_data) => {
                                data = new_data;
                                println!("✅ Loaded data from: {path}");
                            }
                            Err(e) => {
                                println!("❌ Failed to load data: {e}");
                            }
                        }
                        continue;
                    }
                    cmd if cmd.starts_with('.') => {
                        println!("❌ Unknown command: {cmd}. Type .help for available commands.");
                        continue;
                    }
                    _ => {}
                }

                match parser.parse(&input) {
                    Ok(ast) => {
                        let context = EvaluationContext::new(data.clone());
                        match evaluator.evaluate(&ast, &context) {
                            Ok(result) => {
                                let trace_logs = context.get_trace_logs();
                                if !trace_logs.is_empty() {
                                    println!("\n📋 Trace logs:");
                                    for log in trace_logs {
                                        println!("  [TRACE:{}] {}", log.name, log.value);
                                    }
                                    println!();
                                }
                                println!("=> {result:?}");
                            }
                            Err(e) => {
                                println!("❌ Evaluation error: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Parse error: {e}");
                    }
                }
            }
            ReplInput::Interrupted => {
                println!("^C");
                continue;
            }
            ReplInput::Eof => {
                println!("^D");
                break;
            }
        }
    }

    Ok(())
}

fn load_data_file(path: &str) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    let data = serde_json::from_str::<Value>(&content)?;
    Ok(data)
}

fn print_help() {
    println!("FHIRPath REPL Commands:");
    println!("  .help                Show this help");
    println!("  .data                Show currently loaded data");
    println!("  .load <file>         Load FHIR data from JSON file");
    println!("  .quit, .exit         Exit the REPL");
    println!();
    println!("Navigation:");
    println!("  ↑/↓ arrows           Navigate command history");
    println!("  Ctrl+C               Interrupt current input");
    println!("  Ctrl+D               Exit REPL");
    println!();
    println!("FHIRPath Examples:");
    println!("  resourceType         Get the resource type");
    println!("  name.family          Get family names");
    println!("  name.where(use='official').given  Get official given names");
    println!("  telecom.where(system='email').value  Get email addresses");
    println!("  5 + 3 * 2            Arithmetic operations");
    println!("  'hello'.upper()      String functions");
    println!();
}
