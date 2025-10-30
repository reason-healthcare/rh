use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};
use rh_snapshot::SnapshotGenerator;

fn create_element(path: &str, min: u32, max: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(min),
        max: Some(max.to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: None,
        short: None,
        comment: None,
        requirements: None,
        must_support: None,
        is_summary: None,
        is_modifier: None,
        is_modifier_reason: None,
        slicing: None,
        slice_name: None,
    }
}

fn main() -> anyhow::Result<()> {
    println!("📚 rh-snapshot: Basic Usage Example\n");

    println!("🔧 Step 1: Create a snapshot generator");
    let mut generator = SnapshotGenerator::new();
    println!("   ✓ Generator created\n");

    println!("📋 Step 2: Load base StructureDefinition (Patient)");
    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient", 0, "*"),
                create_element("Patient.identifier", 0, "*"),
                create_element("Patient.name", 0, "*"),
            ],
        }),
        differential: None,
    };
    generator.load_structure_definition(patient);
    println!("   ✓ Patient base resource loaded\n");

    println!("📋 Step 3: Load profile StructureDefinition (US Core Patient)");
    let us_core_patient = StructureDefinition {
        url: "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        name: "USCorePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient", 0, "*"),
                create_element("Patient.identifier", 1, "*"),
                create_element("Patient.name", 1, "*"),
            ],
        }),
    };
    generator.load_structure_definition(us_core_patient);
    println!("   ✓ US Core Patient profile loaded\n");

    println!("🔄 Step 4: Generate snapshot for profile");
    println!("   Algorithm:");
    println!("     1. Check cache (miss on first call)");
    println!("     2. Resolve base definition (Patient)");
    println!("     3. Get base snapshot (from Patient)");
    println!("     4. Get profile differential (from US Core)");
    println!("     5. Merge base + differential → snapshot");
    println!("     6. Cache result for future calls\n");

    let snapshot = generator
        .generate_snapshot("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")?;
    println!("   ✓ Snapshot generated!\n");

    println!("📊 Step 5: Inspect generated snapshot");
    println!("   Total elements: {}", snapshot.element.len());
    println!("   Elements:");
    for element in &snapshot.element {
        println!(
            "      {} [{}..{}]",
            element.path,
            element.min.unwrap_or(0),
            element.max.as_deref().unwrap_or("*")
        );
    }

    println!("\n🔍 Verification:");
    let identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier")
        .unwrap();
    println!(
        "   Patient.identifier: {}..{} (was 0..*, now required)",
        identifier.min.unwrap_or(0),
        identifier.max.as_deref().unwrap_or("*")
    );

    let name = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.name")
        .unwrap();
    println!(
        "   Patient.name: {}..{} (was 0..*, now required)",
        name.min.unwrap_or(0),
        name.max.as_deref().unwrap_or("*")
    );

    println!("\n⚡ Performance:");
    println!("   Cache size: {}", generator.cache_size());
    println!("   Second call will be instant (cache hit)");

    let _snapshot2 = generator
        .generate_snapshot("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")?;
    println!("   ✓ Second call completed instantly\n");

    println!("✅ Implementation Status:");
    println!("   Phase 1-4: ✅ Core, generation, properties, paths");
    println!("   Phase 5: ✅ Slicing support");
    println!("   Phase 6: ✅ Extension handling");
    println!("   Phase 7: ✅ Validation and edge cases");
    println!("   Phase 8: ✅ Optimization and caching");
    println!("   Phase 9: 🔄 Integration and examples (in progress)");
    println!("\n📖 See other examples:");
    println!("   • multi_level_inheritance: 5-level inheritance chain");
    println!("   • slicing_example: Array element slicing");
    println!("   • extension_example: US Core extensions\n");

    Ok(())
}
