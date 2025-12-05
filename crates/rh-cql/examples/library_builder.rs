//! Example: Using the LibraryBuilder
//!
//! This example demonstrates how to use the LibraryBuilder for
//! CQL semantic analysis and identifier resolution.
//!
//! Run with: `cargo run --example library_builder`

use rh_cql::builder::{FunctionSignature, LibraryBuilder, Symbol, SymbolKind};
use rh_cql::datatype::{DataType, SystemType};
use rh_cql::elm;
use rh_cql::library::CompiledLibrary;
use rh_cql::parser::CqlParser;
use rh_cql::preprocessor::Preprocessor;

fn main() -> anyhow::Result<()> {
    println!("=== LibraryBuilder Example ===\n");

    // Example 1: Basic Builder Setup
    println!("1. Basic Builder Setup:");
    basic_setup_example();

    // Example 2: Scope Management
    println!("\n2. Scope Management:");
    scope_management_example();

    // Example 3: Identifier Resolution
    println!("\n3. Identifier Resolution:");
    identifier_resolution_example();

    // Example 4: Function Overloads
    println!("\n4. Function Overloads:");
    function_overload_example();

    // Example 5: From Library Info
    println!("\n5. From Preprocessed Library Info:");
    from_library_info_example()?;

    // Example 6: Included Libraries
    println!("\n6. Included Libraries:");
    included_library_example();

    println!("\n=== Example Complete ===");
    Ok(())
}

fn basic_setup_example() {
    let mut builder = LibraryBuilder::new();

    // Set library identity
    builder.set_library_name("MyMeasure");
    builder.set_library_version("1.0.0");

    println!(
        "   Library: {} v{:?}",
        builder.library_name().unwrap_or("?"),
        builder.library_version()
    );

    // Add using declaration
    builder.add_using("FHIR", "http://hl7.org/fhir");
    println!("   Using FHIR: {}", builder.has_model("FHIR"));
    println!("   FHIR URI: {:?}", builder.model_uri("FHIR"));

    // Set context
    builder.set_context("Patient");
    println!("   Context: {:?}", builder.context());

    // Context is automatically added as a symbol
    println!("   'Patient' is defined: {}", builder.is_defined("Patient"));
}

fn scope_management_example() {
    let mut builder = LibraryBuilder::new();

    // Library-level scope (depth 0)
    println!("   Initial scope depth: {}", builder.scope_depth());

    // Define a library-level symbol
    let expr_symbol = Symbol::new("InPopulation", SymbolKind::Expression)
        .with_type(DataType::system(SystemType::Boolean));
    builder.define_symbol(expr_symbol);

    // Push a query scope
    builder.push_scope();
    println!("   After push: depth = {}", builder.scope_depth());

    // Define a query alias
    let alias_symbol = Symbol::new("P", SymbolKind::QueryAlias).with_type(DataType::Model {
        namespace: "FHIR".to_string(),
        name: "Patient".to_string(),
    });
    builder.define_symbol(alias_symbol);

    // Both are accessible
    println!(
        "   'InPopulation' visible: {}",
        builder.is_defined("InPopulation")
    );
    println!("   'P' visible: {}", builder.is_defined("P"));

    // Pop back to library scope
    builder.pop_scope();
    println!("   After pop: depth = {}", builder.scope_depth());

    // 'P' is no longer visible
    println!("   'P' visible: {}", builder.is_defined("P"));
    println!(
        "   'InPopulation' visible: {}",
        builder.is_defined("InPopulation")
    );
}

fn identifier_resolution_example() {
    let mut builder = LibraryBuilder::new();

    // Define symbols at library level
    builder.define_library_symbol(
        Symbol::new("Age", SymbolKind::Expression).with_type(DataType::system(SystemType::Integer)),
    );
    builder.define_library_symbol(
        Symbol::new("MeasurementPeriod", SymbolKind::Parameter).with_type(DataType::Interval(
            Box::new(DataType::system(SystemType::DateTime)),
        )),
    );

    // Resolve an identifier
    if let Some(resolved) = builder.resolve_identifier("Age") {
        println!("   Resolved 'Age':");
        println!("      Kind: {}", resolved.symbol.kind);
        println!("      Type: {:?}", resolved.symbol.result_type);
        println!("      Scope depth: {}", resolved.scope_depth);
        println!("      Is local: {}", resolved.is_local());
    }

    // Try to resolve unknown identifier
    let unknown = builder.resolve_identifier("Unknown");
    println!("   'Unknown' resolved: {}", unknown.is_some());

    // Demonstrate shadowing
    builder.push_scope();
    builder.define_symbol(
        Symbol::new("Age", SymbolKind::Let).with_type(DataType::system(SystemType::String)),
    );

    if let Some(resolved) = builder.resolve_identifier("Age") {
        println!("   After shadowing, 'Age' kind: {}", resolved.symbol.kind);
    }
}

fn function_overload_example() {
    let mut builder = LibraryBuilder::new();

    // Define overloaded ToInteger function
    let sig1 = FunctionSignature::new("ToInteger", DataType::system(SystemType::Integer))
        .with_operands(vec![DataType::system(SystemType::String)]);

    let sig2 = FunctionSignature::new("ToInteger", DataType::system(SystemType::Integer))
        .with_operands(vec![DataType::system(SystemType::Decimal)]);

    let sig3 = FunctionSignature::new("ToInteger", DataType::system(SystemType::Integer))
        .with_operands(vec![DataType::system(SystemType::Boolean)]);

    builder.define_function(sig1);
    builder.define_function(sig2);
    builder.define_function(sig3);

    // Resolve all overloads
    let overloads = builder.resolve_function("ToInteger");
    println!("   'ToInteger' overloads: {}", overloads.len());

    for (i, sig) in overloads.iter().enumerate() {
        let operand_types: Vec<String> =
            sig.operand_types.iter().map(|t| format!("{t:?}")).collect();
        println!(
            "      {}: ({}) -> {:?}",
            i + 1,
            operand_types.join(", "),
            sig.result_type
        );
    }

    // Define a fluent function
    let fluent_sig = FunctionSignature::new("toAge", DataType::system(SystemType::Integer))
        .with_operands(vec![DataType::Model {
            namespace: "FHIR".to_string(),
            name: "Age".to_string(),
        }])
        .fluent();

    builder.define_function(fluent_sig);

    let fluent_overloads = builder.resolve_function("toAge");
    if let Some(sig) = fluent_overloads.first() {
        println!("   'toAge' is fluent: {}", sig.is_fluent);
    }
}

fn from_library_info_example() -> anyhow::Result<()> {
    let source = r#"
library DiabetesMeasure version '1.0.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1'

codesystem LOINC: 'http://loinc.org'

parameter MeasurementPeriod Interval<DateTime>

context Patient

define InPopulation:
  true

define function IsAdult(birthDate Date) returns Boolean:
  true
"#;

    // Parse and preprocess
    let ast = CqlParser::new().parse(source)?;
    let info = Preprocessor::process(&ast);

    // Create builder from library info
    let builder = LibraryBuilder::from_library_info(info);

    println!(
        "   Library: {} v{:?}",
        builder.library_name().unwrap_or("?"),
        builder.library_version()
    );

    // Library info is accessible
    if let Some(info) = builder.library_info() {
        println!("   Model deps: {}", info.model_dependencies().len());
        println!("   Library deps: {}", info.library_dependencies().len());
        println!("   Expressions: {}", info.expressions().len());
        println!("   Functions: {}", info.functions().len());
    }

    Ok(())
}

fn included_library_example() {
    let mut builder = LibraryBuilder::new();
    builder.set_library_name("MyMeasure");

    // Create a mock compiled library (FHIRHelpers)
    let helpers_lib = elm::Library {
        identifier: Some(elm::VersionedIdentifier {
            id: Some("FHIRHelpers".to_string()),
            version: Some("4.0.1".to_string()),
            system: None,
        }),
        statements: Some(elm::ExpressionDefs {
            defs: vec![
                elm::ExpressionDef {
                    name: Some("ToQuantity".to_string()),
                    access_level: Some(elm::AccessModifier::Public),
                    ..Default::default()
                },
                elm::ExpressionDef {
                    name: Some("ToString".to_string()),
                    access_level: Some(elm::AccessModifier::Public),
                    ..Default::default()
                },
            ],
        }),
        ..Default::default()
    };

    let compiled = CompiledLibrary::new(helpers_lib);
    builder.add_included_library("FHIRHelpers", compiled);

    // Library is registered as a symbol
    println!(
        "   'FHIRHelpers' is defined: {}",
        builder.is_defined("FHIRHelpers")
    );

    // Can check if library is included
    println!(
        "   Has FHIRHelpers: {}",
        builder.has_included_library("FHIRHelpers")
    );

    // List included libraries
    println!("   Included libraries:");
    for name in builder.included_library_names() {
        println!("      - {name}");
    }

    // Resolve qualified identifier
    match builder.resolve_qualified_identifier("FHIRHelpers", "ToQuantity") {
        Ok(resolved) => {
            println!("   Resolved 'FHIRHelpers.ToQuantity':");
            println!("      Kind: {}", resolved.symbol.kind);
            println!("      Library: {:?}", resolved.library_name());
        }
        Err(e) => println!("   Error: {e}"),
    }

    // Try to resolve non-existent
    let result = builder.resolve_qualified_identifier("FHIRHelpers", "NonExistent");
    println!("   'FHIRHelpers.NonExistent' resolved: {}", result.is_ok());
}
