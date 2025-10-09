use anyhow::Result;
use clap::{Parser, Subcommand};
use rh_ffq::{parse_start, translate_to_fhir};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::io::{self, Read};

/// FFQ (FHIR Filter Query) - Parse and translate FHIR terminology filter expressions
#[derive(Parser)]
pub struct FfqArgs {
    #[clap(subcommand)]
    pub command: FfqCommands,
}

#[derive(Subcommand)]
pub enum FfqCommands {
    /// Parse an FFQ expression and show the AST
    Parse {
        /// FFQ expression to parse (if not provided, reads from stdin)
        expression: Option<String>,

        /// Read from file instead of expression or stdin
        #[clap(short, long)]
        file: Option<String>,

        /// Pretty print the AST output
        #[clap(short, long)]
        pretty: bool,
    },

    /// Parse and translate FFQ to FHIR ValueSet.compose JSON
    Translate {
        /// FFQ expression to translate (if not provided, reads from stdin)
        expression: Option<String>,

        /// Read from file instead of expression or stdin
        #[clap(short, long)]
        file: Option<String>,

        /// Compact JSON output (no pretty printing)
        #[clap(short, long)]
        compact: bool,

        /// Show the AST as well as the translation
        #[clap(short, long)]
        show_ast: bool,
    },

    /// Start an interactive REPL for testing FFQ expressions
    Repl {
        /// Show AST in addition to FHIR output
        #[clap(short, long)]
        show_ast: bool,

        /// Use compact JSON output
        #[clap(short, long)]
        compact: bool,
    },
}

pub async fn handle_command(args: FfqArgs) -> Result<()> {
    match args.command {
        FfqCommands::Parse {
            expression,
            file,
            pretty,
        } => handle_parse(expression, file, pretty).await,
        FfqCommands::Translate {
            expression,
            file,
            compact,
            show_ast,
        } => handle_translate(expression, file, compact, show_ast).await,
        FfqCommands::Repl { show_ast, compact } => handle_repl(show_ast, compact).await,
    }
}

async fn handle_parse(
    expression: Option<String>,
    file: Option<String>,
    pretty: bool,
) -> Result<()> {
    let input = get_input(expression, file).await?;

    match parse_start(&input) {
        Ok((rest, ast)) => {
            if pretty {
                println!("{ast:#?}");
            } else {
                println!("{ast:?}");
            }

            if !rest.trim().is_empty() {
                eprintln!("Warning: Unparsed input remaining: {rest:?}");
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e:?}");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_translate(
    expression: Option<String>,
    file: Option<String>,
    compact: bool,
    show_ast: bool,
) -> Result<()> {
    let input = get_input(expression, file).await?;

    match parse_start(&input) {
        Ok((rest, ast)) => {
            if show_ast {
                println!("AST:");
                println!("{ast:#?}");
                println!("\nFHIR ValueSet.compose:");
            }

            let compose = translate_to_fhir(&ast);
            let json = if compact {
                serde_json::to_string(&compose)?
            } else {
                serde_json::to_string_pretty(&compose)?
            };

            println!("{json}");

            if !rest.trim().is_empty() {
                eprintln!("Warning: Unparsed input remaining: {rest:?}");
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e:?}");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_repl(show_ast: bool, compact: bool) -> Result<()> {
    println!("FFQ REPL - FHIR Filter Query Interactive Shell");
    println!("Type 'help' for commands, 'exit' or Ctrl+C to quit");
    println!("Enter FFQ expressions to see parsed AST and FHIR translation\n");

    let mut rl = DefaultEditor::new()?;

    // Load history if it exists
    let history_path = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(|home| {
            let mut path = std::path::PathBuf::from(home);
            path.push(".rh_ffq_history");
            path
        });

    if let Some(ref path) = history_path {
        let _ = rl.load_history(path); // Ignore errors if file doesn't exist
    }

    loop {
        let readline = rl.readline("ffq> ");
        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                match line {
                    "exit" | "quit" => break,
                    "help" => {
                        print_repl_help();
                        continue;
                    }
                    "clear" => {
                        print!("\x1B[2J\x1B[1;1H"); // Clear screen
                        continue;
                    }
                    _ => {
                        process_repl_input(line, show_ast, compact);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted - use 'exit' to quit");
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }

    // Save history
    if let Some(ref path) = history_path {
        let _ = rl.save_history(path); // Ignore errors
    }

    println!("Goodbye!");
    Ok(())
}

fn process_repl_input(input: &str, show_ast: bool, compact: bool) {
    match parse_start(input) {
        Ok((rest, ast)) => {
            if show_ast {
                println!("AST:");
                println!("{ast:#?}");
                println!();
            }

            let compose = translate_to_fhir(&ast);
            match if compact {
                serde_json::to_string(&compose)
            } else {
                serde_json::to_string_pretty(&compose)
            } {
                Ok(json) => println!("{json}"),
                Err(e) => eprintln!("JSON serialization error: {e}"),
            }

            if !rest.trim().is_empty() {
                eprintln!("Warning: Unparsed input remaining: {rest:?}");
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e:?}");
        }
    }
    println!(); // Add spacing
}

fn print_repl_help() {
    println!("FFQ REPL Commands:");
    println!("  help          - Show this help message");
    println!("  clear         - Clear the screen");
    println!("  exit, quit    - Exit the REPL");
    println!("  Ctrl+C        - Interrupt current input");
    println!();
    println!("FFQ Syntax Examples:");
    println!("  http://snomed.info/sct: < 22298006");
    println!("  http://loinc.org: component = \"Glucose\"");
    println!("  @alias sct = http://snomed.info/sct");
    println!("  sct: << 73211009 & component = \"test\"");
    println!();
}

async fn get_input(expression: Option<String>, file: Option<String>) -> Result<String> {
    if let Some(filename) = file {
        // Read from file
        Ok(fs::read_to_string(filename)?)
    } else if let Some(expr) = expression {
        // Use provided expression
        Ok(expr)
    } else {
        // Read from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(input)
    }
}
