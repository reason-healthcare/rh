//! Integration test modules for FHIRPath functionality
//!
//! This module organizes integration tests to mirror the source code structure
//! for better maintainability and organization.

pub mod parser_integration;
pub mod evaluator_basic;
pub mod arithmetic_operations;
pub mod comparison_operations;
pub mod membership_operations;
pub mod collection_functions;
pub mod filtering_functions;

// Common test utilities
use fhirpath::*;
use serde_json::json;

/// Create a sample FHIR Patient resource for testing
pub fn sample_patient() -> serde_json::Value {
    json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "James"]
            },
            {
                "use": "usual",
                "family": "Doe",
                "given": ["Johnny"]
            },
            {
                "use": "maiden",
                "family": "Smith",
                "given": ["Jane"]
            }
        ],
        "birthDate": "1974-12-25",
        "telecom": []
    })
}

/// Create a simple FHIR Patient for arithmetic testing
pub fn simple_patient() -> serde_json::Value {
    json!({
        "resourceType": "Patient",
        "age": 30,
        "weight": 70.5,
        "height": 175
    })
}
