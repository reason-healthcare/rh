use anyhow::Result;
use clap::{Parser, Subcommand};
use rh_ffq::{parse_start, translate_to_fhir};
use rh_foundation::cli;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

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
    println!("Enter alias definitions with '@alias name = value' or FFQ expressions");
    println!("Aliases persist until 'clear' or 'reset' command\n");

    let mut rl = DefaultEditor::new()?;
    let mut aliases = Vec::<String>::new(); // Store alias lines

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
                    "reset" => {
                        aliases.clear();
                        println!("Aliases cleared");
                        continue;
                    }
                    "aliases" => {
                        if aliases.is_empty() {
                            println!("No aliases defined");
                        } else {
                            println!("Current aliases:");
                            for alias in &aliases {
                                println!("  {alias}");
                            }
                        }
                        continue;
                    }
                    _ => {
                        process_repl_input(line, &mut aliases, show_ast, compact);
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

fn process_repl_input(input: &str, aliases: &mut Vec<String>, show_ast: bool, compact: bool) {
    // Check if this is an alias definition
    if input.trim_start().starts_with("@alias") {
        aliases.push(input.to_string());
        println!("Alias added: {}", input.trim());
        return;
    }

    // Build complete input with all aliases
    let mut complete_input = String::new();
    for alias in aliases {
        complete_input.push_str(alias);
        complete_input.push('\n');
    }
    complete_input.push_str(input);

    match parse_start(&complete_input) {
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
    println!("  reset         - Clear all stored aliases");
    println!("  aliases       - Show currently stored aliases");
    println!("  exit, quit    - Exit the REPL");
    println!();
    println!("Usage:");
    println!("  @alias name = value   - Define an alias (persists until reset)");
    println!("  expression            - Evaluate FFQ expression with current aliases");
    println!("  Ctrl+C        - Interrupt current input");
    println!();
    println!("FFQ Syntax Examples:");
    println!("  http://snomed.info/sct: < 22298006");
    println!("  http://loinc.org: component = \"Glucose\"");
    println!();
    println!("REPL Alias Examples:");
    println!("  @alias sct = http://snomed.info/sct");
    println!("  @alias dm = vs(https://example.org/fhir/ValueSet/diabetes)");
    println!("  sct: << 73211009 & component = \"test\"");
    println!("  sct: << 22298006 - << 1755008");
    println!();
}

async fn get_input(expression: Option<String>, file: Option<String>) -> Result<String> {
    cli::read_input(file.as_deref(), expression).await.map_err(Into::into)
}
