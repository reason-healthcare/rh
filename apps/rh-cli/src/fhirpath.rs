use anyhow::Result;
use clap::Subcommand;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::{error, info};

use crate::output::{Envelope, OutputContext, OutputFormat};
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};

fn print_envelope<T: Serialize>(ctx: &OutputContext, envelope: &Envelope<T>) -> Result<()> {
    let json = if matches!(ctx.format, OutputFormat::Json) {
        serde_json::to_string_pretty(envelope)?
    } else {
        serde_json::to_string(envelope)?
    };
    println!("{json}");
    Ok(())
}

/// Convert FhirPathValue to JSON Value for serialization
fn fhirpath_value_to_json(value: &FhirPathValue) -> Value {
    match value {
        FhirPathValue::Boolean(b) => Value::Bool(*b),
        FhirPathValue::TypedBoolean { value, .. } => Value::Bool(*value),
        FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
            Value::String(s.clone())
        }
        FhirPathValue::Number(n) => {
            use rust_decimal::prelude::ToPrimitive;
            if let Some(f) = n.to_f64() {
                if let Some(json_num) = serde_json::Number::from_f64(f) {
                    Value::Number(json_num)
                } else {
                    Value::Null
                }
            } else {
                Value::Null
            }
        }
        FhirPathValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
        FhirPathValue::Long(l) => Value::Number(serde_json::Number::from(*l)),
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) => Value::String(s.clone()),
        FhirPathValue::Time(s) => Value::String(s.strip_prefix('T').unwrap_or(s).to_string()),
        FhirPathValue::TypedDateTime { value, .. } => Value::String(value.clone()),
        FhirPathValue::Quantity { value, unit } => {
            use rust_decimal::prelude::ToPrimitive;
            let mut obj = serde_json::Map::new();
            obj.insert(
                "value".to_string(),
                Value::Number(
                    value
                        .to_f64()
                        .and_then(serde_json::Number::from_f64)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                ),
            );
            if let Some(u) = unit {
                obj.insert("unit".to_string(), Value::String(u.clone()));
            }
            Value::Object(obj)
        }
        FhirPathValue::DateTimePrecision(precision) => Value::String(precision.to_string()),
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
            let json_items: Vec<Value> = items.iter().map(fhirpath_value_to_json).collect();
            Value::Array(json_items)
        }
        FhirPathValue::Object(obj) => obj.clone(),
        FhirPathValue::TypedObject { value, .. } => value.clone(),
        FhirPathValue::FhirPrimitive { inner, .. } => fhirpath_value_to_json(inner),
        FhirPathValue::Empty => Value::Null,
    }
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
        } => {
            eval_expression(&expression, data.as_deref(), &display_format, ctx)?;
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

    // Load data if provided
    let data = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Value>(&content)?
    } else {
        info!("No data file provided, using empty context");
        Value::Null
    };

    // Create evaluation context
    let context = EvaluationContext::new(data);

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

            match format {
                "json" => {
                    let json_value = fhirpath_value_to_json(&result);
                    if ctx.is_json() {
                        print_envelope(ctx, &Envelope::ok(json_value, "fhirpath eval"))?;
                    } else {
                        let json = serde_json::to_string_pretty(&json_value)?;
                        println!("{json}");
                    }
                }
                "debug" => {
                    if ctx.is_json() {
                        let json_value = fhirpath_value_to_json(&result);
                        print_envelope(ctx, &Envelope::ok(json_value, "fhirpath eval"))?;
                    } else {
                        println!("{result:#?}");
                    }
                }
                "pretty" => {
                    if ctx.is_json() {
                        let json_value = fhirpath_value_to_json(&result);
                        print_envelope(ctx, &Envelope::ok(json_value, "fhirpath eval"))?;
                    } else {
                        println!("✅ Expression: {expression}");
                        println!("Result: {result:?}");
                    }
                }
                _ => {
                    if ctx.is_json() {
                        let json_value = fhirpath_value_to_json(&result);
                        print_envelope(ctx, &Envelope::ok(json_value, "fhirpath eval"))?;
                    } else {
                        println!("✅ Expression: {expression}");
                        println!("Result: {result:?}");
                    }
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

    // Load data if provided
    let data = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Value>(&content)?
    } else {
        Value::Null
    };

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(data);

    let mut passed = 0;
    let mut failed = 0;

    for (i, test_case) in test_cases.iter().enumerate() {
        print!("Test {}: {} ... ", i + 1, test_case.expression);
        io::stdout().flush()?;

        match parser.parse(&test_case.expression) {
            Ok(ast) => match evaluator.evaluate(&ast, &context) {
                Ok(result) => {
                    let result_json = fhirpath_value_to_json(&result);
                    if result_json == test_case.expected {
                        println!("✅ PASS");
                        passed += 1;
                    } else {
                        println!("❌ FAIL");
                        println!(
                            "  Expected: {}",
                            serde_json::to_string_pretty(&test_case.expected)?
                        );
                        println!(
                            "  Got:      {}",
                            serde_json::to_string_pretty(&result_json)?
                        );
                        failed += 1;
                    }
                }
                Err(e) => {
                    if test_case.expected.is_null() && test_case.should_error.unwrap_or(false) {
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
    expected: Value,
    #[serde(default)]
    should_error: Option<bool>,
}
