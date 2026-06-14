//! Interactive REPL for VCL expressions.
//!
//! Enabled via the `repl` crate feature.

use anyhow::Result;
use rh_foundation::cli::repl::{ReplInput, ReplSession};

use crate::{parse_vcl, VclExplainer, VclExpression, VclTranslator};

/// Run an interactive REPL for VCL expressions.
pub fn run_repl(
    translate_mode: bool,
    explain_mode: bool,
    default_system: Option<String>,
) -> Result<()> {
    println!("🚀 VCL Interactive REPL");
    let mode_description = match (translate_mode, explain_mode) {
        (true, true) => ", translate, and explain",
        (true, false) => " and translate",
        (false, true) => " and explain",
        (false, false) => "",
    };
    println!("Type VCL expressions to parse{mode_description}. Type 'exit' or 'quit' to exit.");
    if let Some(ref system) = default_system {
        println!("Default code system: {system}");
    }
    println!("Commands: .help, .exit, .quit");
    println!();

    let mut session = ReplSession::new()?;

    loop {
        match session.readline("vcl> ")? {
            ReplInput::Line(line) => {
                let line = line.trim().to_owned();

                if line.is_empty() {
                    continue;
                }

                if line.starts_with('.') {
                    match line.as_str() {
                        ".help" => {
                            print_help(translate_mode, explain_mode, default_system.as_deref());
                        }
                        ".exit" | ".quit" => {
                            println!("Goodbye! 👋");
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
                    println!("Goodbye! 👋");
                    break;
                }

                session.add_history_entry(&line)?;

                match parse_vcl(&line) {
                    Ok(ast) => {
                        println!("✅ Parsed successfully:");
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
                                    println!("🔄 FHIR Translation:");
                                    let json = serde_json::to_string_pretty(&fhir_compose)?;
                                    println!("{}", indent_json(&json, 1));
                                }
                                Err(e) => {
                                    println!("❌ Translation error: {e}");
                                }
                            }
                        }

                        if explain_mode {
                            println!();
                            let explainer = VclExplainer::new();
                            match explainer.explain_with_text(&ast, &line) {
                                Ok(explanation_result) => {
                                    println!("💡 Explanation:");
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
                                                "    • {} ({}): {}",
                                                component.component,
                                                component.component_type,
                                                component.meaning
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("❌ Explanation error: {e}");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Parse error: {e}");
                    }
                }
                println!();
            }
            ReplInput::Interrupted => {
                println!("CTRL-C");
                break;
            }
            ReplInput::Eof => {
                println!("CTRL-D");
                break;
            }
        }
    }

    Ok(())
}

fn print_help(translate_mode: bool, explain_mode: bool, default_system: Option<&str>) {
    println!("VCL REPL Commands:");
    println!("  .help     - Show this help");
    println!("  .exit     - Exit the REPL");
    println!("  .quit     - Exit the REPL");
    println!();
    println!("Current REPL Mode:");
    println!("  Parse Mode: ✅ Always enabled");
    if translate_mode {
        println!("  Translation Mode: ✅ Enabled (shows FHIR ValueSet.compose)");
    } else {
        println!("  Translation Mode: ❌ Disabled (use --translate to enable)");
    }
    if explain_mode {
        println!("  Explanation Mode: ✅ Enabled (shows plain English explanations)");
    } else {
        println!("  Explanation Mode: ❌ Disabled (use --explain to enable)");
    }
    println!();
    println!("VCL Syntax Examples:");
    if default_system.is_some() {
        println!("  123456     - Simple code (uses default system)");
        println!("  *          - Wildcard (uses default system)");
        println!("  status = \"active\"  - Filter (uses default system)");
    }
    println!("  (http://snomed.info/sct)123456");
    println!("  (http://snomed.info/sct)*");
    println!("  (http://snomed.info/sct)status = \"active\"");
    println!("  (http://snomed.info/sct)category << 123456");
    println!("  (http://snomed.info/sct)123456, 789012  - Multiple codes");
    println!("  123456; 789012; 345678  - Disjunction (OR)");
    println!("  * - inactive  - Exclusion (NOT)");
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
