use rh_codegen::StructureDefinition;
use serde_json::json;

/// Test to validate that array method bodies use correct access patterns
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing array method body generation...");

    // Create a test structure with both optional and required arrays
    let test_structure = json!({
        "resourceType": "StructureDefinition",
        "id": "test-array-access",
        "url": "http://hl7.org/fhir/StructureDefinition/TestArrayAccess",
        "version": "4.0.1",
        "name": "TestArrayAccess",
        "title": "Test Array Access",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "TestArrayAccess",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "differential": {
            "element": [
                {
                    "id": "TestArrayAccess",
                    "path": "TestArrayAccess",
                    "definition": "Test resource for array access patterns",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "TestArrayAccess.requiredArray",
                    "path": "TestArrayAccess.requiredArray",
                    "definition": "A required array (min=1, max=*)",
                    "min": 1,
                    "max": "*",
                    "type": [{"code": "BackboneElement"}]
                },
                {
                    "id": "TestArrayAccess.optionalArray",
                    "path": "TestArrayAccess.optionalArray",
                    "definition": "An optional array (min=0, max=*)",
                    "min": 0,
                    "max": "*",
                    "type": [{"code": "BackboneElement"}]
                },
                {
                    "id": "TestArrayAccess.requiredSingle",
                    "path": "TestArrayAccess.requiredSingle",
                    "definition": "A required single element (min=1, max=1)",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "string"}]
                },
                {
                    "id": "TestArrayAccess.optionalSingle",
                    "path": "TestArrayAccess.optionalSingle",
                    "definition": "An optional single element (min=0, max=1)",
                    "min": 0,
                    "max": "1",
                    "type": [{"code": "string"}]
                }
            ]
        }
    });

    // Parse the structure definition
    let structure_def: StructureDefinition = serde_json::from_value(test_structure)?;

    // Generate trait implementations
    let trait_impl_generator = rh_codegen::generators::TraitImplGenerator::new();
    let trait_impls = trait_impl_generator.generate_trait_impls(&structure_def)?;

    println!("Generated {} trait implementations", trait_impls.len());

    // Find the specific resource trait implementation
    if let Some(test_impl) = trait_impls
        .iter()
        .find(|impl_| impl_.trait_name.contains("TestArrayAccessAccessors"))
    {
        println!(
            "Found TestArrayAccessAccessors implementation with {} methods",
            test_impl.methods.len()
        );

        // Check required array method
        if let Some(required_array_method) = test_impl
            .methods
            .iter()
            .find(|m| m.name == "required_array")
        {
            println!(
                "required_array() return type: {}",
                required_array_method.return_type
            );
            println!("required_array() body: {}", required_array_method.body);

            // Required array should use direct reference: &self.required_array
            if required_array_method.body.contains("&self.required_array") {
                println!("✅ Required array method correctly uses direct reference!");
            } else if required_array_method.body.contains("as_deref") {
                println!("❌ Required array method incorrectly uses as_deref()");
                println!("   Expected: &self.required_array");
                println!("   Got: {}", required_array_method.body);
            } else {
                println!("⚠️  Unexpected method body: {}", required_array_method.body);
            }
        }

        // Check optional array method
        if let Some(optional_array_method) = test_impl
            .methods
            .iter()
            .find(|m| m.name == "optional_array")
        {
            println!(
                "optional_array() return type: {}",
                optional_array_method.return_type
            );
            println!("optional_array() body: {}", optional_array_method.body);

            // Optional array should use as_deref(): self.optional_array.as_deref().unwrap_or(&[])
            if optional_array_method.body.contains("as_deref") {
                println!("✅ Optional array method correctly uses as_deref()!");
            } else if optional_array_method.body.contains("&self.optional_array") {
                println!("❌ Optional array method incorrectly uses direct reference");
                println!("   Expected: self.optional_array.as_deref().unwrap_or(&[])");
                println!("   Got: {}", optional_array_method.body);
            } else {
                println!("⚠️  Unexpected method body: {}", optional_array_method.body);
            }
        }

        // Check required single element
        if let Some(required_single_method) = test_impl
            .methods
            .iter()
            .find(|m| m.name == "required_single")
        {
            println!(
                "required_single() return type: {}",
                required_single_method.return_type
            );
            println!("required_single() body: {}", required_single_method.body);

            // Required single should return String directly, not Option<String>
            if required_single_method.return_type.contains("StringType")
                && !required_single_method.return_type.contains("Option")
            {
                println!("✅ Required single method correctly returns StringType directly!");
            } else {
                println!(
                    "⚠️  Unexpected return type: {}",
                    required_single_method.return_type
                );
            }
        }

        // Check optional single element
        if let Some(optional_single_method) = test_impl
            .methods
            .iter()
            .find(|m| m.name == "optional_single")
        {
            println!(
                "optional_single() return type: {}",
                optional_single_method.return_type
            );
            println!("optional_single() body: {}", optional_single_method.body);

            // Optional single should return Option<StringType>
            if optional_single_method
                .return_type
                .contains("Option<StringType>")
            {
                println!("✅ Optional single method correctly returns Option<StringType>!");
            } else {
                println!(
                    "⚠️  Unexpected return type: {}",
                    optional_single_method.return_type
                );
            }
        }
    } else {
        println!("❌ No TestArrayAccessAccessors trait implementation found");
        println!("Available trait implementations:");
        for impl_ in &trait_impls {
            println!(
                "  - {} (with {} methods)",
                impl_.trait_name,
                impl_.methods.len()
            );
        }
    }

    println!("🎯 Array method body generation test completed!");

    Ok(())
}
