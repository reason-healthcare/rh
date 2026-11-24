use anyhow::Result;
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::{value::RawValue, Value};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::{error, info};

use crate::output::{Envelope, OutputContext, OutputFormat};
use rh_fhirpath::{EvaluationContext, FhirModel, FhirPathEvaluator, FhirPathParser};

fn print_envelope<T: Serialize>(ctx: &OutputContext, envelope: &Envelope<T>) -> Result<()> {
    let json = if matches!(ctx.format, OutputFormat::Json) {
        serde_json::to_string_pretty(envelope)?
    } else {
        serde_json::to_string(envelope)?
    };
    println!("{json}");
    Ok(())
}

#[derive(Args)]
pub struct EvalOptions {
    /// FHIR release used for model-aware navigation
    #[clap(long, default_value = "r4")]
    fhir_version: FhirModel,

    /// Select the initial focus with a FHIRPath expression evaluated from the resource
    #[clap(long)]
    context: Option<String>,

    /// Enforce model-aware navigation rules
    #[clap(long)]
    strict: bool,

    /// Reject order-sensitive functions on unordered results
    #[clap(long)]
    check_ordered_functions: bool,
}

#[derive(Subcommand)]
pub enum FhirpathCommands {
    /// Parse a FHIRPath expression and show the AST
    Parse {
        /// FHIRPath expression to parse
        expression: String,

        /// Display format: pretty, json, debug
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,
    },
    /// Evaluate a FHIRPath expression against FHIR data
    Eval {
        /// FHIRPath expression to evaluate
        expression: String,

        /// Path to JSON file containing FHIR data
        #[clap(short, long)]
        data: Option<PathBuf>,

        /// Display format: pretty, json, debug
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        #[clap(flatten)]
        options: EvalOptions,
    },
    /// Interactive REPL for FHIRPath expressions
    Repl {
        /// Path to JSON file containing FHIR data
        #[clap(short, long)]
        data: Option<PathBuf>,
    },
    /// Test FHIRPath expressions from a file
    Test {
        /// Path to file containing test cases
        #[clap(short, long)]
        file: PathBuf,

        /// Path to JSON file containing FHIR data
        #[clap(short, long)]
        data: Option<PathBuf>,
    },
}

pub async fn handle_command(cmd: FhirpathCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        FhirpathCommands::Parse {
            expression,
            display_format,
        } => {
            parse_expression(&expression, &display_format, ctx)?;
        }
        FhirpathCommands::Eval {
            expression,
            data,
            display_format,
            options,
        } => {
            eval_expression(&expression, data.as_deref(), &display_format, &options, ctx)?;
        }
        FhirpathCommands::Repl { data } => {
            rh_fhirpath::repl::run_repl(data.as_deref())?;
        }
        FhirpathCommands::Test { file, data } => {
            run_tests(&file, data.as_deref()).await?;
        }
    }

    Ok(())
}

/// Parse a FHIRPath expression and display the AST
fn parse_expression(expression: &str, format: &str, ctx: &OutputContext) -> Result<()> {
    info!("Parsing FHIRPath expression: {}", expression);

    let parser = FhirPathParser::new();

    match parser.parse(expression) {
        Ok(ast) => {
            if ctx.is_json() {
                return print_envelope(ctx, &Envelope::ok(ast, "fhirpath parse"));
            }
            match format {
                "json" => {
                    let json = serde_json::to_string_pretty(&ast)?;
                    println!("{json}");
                }
                "debug" => {
                    println!("{ast:#?}");
                }
                "pretty" => {
                    println!("✅ Successfully parsed: {expression}");
                    println!("AST: {ast}");
                }
                _ => {
                    println!("✅ Successfully parsed: {expression}");
                    println!("AST: {ast}");
                }
            }
        }
        Err(e) => {
            error!("❌ Parse error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

/// Evaluate a FHIRPath expression against FHIR data
fn eval_expression(
    expression: &str,
    data_file: Option<&std::path::Path>,
    format: &str,
    options: &EvalOptions,
    ctx: &OutputContext,
) -> Result<()> {
    info!("Evaluating FHIRPath expression: {}", expression);

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Parse the expression
    let ast = parser.parse(expression).map_err(|e| {
        error!("❌ Parse error: {}", e);
        e
    })?;

    let mut context = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        EvaluationContext::from_json_str(&content)?
    } else {
        info!("No data file provided, using empty context");
        EvaluationContext::new(Value::Null)
    };

    context.model = options.fhir_version;
    context.strict_mode = options.strict;
    context.check_ordered_functions = options.check_ordered_functions;
    if let Some(focus) = &options.context {
        let focus = evaluator.evaluate_nodes(&parser.parse(focus)?, &context)?;
        context = context.with_initial_focus(focus);
    }

    // Evaluate the expression
    match evaluator.evaluate(&ast, &context) {
        Ok(result) => {
            // Display trace logs if any
            let trace_logs = context.get_trace_logs();
            if !trace_logs.is_empty() {
                eprintln!("\n📋 Trace logs:");
                for log in trace_logs {
                    eprintln!("  [TRACE:{}] {}", log.name, log.value);
                }
                eprintln!();
            }

            if ctx.is_json() {
                return print_envelope(ctx, &Envelope::ok(result.as_json(), "fhirpath eval"));
            }

            match format {
                "json" => {
                    let json = serde_json::to_string_pretty(&result.as_json())?;
                    println!("{json}");
                }
                "debug" => {
                    println!("{result:#?}");
                }
                _ => {
                    println!("✅ Expression: {expression}");
                    println!("Result: {result:?}");
                }
            }
        }
        Err(e) => {
            error!("❌ Evaluation error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

// (REPL moved to rh-fhirpath::repl; see crates/rh-fhirpath/src/repl.rs)

/// Run test cases from a file
async fn run_tests(test_file: &std::path::Path, data_file: Option<&std::path::Path>) -> Result<()> {
    info!("Running FHIRPath tests from: {}", test_file.display());

    let test_content = fs::read_to_string(test_file)?;
    let test_cases: Vec<TestCase> = serde_json::from_str(&test_content)?;

    let context = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        EvaluationContext::from_json_str(&content)?
    } else {
        EvaluationContext::new(Value::Null)
    };

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let mut passed = 0;
    let mut failed = 0;

    for (i, test_case) in test_cases.iter().enumerate() {
        print!("Test {}: {} ... ", i + 1, test_case.expression);
        io::stdout().flush()?;

        match parser.parse(&test_case.expression) {
            Ok(ast) => match evaluator.evaluate(&ast, &context) {
                Ok(result) => {
                    let expected = EvaluationContext::from_json_str(test_case.expected.get())?;
                    if result.to_raw_json()?.get()
                        == serde_json::to_string(&expected.root_as_json())?
                    {
                        println!("✅ PASS");
                        passed += 1;
                    } else {
                        println!("❌ FAIL");
                        println!(
                            "  Expected: {}",
                            serde_json::to_string_pretty(&expected.root_as_json())?
                        );
                        println!(
                            "  Got:      {}",
                            serde_json::to_string_pretty(&result.as_json())?
                        );
                        failed += 1;
                    }
                }
                Err(e) => {
                    if test_case.expected.get() == "null" && test_case.should_error.unwrap_or(false)
                    {
                        println!("✅ PASS (expected error)");
                        passed += 1;
                    } else {
                        println!("❌ FAIL (evaluation error)");
                        println!("  Error: {e}");
                        failed += 1;
                    }
                }
            },
            Err(e) => {
                if test_case.should_error.unwrap_or(false) {
                    println!("✅ PASS (expected parse error)");
                    passed += 1;
                } else {
                    println!("❌ FAIL (parse error)");
                    println!("  Error: {e}");
                    failed += 1;
                }
            }
        }
    }

    println!();
    println!("Test Results: {passed} passed, {failed} failed");

    if failed > 0 {
        crate::output::ExitCode::ValidationFailure.exit();
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct TestCase {
    expression: String,
    expected: Box<RawValue>,
    #[serde(default)]
    should_error: Option<bool>,
}
