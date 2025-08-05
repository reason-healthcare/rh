//! Tests for choice type field generation
//!
//! These tests verify that FHIR choice types (fields ending with [x]) are properly
//! expanded into multiple typed fields according to the FHIR specification.

use crate::fhir_types::{ElementDefinition, ElementType};
use crate::generators::FieldGenerator;
use crate::config::CodegenConfig;
use crate::value_sets::ValueSetManager;
use std::collections::HashMap;

#[test]
fn test_choice_type_field_generation() {
    let config = CodegenConfig::default();
    let type_cache = HashMap::new();
    let mut value_set_manager = ValueSetManager::new();
    
    let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);
    
    // Create a test element definition for a choice type
    let element = ElementDefinition {
        id: Some("Observation.effective[x]".to_string()),
        path: "Observation.effective[x]".to_string(),
        short: Some("Clinically relevant time/time-period for observation".to_string()),
        definition: Some("The time or time-period the observed value is asserted as being true.".to_string()),
        min: Some(0),
        max: Some("1".to_string()),
        element_type: Some(vec![
            ElementType {
                code: Some("dateTime".to_string()),
                target_profile: None,
            },
            ElementType {
                code: Some("Period".to_string()),
                target_profile: None,
            },
            ElementType {
                code: Some("Timing".to_string()),
                target_profile: None,
            },
            ElementType {
                code: Some("instant".to_string()),
                target_profile: None,
            },
        ]),
        fixed: None,
        pattern: None,
        binding: None,
    };
    
    // Generate fields
    let fields = field_generator.create_fields_from_element(&element).unwrap();
    
    // Verify that multiple fields were generated
    assert_eq!(fields.len(), 4, "Should generate 4 fields for 4 choice types");
    
    // Verify field names
    let field_names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
    assert!(field_names.contains(&"effective_date_time"));
    assert!(field_names.contains(&"effective_period"));
    assert!(field_names.contains(&"effective_timing"));
    assert!(field_names.contains(&"effective_instant"));
    
    // Verify serde rename attributes
    assert_eq!(fields[0].serde_rename, Some("effectiveDateTime".to_string()));
    assert_eq!(fields[1].serde_rename, Some("effectivePeriod".to_string()));
    assert_eq!(fields[2].serde_rename, Some("effectiveTiming".to_string()));
    assert_eq!(fields[3].serde_rename, Some("effectiveInstant".to_string()));
    
    // Verify all fields are optional
    for field in &fields {
        assert!(field.is_optional, "Choice type fields should be optional");
    }
}

#[test]
fn test_non_choice_type_field_generation() {
    let config = CodegenConfig::default();
    let type_cache = HashMap::new();
    let mut value_set_manager = ValueSetManager::new();
    
    let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);
    
    // Create a test element definition for a regular field
    let element = ElementDefinition {
        id: Some("Observation.status".to_string()),
        path: "Observation.status".to_string(),
        short: Some("Status of the observation".to_string()),
        definition: Some("The status of the result value.".to_string()),
        min: Some(1),
        max: Some("1".to_string()),
        element_type: Some(vec![
            ElementType {
                code: Some("code".to_string()),
                target_profile: None,
            },
        ]),
        fixed: None,
        pattern: None,
        binding: None,
    };
    
    // Generate fields
    let fields = field_generator.create_fields_from_element(&element).unwrap();
    
    // Verify that only one field was generated
    assert_eq!(fields.len(), 1, "Should generate 1 field for non-choice type");
    
    // Verify field name
    assert_eq!(fields[0].name, "status");
    
    // Verify it's not optional (min = 1)
    assert!(!fields[0].is_optional, "Required fields should not be optional");
}

#[test] 
fn test_choice_type_documentation() {
    let config = CodegenConfig::default();
    let type_cache = HashMap::new();
    let mut value_set_manager = ValueSetManager::new();
    
    let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);
    
    // Create a test element definition for a choice type
    let element = ElementDefinition {
        id: Some("Observation.value[x]".to_string()),
        path: "Observation.value[x]".to_string(),
        short: Some("Actual result".to_string()),
        definition: Some("The information determined as a result of making the observation.".to_string()),
        min: Some(0),
        max: Some("1".to_string()),
        element_type: Some(vec![
            ElementType {
                code: Some("Quantity".to_string()),
                target_profile: None,
            },
            ElementType {
                code: Some("string".to_string()),
                target_profile: None,
            },
        ]),
        fixed: None,
        pattern: None,
        binding: None,
    };
    
    // Generate fields
    let fields = field_generator.create_fields_from_element(&element).unwrap();
    
    // Verify that documentation includes type information
    assert_eq!(fields.len(), 2);
    
    let quantity_field = &fields[0];
    let string_field = &fields[1];
    
    assert!(quantity_field.doc_comment.as_ref().unwrap().contains("(Quantity)"));
    assert!(string_field.doc_comment.as_ref().unwrap().contains("(string)"));
}
