//! Example demonstrating the TypeResolver and TypeBuilder for CQL type resolution.
//!
//! This example shows how to:
//! - Resolve AST type specifiers to DataTypes
//! - Convert DataTypes back to ELM TypeSpecifiers
//! - Use type inference utilities for literals and operators
//!
//! Run with: cargo run --example type_resolver -p rh-cql

use rh_cql::datatype::{DataType, SystemType};
use rh_cql::parser::ast;
use rh_cql::types::{TypeBuilder, TypeInference, TypeResolver};

fn main() {
    println!("=== CQL Type Resolution Example ===\n");

    // Example 1: Basic Type Resolution
    basic_type_resolution();

    // Example 2: Complex Type Resolution
    complex_type_resolution();

    // Example 3: Type Building (DataType → ELM)
    type_building();

    // Example 4: Type Inference
    type_inference();

    // Example 5: Type Compatibility
    type_compatibility();

    println!("=== Example Complete ===");
}

fn basic_type_resolution() {
    println!("--- 1. Basic Type Resolution ---\n");

    let resolver = TypeResolver::new();

    // Resolve system types
    let int_spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
        namespace: None,
        name: "Integer".to_string(),
    });

    let result = resolver.resolve_type_specifier(&int_spec).unwrap();
    println!("Resolved 'Integer' → {result:?}");
    assert_eq!(result, DataType::integer());

    // Resolve with System namespace
    let str_spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
        namespace: Some("System".to_string()),
        name: "String".to_string(),
    });

    let result = resolver.resolve_type_specifier(&str_spec).unwrap();
    println!("Resolved 'System.String' → {result:?}");
    assert_eq!(result, DataType::string());

    // Resolve qualified name
    let dt = resolver
        .resolve_qualified_name("{urn:hl7-org:elm-types:r1}Boolean")
        .unwrap();
    println!("Resolved '{{urn:hl7-org:elm-types:r1}}Boolean' → {dt:?}");
    assert_eq!(dt, DataType::boolean());

    println!();
}

fn complex_type_resolution() {
    println!("--- 2. Complex Type Resolution ---\n");

    let resolver = TypeResolver::new();

    // List type
    let list_spec = ast::TypeSpecifier::List(ast::ListTypeSpecifier {
        element_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Integer".to_string(),
        })),
    });

    let list_type = resolver.resolve_type_specifier(&list_spec).unwrap();
    println!("Resolved 'List<Integer>' → {list_type:?}");
    assert_eq!(list_type, DataType::list(DataType::integer()));

    // Interval type
    let interval_spec = ast::TypeSpecifier::Interval(ast::IntervalTypeSpecifier {
        point_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "DateTime".to_string(),
        })),
    });

    let interval_type = resolver.resolve_type_specifier(&interval_spec).unwrap();
    println!("Resolved 'Interval<DateTime>' → {interval_type:?}");
    assert_eq!(interval_type, DataType::interval(DataType::date_time()));

    // Tuple type
    let tuple_spec = ast::TypeSpecifier::Tuple(ast::TupleTypeSpecifier {
        elements: vec![
            ast::TupleElementDef {
                name: "id".to_string(),
                element_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "String".to_string(),
                }),
            },
            ast::TupleElementDef {
                name: "value".to_string(),
                element_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "Decimal".to_string(),
                }),
            },
        ],
    });

    let tuple_type = resolver.resolve_type_specifier(&tuple_spec).unwrap();
    println!("Resolved 'Tuple{{id: String, value: Decimal}}' → {tuple_type:?}");

    // Choice type
    let choice_spec = ast::TypeSpecifier::Choice(ast::ChoiceTypeSpecifier {
        types: vec![
            ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            }),
            ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "String".to_string(),
            }),
        ],
    });

    let choice_type = resolver.resolve_type_specifier(&choice_spec).unwrap();
    println!("Resolved 'Choice<Integer, String>' → {choice_type:?}");
    assert_eq!(
        choice_type,
        DataType::choice(vec![DataType::integer(), DataType::string()])
    );

    println!();
}

fn type_building() {
    println!("--- 3. Type Building (DataType → ELM) ---\n");

    // Convert system type
    let elm_int = TypeBuilder::to_type_specifier(&DataType::integer()).unwrap();
    println!("DataType::integer() → ELM: {elm_int:?}");

    // Convert list type
    let list_dt = DataType::list(DataType::string());
    let elm_list = TypeBuilder::to_type_specifier(&list_dt).unwrap();
    println!("DataType::list(String) → ELM: {elm_list:?}");

    // Convert interval type
    let interval_dt = DataType::interval(DataType::date());
    let elm_interval = TypeBuilder::to_type_specifier(&interval_dt).unwrap();
    println!("DataType::interval(Date) → ELM: {elm_interval:?}");

    // Convert to qualified name
    let qname = TypeBuilder::to_qualified_name(&DataType::decimal()).unwrap();
    println!("DataType::decimal() → qualified name: {qname}");
    assert_eq!(qname, "{urn:hl7-org:elm-types:r1}Decimal");

    // Model type qualified name
    let model_dt = DataType::model("FHIR", "Patient");
    let model_qname = TypeBuilder::to_qualified_name(&model_dt).unwrap();
    println!("DataType::model(FHIR, Patient) → qualified name: {model_qname}");
    assert_eq!(model_qname, "{FHIR}Patient");

    // Unknown returns None
    let none = TypeBuilder::to_type_specifier(&DataType::Unknown);
    println!("DataType::Unknown → ELM: {none:?}");
    assert!(none.is_none());

    println!();
}

fn type_inference() {
    println!("--- 4. Type Inference ---\n");

    // Literal types
    println!("Literal type inference:");
    println!(
        "  null → {:?}",
        TypeInference::literal_type(&ast::Literal::Null)
    );
    println!(
        "  true → {:?}",
        TypeInference::literal_type(&ast::Literal::Boolean(true))
    );
    println!(
        "  42 → {:?}",
        TypeInference::literal_type(&ast::Literal::Integer(42))
    );
    println!(
        "  3.5 → {:?}",
        TypeInference::literal_type(&ast::Literal::Decimal(3.5))
    );
    println!(
        "  'hello' → {:?}",
        TypeInference::literal_type(&ast::Literal::String("hello".to_string()))
    );
    println!(
        "  @2024-01-15 → {:?}",
        TypeInference::literal_type(&ast::Literal::Date("2024-01-15".to_string()))
    );

    // Numeric promotion
    println!("\nNumeric promotion:");
    let int = DataType::integer();
    let long = DataType::long();
    let decimal = DataType::decimal();

    println!(
        "  Integer + Integer → {:?}",
        TypeInference::promote_numeric(&int, &int)
    );
    println!(
        "  Integer + Long → {:?}",
        TypeInference::promote_numeric(&int, &long)
    );
    println!(
        "  Integer + Decimal → {:?}",
        TypeInference::promote_numeric(&int, &decimal)
    );
    println!(
        "  Long + Decimal → {:?}",
        TypeInference::promote_numeric(&long, &decimal)
    );

    println!();
}

fn type_compatibility() {
    println!("--- 5. Type Compatibility ---\n");

    // Implicit conversions
    println!("Implicit conversions (can_convert_implicitly):");
    let tests = [
        (DataType::integer(), DataType::long(), true),
        (DataType::integer(), DataType::decimal(), true),
        (DataType::long(), DataType::decimal(), true),
        (DataType::date(), DataType::date_time(), true),
        (
            DataType::System(SystemType::Code),
            DataType::concept(),
            true,
        ),
        (DataType::string(), DataType::integer(), false),
    ];

    for (from, to, expected) in &tests {
        let can = TypeInference::can_convert_implicitly(from, to);
        println!("  {from:?} → {to:?}: {can} (expected {expected})");
        assert_eq!(can, *expected);
    }

    // Common types
    println!("\nFinding common types:");
    let int = DataType::integer();
    let long = DataType::long();
    let decimal = DataType::decimal();

    println!(
        "  common(Integer, Integer) → {:?}",
        TypeInference::common_type(&int, &int)
    );
    println!(
        "  common(Integer, Long) → {:?}",
        TypeInference::common_type(&int, &long)
    );
    println!(
        "  common(Integer, Decimal) → {:?}",
        TypeInference::common_type(&int, &decimal)
    );
    println!(
        "  common(String, Integer) → {:?}",
        TypeInference::common_type(&DataType::string(), &int)
    );

    // Operator result types
    println!("\nOperator result types:");
    println!(
        "  not Boolean → {:?}",
        TypeInference::unary_result_type("not", &DataType::boolean())
    );
    println!(
        "  -Integer → {:?}",
        TypeInference::unary_result_type("-", &DataType::integer())
    );
    println!(
        "  Integer + Decimal → {:?}",
        TypeInference::arithmetic_result_type("+", &int, &decimal)
    );
    println!(
        "  Integer = Long → {:?}",
        TypeInference::comparison_result_type(&int, &long)
    );
    println!(
        "  Boolean and Boolean → {:?}",
        TypeInference::logical_result_type(&DataType::boolean(), &DataType::boolean())
    );

    println!();
}
