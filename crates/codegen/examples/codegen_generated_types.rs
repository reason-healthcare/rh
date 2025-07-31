/// FHIR Code Generation - Working with Generated Types
///
/// This example shows how to generate Rust types from FHIR StructureDefinitions
/// and demonstrates what the generated code workflow looks like.
use rh_codegen::{CodeGenerator, CodegenConfig, StructureDefinition};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  FHIR Code Generation - Generated Types Example");

    // Example 1: Set up code generation with Serde support
    println!("\n📋 Setting up code generation with Serde support...");

    let config = CodegenConfig {
        output_dir: "generated".to_string(),
        module_name: "fhir_types".to_string(),
        with_serde: true, // Enable Serialize/Deserialize
        with_docs: true,  // Include documentation comments
        type_mappings: HashMap::new(),
    };

    let mut generator = CodeGenerator::new(config);

    // Example 2: Create StructureDefinitions for common FHIR types
    println!("🏥 Creating StructureDefinitions for FHIR resources...");

    // Patient resource
    let patient_def = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        version: Some("4.0.1".to_string()),
        name: "Patient".to_string(),
        title: Some("Patient".to_string()),
        status: "active".to_string(),
        description: Some("A person who is the subject of healthcare services".to_string()),
        purpose: Some("Patient resource for healthcare management".to_string()),
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: None,
        snapshot: None,
    };

    // Observation resource
    let observation_def = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Observation".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        version: Some("4.0.1".to_string()),
        name: "Observation".to_string(),
        title: Some("Observation".to_string()),
        status: "active".to_string(),
        description: Some("Measurements and simple assertions made about a patient".to_string()),
        purpose: Some("Observation resource for capturing healthcare measurements".to_string()),
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: None,
        snapshot: None,
    };

    // Example 3: Generate Rust structs
    println!("⚙️  Generating Rust structs from StructureDefinitions...");

    let patient_struct = generator.generate_struct(&patient_def)?;
    let observation_struct = generator.generate_struct(&observation_def)?;

    println!("✅ Generated struct: {}", patient_struct.name);
    println!(
        "   - With Serde: {}",
        patient_struct.derives.contains(&"Serialize".to_string())
    );
    println!("   - Documentation: {:?}", patient_struct.doc_comment);

    println!("✅ Generated struct: {}", observation_struct.name);
    println!(
        "   - With Serde: {}",
        observation_struct
            .derives
            .contains(&"Serialize".to_string())
    );
    println!("   - Documentation: {:?}", observation_struct.doc_comment);

    // Example 4: Generate to files
    println!("\n📁 Writing generated types to files...");

    std::fs::create_dir_all("generated")?;
    generator.generate_to_file(&patient_def, "generated/patient.rs")?;
    generator.generate_to_file(&observation_def, "generated/observation.rs")?;

    println!("✅ Generated files:");
    println!("   - generated/patient.rs");
    println!("   - generated/observation.rs");

    // Example 5: Demonstrate type safety benefits
    println!("\n🛡️  Benefits of generated types:");
    println!("   ✅ Type-safe field access");
    println!("   ✅ Automatic Serde serialization/deserialization");
    println!("   ✅ IDE autocomplete and error checking");
    println!("   ✅ Documentation generation");
    println!("   ✅ Consistent naming conventions");

    // Example 6: Show what generated code enables
    println!("\n💡 Generated types enable patterns like:");
    println!("   // Type-safe resource creation");
    println!("   let patient = Patient {{ /* fields */ }};");
    println!("   ");
    println!("   // JSON serialization");
    println!("   let json = serde_json::to_string(&patient)?;");
    println!("   ");
    println!("   // JSON deserialization");
    println!("   let patient: Patient = serde_json::from_str(&json)?;");

    println!("\n🎉 Code generation completed successfully!");
    println!("📖 Check the generated/*.rs files to see the generated Rust code");

    Ok(())
}
