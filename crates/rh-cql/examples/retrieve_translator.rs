//! Example: CQL Retrieve Translation (Phase 4.5e)
//!
//! This example demonstrates how to translate CQL retrieve expressions to ELM,
//! covering data type retrieval with code filters, date ranges, and contexts.
//!
//! Run with: cargo run -p rh-cql --example retrieve_translator

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::{ExpressionTranslator, ResolvedRefKind};

fn main() {
    println!("=== CQL Retrieve Translator Example ===\n");

    let mut translator = ExpressionTranslator::new();

    // ========================================================================
    // Simple Retrieve: [Condition]
    // ========================================================================
    println!("--- Simple Retrieve ---\n");

    let condition_type = make_named_type("Condition");
    let result = translator.translate_simple_retrieve(&condition_type, None);
    println!("[Condition]");
    print_retrieve_structure(&result);

    // ========================================================================
    // Qualified Type Retrieve: [FHIR.Patient]
    // ========================================================================
    println!("\n--- Qualified Type Retrieve ---\n");

    let patient_type = make_qualified_type("FHIR", "Patient");
    let result = translator.translate_simple_retrieve(&patient_type, None);
    println!("[FHIR.Patient]");
    print_retrieve_structure(&result);

    // ========================================================================
    // Retrieve with Code Filter: [Condition: code in "DiabetesCodes"]
    // ========================================================================
    println!("\n--- Retrieve with Code Filter ---\n");

    let condition_type = make_named_type("Condition");
    let codes =
        translator.translate_identifier_ref("DiabetesCodes", ResolvedRefKind::ValueSet, None);
    let result = translator.translate_retrieve_with_codes(&condition_type, "code", codes, None);
    println!("[Condition: code in \"DiabetesCodes\"]");
    print_retrieve_structure(&result);

    // ========================================================================
    // Retrieve with Date Range: [Encounter: during MeasurementPeriod]
    // ========================================================================
    println!("\n--- Retrieve with Date Range ---\n");

    let encounter_type = make_named_type("Encounter");
    let date_range =
        translator.translate_identifier_ref("MeasurementPeriod", ResolvedRefKind::Parameter, None);
    let result =
        translator.translate_retrieve_with_date_range(&encounter_type, "period", date_range, None);
    println!("[Encounter: period during MeasurementPeriod]");
    print_retrieve_structure(&result);

    // ========================================================================
    // Retrieve with Code and Date: [Observation: code in "BPCodes" during MP]
    // ========================================================================
    println!("\n--- Retrieve with Code and Date Range ---\n");

    let observation_type = make_named_type("Observation");
    let codes =
        translator.translate_identifier_ref("BloodPressureCodes", ResolvedRefKind::ValueSet, None);
    let date_range =
        translator.translate_identifier_ref("MeasurementPeriod", ResolvedRefKind::Parameter, None);
    let result = translator.translate_retrieve_with_codes_and_date(
        &observation_type,
        "code",
        codes,
        "effective",
        date_range,
        None,
    );
    println!("[Observation: code in \"BloodPressureCodes\", effective during MeasurementPeriod]");
    print_retrieve_structure(&result);

    // ========================================================================
    // Retrieve with Context: [Patient.Condition]
    // ========================================================================
    println!("\n--- Retrieve with Context ---\n");

    let condition_type = make_named_type("Condition");
    let context = translator.translate_identifier_ref("Patient", ResolvedRefKind::Context, None);
    let result = translator.translate_retrieve_with_context(&condition_type, context, None);
    println!("[Condition] (in Patient context)");
    print_retrieve_structure(&result);

    // ========================================================================
    // Full AST Retrieve Translation
    // ========================================================================
    println!("\n--- Full AST Retrieve Translation ---\n");

    let retrieve = ast::Retrieve {
        data_type: make_named_type("MedicationRequest"),
        context: Some(Box::new(ast::Expression::IdentifierRef(
            ast::IdentifierRef {
                name: "Patient".to_string(),
                location: None,
            },
        ))),
        code_path: Some("medication.code".to_string()),
        codes: Some(Box::new(ast::Expression::IdentifierRef(
            ast::IdentifierRef {
                name: "OpioidMedications".to_string(),
                location: None,
            },
        ))),
        date_path: Some("authoredOn".to_string()),
        date_range: Some(Box::new(ast::Expression::IdentifierRef(
            ast::IdentifierRef {
                name: "MeasurementPeriod".to_string(),
                location: None,
            },
        ))),
        location: None,
    };

    let result = translator.translate_retrieve(&retrieve, translate_expr, None);
    println!("[MedicationRequest: medication.code in \"OpioidMedications\", authoredOn during MeasurementPeriod] (Patient context)");
    print_retrieve_structure(&result);

    // ========================================================================
    // Various FHIR Resource Types
    // ========================================================================
    println!("\n--- Various FHIR Resource Types ---\n");

    let resources = [
        "Patient",
        "Encounter",
        "Condition",
        "Observation",
        "Procedure",
        "MedicationRequest",
        "DiagnosticReport",
        "Immunization",
    ];

    for resource in resources {
        let data_type = make_named_type(resource);
        let result = translator.translate_simple_retrieve(&data_type, None);
        if let elm::Expression::Retrieve(r) = &result {
            println!(
                "[{resource}] => dataType: {:?}",
                r.data_type.as_ref().map(|d| extract_type_name(d))
            );
        }
    }

    println!("\n=== Retrieve Translation Complete ===");
}

/// Helper to translate expressions in retrieve context
fn translate_expr(
    translator: &mut ExpressionTranslator,
    expr: &ast::Expression,
) -> elm::Expression {
    match expr {
        ast::Expression::IdentifierRef(id_ref) => translator.translate_ast_identifier_ref(id_ref),
        ast::Expression::Literal(lit) => translator.translate_literal(lit),
        _ => panic!("Unexpected expression type in example"),
    }
}

/// Create a named type specifier
fn make_named_type(name: &str) -> ast::TypeSpecifier {
    ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
        namespace: None,
        name: name.to_string(),
    })
}

/// Create a qualified type specifier
fn make_qualified_type(ns: &str, name: &str) -> ast::TypeSpecifier {
    ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
        namespace: Some(ns.to_string()),
        name: name.to_string(),
    })
}

/// Extract the type name from a QName
fn extract_type_name(qname: &str) -> &str {
    if let Some(idx) = qname.rfind('}') {
        &qname[idx + 1..]
    } else {
        qname
    }
}

/// Print the structure of a translated retrieve
fn print_retrieve_structure(result: &elm::Expression) {
    if let elm::Expression::Retrieve(r) = result {
        println!("  ELM Retrieve:");
        if let Some(dt) = &r.data_type {
            println!("    dataType: {}", extract_type_name(dt));
        }
        if r.context.is_some() {
            println!("    context: present");
        }
        if let Some(cp) = &r.code_property {
            println!("    codeProperty: {cp}");
        }
        if r.codes.is_some() {
            println!("    codes: present");
        }
        if let Some(dp) = &r.date_property {
            println!("    dateProperty: {dp}");
        }
        if r.date_range.is_some() {
            println!("    dateRange: present");
        }
    } else {
        println!("  (not a Retrieve expression)");
    }
}
