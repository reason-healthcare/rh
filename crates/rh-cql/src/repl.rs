//! Interactive REPL for CQL compilation.
//!
//! Enabled via the `repl` crate feature.

use anyhow::Result;
use rh_foundation::cli::repl::{ReplInput, ReplSession};

use crate::{compile_to_json, CompilerOptions};

/// Run an interactive CQL compiler REPL.
pub fn run_repl(debug: bool) -> Result<()> {
    println!("CQL Compiler REPL");
    println!("Enter CQL source (multi-line supported, end with blank line)");
    println!("Commands: :quit, :help, :debug, :compact");
    println!();

    let mut session = ReplSession::new()?;
    let mut compact_output = false;
    let mut debug_mode = debug;

    loop {
        // Collect multi-line input
        let mut source = String::new();
        let mut line_num = 1;

        loop {
            let prompt = if source.is_empty() {
                "cql> ".to_string()
            } else {
                format!("{line_num:3}> ")
            };

            match session.readline(&prompt)? {
                ReplInput::Line(line) => {
                    // Check for commands on the first line only
                    if source.is_empty() && line.starts_with(':') {
                        match line.trim() {
                            ":quit" | ":q" | ":exit" => {
                                println!("Goodbye!");
                                return Ok(());
                            }
                            ":help" | ":h" => {
                                print_help();
                                break;
                            }
                            ":debug" => {
                                debug_mode = !debug_mode;
                                println!("Debug mode: {}", if debug_mode { "ON" } else { "OFF" });
                                break;
                            }
                            ":compact" => {
                                compact_output = !compact_output;
                                println!(
                                    "Compact output: {}",
                                    if compact_output { "ON" } else { "OFF" }
                                );
                                break;
                            }
                            _ => {
                                println!("Unknown command. Type :help for help.");
                                break;
                            }
                        }
                    }

                    // Empty line ends multi-line input
                    if line.is_empty() && !source.is_empty() {
                        break;
                    }

                    if !line.is_empty() {
                        if !source.is_empty() {
                            source.push('\n');
                        }
                        source.push_str(&line);
                        line_num += 1;
                    }
                }
                ReplInput::Interrupted => {
                    println!("^C");
                    source.clear();
                    break;
                }
                ReplInput::Eof => {
                    println!("Goodbye!");
                    return Ok(());
                }
            }
        }

        if source.is_empty() {
            continue;
        }

        let options = if debug_mode {
            CompilerOptions::debug()
        } else {
            CompilerOptions::default()
        };

        match compile_to_json(&source, Some(options), !compact_output) {
            Ok(json) => {
                println!("\n{json}\n");
            }
            Err(e) => {
                println!("\n✗ Error: {e}\n");
            }
        }
    }
}

fn print_help() {
    println!("CQL REPL Commands:");
    println!("  :quit, :q, :exit  Exit the REPL");
    println!("  :help, :h         Show this help");
    println!("  :debug            Toggle debug mode (annotations, locators)");
    println!("  :compact          Toggle compact JSON output");
    println!();
    println!("Enter CQL source code (can be multi-line).");
    println!("Press Enter twice to compile.");
    println!();
}
