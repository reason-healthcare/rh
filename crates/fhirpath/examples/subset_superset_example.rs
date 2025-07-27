//! # Subset and Superset Operations Example
//!
//! This example demonstrates FHIRPath's `subsetOf()` and `supersetOf()` collection functions
//! for testing set relationships between collections.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Sample FHIR Patient resource with various collections
    let patient_data = json!({
        "resourceType": "Patient",
        "id": "example-patient",
        "active": true,
        "name": [
            {
                "use": "official",
                "family": "Smith",
                "given": ["John", "William"]
            },
            {
                "use": "nickname",
                "given": ["Johnny"]
            }
        ],
        "telecom": [
            {"system": "phone", "value": "555-1234", "use": "home"},
            {"system": "phone", "value": "555-5678", "use": "work"},
            {"system": "email", "value": "john@example.com", "use": "home"}
        ],
        "address": [
            {
                "use": "home",
                "type": "physical",
                "line": ["123 Main St"],
                "city": "Springfield",
                "state": "IL",
                "postalCode": "62701",
                "country": "US"
            }
        ],
        "extension": [
            {
                "url": "http://example.org/patient-category",
                "valueString": "outpatient"
            },
            {
                "url": "http://example.org/patient-priority",
                "valueString": "standard"
            }
        ],
        // For testing purposes - some custom arrays
        "approvedTreatments": ["medication", "therapy", "surgery"],
        "availableTreatments": ["medication", "therapy", "surgery", "counseling", "rehabilitation"],
        "contactTypes": ["phone", "email"],
        "allContactTypes": ["phone", "email", "fax", "sms", "video"],
        "activeSystems": ["phone", "email"],
        "emergencyContacts": ["555-9999"]
    });

    let context = EvaluationContext::new(patient_data.clone());

    println!("=== FHIRPath Subset and Superset Operations ===\n");

    // Basic subset operations
    println!("1. Basic Subset Testing:");

    // Test if approved treatments are a subset of available treatments
    let expr = parser.parse("approvedTreatments.subsetOf(availableTreatments)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   approvedTreatments ⊆ availableTreatments: {:?}", result);

    // Test a partial subset
    let expr = parser.parse("('medication' | 'therapy').subsetOf(approvedTreatments)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   {{medication, therapy}} ⊆ approvedTreatments: {:?}",
        result
    );

    // Test a non-subset
    let expr = parser.parse("('medication' | 'counseling').subsetOf(approvedTreatments)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   {{medication, counseling}} ⊆ approvedTreatments: {:?}",
        result
    );

    println!("\n2. Basic Superset Testing:");

    // Test if available treatments are a superset of approved treatments
    let expr = parser.parse("availableTreatments.supersetOf(approvedTreatments)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   availableTreatments ⊇ approvedTreatments: {:?}", result);

    // Test contact types superset relationship
    let expr = parser.parse("allContactTypes.supersetOf(contactTypes)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   allContactTypes ⊇ contactTypes: {:?}", result);

    // Test a non-superset
    let expr = parser.parse("contactTypes.supersetOf(allContactTypes)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   contactTypes ⊇ allContactTypes: {:?}", result);

    println!("\n3. FHIR Resource Field Testing:");

    // Test if patient's given names are a subset of all given names
    let expr = parser.parse("name[0].given.subsetOf(name.given)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   first name's given names ⊆ all given names: {:?}",
        result
    );

    // Test if active systems are a subset of telecom systems
    let expr = parser.parse("activeSystems.subsetOf(telecom.system)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   activeSystems ⊆ telecom.system: {:?}", result);

    // Test if telecom systems are a superset of contact types
    let expr = parser.parse("telecom.system.supersetOf(contactTypes)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   telecom.system ⊇ contactTypes: {:?}", result);

    println!("\n4. Empty Collection Edge Cases:");

    // Empty set is a subset of any set
    let expr = parser.parse("{}.subsetOf(approvedTreatments)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   empty set ⊆ approvedTreatments: {:?}", result);

    // Any set is a superset of empty set
    let expr = parser.parse("approvedTreatments.supersetOf({})")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   approvedTreatments ⊇ empty set: {:?}", result);

    // Empty set relationships to itself
    let expr = parser.parse("{}.subsetOf({})")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   empty set ⊆ empty set: {:?}", result);

    let expr = parser.parse("{}.supersetOf({})")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   empty set ⊇ empty set: {:?}", result);

    println!("\n5. Mixed Type Collections:");

    // Test with mixed numeric types
    let expr = parser.parse("(1 | 2).subsetOf(1 | 2 | 3.0)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   {{1, 2}} ⊆ {{1, 2, 3.0}}: {:?}", result);

    // Test with strings and numbers
    let expr = parser.parse("(true | 'test').subsetOf(true | 'test' | 42)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   {{true, 'test'}} ⊆ {{true, 'test', 42}}: {:?}", result);

    println!("\n6. Practical FHIR Use Cases:");

    // Check if patient has all required contact methods
    let _required_contact = json!(["phone", "email"]);
    let required_context = EvaluationContext::new(json!({
        "required": ["phone", "email"],
        "patient": patient_data.clone()
    }));

    let expr = parser.parse("required.subsetOf(patient.telecom.system.distinct())")?;
    let result = evaluator.evaluate(&expr, &required_context)?;
    println!("   Patient has all required contact methods: {:?}", result);

    // Check if patient's treatments include emergency options
    let _emergency_treatments = json!(["surgery"]);
    let emergency_context = EvaluationContext::new(json!({
        "emergency": ["surgery"],
        "patient": patient_data
    }));

    let expr = parser.parse("emergency.subsetOf(patient.approvedTreatments)")?;
    let result = evaluator.evaluate(&expr, &emergency_context)?;
    println!("   Patient approved for emergency treatments: {:?}", result);

    println!("\n7. Set Equality (both subset and superset):");

    // Two sets are equal if each is a subset of the other
    let expr =
        parser.parse("(1 | 2 | 3).subsetOf(3 | 2 | 1) and (3 | 2 | 1).subsetOf(1 | 2 | 3)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   {{1,2,3}} = {{3,2,1}} (order-independent equality): {:?}",
        result
    );

    // Same test using superset
    let expr =
        parser.parse("(1 | 2 | 3).supersetOf(3 | 2 | 1) and (3 | 2 | 1).supersetOf(1 | 2 | 3)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   {{1,2,3}} = {{3,2,1}} (using superset): {:?}", result);

    println!("\n=== Summary ===");
    println!("✅ subsetOf(other): Returns true if all elements in this collection are also in the other collection");
    println!("✅ supersetOf(other): Returns true if this collection contains all elements from the other collection");
    println!("✅ Empty collections: ∅ ⊆ any set, any set ⊇ ∅, ∅ ⊆ ∅, ∅ ⊇ ∅");
    println!("✅ Set equality: A = B iff A ⊆ B and B ⊆ A (or A ⊇ B and B ⊇ A)");
    println!(
        "✅ Order independence: {{1,2,3}} ⊆ {{3,1,2}} evaluates element membership, not order"
    );
    println!("✅ Type safety: Elements are compared using FHIRPath type equality rules");

    Ok(())
}
