use rh_codegen::StructureDefinition;
use serde_json::json;

/// Test to validate that trait implementations correctly use nested types for BackboneElement fields
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing trait implementation BackboneElement type handling...");

    // Create a test Account-like structure with coverage field
    let test_structure = json!({
        "resourceType": "StructureDefinition",
        "id": "account",
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
                }
            ]
        }
    });

    // Parse the structure definition
    let structure_def: StructureDefinition = serde_json::from_value(test_structure)?;

    // Create generator and generate trait implementations

    // Generate the trait implementation using the internal trait impl generator
    let trait_impl_generator = rh_codegen::generators::TraitImplGenerator::new();
    let trait_impls = trait_impl_generator.generate_trait_impls(&structure_def)?;

    println!("Generated {} trait implementations", trait_impls.len());

    // Find the specific Account trait implementation
    if let Some(account_impl) = trait_impls
        .iter()
        .find(|impl_| impl_.trait_name.contains("AccountAccessors"))
    {
        println!(
            "Found AccountAccessors implementation with {} methods",
            account_impl.methods.len()
        );

        // Check coverage method
        if let Some(coverage_method) = account_impl.methods.iter().find(|m| m.name == "coverage") {
            println!("coverage() return type: {}", coverage_method.return_type);
            println!("coverage() body: {}", coverage_method.body);

            // Should return &[AccountCoverage], not &[BackboneElement]
            if coverage_method.return_type.contains("AccountCoverage") {
                println!("✅ Coverage method correctly returns &[AccountCoverage]!");
            } else if coverage_method.return_type.contains("BackboneElement") {
                println!("❌ Coverage method incorrectly returns &[BackboneElement]");
                println!("   Expected: &[AccountCoverage]");
                println!("   Got: {}", coverage_method.return_type);
            } else {
                println!(
                    "⚠️  Unexpected return type: {}",
                    coverage_method.return_type
                );
            }
        } else {
            println!("❌ No coverage method found in trait implementation");
        }

        // Check guarantor method
        if let Some(guarantor_method) = account_impl.methods.iter().find(|m| m.name == "guarantor")
        {
            println!("guarantor() return type: {}", guarantor_method.return_type);
            println!("guarantor() body: {}", guarantor_method.body);

            // Should return &[AccountGuarantor], not &[BackboneElement]
            if guarantor_method.return_type.contains("AccountGuarantor") {
                println!("✅ Guarantor method correctly returns &[AccountGuarantor]!");
            } else if guarantor_method.return_type.contains("BackboneElement") {
                println!("❌ Guarantor method incorrectly returns &[BackboneElement]");
                println!("   Expected: &[AccountGuarantor]");
                println!("   Got: {}", guarantor_method.return_type);
            } else {
                println!(
                    "⚠️  Unexpected return type: {}",
                    guarantor_method.return_type
                );
            }
        } else {
            println!("❌ No guarantor method found in trait implementation");
        }
    } else {
        println!("❌ No AccountAccessors trait implementation found");
        println!("Available trait implementations:");
        for impl_ in &trait_impls {
            println!(
                "  - {} (with {} methods)",
                impl_.trait_name,
                impl_.methods.len()
            );
        }
    }

    println!("🎯 Trait implementation nested BackboneElement type handling test completed!");

    Ok(())
}
