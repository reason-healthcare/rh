//! Example: Reference Translation (Phase 4.5b)
//!
//! Demonstrates translating CQL identifier and qualified references to ELM.
//!
//! Run with: `cargo run --example reference_translator`

use rh_cql::datatype::DataType;
use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::{ExpressionTranslator, QualifiedRefKind, ResolvedRefKind};

fn main() {
    println!("=== CQL Reference Translation Example ===\n");

    let mut translator = ExpressionTranslator::new();

    // -------------------------------------------------------------------------
    // Simple Identifier References
    // -------------------------------------------------------------------------
    println!("--- Simple Identifier References ---\n");

    // Expression Reference
    println!("1. Expression Reference");
    println!("   CQL: InpatientEncounters");
    let expr_ref = translator.translate_identifier_ref(
        "InpatientEncounters",
        ResolvedRefKind::Expression,
        Some(&DataType::list(DataType::Model {
            namespace: "http://hl7.org/fhir".to_string(),
            name: "Encounter".to_string(),
        })),
    );
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&expr_ref).unwrap()
    );

    // Parameter Reference
    println!("2. Parameter Reference");
    println!("   CQL: MeasurementPeriod");
    let param_ref = translator.translate_identifier_ref(
        "MeasurementPeriod",
        ResolvedRefKind::Parameter,
        Some(&DataType::interval(DataType::date_time())),
    );
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&param_ref).unwrap()
    );

    // Operand Reference (within function body)
    println!("3. Operand Reference (function parameter)");
    println!("   CQL: value (inside function)");
    let operand_ref = translator.translate_identifier_ref("value", ResolvedRefKind::Operand, None);
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&operand_ref).unwrap()
    );

    // Query Alias Reference
    println!("4. Query Alias Reference");
    println!("   CQL: E (in query context)");
    let alias_ref = translator.translate_identifier_ref(
        "E",
        ResolvedRefKind::QueryAlias,
        Some(&DataType::Model {
            namespace: "http://hl7.org/fhir".to_string(),
            name: "Encounter".to_string(),
        }),
    );
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&alias_ref).unwrap()
    );

    // Let Binding Reference
    println!("5. Let Binding Reference");
    println!("   CQL: x (from let clause)");
    let let_ref =
        translator.translate_identifier_ref("x", ResolvedRefKind::Let, Some(&DataType::integer()));
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&let_ref).unwrap()
    );

    // -------------------------------------------------------------------------
    // Terminology References
    // -------------------------------------------------------------------------
    println!("--- Terminology References ---\n");

    // Code System Reference
    println!("6. Code System Reference");
    println!("   CQL: LOINC");
    let cs_ref = translator.translate_identifier_ref("LOINC", ResolvedRefKind::CodeSystem, None);
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&cs_ref).unwrap()
    );

    // Value Set Reference
    println!("7. Value Set Reference");
    println!("   CQL: \"Diabetes Conditions\"");
    let vs_ref =
        translator.translate_identifier_ref("Diabetes Conditions", ResolvedRefKind::ValueSet, None);
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&vs_ref).unwrap()
    );

    // Code Reference
    println!("8. Code Reference");
    println!("   CQL: BodyWeightCode");
    let code_ref =
        translator.translate_identifier_ref("BodyWeightCode", ResolvedRefKind::Code, None);
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&code_ref).unwrap()
    );

    // Concept Reference
    println!("9. Concept Reference");
    println!("   CQL: DiabetesConcept");
    let concept_ref =
        translator.translate_identifier_ref("DiabetesConcept", ResolvedRefKind::Concept, None);
    println!(
        "   ELM: {}\n",
        serde_json::to_string_pretty(&concept_ref).unwrap()
    );

    // Context Reference
    println!("10. Context Reference");
    println!("    CQL: Patient");
    let context_ref =
        translator.translate_identifier_ref("Patient", ResolvedRefKind::Context, None);
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&context_ref).unwrap()
    );

    // -------------------------------------------------------------------------
    // Qualified References (Library.Name)
    // -------------------------------------------------------------------------
    println!("--- Qualified References (Library.Name) ---\n");

    // Library Expression Reference
    println!("11. Library Expression Reference");
    println!("    CQL: Common.\"Inpatient Encounters\"");
    let lib_expr = translator.translate_qualified_ref(
        "Common",
        "Inpatient Encounters",
        QualifiedRefKind::LibraryExpression,
        None,
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&lib_expr).unwrap()
    );

    // Library Parameter Reference
    println!("12. Library Parameter Reference");
    println!("    CQL: Common.MeasurementPeriod");
    let lib_param = translator.translate_qualified_ref(
        "Common",
        "MeasurementPeriod",
        QualifiedRefKind::LibraryParameter,
        None,
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&lib_param).unwrap()
    );

    // Library Value Set Reference
    println!("13. Library Value Set Reference");
    println!("    CQL: Terminology.\"Diabetes Conditions\"");
    let lib_vs = translator.translate_qualified_ref(
        "Terminology",
        "Diabetes Conditions",
        QualifiedRefKind::LibraryValueSet,
        None,
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&lib_vs).unwrap()
    );

    // -------------------------------------------------------------------------
    // Property Access
    // -------------------------------------------------------------------------
    println!("--- Property Access ---\n");

    // Property on alias (scope-based)
    println!("14. Property Access on Query Alias");
    println!("    CQL: E.status (in query where E is alias)");
    let prop_ref = translator.translate_qualified_ref(
        "E",
        "status",
        QualifiedRefKind::Property { source: None },
        Some(&DataType::string()),
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&prop_ref).unwrap()
    );

    // Property with source expression
    println!("15. Property Access on Expression");
    println!("    CQL: (some expression).birthDate");
    let source_expr = elm::Expression::ExpressionRef(elm::ExpressionRef {
        element: elm::ElementFields::default(),
        name: Some("CurrentPatient".to_string()),
        library_name: None,
    });
    let prop_with_source = translator.translate_qualified_ref(
        "",
        "birthDate",
        QualifiedRefKind::Property {
            source: Some(Box::new(source_expr)),
        },
        Some(&DataType::date()),
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&prop_with_source).unwrap()
    );

    // -------------------------------------------------------------------------
    // Function References
    // -------------------------------------------------------------------------
    println!("--- Function References ---\n");

    // Function with arguments
    println!("16. Function Reference with Arguments");
    println!("    CQL: Add(1, 2)");
    let arg1 = elm::Expression::Literal(elm::Literal {
        element: elm::ElementFields::default(),
        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
        value: Some("1".to_string()),
    });
    let arg2 = elm::Expression::Literal(elm::Literal {
        element: elm::ElementFields::default(),
        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
        value: Some("2".to_string()),
    });
    let func_ref = translator.translate_function_ref(
        "Add",
        None,
        vec![arg1, arg2],
        vec![],
        Some(&DataType::integer()),
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&func_ref).unwrap()
    );

    // Function from included library
    println!("17. Library Function Reference");
    println!("    CQL: FHIRHelpers.ToQuantity(value)");
    let lib_func = translator.translate_function_ref(
        "ToQuantity",
        Some("FHIRHelpers"),
        vec![],
        vec![],
        Some(&DataType::quantity()),
    );
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&lib_func).unwrap()
    );

    // -------------------------------------------------------------------------
    // Unresolved AST References
    // -------------------------------------------------------------------------
    println!("--- Unresolved AST References ---\n");

    // Unresolved identifier
    println!("18. Unresolved Identifier (from parser)");
    println!("    CQL: SomeIdentifier");
    let ast_id = ast::IdentifierRef {
        name: "SomeIdentifier".to_string(),
        location: None,
    };
    let unresolved_id = translator.translate_ast_identifier_ref(&ast_id);
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&unresolved_id).unwrap()
    );

    // Unresolved qualified identifier
    println!("19. Unresolved Qualified Identifier (from parser)");
    println!("    CQL: Library.Definition");
    let ast_qual = ast::QualifiedIdentifierRef {
        qualifier: "Library".to_string(),
        name: "Definition".to_string(),
        location: None,
    };
    let unresolved_qual = translator.translate_ast_qualified_ref(&ast_qual);
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&unresolved_qual).unwrap()
    );

    // Unresolved function invocation
    println!("20. Unresolved Function Invocation (from parser)");
    println!("    CQL: MyFunc(1, 2, 3)");
    let ast_func = ast::FunctionInvocation {
        library: None,
        name: "MyFunc".to_string(),
        arguments: vec![
            ast::Expression::Literal(ast::Literal::Integer(1)),
            ast::Expression::Literal(ast::Literal::Integer(2)),
            ast::Expression::Literal(ast::Literal::Integer(3)),
        ],
        location: None,
    };
    let unresolved_func = translator.translate_ast_function_invocation(&ast_func, |t, expr| {
        if let ast::Expression::Literal(lit) = expr {
            t.translate_literal(lit)
        } else {
            panic!("Expected literal");
        }
    });
    println!(
        "    ELM: {}\n",
        serde_json::to_string_pretty(&unresolved_func).unwrap()
    );

    println!("=== Example Complete ===");
}
