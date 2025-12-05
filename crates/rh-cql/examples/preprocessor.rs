//! Example: Using the CQL Preprocessor
//!
//! This example demonstrates how to use the Preprocessor to extract
//! library information from a parsed CQL AST.
//!
//! Run with: `cargo run --example preprocessor`

use rh_cql::parser::CqlParser;
use rh_cql::preprocessor::Preprocessor;

fn main() -> anyhow::Result<()> {
    println!("=== CQL Preprocessor Example ===\n");

    // Sample CQL library with various definitions
    let source = r#"
library DiabetesMeasure version '1.0.0'

using FHIR version '4.0.1'

include FHIRHelpers version '4.0.1'
include CommonLogic version '2.0' called Common

codesystem LOINC: 'http://loinc.org' version '2.73'
codesystem SNOMED: 'http://snomed.info/sct'

valueset DiabetesDiagnoses: 'http://cts.nlm.nih.gov/fhir/ValueSet/diabetes'

code HbA1c: '4548-4' from LOINC display 'Hemoglobin A1c'
code Glucose: '2339-0' from LOINC

concept DiabetesMarkers: { HbA1c, Glucose } display 'Diabetes Lab Markers'

parameter MeasurementPeriod Interval<DateTime>

context Patient

define InInitialPopulation:
  true

define HasDiabetesDiagnosis:
  true

private define HelperExpression:
  false

define function IsAdult(birthDate Date) returns Boolean:
  true

fluent define function ToAge(value Integer) returns Integer:
  value
"#;

    // Parse the CQL source
    let parser = CqlParser::new();
    let ast = parser.parse(source)?;

    // Preprocess to extract library info
    let info = Preprocessor::process(&ast);

    // Example 1: Library Identity
    println!("1. Library Identity:");
    println!("   Name: {:?}", info.name());
    println!("   Version: {:?}", info.version());
    if let Some(id) = info.identifier() {
        println!("   Identifier: {id}");
    }

    // Example 2: Model Dependencies
    println!("\n2. Model Dependencies (using):");
    for model in info.model_dependencies() {
        println!(
            "   {} version {:?}",
            model.name,
            model.version.as_deref().unwrap_or("(none)")
        );
    }
    println!("   Uses FHIR? {}", info.uses_model("FHIR"));
    println!("   Uses QDM? {}", info.uses_model("QDM"));

    // Example 3: Library Dependencies
    println!("\n3. Library Dependencies (include):");
    for lib in info.library_dependencies() {
        let alias_str = lib
            .alias
            .as_ref()
            .map(|a| format!(" called {a}"))
            .unwrap_or_default();
        println!(
            "   {} version {:?}{alias_str}",
            lib.path,
            lib.version.as_deref().unwrap_or("(none)")
        );
        println!("      Local name: {}", lib.local_name());
    }

    // Get dependency identifiers for LibraryManager
    println!("\n   Dependency IDs for resolution:");
    for id in info.library_dependency_ids() {
        println!("      - {id}");
    }

    // Example 4: Terminology Definitions
    println!("\n4. Terminology Definitions:");
    println!("   Code Systems ({}):", info.code_systems().len());
    for cs in info.code_systems() {
        println!("      - {}: {} (public: {})", cs.name, cs.id, cs.is_public);
    }

    println!("   Value Sets ({}):", info.value_sets().len());
    for vs in info.value_sets() {
        println!("      - {}: {}", vs.name, vs.id);
    }

    println!("   Codes ({}):", info.codes().len());
    for code in info.codes() {
        println!(
            "      - {}: '{}' from {} ({:?})",
            code.name, code.code, code.codesystem, code.display
        );
    }

    println!("   Concepts ({}):", info.concepts().len());
    for concept in info.concepts() {
        println!("      - {}: {:?}", concept.name, concept.codes);
    }

    // Example 5: Parameters
    println!("\n5. Parameters ({}):", info.parameters().len());
    for param in info.parameters() {
        let visibility = if param.is_public { "public" } else { "private" };
        let default = if param.has_default {
            " (has default)"
        } else {
            ""
        };
        println!("      - {visibility} {}{default}", param.name);
    }

    // Example 6: Contexts
    println!("\n6. Contexts:");
    for ctx in info.contexts() {
        println!("      - {ctx}");
    }

    // Example 7: Expression Definitions
    println!(
        "\n7. Expression Definitions ({}):",
        info.expressions().len()
    );
    for expr in info.expressions() {
        let visibility = if expr.is_public { "public" } else { "private" };
        println!("      - {visibility} define \"{}\"", expr.name);
    }

    // Example 8: Function Definitions
    println!("\n8. Function Definitions ({}):", info.functions().len());
    for func in info.functions() {
        let visibility = if func.is_public { "public" } else { "private" };
        let fluent = if func.is_fluent { "fluent " } else { "" };
        let external = if func.is_external { " (external)" } else { "" };
        println!(
            "      - {visibility} {fluent}function \"{}\"({} params){external}",
            func.name, func.arity
        );
        println!("        Parameters: {:?}", func.parameter_names);
    }

    // Example 9: Definition Lookup
    println!("\n9. Definition Lookup:");
    let names_to_check = [
        "InInitialPopulation",
        "IsAdult",
        "MeasurementPeriod",
        "LOINC",
        "DiabetesDiagnoses",
        "HbA1c",
        "DiabetesMarkers",
        "NonExistent",
    ];

    for name in names_to_check {
        match info.get_definition_kind(name) {
            Some(kind) => println!("      \"{name}\" -> {kind}"),
            None => println!("      \"{name}\" -> NOT FOUND"),
        }
    }

    // Example 10: Public vs Private Definitions
    println!("\n10. Access Control:");
    let all_names = info.definition_names();
    let public_names = info.public_definition_names();
    println!("    Total definitions: {}", all_names.len());
    println!("    Public definitions: {}", public_names.len());
    println!(
        "    Private definitions: {}",
        all_names.len() - public_names.len()
    );

    println!("\n    Public definitions:");
    for name in &public_names {
        println!("      - {name}");
    }

    // Example 11: Specific Lookups
    println!("\n11. Specific Lookups:");

    if let Some(expr) = info.get_expression("InInitialPopulation") {
        println!(
            "    Expression 'InInitialPopulation': public={}",
            expr.is_public
        );
    }

    if let Some(func) = info.get_function("IsAdult") {
        println!(
            "    Function 'IsAdult': arity={}, params={:?}",
            func.arity, func.parameter_names
        );
    }

    if let Some(param) = info.get_parameter("MeasurementPeriod") {
        println!(
            "    Parameter 'MeasurementPeriod': public={}, has_default={}",
            param.is_public, param.has_default
        );
    }

    if let Some(lib) = info.get_library("Common") {
        println!("    Library 'Common' (by alias): path={}", lib.path);
    }

    if let Some(model) = info.get_model("FHIR") {
        println!("    Model 'FHIR': version={:?}", model.version);
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
