use anyhow::Result;
use clap::Subcommand;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::{error, info};

use crate::output::{ExitCode, Format, OutputContext};

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};

fn fhirpath_value_to_json(value: &FhirPathValue) -> Value {
    match value {
        FhirPathValue::Boolean(b) => Value::Bool(*b),
        FhirPathValue::String(s) => Value::String(s.clone()),
        FhirPathValue::Number(n) => {
            if let Some(json_num) = serde_json::Number::from_f64(*n) {
                Value::Number(json_num)
            } else {
                Value::Null
            }
        }
        FhirPathValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
        FhirPathValue::Long(l) => Value::Number(serde_json::Number::from(*l)),
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) | FhirPathValue::Time(s) => {
            Value::String(s.clone())
        }
        FhirPathValue::Quantity { value, unit } => {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "value".to_string(),
                Value::Number(
                    serde_json::Number::from_f64(*value)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                ),
            );
            if let Some(u) = unit {
                obj.insert("unit".to_string(), Value::String(u.clone()));
            }
            Value::Object(obj)
        }
        FhirPathValue::DateTimePrecision(precision) => Value::String(precision.to_string()),
        FhirPathValue::Collection(items) => {
            let json_items: Vec<Value> = items.iter().map(fhirpath_value_to_json).collect();
            Value::Array(json_items)
        }
        FhirPathValue::Object(obj) => obj.clone(),
        FhirPathValue::Empty => Value::Null,
    }
}

#[derive(Subcommand)]
pub enum FhirpathCommands {
    /// Parse a FHIRPath expression and show the AST
    Parse {
        /// FHIRPath expression to parse
        expression: String,

        /// Output format: pretty, json, debug
        #[clap(short, long, default_value = "pretty")]
        format: String,
    },
    /// Evaluate a FHIRPath expression against FHIR data
    Eval {
        /// FHIRPath expression to evaluate
        expression: String,

        /// Path to JSON file containing FHIR data
        #[clap(short, long)]
        data: Option<PathBuf>,

        /// Output format: pretty, json, debug
        #[clap(short, long, default_value = "pretty")]
        format: String,
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

pub async fn handle_command(cmd: FhirpathCommands, ctx: &OutputContext) -> Result<ExitCode> {
    match cmd {
        FhirpathCommands::Parse { expression, format } => {
            parse_expression(&expression, &format, ctx)?;
        }
        FhirpathCommands::Eval {
            expression,
            data,
            format,
        } => {
            eval_expression(&expression, data.as_deref(), &format, ctx)?;
        }
        FhirpathCommands::Repl { data } => {
            run_repl(data.as_deref()).await?;
        }
        FhirpathCommands::Test { file, data } => {
            let code = run_tests(&file, data.as_deref(), ctx).await?;
            return Ok(code);
        }
    }

    Ok(ExitCode::Success)
}

fn parse_expression(expression: &str, format: &str, ctx: &OutputContext) -> Result<()> {
    info!("Parsing FHIRPath expression: {}", expression);

    let parser = FhirPathParser::new();

    match parser.parse(expression) {
        Ok(ast) => match ctx.format {
            Format::Json | Format::Ndjson => {
                let result = serde_json::to_value(&ast)?;
                ctx.write_success(result)?;
            }
            Format::Human => match format {
                "json" => {
                    let json = serde_json::to_string_pretty(&ast)?;
                    println!("{json}");
                }
                "debug" => {
                    println!("{ast:#?}");
                }
                _ => {
                    println!("OK: {}", expression);
                    println!("AST: {ast}");
                }
            },
        },
        Err(e) => {
            error!("Parse error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn eval_expression(
    expression: &str,
    data_file: Option<&std::path::Path>,
    format: &str,
    ctx: &OutputContext,
) -> Result<()> {
    info!("Evaluating FHIRPath expression: {}", expression);

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let ast = parser.parse(expression).map_err(|e| {
        error!("Parse error: {}", e);
        e
    })?;

    let data = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Value>(&content)?
    } else {
        info!("No data file provided, using empty context");
        Value::Null
    };

    let context = EvaluationContext::new(data);

    match evaluator.evaluate(&ast, &context) {
        Ok(result) => {
            let trace_logs = context.get_trace_logs();
            if !trace_logs.is_empty() {
                eprintln!("Trace logs:");
                for log in trace_logs {
                    eprintln!("  [TRACE:{}] {}", log.name, log.value);
                }
            }

            match ctx.format {
                Format::Json | Format::Ndjson => {
                    let json_value = fhirpath_value_to_json(&result);
                    ctx.write_success(json_value)?;
                }
                Format::Human => match format {
                    "json" => {
                        let json_value = fhirpath_value_to_json(&result);
                        let json = serde_json::to_string_pretty(&json_value)?;
                        println!("{json}");
                    }
                    "debug" => {
                        println!("{result:#?}");
                    }
                    _ => {
                        println!("Result: {result:?}");
                    }
                },
            }
        }
        Err(e) => {
            error!("Evaluation error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn run_repl(data_file: Option<&std::path::Path>) -> Result<()> {
    println!("FHIRPath Interactive REPL");
    println!("Type FHIRPath expressions to evaluate them.");
    println!("Commands: .help, .data, .quit");
    println!("Use arrow keys to navigate command history (persistent across sessions).");
    println!();

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let mut data = if let Some(path) = data_file {
        info!("Loading FHIR data from: {}", path.display());
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Value>(&content)?
    } else {
        Value::Null
    };

    let mut rl = DefaultEditor::new()?;

    let history_file = std::env::temp_dir().join("fhirpath_repl_history.txt");
    let _ = rl.load_history(&history_file);

    loop {
        let readline = rl.readline("fhirpath> ");

        match readline {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(input)?;

                match input {
                    ".quit" | ".exit" => {
                        println!("Goodbye!");
                        break;
                    }
                    ".help" => {
                        print_repl_help();
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
                                println!("Loaded data from: {path}");
                            }
                            Err(e) => {
                                println!("Failed to load data: {e}");
                            }
                        }
                        continue;
                    }
                    cmd if cmd.starts_with(".") => {
                        println!("Unknown command: {cmd}. Type .help for available commands.");
                        continue;
                    }
                    _ => {}
                }

                match parser.parse(input) {
                    Ok(ast) => {
                        let context = EvaluationContext::new(data.clone());
                        match evaluator.evaluate(&ast, &context) {
                            Ok(result) => {
                                let trace_logs = context.get_trace_logs();
                                if !trace_logs.is_empty() {
                                    println!("Trace logs:");
                                    for log in trace_logs {
                                        println!("  [TRACE:{}] {}", log.name, log.value);
                                    }
                                }
                                println!("=> {result:?}");
                            }
                            Err(e) => {
                                println!("Evaluation error: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        println!("Parse error: {e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }

    let _ = rl.save_history(&history_file);

    Ok(())
}

fn load_data_file(path: &str) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    let data = serde_json::from_str::<Value>(&content)?;
    Ok(data)
}

fn print_repl_help() {
    println!("FHIRPath REPL Commands:");
    println!("  .help                Show this help");
    println!("  .data                Show currently loaded data");
    println!("  .load <file>         Load FHIR data from JSON file");
    println!("  .quit, .exit         Exit the REPL");
    println!();
    println!("Navigation:");
    println!("  Up/Down arrows       Navigate command history");
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

async fn run_tests(
    test_file: &std::path::Path,
    data_file: Option<&std::path::Path>,
    ctx: &OutputContext,
) -> Result<ExitCode> {
    info!("Running FHIRPath tests from: {}", test_file.display());

    let test_content = fs::read_to_string(test_file)?;
    let test_cases: Vec<TestCase> = serde_json::from_str(&test_content)?;

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
    let mut results = Vec::new();

    for (i, test_case) in test_cases.iter().enumerate() {
        let _label = format!("Test {}: {}", i + 1, test_case.expression);

        match parser.parse(&test_case.expression) {
            Ok(ast) => match evaluator.evaluate(&ast, &context) {
                Ok(result) => {
                    let result_json = fhirpath_value_to_json(&result);
                    if result_json == test_case.expected {
                        passed += 1;
                        results.push(serde_json::json!({
                            "test": i + 1,
                            "expression": test_case.expression,
                            "status": "pass",
                        }));
                        if ctx.format == Format::Human {
                            print!("Test {}: {} ... ", i + 1, test_case.expression);
                            io::stdout().flush()?;
                            println!("PASS");
                        }
                    } else {
                        failed += 1;
                        results.push(serde_json::json!({
                            "test": i + 1,
                            "expression": test_case.expression,
                            "status": "fail",
                            "expected": test_case.expected,
                            "actual": result_json,
                        }));
                        if ctx.format == Format::Human {
                            print!("Test {}: {} ... ", i + 1, test_case.expression);
                            io::stdout().flush()?;
                            println!("FAIL");
                            eprintln!(
                                "  Expected: {}",
                                serde_json::to_string_pretty(&test_case.expected)?
                            );
                            eprintln!(
                                "  Got:      {}",
                                serde_json::to_string_pretty(&result_json)?
                            );
                        }
                    }
                }
                Err(e) => {
                    if test_case.expected.is_null() && test_case.should_error.unwrap_or(false) {
                        passed += 1;
                        results.push(serde_json::json!({
                            "test": i + 1,
                            "expression": test_case.expression,
                            "status": "pass",
                            "note": "expected error",
                        }));
                        if ctx.format == Format::Human {
                            print!("Test {}: {} ... ", i + 1, test_case.expression);
                            io::stdout().flush()?;
                            println!("PASS (expected error)");
                        }
                    } else {
                        failed += 1;
                        results.push(serde_json::json!({
                            "test": i + 1,
                            "expression": test_case.expression,
                            "status": "fail",
                            "error": format!("evaluation error: {e}"),
                        }));
                        if ctx.format == Format::Human {
                            print!("Test {}: {} ... ", i + 1, test_case.expression);
                            io::stdout().flush()?;
                            println!("FAIL (evaluation error)");
                            eprintln!("  Error: {e}");
                        }
                    }
                }
            },
            Err(e) => {
                if test_case.should_error.unwrap_or(false) {
                    passed += 1;
                    results.push(serde_json::json!({
                        "test": i + 1,
                        "expression": test_case.expression,
                        "status": "pass",
                        "note": "expected parse error",
                    }));
                    if ctx.format == Format::Human {
                        print!("Test {}: {} ... ", i + 1, test_case.expression);
                        io::stdout().flush()?;
                        println!("PASS (expected parse error)");
                    }
                } else {
                    failed += 1;
                    results.push(serde_json::json!({
                        "test": i + 1,
                        "expression": test_case.expression,
                        "status": "fail",
                        "error": format!("parse error: {e}"),
                    }));
                    if ctx.format == Format::Human {
                        print!("Test {}: {} ... ", i + 1, test_case.expression);
                        io::stdout().flush()?;
                        println!("FAIL (parse error)");
                        eprintln!("  Error: {e}");
                    }
                }
            }
        }
    }

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "passed": passed,
                "failed": failed,
                "total": passed + failed,
                "results": results,
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!();
            println!("Test Results: {passed} passed, {failed} failed");
        }
    }

    if failed > 0 {
        return Ok(ExitCode::ValidationError);
    }

    Ok(ExitCode::Success)
}

#[derive(serde::Deserialize)]
struct TestCase {
    expression: String,
    expected: Value,
    #[serde(default)]
    should_error: Option<bool>,
}
