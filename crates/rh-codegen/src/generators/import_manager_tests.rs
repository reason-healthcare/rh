use serde::{Deserialize, Serialize};

// Test the import manager with some example struct definitions
use crate::generators::import_manager::ImportManager;
use crate::rust_types::{RustField, RustStruct, RustType};
use std::collections::HashSet;

#[test]
fn test_import_collection_comprehensive() {
    // Create a test struct that uses various FHIR types
    let mut test_struct = RustStruct::new("TestResource".to_string());

    // Add fields with different types that were causing import issues
    test_struct.fields.push(RustField::new(
        "identifier".to_string(),
        RustType::Option(Box::new(RustType::Vec(Box::new(RustType::Custom(
            "Identifier".to_string(),
        ))))),
    ));

    test_struct.fields.push(RustField::new(
        "status".to_string(),
        RustType::Custom("PublicationStatus".to_string()),
    ));

    test_struct.fields.push(RustField::new(
        "subject".to_string(),
        RustType::Option(Box::new(RustType::Custom("Reference".to_string()))),
    ));

    test_struct.fields.push(RustField::new(
        "contact".to_string(),
        RustType::Option(Box::new(RustType::Vec(Box::new(RustType::Custom(
            "ContactDetail".to_string(),
        ))))),
    ));

    test_struct.fields.push(RustField::new(
        "note".to_string(),
        RustType::Option(Box::new(RustType::Vec(Box::new(RustType::Custom(
            "Annotation".to_string(),
        ))))),
    ));

    test_struct.fields.push(RustField::new(
        "timing".to_string(),
        RustType::Option(Box::new(RustType::Custom("Timing".to_string()))),
    ));

    test_struct.fields.push(RustField::new(
        "date".to_string(),
        RustType::Option(Box::new(RustType::Custom("DateTimeType".to_string()))),
    ));

    // Test import collection
    let mut imports = HashSet::new();
    let structs_in_file = HashSet::new();

    ImportManager::collect_custom_types_from_struct(&test_struct, &mut imports, &structs_in_file);

    // Verify that all expected imports are collected
    assert!(imports.contains("crate::datatypes::identifier::Identifier"));
    assert!(imports.contains("crate::bindings::publication_status::PublicationStatus"));
    assert!(imports.contains("crate::datatypes::reference::Reference"));
    assert!(imports.contains("crate::datatypes::contact_detail::ContactDetail"));
    assert!(imports.contains("crate::datatypes::annotation::Annotation"));
    assert!(imports.contains("crate::datatypes::timing::Timing"));
    assert!(imports.contains("crate::primitives::date_time::DateTimeType"));

    println!("Collected imports:");
    for import in &imports {
        println!("  {}", import);
    }

    assert_eq!(imports.len(), 7);
}

#[test]
fn test_type_categorization() {
    // Test that our type categorization is working correctly
    assert!(ImportManager::is_fhir_datatype("Identifier"));
    assert!(ImportManager::is_fhir_datatype("Reference"));
    assert!(ImportManager::is_fhir_datatype("ContactDetail"));
    assert!(ImportManager::is_fhir_datatype("Annotation"));
    assert!(ImportManager::is_fhir_datatype("Timing"));

    assert!(ImportManager::is_fhir_primitive_type("DateTimeType"));
    assert!(ImportManager::is_fhir_primitive_type("StringType"));
    assert!(ImportManager::is_fhir_primitive_type("BooleanType"));

    // Test import path generation
    assert_eq!(
        ImportManager::get_import_path_for_type("Identifier"),
        "crate::datatypes::identifier::Identifier"
    );
    assert_eq!(
        ImportManager::get_import_path_for_type("DateTimeType"),
        "crate::primitives::date_time::DateTimeType"
    );
    assert_eq!(
        ImportManager::get_import_path_for_type("PublicationStatus"),
        "crate::bindings::publication_status::PublicationStatus"
    );
}
