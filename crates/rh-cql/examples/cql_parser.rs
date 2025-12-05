//! CQL Parser Example
//!
//! Demonstrates parsing CQL source code into an AST using the CqlParser.
//!
//! Run with: cargo run -p rh-cql --example cql_parser

use rh_cql::parser::ast::{Expression, Literal, Statement};
use rh_cql::parser::CqlParser;

fn main() {
    println!("=== CQL Parser Example ===\n");

    // Create a parser
    let parser = CqlParser::new();

    // Example 1: Parse a complete library
    println!("1. Parsing a complete CQL library:\n");
    let library_source = r#"
library DiabetesManagement version '1.0.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1' called FHIRHelpers

codesystem LOINC: 'http://loinc.org'
valueset DiabetesDiagnosis: 'http://example.org/valueset/diabetes-dx'
code Hemoglobin: '4548-4' from LOINC display 'Hemoglobin A1c'

parameter MeasurementPeriod Interval<DateTime>

context Patient

define InPatient:
    true

define SumOfValues:
    1 + 2 + 3

define function IsAdult(age Integer) returns Boolean:
    age >= 18
"#;

    match parser.parse(library_source) {
        Ok(library) => {
            println!("✓ Successfully parsed library!");
            if let Some(id) = &library.identifier {
                println!("  Library: {} version {:?}", id.name, id.version);
            }
            println!("  Using definitions: {}", library.usings.len());
            for using in &library.usings {
                println!("    - {} version {:?}", using.model_name, using.version);
            }
            println!("  Include definitions: {}", library.includes.len());
            println!("  CodeSystem definitions: {}", library.codesystems.len());
            println!("  ValueSet definitions: {}", library.valuesets.len());
            println!("  Code definitions: {}", library.codes.len());
            println!("  Parameters: {}", library.parameters.len());
            println!("  Contexts: {}", library.contexts.len());
            println!("  Statements: {}", library.statements.len());
            for stmt in &library.statements {
                match stmt {
                    Statement::ExpressionDef(e) => println!("    - define {}", e.name),
                    Statement::FunctionDef(f) => println!(
                        "    - define function {}({} params)",
                        f.name,
                        f.parameters.len()
                    ),
                }
            }
        }
        Err(e) => println!("✗ Error: {e}"),
    }

    // Example 2: Parse individual expressions
    println!("\n2. Parsing individual expressions:\n");

    let expressions = [
        ("42", "Integer literal"),
        ("3.14", "Decimal literal"),
        ("'Hello World'", "String literal"),
        ("true", "Boolean literal"),
        ("null", "Null literal"),
        ("@2024-01-15", "Date literal"),
        ("@2024-01-15T10:30:00Z", "DateTime literal"),
        ("@T14:30:00", "Time literal"),
        ("100 'mg'", "Quantity literal"),
        ("1 + 2 * 3", "Arithmetic with precedence"),
        ("a and b or c", "Logical operators"),
        ("x > 5 and x < 10", "Comparison chain"),
        (
            "if x > 0 then 'positive' else 'non-positive'",
            "Conditional",
        ),
        ("{ 1, 2, 3 }", "List literal"),
        ("Patient.name", "Member access"),
        ("items[0]", "Index access"),
        ("Sum(values)", "Function call"),
        ("FHIRHelpers.ToQuantity(x)", "Qualified function call"),
        ("not exists items", "Unary operators"),
        ("x is Integer", "Type check"),
        ("value as FHIR.Quantity", "Type cast"),
    ];

    for (source, description) in expressions {
        match parser.parse_expression(source) {
            Ok(expr) => {
                let kind = expression_kind(&expr);
                println!("✓ {description}: {source} -> {kind}");
            }
            Err(e) => println!("✗ {description}: {source} -> Error: {e}"),
        }
    }

    // Example 3: Interval expressions
    println!("\n3. Interval and collection expressions:\n");

    let collection_examples = [
        ("Interval[1, 10]", "Closed interval"),
        ("Interval(1, 10)", "Open interval"),
        ("Interval[1, 10)", "Half-open interval"),
        ("{ 'a', 'b', 'c' }", "String list"),
        ("Tuple { name: 'John', age: 30 }", "Tuple expression"),
    ];

    for (source, description) in collection_examples {
        match parser.parse_expression(source) {
            Ok(_) => println!("✓ {description}: {source}"),
            Err(e) => println!("✗ {description}: {source} -> Error: {e}"),
        }
    }

    // Example 4: Operator precedence demonstration
    println!("\n4. Operator precedence:\n");

    // Show how operators bind
    let precedence_examples = [
        ("1 + 2 * 3", "Multiplication binds tighter: 1 + (2 * 3) = 7"),
        ("a or b and c", "AND binds tighter: a or (b and c)"),
        ("not a and b", "NOT binds tightest: (not a) and b"),
        ("x + y > z", "Comparison after arithmetic: (x + y) > z"),
    ];

    for (source, explanation) in precedence_examples {
        match parser.parse_expression(source) {
            Ok(_) => println!("  {source} => {explanation}"),
            Err(e) => println!("  {source} => Error: {e}"),
        }
    }

    println!("\n=== Example Complete ===");
}

fn expression_kind(expr: &Expression) -> &'static str {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Null => "Null",
            Literal::Boolean(_) => "Boolean",
            Literal::Integer(_) => "Integer",
            Literal::Long(_) => "Long",
            Literal::Decimal(_) => "Decimal",
            Literal::String(_) => "String",
            Literal::Date(_) => "Date",
            Literal::DateTime(_) => "DateTime",
            Literal::Time(_) => "Time",
            Literal::Quantity { .. } => "Quantity",
            Literal::Ratio { .. } => "Ratio",
            Literal::Code { .. } => "Code",
        },
        Expression::IdentifierRef(_) => "IdentifierRef",
        Expression::QualifiedIdentifierRef(_) => "QualifiedIdentifierRef",
        Expression::UnaryExpression(_) => "UnaryExpression",
        Expression::BinaryExpression(_) => "BinaryExpression",
        Expression::TernaryExpression(_) => "TernaryExpression",
        Expression::TypeExpression(_) => "TypeExpression",
        Expression::FunctionInvocation(_) => "FunctionInvocation",
        Expression::MemberInvocation(_) => "MemberInvocation",
        Expression::IndexInvocation(_) => "IndexInvocation",
        Expression::Query(_) => "Query",
        Expression::Retrieve(_) => "Retrieve",
        Expression::IfThenElse(_) => "IfThenElse",
        Expression::Case(_) => "Case",
        Expression::IntervalExpression(_) => "IntervalExpression",
        Expression::ListExpression(_) => "ListExpression",
        Expression::TupleExpression(_) => "TupleExpression",
        Expression::Instance(_) => "Instance",
        Expression::Let(_) => "Let",
        Expression::Parenthesized(_) => "Parenthesized",
    }
}
