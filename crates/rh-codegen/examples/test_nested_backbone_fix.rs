use rh_codegen::{CodeGenerator, CodegenConfig, StructureDefinition};
use serde_json::json;

/// Test to validate that nested BackboneElement fields use specific nested types
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing nested BackboneElement type handling...");

    // Create a test Account-like structure with coverage field
    let test_structure = json!({
        "resourceType": "StructureDefinition",
        "id": "test-account",
        "url": "http://hl7.org/fhir/StructureDefinition/Account",
        "version": "4.0.1",
        "name": "Account",
        "title": "Account",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "Account",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "differential": {
            "element": [
                {
                    "id": "Account",
                    "path": "Account",
                    "definition": "A financial tool for tracking value",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "Account.coverage",
                    "path": "Account.coverage",
                    "definition": "The party(s) that are responsible for covering the payment",
                    "min": 0,
                    "max": "*",
                    "type": [{"code": "BackboneElement"}]
                },
                {
                    "id": "Account.guarantor",
                    "path": "Account.guarantor",
                    "definition": "The parties ultimately responsible for balancing the Account",
                    "min": 0,
                    "max": "*",
                    "type": [{"code": "BackboneElement"}]
                },
                {
                    "id": "Account.name",
                    "path": "Account.name",
                    "definition": "Human-readable label",
                    "min": 0,
                    "max": "1",
                    "type": [{"code": "string"}]
                }
            ]
        }
    });

    // Parse the structure definition
    let structure_def: StructureDefinition = serde_json::from_value(test_structure)?;

    // Create generator
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Generate trait
    let traits = generator.generate_trait(&structure_def)?;

    println!("Generated {} traits", traits.len());

    // Check accessor trait for the right return types
    if let Some(accessor_trait) = traits.iter().find(|t| t.name.contains("Accessor")) {
        println!("Found accessor trait: {}", accessor_trait.name);

        // Check coverage method
        if let Some(coverage_method) = accessor_trait.methods.iter().find(|m| m.name == "coverage")
        {
            if let Some(return_type) = &coverage_method.return_type {
                let return_type_str = format!("{return_type:?}");
                println!("coverage() return type: {return_type_str}");

                // Should return AccountCoverage slice, not BackboneElement slice
                if return_type_str.contains("AccountCoverage") {
                    println!("✅ Coverage method correctly returns AccountCoverage type!");
                } else if return_type_str.contains("BackboneElement") {
                    println!("❌ Coverage method incorrectly returns BackboneElement type");
                    println!("   Expected: AccountCoverage");
                    println!("   Got: {return_type_str}");
                } else {
                    println!("⚠️  Unexpected return type: {return_type_str}");
                }
            }
        }

        // Check guarantor method
        if let Some(guarantor_method) = accessor_trait
            .methods
            .iter()
            .find(|m| m.name == "guarantor")
        {
            if let Some(return_type) = &guarantor_method.return_type {
                let return_type_str = format!("{return_type:?}");
                println!("guarantor() return type: {return_type_str}");

                // Should return AccountGuarantor slice, not BackboneElement slice
                if return_type_str.contains("AccountGuarantor") {
                    println!("✅ Guarantor method correctly returns AccountGuarantor type!");
                } else if return_type_str.contains("BackboneElement") {
                    println!("❌ Guarantor method incorrectly returns BackboneElement type");
                    println!("   Expected: AccountGuarantor");
                    println!("   Got: {return_type_str}");
                } else {
                    println!("⚠️  Unexpected return type: {return_type_str}");
                }
            }
        }

        // Check name method (should still be string, not BackboneElement)
        if let Some(name_method) = accessor_trait.methods.iter().find(|m| m.name == "name") {
            if let Some(return_type) = &name_method.return_type {
                let return_type_str = format!("{return_type:?}");
                println!("name() return type: {return_type_str}");

                if return_type_str.contains("String") {
                    println!("✅ Name method correctly returns String type!");
                } else {
                    println!("❌ Name method has unexpected type: {return_type_str}");
                }
            }
        }
    }

    println!("🎯 Nested BackboneElement type handling test completed!");

    Ok(())
}
