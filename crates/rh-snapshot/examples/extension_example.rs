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

fn create_extension_slice(slice_name: &str, min: u32, url: &str) -> ElementDefinition {
    ElementDefinition {
        path: "Patient.extension".to_string(),
        id: Some(format!("Patient.extension:{slice_name}")),
        min: Some(min),
        max: Some("1".to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: Some(format!("Extension: {url}")),
        short: Some(format!("{slice_name} extension")),
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
    println!("🔌 FHIR Extension Example\n");
    println!("Demonstrates extension handling as specialized slicing");
    println!("Example: US Core Patient with race, ethnicity, and birthsex extensions\n");

    let mut generator = SnapshotGenerator::new();

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.extension"),
                create_element("Patient.extension.url"),
                create_element("Patient.extension.value[x]"),
            ],
        }),
        differential: None,
    };

    let extension_slicing = ElementSlicing {
        discriminator: Some(vec![ElementDiscriminator {
            type_: "value".to_string(),
            path: "url".to_string(),
        }]),
        rules: Some("open".to_string()),
        ordered: Some(false),
        description: Some("Extensions are sliced by URL".to_string()),
    };

    let us_core_patient = StructureDefinition {
        url: "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        name: "USCorePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.extension".to_string(),
                    id: Some("Patient.extension".to_string()),
                    min: Some(0),
                    max: Some("*".to_string()),
                    slicing: Some(extension_slicing),
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
                create_extension_slice(
                    "race",
                    0,
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                ),
                create_extension_slice(
                    "ethnicity",
                    0,
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                ),
                create_extension_slice(
                    "birthsex",
                    0,
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                ),
            ],
        }),
    };

    println!("📥 Loading StructureDefinitions...");
    generator.load_structure_definition(patient);
    generator.load_structure_definition(us_core_patient);

    println!("\n🔄 Generating snapshot with extensions...");
    let snapshot = generator
        .generate_snapshot("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")?;

    println!("\n✅ Snapshot generated successfully!");
    println!("   Total elements: {}", snapshot.element.len());

    println!("\n📊 Base extension element:");
    let base_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name.is_none());
    if let Some(elem) = base_extension {
        println!(
            "   {} [{}..{}]",
            elem.path,
            elem.min.unwrap_or(0),
            elem.max.as_deref().unwrap_or("*")
        );
        if let Some(ref slicing) = elem.slicing {
            println!("      🔪 Extension slicing:");
            if let Some(ref discriminators) = slicing.discriminator {
                for disc in discriminators {
                    println!("         - discriminator: {} by {}", disc.type_, disc.path);
                }
            }
        }
    }

    println!("\n📊 Extension slices:");
    let extension_slices: Vec<_> = snapshot
        .element
        .iter()
        .filter(|e| e.path == "Patient.extension" && e.slice_name.is_some())
        .collect();

    for slice in &extension_slices {
        println!(
            "   {}:{} [{}..{}]",
            slice.path,
            slice.slice_name.as_deref().unwrap_or(""),
            slice.min.unwrap_or(0),
            slice.max.as_deref().unwrap_or("*")
        );
        if let Some(ref def) = slice.definition {
            println!("      {def}");
        }
    }

    println!("\n📊 Extension slice children (auto-generated):");
    let extension_children: Vec<_> = snapshot
        .element
        .iter()
        .filter(|e| {
            e.path.starts_with("Patient.extension.")
                && e.slice_name.is_some()
                && e.path != "Patient.extension"
        })
        .collect();

    for child in &extension_children {
        if let Some(slice_name) = &child.slice_name {
            println!("   {} [{}]", child.path, slice_name);
        }
    }

    println!("\n🔍 Key concepts:");
    println!("   ✓ Extensions are standard FHIR elements (Patient.extension)");
    println!("   ✓ Extensions are sliced by discriminator type='value' path='url'");
    println!("   ✓ Each extension slice represents a different extension URL");
    println!("   ✓ Extension children (url, value[x]) auto-created for each slice");
    println!("   ✓ All extensions optional by default (min=0)");

    println!("\n💡 US Core extensions:");
    println!("   • race: OMB race categories");
    println!("   • ethnicity: OMB ethnicity categories");
    println!("   • birthsex: Birth sex (M/F/UNK)");
    println!("\n   These extensions are widely used in US healthcare systems");

    println!("\n🎯 Extension vs regular slicing:");
    println!("   Extensions ARE slicing, just with a standardized pattern:");
    println!("   - Always slice by 'url' with discriminator type 'value'");
    println!("   - Each slice references a different StructureDefinition");
    println!("   - Children (url, value[x], extension) inherited from base");

    Ok(())
}
