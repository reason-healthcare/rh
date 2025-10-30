use rh_snapshot::types::{
    Differential, ElementDefinition, ElementDiscriminator, ElementSlicing, Snapshot,
    StructureDefinition,
};
use rh_snapshot::SnapshotGenerator;

fn create_element(path: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(0),
        max: Some("*".to_string()),
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

fn create_slice(path: &str, slice_name: &str, min: u32) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(format!("{path}:{slice_name}")),
        min: Some(min),
        max: Some("1".to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: None,
        short: Some(format!("{slice_name} slice")),
        comment: None,
        requirements: None,
        must_support: None,
        is_summary: None,
        is_modifier: None,
        is_modifier_reason: None,
        slicing: None,
        slice_name: Some(slice_name.to_string()),
    }
}

fn main() -> anyhow::Result<()> {
    println!("✂️  FHIR Slicing Example\n");
    println!("Demonstrates how slicing splits array elements into named slices");
    println!("Example: Patient.identifier sliced into MRN, SSN, and DL slices\n");

    let mut generator = SnapshotGenerator::new();

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.identifier"),
                create_element("Patient.identifier.system"),
                create_element("Patient.identifier.value"),
            ],
        }),
        differential: None,
    };

    let slicing = ElementSlicing {
        discriminator: Some(vec![ElementDiscriminator {
            type_: "value".to_string(),
            path: "system".to_string(),
        }]),
        rules: Some("open".to_string()),
        ordered: Some(false),
        description: Some("Slice identifier by system".to_string()),
    };

    let sliced_patient = StructureDefinition {
        url: "http://example.org/StructureDefinition/patient-with-identifiers".to_string(),
        name: "PatientWithIdentifiers".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                ElementDefinition {
                    path: "Patient".to_string(),
                    id: Some("Patient".to_string()),
                    min: Some(0),
                    max: Some("*".to_string()),
                    slicing: None,
                    slice_name: None,
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
                },
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: Some("Patient.identifier".to_string()),
                    min: Some(0),
                    max: Some("*".to_string()),
                    slicing: Some(slicing),
                    slice_name: None,
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
                },
                create_slice("Patient.identifier", "MRN", 1),
                create_slice("Patient.identifier", "SSN", 0),
                create_slice("Patient.identifier", "DL", 0),
            ],
        }),
    };

    println!("📥 Loading StructureDefinitions...");
    generator.load_structure_definition(patient);
    generator.load_structure_definition(sliced_patient);

    println!("\n🔄 Generating snapshot with slicing...");
    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/patient-with-identifiers")?;

    println!("\n✅ Snapshot generated successfully!");
    println!("   Total elements: {}", snapshot.element.len());

    println!("\n📊 Base element:");
    let base_identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name.is_none());
    if let Some(elem) = base_identifier {
        println!(
            "   {} [{}..{}]",
            elem.path,
            elem.min.unwrap_or(0),
            elem.max.as_deref().unwrap_or("*")
        );
        if let Some(ref slicing) = elem.slicing {
            println!("      🔪 Slicing defined:");
            if let Some(ref discriminators) = slicing.discriminator {
                for disc in discriminators {
                    println!("         - type: {}, path: {}", disc.type_, disc.path);
                }
            }
            println!(
                "         - rules: {}",
                slicing.rules.as_deref().unwrap_or("closed")
            );
        }
    }

    println!("\n📊 Slices:");
    let slices: Vec<_> = snapshot
        .element
        .iter()
        .filter(|e| e.path == "Patient.identifier" && e.slice_name.is_some())
        .collect();

    for slice in &slices {
        println!(
            "   {}:{} [{}..{}]",
            slice.path,
            slice.slice_name.as_deref().unwrap_or(""),
            slice.min.unwrap_or(0),
            slice.max.as_deref().unwrap_or("*")
        );
    }

    println!("\n📊 Slice children (auto-generated):");
    let slice_children: Vec<_> = snapshot
        .element
        .iter()
        .filter(|e| {
            (e.path.starts_with("Patient.identifier.") && e.slice_name.is_some())
                || (e.path == "Patient.identifier" && e.slice_name.is_some())
        })
        .collect();

    for child in &slice_children {
        if let Some(slice_name) = &child.slice_name {
            println!("   {} [slice: {}]", child.path, slice_name);
        }
    }

    println!("\n🔍 Key concepts:");
    println!("   ✓ Slicing definition on base element (Patient.identifier)");
    println!("   ✓ Discriminator specifies how to match instances to slices");
    println!("   ✓ Each slice inherits from base element");
    println!("   ✓ Slice children (system, value) auto-created for each slice");
    println!("   ✓ Slices can have different cardinality (MRN required, others optional)");

    println!("\n💡 Use case:");
    println!("   A hospital system requires:");
    println!("   - At least one MRN (Medical Record Number)");
    println!("   - Optional SSN (Social Security Number)");
    println!("   - Optional DL (Driver's License)");
    println!("   Each identified by different system URL");

    Ok(())
}
