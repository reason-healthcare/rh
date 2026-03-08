//! Example: CQL DataType System
//!
//! This example demonstrates the internal DataType system used for CQL
//! type checking, type compatibility, and implicit conversions.
//!
//! Run with: `cargo run --example datatype_system`

use rh_cql::datatype::{find_conversion, implicit_conversions, DataType, SystemType, TupleElement};

fn main() {
    println!("=== CQL DataType System Example ===\n");

    // Example 1: System (primitive) types
    println!("1. System Types:");
    let primitives = [
        SystemType::Boolean,
        SystemType::Integer,
        SystemType::Long,
        SystemType::Decimal,
        SystemType::String,
        SystemType::Date,
        SystemType::DateTime,
        SystemType::Time,
        SystemType::Quantity,
        SystemType::Code,
        SystemType::Concept,
    ];

    for sys_type in &primitives {
        let simple = sys_type.simple_name();
        let qualified = sys_type.qualified_name();
        println!("   {simple} -> {qualified}");
    }

    // Example 2: Creating DataTypes
    println!("\n2. Creating DataTypes:");

    let int_type = DataType::integer();
    let string_type = DataType::string();
    let patient_type = DataType::model("FHIR", "Patient");
    let list_of_strings = DataType::list(DataType::string());
    let date_interval = DataType::interval(DataType::date());

    println!("   Integer: {int_type}");
    println!("   String: {string_type}");
    println!("   Patient: {patient_type}");
    println!("   List<String>: {list_of_strings}");
    println!("   Interval<Date>: {date_interval}");

    // Example 3: Tuple types
    println!("\n3. Tuple Types:");
    let address_tuple = DataType::tuple(vec![
        TupleElement {
            name: "street".to_string(),
            element_type: Box::new(DataType::string()),
        },
        TupleElement {
            name: "city".to_string(),
            element_type: Box::new(DataType::string()),
        },
        TupleElement {
            name: "zip".to_string(),
            element_type: Box::new(DataType::string()),
        },
    ]);
    println!("   Address: {address_tuple}");

    // Example 4: Choice types
    println!("\n4. Choice Types:");
    let value_x = DataType::choice(vec![
        DataType::integer(),
        DataType::string(),
        DataType::boolean(),
    ]);
    println!("   value[x]: {value_x}");

    // Example 5: Subtype relationships
    println!("\n5. Subtype Relationships:");

    // Numeric tower: Integer <: Long <: Decimal
    let int_subtype_long = DataType::integer().is_subtype_of(&DataType::long());
    let int_subtype_decimal = DataType::integer().is_subtype_of(&DataType::decimal());
    let long_subtype_decimal = DataType::long().is_subtype_of(&DataType::decimal());
    let decimal_subtype_int = DataType::decimal().is_subtype_of(&DataType::integer());

    println!("   Integer <: Long: {int_subtype_long}");
    println!("   Integer <: Decimal: {int_subtype_decimal}");
    println!("   Long <: Decimal: {long_subtype_decimal}");
    println!("   Decimal <: Integer: {decimal_subtype_int}");

    // Temporal: Date <: DateTime
    let date_subtype_datetime = DataType::date().is_subtype_of(&DataType::date_time());
    println!("   Date <: DateTime: {date_subtype_datetime}");

    // Everything <: Any
    let string_subtype_any = DataType::string().is_subtype_of(&DataType::any());
    let patient_subtype_any = patient_type.is_subtype_of(&DataType::any());
    println!("   String <: Any: {string_subtype_any}");
    println!("   Patient <: Any: {patient_subtype_any}");

    // List covariance
    let list_int = DataType::list(DataType::integer());
    let list_decimal = DataType::list(DataType::decimal());
    let list_covariance = list_int.is_subtype_of(&list_decimal);
    println!("   List<Integer> <: List<Decimal>: {list_covariance}");

    // Choice subtyping
    let choice = DataType::choice(vec![DataType::integer(), DataType::string()]);
    let int_subtype_choice = DataType::integer().is_subtype_of(&choice);
    println!("   Integer <: Choice<Integer, String>: {int_subtype_choice}");

    // Example 6: Implicit conversions
    println!("\n6. Implicit Conversions:");

    let int_to_decimal = DataType::integer().can_convert_to(&DataType::decimal());
    let code_to_concept = DataType::code().can_convert_to(&DataType::concept());
    let string_to_int = DataType::string().can_convert_to(&DataType::integer());

    println!("   Integer -> Decimal: {int_to_decimal}");
    println!("   Code -> Concept: {code_to_concept}");
    println!("   String -> Integer: {string_to_int}");

    // Find conversion function
    if let Some(conv) = find_conversion(&DataType::integer(), &DataType::decimal()) {
        let func = conv.function;
        println!("   Integer->Decimal function: {func}");
    }

    // List all implicit conversions
    println!("\n   All implicit conversions:");
    for conv in implicit_conversions() {
        let from = &conv.from;
        let to = &conv.to;
        let func = conv.function;
        println!("      {from} -> {to} ({func})");
    }

    // Example 7: Common type resolution
    println!("\n7. Common Type Resolution:");

    let common_numeric = DataType::integer().common_type(&DataType::decimal());
    let common_temporal = DataType::date().common_type(&DataType::date_time());
    let common_unrelated = DataType::string().common_type(&DataType::boolean());
    let common_lists =
        DataType::list(DataType::integer()).common_type(&DataType::list(DataType::long()));

    println!("   Common(Integer, Decimal) = {common_numeric}");
    println!("   Common(Date, DateTime) = {common_temporal}");
    println!("   Common(String, Boolean) = {common_unrelated}");
    println!("   Common(List<Integer>, List<Long>) = {common_lists}");

    // Example 8: Type queries
    println!("\n8. Type Queries:");

    let types = [
        DataType::integer(),
        DataType::string(),
        DataType::date(),
        DataType::list(DataType::integer()),
        DataType::interval(DataType::date()),
        DataType::model("FHIR", "Patient"),
    ];

    for t in &types {
        let numeric = t.is_numeric();
        let temporal = t.is_temporal();
        let list = t.is_list();
        let interval = t.is_interval();
        let model = t.is_model();
        println!("   {t} - numeric: {numeric}, temporal: {temporal}, list: {list}, interval: {interval}, model: {model}");
    }

    // Example 9: Qualified names for ELM
    println!("\n9. Qualified Names (for ELM serialization):");

    let types_for_elm = [
        DataType::integer(),
        DataType::string(),
        DataType::model("FHIR", "Patient"),
        DataType::list(DataType::integer()),
    ];

    for t in &types_for_elm {
        let qname = t.to_qualified_name();
        println!("   {t} -> {qname}");
    }

    // Example 10: SystemType parsing
    println!("\n10. Parsing System Types:");

    let names = [
        "Integer",
        "{urn:hl7-org:elm-types:r1}Decimal",
        "System.String",
        "Unknown",
    ];

    for name in &names {
        match SystemType::from_name(name) {
            Some(st) => println!("   '{name}' -> {st}"),
            None => println!("   '{name}' -> (not a system type)"),
        }
    }

    println!("\n=== Example Complete ===");
}
