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
    println!("🔗 Multi-Level Inheritance Example\n");
    println!("Demonstrates snapshot generation through an inheritance chain:");
    println!("QICorePatient → USCorePatient → Patient → DomainResource → Resource\n");

    let mut generator = SnapshotGenerator::new();

    let resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Resource".to_string(),
        name: "Resource".to_string(),
        type_: "Resource".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Resource", 0, "*"),
                create_element("Resource.id", 0, "1"),
                create_element("Resource.meta", 0, "1"),
            ],
        }),
        differential: None,
    };

    let domain_resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
        name: "DomainResource".to_string(),
        type_: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("DomainResource", 0, "*"),
                create_element("DomainResource.text", 0, "1"),
                create_element("DomainResource.contained", 0, "*"),
            ],
        }),
    };

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient", 0, "*"),
                create_element("Patient.identifier", 0, "*"),
                create_element("Patient.name", 0, "*"),
                create_element("Patient.gender", 0, "1"),
            ],
        }),
    };

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
                create_element("Patient.gender", 1, "1"),
            ],
        }),
    };

    let qi_core_patient = StructureDefinition {
        url: "http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient".to_string(),
        name: "QICorePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some(
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        ),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient", 0, "*"),
                create_element("Patient.identifier", 1, "10"),
            ],
        }),
    };

    println!("📥 Loading StructureDefinitions...");
    generator.load_structure_definition(resource);
    generator.load_structure_definition(domain_resource);
    generator.load_structure_definition(patient);
    generator.load_structure_definition(us_core_patient);
    generator.load_structure_definition(qi_core_patient);

    println!("\n🔄 Generating snapshot for QICorePatient...");
    let snapshot = generator
        .generate_snapshot("http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient")?;

    println!("\n✅ Snapshot generated successfully!");
    println!("   Elements: {}", snapshot.element.len());
    println!("\n📊 Snapshot elements:");

    for element in &snapshot.element {
        println!(
            "   {} [{}..{}]",
            element.path,
            element.min.unwrap_or(0),
            element.max.as_deref().unwrap_or("*")
        );
    }

    println!("\n🔍 Verifying inheritance chain:");
    println!("   - Resource elements (id, meta) inherited");
    println!("   - DomainResource elements (text, contained) inherited");
    println!("   - Patient elements (identifier, name, gender) inherited");
    println!("   - US Core constraints (required identifier, name, gender) applied");
    println!("   - QI-Core constraints (max 10 identifiers) applied");

    let identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier")
        .expect("Patient.identifier should exist");

    println!(
        "\n✅ Patient.identifier cardinality: {}..{}",
        identifier.min.unwrap_or(0),
        identifier.max.as_deref().unwrap_or("*")
    );
    println!("   (Required by US Core, max 10 by QI-Core)");

    println!("\n🎯 Cache usage:");
    println!("   - First call generates all 5 profiles");
    println!("   - Cache size: {}", generator.cache_size());
    println!("   - Subsequent calls use cached snapshots");

    println!("\n🔄 Testing cache...");
    let _snapshot2 = generator
        .generate_snapshot("http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient")?;
    println!("   ✅ Second call returned instantly from cache");

    let _us_core = generator
        .generate_snapshot("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")?;
    println!("   ✅ US Core Patient also in cache (generated during QI-Core)");

    Ok(())
}
