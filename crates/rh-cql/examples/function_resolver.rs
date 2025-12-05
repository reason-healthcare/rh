//! Example demonstrating Function Resolution in the CQL-to-ELM translator.
//!
//! This example shows how to:
//! - Resolve system functions to ELM operators (Sum, Count, First, Now, etc.)
//! - Translate user-defined function invocations
//! - Handle fluent function syntax (value.function(args))
//! - Translate function definitions

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::ExpressionTranslator;

fn main() {
    let mut translator = ExpressionTranslator::new();

    println!("=== CQL Function Resolution Demo ===\n");

    // -------------------------------------------------------------------------
    // Nullary System Functions (0 arguments)
    // -------------------------------------------------------------------------
    println!("--- Nullary System Functions ---\n");

    // Now() → elm::Now
    let now_func = ast::FunctionInvocation {
        library: None,
        name: "Now".to_string(),
        arguments: vec![],
        location: None,
    };
    let result = translator.resolve_function_invocation(
        &now_func,
        |_, _| panic!("Should not translate args"),
        None,
    );
    println!("Now() → {}", expression_type_name(&result));

    // Today() → elm::Today
    let today_func = ast::FunctionInvocation {
        library: None,
        name: "Today".to_string(),
        arguments: vec![],
        location: None,
    };
    let result = translator.resolve_function_invocation(
        &today_func,
        |_, _| panic!("Should not translate args"),
        None,
    );
    println!("Today() → {}", expression_type_name(&result));

    // TimeOfDay() → elm::TimeOfDay
    let time_of_day_func = ast::FunctionInvocation {
        library: None,
        name: "TimeOfDay".to_string(),
        arguments: vec![],
        location: None,
    };
    let result = translator.resolve_function_invocation(
        &time_of_day_func,
        |_, _| panic!("Should not translate args"),
        None,
    );
    println!("TimeOfDay() → {}", expression_type_name(&result));

    println!();

    // -------------------------------------------------------------------------
    // Aggregate Functions
    // -------------------------------------------------------------------------
    println!("--- Aggregate Functions ---\n");

    // Sum(values) → elm::Sum
    let sum_func = ast::FunctionInvocation {
        library: None,
        name: "Sum".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "values".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&sum_func, translate_expr, None);
    println!("Sum(values) → {}", expression_type_name(&result));

    // Count(items) → elm::Count
    let count_func = ast::FunctionInvocation {
        library: None,
        name: "Count".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "items".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&count_func, translate_expr, None);
    println!("Count(items) → {}", expression_type_name(&result));

    // Avg(numbers) → elm::Avg
    let avg_func = ast::FunctionInvocation {
        library: None,
        name: "Avg".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "numbers".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&avg_func, translate_expr, None);
    println!("Avg(numbers) → {}", expression_type_name(&result));

    println!();

    // -------------------------------------------------------------------------
    // List Functions
    // -------------------------------------------------------------------------
    println!("--- List Functions ---\n");

    // First(list) → elm::First
    let first_func = ast::FunctionInvocation {
        library: None,
        name: "First".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "list".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&first_func, translate_expr, None);
    println!("First(list) → {}", expression_type_name(&result));

    // Last(list) → elm::Last
    let last_func = ast::FunctionInvocation {
        library: None,
        name: "Last".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "list".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&last_func, translate_expr, None);
    println!("Last(list) → {}", expression_type_name(&result));

    // Flatten(nested) → elm::Flatten
    let flatten_func = ast::FunctionInvocation {
        library: None,
        name: "Flatten".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "nested".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&flatten_func, translate_expr, None);
    println!("Flatten(nested) → {}", expression_type_name(&result));

    // Distinct(list) → elm::Distinct
    let distinct_func = ast::FunctionInvocation {
        library: None,
        name: "Distinct".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "list".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&distinct_func, translate_expr, None);
    println!("Distinct(list) → {}", expression_type_name(&result));

    println!();

    // -------------------------------------------------------------------------
    // String Functions
    // -------------------------------------------------------------------------
    println!("--- String Functions ---\n");

    // Length(str) → elm::Length
    let length_func = ast::FunctionInvocation {
        library: None,
        name: "Length".to_string(),
        arguments: vec![ast::Expression::Literal(ast::Literal::String(
            "hello".to_string(),
        ))],
        location: None,
    };
    let result = translator.resolve_function_invocation(&length_func, translate_expr, None);
    println!("Length('hello') → {}", expression_type_name(&result));

    // Upper(str) → elm::Upper
    let upper_func = ast::FunctionInvocation {
        library: None,
        name: "Upper".to_string(),
        arguments: vec![ast::Expression::Literal(ast::Literal::String(
            "hello".to_string(),
        ))],
        location: None,
    };
    let result = translator.resolve_function_invocation(&upper_func, translate_expr, None);
    println!("Upper('hello') → {}", expression_type_name(&result));

    // StartsWith(str, prefix) → elm::StartsWith
    let starts_with_func = ast::FunctionInvocation {
        library: None,
        name: "StartsWith".to_string(),
        arguments: vec![
            ast::Expression::Literal(ast::Literal::String("hello world".to_string())),
            ast::Expression::Literal(ast::Literal::String("hello".to_string())),
        ],
        location: None,
    };
    let result = translator.resolve_function_invocation(&starts_with_func, translate_expr, None);
    println!(
        "StartsWith('hello world', 'hello') → {}",
        expression_type_name(&result)
    );

    println!();

    // -------------------------------------------------------------------------
    // Nullological Functions
    // -------------------------------------------------------------------------
    println!("--- Nullological Functions ---\n");

    // Coalesce(a, b, c) → elm::Coalesce
    let coalesce_func = ast::FunctionInvocation {
        library: None,
        name: "Coalesce".to_string(),
        arguments: vec![
            ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "a".to_string(),
                location: None,
            }),
            ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "b".to_string(),
                location: None,
            }),
            ast::Expression::Literal(ast::Literal::Integer(0)),
        ],
        location: None,
    };
    let result = translator.resolve_function_invocation(&coalesce_func, translate_expr, None);
    println!("Coalesce(a, b, 0) → {}", expression_type_name(&result));

    // IsNull(value) → elm::IsNull
    let is_null_func = ast::FunctionInvocation {
        library: None,
        name: "IsNull".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "value".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&is_null_func, translate_expr, None);
    println!("IsNull(value) → {}", expression_type_name(&result));

    println!();

    // -------------------------------------------------------------------------
    // User-Defined Functions
    // -------------------------------------------------------------------------
    println!("--- User-Defined Functions ---\n");

    // MyCustomFunction(x) → elm::FunctionRef (unknown function)
    let custom_func = ast::FunctionInvocation {
        library: None,
        name: "MyCustomFunction".to_string(),
        arguments: vec![ast::Expression::Literal(ast::Literal::Integer(42))],
        location: None,
    };
    let result = translator.resolve_function_invocation(&custom_func, translate_expr, None);
    println!("MyCustomFunction(42) → {}", expression_type_name(&result));
    if let elm::Expression::FunctionRef(func_ref) = &result {
        println!("  Function name: {:?}", func_ref.name);
        println!("  Library: {:?}", func_ref.library_name);
    }

    // FHIRHelpers.ToQuantity(value) → elm::FunctionRef with library
    let library_func = ast::FunctionInvocation {
        library: Some("FHIRHelpers".to_string()),
        name: "ToQuantity".to_string(),
        arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "value".to_string(),
            location: None,
        })],
        location: None,
    };
    let result = translator.resolve_function_invocation(&library_func, translate_expr, None);
    println!(
        "FHIRHelpers.ToQuantity(value) → {}",
        expression_type_name(&result)
    );
    if let elm::Expression::FunctionRef(func_ref) = &result {
        println!("  Function name: {:?}", func_ref.name);
        println!("  Library: {:?}", func_ref.library_name);
    }

    println!();

    // -------------------------------------------------------------------------
    // Fluent Function Syntax
    // -------------------------------------------------------------------------
    println!("--- Fluent Function Syntax ---\n");

    // list.First() → First(list)
    let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
        name: "myList".to_string(),
        location: None,
    });
    let result =
        translator.resolve_fluent_function(&source, "First", &[], None, translate_expr, None);
    println!("myList.First() → {}", expression_type_name(&result));

    // encounters.Count() → Count(encounters)
    let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
        name: "encounters".to_string(),
        location: None,
    });
    let result =
        translator.resolve_fluent_function(&source, "Count", &[], None, translate_expr, None);
    println!("encounters.Count() → {}", expression_type_name(&result));

    // str.StartsWith("prefix") → StartsWith(str, "prefix")
    let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
        name: "str".to_string(),
        location: None,
    });
    let result = translator.resolve_fluent_function(
        &source,
        "StartsWith",
        &[ast::Expression::Literal(ast::Literal::String(
            "prefix".to_string(),
        ))],
        None,
        translate_expr,
        None,
    );
    println!(
        "str.StartsWith('prefix') → {}",
        expression_type_name(&result)
    );

    // patient.IsAdult() → IsAdult(patient) (user-defined fluent)
    let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
        name: "patient".to_string(),
        location: None,
    });
    let result =
        translator.resolve_fluent_function(&source, "IsAdult", &[], None, translate_expr, None);
    println!("patient.IsAdult() → {}", expression_type_name(&result));

    println!();

    // -------------------------------------------------------------------------
    // Function Definition Translation
    // -------------------------------------------------------------------------
    println!("--- Function Definition Translation ---\n");

    // define function Double(x Integer) returns Integer: x * 2
    let double_func = ast::FunctionDef {
        name: "Double".to_string(),
        parameters: vec![ast::FunctionParameter {
            name: "x".to_string(),
            type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
        }],
        return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Integer".to_string(),
        })),
        body: Some(ast::Expression::Literal(ast::Literal::Integer(42))), // Simplified body
        fluent: false,
        external: false,
        access: ast::AccessModifier::Public,
        location: None,
    };

    let elm_func = translator.translate_function_def(&double_func, translate_expr);
    println!("Function: {:?}", elm_func.name);
    println!("  Operands: {:?}", elm_func.operand.len());
    println!("  Has body: {}", elm_func.expression.is_some());
    println!("  Fluent: {:?}", elm_func.fluent);
    println!("  External: {:?}", elm_func.external);

    // define fluent function IsAdult(p Patient) returns Boolean: ...
    let is_adult_func = ast::FunctionDef {
        name: "IsAdult".to_string(),
        parameters: vec![ast::FunctionParameter {
            name: "p".to_string(),
            type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: Some("FHIR".to_string()),
                name: "Patient".to_string(),
            })),
        }],
        return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Boolean".to_string(),
        })),
        body: Some(ast::Expression::Literal(ast::Literal::Boolean(true))),
        fluent: true,
        external: false,
        access: ast::AccessModifier::Public,
        location: None,
    };

    let elm_func = translator.translate_function_def(&is_adult_func, translate_expr);
    println!("\nFunction: {:?}", elm_func.name);
    println!("  Operands: {:?}", elm_func.operand.len());
    println!("  Fluent: {:?}", elm_func.fluent);

    // define external function GetData() returns String
    let external_func = ast::FunctionDef {
        name: "GetData".to_string(),
        parameters: vec![],
        return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "String".to_string(),
        })),
        body: None,
        fluent: false,
        external: true,
        access: ast::AccessModifier::Private,
        location: None,
    };

    let elm_func = translator.translate_function_def(&external_func, |_, _| {
        panic!("External functions have no body")
    });
    println!("\nFunction: {:?}", elm_func.name);
    println!("  Has body: {}", elm_func.expression.is_some());
    println!("  External: {:?}", elm_func.external);
    println!("  Access: {:?}", elm_func.access_level);

    println!("\n=== Demo Complete ===");
}

/// Helper to translate expressions for the demo.
fn translate_expr(t: &mut ExpressionTranslator, expr: &ast::Expression) -> elm::Expression {
    match expr {
        ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
        ast::Expression::Literal(lit) => t.translate_literal(lit),
        _ => panic!("Unsupported expression type in demo"),
    }
}

/// Get the ELM expression type name for display.
fn expression_type_name(expr: &elm::Expression) -> &'static str {
    match expr {
        elm::Expression::Now(_) => "Now",
        elm::Expression::Today(_) => "Today",
        elm::Expression::TimeOfDay(_) => "TimeOfDay",
        elm::Expression::Sum(_) => "Sum",
        elm::Expression::Count(_) => "Count",
        elm::Expression::Min(_) => "Min",
        elm::Expression::Max(_) => "Max",
        elm::Expression::Avg(_) => "Avg",
        elm::Expression::First(_) => "First",
        elm::Expression::Last(_) => "Last",
        elm::Expression::Flatten(_) => "Flatten",
        elm::Expression::Distinct(_) => "Distinct",
        elm::Expression::Exists(_) => "Exists",
        elm::Expression::Length(_) => "Length",
        elm::Expression::Upper(_) => "Upper",
        elm::Expression::Lower(_) => "Lower",
        elm::Expression::StartsWith(_) => "StartsWith",
        elm::Expression::EndsWith(_) => "EndsWith",
        elm::Expression::Coalesce(_) => "Coalesce",
        elm::Expression::IsNull(_) => "IsNull",
        elm::Expression::FunctionRef(_) => "FunctionRef",
        elm::Expression::Literal(_) => "Literal",
        elm::Expression::IdentifierRef(_) => "IdentifierRef",
        _ => "Other",
    }
}
