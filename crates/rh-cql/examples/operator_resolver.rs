//! Example demonstrating the OperatorResolver for CQL semantic analysis.
//!
//! This example shows how to:
//! - Resolve operator overloads based on operand types
//! - Handle implicit type conversions (e.g., Integer → Decimal)
//! - Work with generic operators (e.g., list operations)
//! - Resolve unary, binary, and ternary operators

use rh_cql::datatype::DataType;
use rh_cql::operators::{OperatorResolver, OperatorSignature};
use rh_cql::parser::ast::{BinaryOperator, TernaryOperator, UnaryOperator};

fn main() {
    let resolver = OperatorResolver::new();

    println!("=== CQL Operator Resolution Demo ===\n");

    // -------------------------------------------------------------------------
    // Arithmetic Operators
    // -------------------------------------------------------------------------
    println!("--- Arithmetic Operators ---\n");

    // Integer addition: Integer + Integer → Integer
    let result = resolver
        .resolve_binary(
            BinaryOperator::Add,
            &DataType::integer(),
            &DataType::integer(),
        )
        .unwrap();
    println!("Integer + Integer = {:?}", result.result_type);

    // Mixed addition: Integer + Decimal → Decimal (with implicit conversion)
    let result = resolver
        .resolve_binary(
            BinaryOperator::Add,
            &DataType::integer(),
            &DataType::decimal(),
        )
        .unwrap();
    println!("Integer + Decimal = {:?}", result.result_type);
    println!("  Conversions needed: {:?}", result.conversions);

    // Division always returns Decimal
    let result = resolver
        .resolve_binary(
            BinaryOperator::Divide,
            &DataType::integer(),
            &DataType::integer(),
        )
        .unwrap();
    println!("Integer / Integer = {:?}", result.result_type);

    // Quantity arithmetic
    let result = resolver
        .resolve_binary(
            BinaryOperator::Add,
            &DataType::quantity(),
            &DataType::quantity(),
        )
        .unwrap();
    println!("Quantity + Quantity = {:?}", result.result_type);

    println!();

    // -------------------------------------------------------------------------
    // Comparison Operators
    // -------------------------------------------------------------------------
    println!("--- Comparison Operators ---\n");

    // Equality works on any comparable types
    let result = resolver
        .resolve_binary(
            BinaryOperator::Equal,
            &DataType::string(),
            &DataType::string(),
        )
        .unwrap();
    println!("String = String → {:?}", result.result_type);

    // Less than on integers
    let result = resolver
        .resolve_binary(
            BinaryOperator::Less,
            &DataType::integer(),
            &DataType::integer(),
        )
        .unwrap();
    println!("Integer < Integer → {:?}", result.result_type);

    // Mixed comparison with implicit conversion
    let result = resolver
        .resolve_binary(
            BinaryOperator::Less,
            &DataType::integer(),
            &DataType::decimal(),
        )
        .unwrap();
    println!("Integer < Decimal → {:?}", result.result_type);
    println!("  Conversions: {:?}", result.conversions);

    println!();

    // -------------------------------------------------------------------------
    // Logical Operators
    // -------------------------------------------------------------------------
    println!("--- Logical Operators ---\n");

    let result = resolver
        .resolve_binary(
            BinaryOperator::And,
            &DataType::boolean(),
            &DataType::boolean(),
        )
        .unwrap();
    println!("Boolean and Boolean → {:?}", result.result_type);

    let result = resolver
        .resolve_unary(UnaryOperator::Not, &DataType::boolean())
        .unwrap();
    println!("not Boolean → {:?}", result.result_type);

    println!();

    // -------------------------------------------------------------------------
    // List Operators (Generic)
    // -------------------------------------------------------------------------
    println!("--- List Operators ---\n");

    let list_int = DataType::list(DataType::integer());
    let list_str = DataType::list(DataType::string());

    // Exists checks if a list is non-empty
    let result = resolver
        .resolve_unary(UnaryOperator::Exists, &list_int)
        .unwrap();
    println!("exists List<Integer> → {:?}", result.result_type);

    // Union of two lists
    let result = resolver
        .resolve_binary(BinaryOperator::Union, &list_int, &list_int)
        .unwrap();
    println!(
        "List<Integer> union List<Integer> → {:?}",
        result.result_type
    );

    // Membership: element in list
    let result = resolver
        .resolve_binary(BinaryOperator::In, &DataType::integer(), &list_int)
        .unwrap();
    println!("Integer in List<Integer> → {:?}", result.result_type);

    // Contains: list contains element
    let result = resolver
        .resolve_binary(BinaryOperator::Contains, &list_str, &DataType::string())
        .unwrap();
    println!("List<String> contains String → {:?}", result.result_type);

    println!();

    // -------------------------------------------------------------------------
    // Interval Operators
    // -------------------------------------------------------------------------
    println!("--- Interval Operators ---\n");

    let interval_int = DataType::interval(DataType::integer());
    let interval_dt = DataType::interval(DataType::date_time());

    // Start of interval
    let result = resolver
        .resolve_unary(UnaryOperator::Start, &interval_int)
        .unwrap();
    println!("start of Interval<Integer> → {:?}", result.result_type);

    // End of interval
    let result = resolver
        .resolve_unary(UnaryOperator::End, &interval_dt)
        .unwrap();
    println!("end of Interval<DateTime> → {:?}", result.result_type);

    // Point in interval
    let result = resolver
        .resolve_binary(BinaryOperator::In, &DataType::integer(), &interval_int)
        .unwrap();
    println!("Integer in Interval<Integer> → {:?}", result.result_type);

    // Interval overlaps
    let result = resolver
        .resolve_binary(BinaryOperator::Overlaps, &interval_int, &interval_int)
        .unwrap();
    println!(
        "Interval<Integer> overlaps Interval<Integer> → {:?}",
        result.result_type
    );

    println!();

    // -------------------------------------------------------------------------
    // Ternary Operators
    // -------------------------------------------------------------------------
    println!("--- Ternary Operators ---\n");

    // Between: value between low and high
    let result = resolver
        .resolve_ternary(
            TernaryOperator::Between,
            &DataType::integer(),
            &DataType::integer(),
            &DataType::integer(),
        )
        .unwrap();
    println!(
        "Integer between Integer and Integer → {:?}",
        result.result_type
    );

    println!();

    // -------------------------------------------------------------------------
    // Type Conversion Operators
    // -------------------------------------------------------------------------
    println!("--- Type Conversion ---\n");

    let result = resolver
        .resolve_unary(UnaryOperator::ToInteger, &DataType::string())
        .unwrap();
    println!("ToInteger(String) → {:?}", result.result_type);

    let result = resolver
        .resolve_unary(UnaryOperator::ToBoolean, &DataType::string())
        .unwrap();
    println!("ToBoolean(String) → {:?}", result.result_type);

    println!();

    // -------------------------------------------------------------------------
    // Custom Operator Registration
    // -------------------------------------------------------------------------
    println!("--- Custom Operator Registration ---\n");

    let mut custom_resolver = OperatorResolver::new();

    // Register a custom operator
    custom_resolver.register(OperatorSignature::binary(
        "CustomMerge",
        DataType::string(),
        DataType::string(),
        DataType::string(),
    ));

    let sigs = custom_resolver.get_signatures("CustomMerge").unwrap();
    println!("CustomMerge signatures: {}", sigs.len());
    println!("  Operand types: {:?}", sigs[0].operand_types);
    println!("  Result type: {:?}", sigs[0].result_type);

    println!();

    // -------------------------------------------------------------------------
    // Error Handling
    // -------------------------------------------------------------------------
    println!("--- Error Handling ---\n");

    // Unsupported operator
    let result = resolver.resolve_with_operands("NonExistentOperator", &[DataType::integer()]);
    println!("NonExistentOperator: {:?}", result.err());

    // No matching signature (String + Integer doesn't work)
    let result = resolver.resolve_binary(
        BinaryOperator::Add,
        &DataType::string(),
        &DataType::integer(),
    );
    println!("String + Integer: {:?}", result.err());

    println!("\n=== Demo Complete ===");
}
