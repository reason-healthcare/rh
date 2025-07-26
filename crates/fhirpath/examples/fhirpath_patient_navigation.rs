/// FHIRPath - Working with FHIR Resources Example
/// 
/// This example demonstrates how to use FHIRPath expressions to navigate
/// and extract data from real FHIR resources.

use anyhow::Result;
use serde_json::json;
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("🏥 FHIRPath with FHIR Resources Examples");
    println!("========================================\n");

    // Create a comprehensive Patient resource for testing
    let patient_resource = json!({
        "resourceType": "Patient",
        "id": "example-patient",
        "meta": {
            "versionId": "1",
            "lastUpdated": "2023-01-15T10:30:00Z"
        },
        "identifier": [
            {
                "use": "usual",
                "type": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                            "code": "MR",
                            "display": "Medical Record Number"
                        }
                    ]
                },
                "system": "http://hospital.example.org",
                "value": "123456"
            }
        ],
        "active": true,
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "James"],
                "prefix": ["Mr."]
            },
            {
                "use": "usual",
                "given": ["Johnny"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "+1-555-123-4567",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john.doe@example.com",
                "use": "home"
            },
            {
                "system": "phone",
                "value": "+1-555-987-6543",
                "use": "mobile"
            }
        ],
        "gender": "male",
        "birthDate": "1990-01-15",
        "address": [
            {
                "use": "home",
                "type": "physical",
                "line": ["123 Main Street", "Apt 4B"],
                "city": "Anytown",
                "state": "NY",
                "postalCode": "12345",
                "country": "US"
            }
        ],
        "contact": [
            {
                "relationship": [
                    {
                        "coding": [
                            {
                                "system": "http://terminology.hl7.org/CodeSystem/v2-0131",
                                "code": "C",
                                "display": "Emergency Contact"
                            }
                        ]
                    }
                ],
                "name": {
                    "family": "Doe",
                    "given": ["Jane"]
                },
                "telecom": [
                    {
                        "system": "phone",
                        "value": "+1-555-555-5555"
                    }
                ]
            }
        ]
    });

    let context = EvaluationContext::new(patient_resource);

    // Example 1: Basic resource navigation
    println!("1️⃣ Basic Resource Navigation:");
    let expressions = vec![
        "resourceType",                    // Get resource type
        "id",                             // Get resource ID
        "active",                         // Get active status
        "gender",                         // Get gender
        "birthDate",                      // Get birth date
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 2: Working with names
    println!("\n2️⃣ Name Operations:");
    let expressions = vec![
        "name.family",                    // Get all family names
        "name.given",                     // Get all given names
        "name.where(use = 'official').family",  // Official family name
        "name.where(use = 'official').given",   // Official given names
        "name.prefix",                    // Get prefixes
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 3: Contact information
    println!("\n3️⃣ Contact Information:");
    let expressions = vec![
        "telecom.value",                          // All contact values
        "telecom.where(system = 'email').value", // Email addresses
        "telecom.where(system = 'phone').value", // Phone numbers
        "telecom.where(use = 'home').value",     // Home contacts
        "telecom.where(use = 'mobile').value",   // Mobile contacts
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 4: Address information
    println!("\n4️⃣ Address Information:");
    let expressions = vec![
        "address.line",                   // Street address lines
        "address.city",                   // City
        "address.state",                  // State
        "address.postalCode",            // Postal code
        "address.country",               // Country
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 5: Identifiers
    println!("\n5️⃣ Identifiers:");
    let expressions = vec![
        "identifier.value",              // All identifier values
        "identifier.system",             // All identifier systems
        "identifier.where(use = 'usual').value",  // Usual identifier
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 6: Emergency contacts
    println!("\n6️⃣ Emergency Contacts:");
    let expressions = vec![
        "contact.name.family",           // Contact family names
        "contact.name.given",            // Contact given names
        "contact.telecom.value",         // Contact phone numbers
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 7: Metadata operations
    println!("\n7️⃣ Metadata Operations:");
    let expressions = vec![
        "meta.versionId",               // Version ID
        "meta.lastUpdated",            // Last updated timestamp
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 8: Conditional expressions
    println!("\n8️⃣ Conditional Expressions:");
    let expressions = vec![
        "gender = 'male'",              // Check if male
        "active = true",                // Check if active
        "name.exists()",                // Check if names exist
        "telecom.where(system = 'email').exists()",  // Check if email exists
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => println!("   {} = {:?}", expr_str, result),
                    Err(e) => println!("   {} = Error: {}", expr_str, e),
                }
            }
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    println!("\n✅ All FHIR resource examples completed!");
    println!("💡 Use the 'where()' function to filter collections");
    println!("💡 Chain expressions with '.' to navigate nested structures");
    println!("💡 Use 'exists()' to check for presence of data");
    println!("💡 Some complex expressions may require full FHIRPath implementation features");

    Ok(())
}
