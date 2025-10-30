use anyhow::Result;
use rh_snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🧪 Testing rh-snapshot against real QI-Core Patient\n");

    let packages_dir = PathBuf::from(std::env::var("HOME")?)
        .join(".fhir")
        .join("packages");

    println!("📦 Loading FHIR packages...");

    let mut generator = SnapshotGenerator::new();

    let packages = vec![
        ("hl7.fhir.r4.core", "4.0.1"),
        ("hl7.fhir.us.core", "6.1.0"),
        ("hl7.fhir.us.qicore", "6.0.0"),
    ];

    for (name, version) in &packages {
        println!("   Loading {name}@{version}");
        let structure_defs =
            StructureDefinitionLoader::load_from_package(name, version, &packages_dir)?;
        println!("      Loaded {} definitions", structure_defs.len());
        for sd in structure_defs {
            generator.load_structure_definition(sd);
        }
    }

    println!("\n🔄 Generating snapshot for QI-Core Patient...");
    let qi_core_url = "http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient";

    let generated_snapshot = generator.generate_snapshot(qi_core_url)?;
    println!("   Generated {} elements", generated_snapshot.element.len());

    println!("\n📄 Loading official QI-Core Patient StructureDefinition...");
    let qi_core_file = packages_dir
        .join("hl7.fhir.us.qicore#6.0.0")
        .join("package")
        .join("StructureDefinition-qicore-patient.json");

    let official_json = std::fs::read_to_string(&qi_core_file)?;
    let official_sd: Value = serde_json::from_str(&official_json)?;

    let official_snapshot = official_sd
        .get("snapshot")
        .and_then(|s| s.get("element"))
        .and_then(|e| e.as_array())
        .ok_or_else(|| anyhow::anyhow!("No snapshot found in official QI-Core Patient"))?;

    println!(
        "   Official snapshot has {} elements",
        official_snapshot.len()
    );

    println!("\n🔍 Comparing snapshots...\n");

    let mut generated_map: HashMap<String, &rh_snapshot::types::ElementDefinition> =
        generated_snapshot
            .element
            .iter()
            .map(|e| {
                let id = e.id.as_ref().unwrap_or(&e.path);
                (id.clone(), e)
            })
            .collect();

    let generated_ids: HashSet<String> = generated_snapshot
        .element
        .iter()
        .map(|e| e.id.as_ref().unwrap_or(&e.path).clone())
        .collect();

    let official_paths: HashSet<String> = official_snapshot
        .iter()
        .filter_map(|e| {
            let _path = e.get("path")?.as_str()?;
            let id = e.get("id")?.as_str()?;
            Some(id.to_string())
        })
        .collect();

    let mut missing_in_generated = Vec::new();
    let mut extra_in_generated = Vec::new();
    let mut cardinality_mismatches = Vec::new();

    for official_elem in official_snapshot {
        let id = official_elem.get("id").and_then(|v| v.as_str());
        let path = official_elem.get("path").and_then(|v| v.as_str());

        if let (Some(id), Some(path)) = (id, path) {
            if let Some(generated_elem) = generated_map.remove(id) {
                let official_min = official_elem
                    .get("min")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let official_max = official_elem
                    .get("max")
                    .and_then(|v| v.as_str())
                    .unwrap_or("*");

                let generated_min = generated_elem.min.unwrap_or(0);
                let generated_max = generated_elem.max.as_deref().unwrap_or("*");

                if official_min != generated_min as u64 || official_max != generated_max {
                    cardinality_mismatches.push((
                        id.to_string(),
                        format!("{official_min}..{official_max}"),
                        format!("{generated_min}..{generated_max}"),
                    ));
                }
            } else {
                missing_in_generated.push((id.to_string(), path.to_string()));
            }
        }
    }

    for id in generated_ids.iter() {
        if !official_paths.contains(id) {
            extra_in_generated.push(id.clone());
        }
    }

    println!("📊 Comparison Results:\n");
    println!("   Official snapshot: {} elements", official_snapshot.len());
    println!(
        "   Generated snapshot: {} elements\n",
        generated_snapshot.element.len()
    );

    if missing_in_generated.is_empty()
        && extra_in_generated.is_empty()
        && cardinality_mismatches.is_empty()
    {
        println!("✅ Perfect match! No differences found.\n");
    } else {
        if !missing_in_generated.is_empty() {
            println!(
                "❌ Elements in official but missing in generated ({}):",
                missing_in_generated.len()
            );
            for (id, path) in missing_in_generated.iter().take(20) {
                println!("   - {id} ({path})");
            }
            if missing_in_generated.len() > 20 {
                println!("   ... and {} more", missing_in_generated.len() - 20);
            }
            println!();
        }

        if !extra_in_generated.is_empty() {
            println!(
                "⚠️  Elements in generated but not in official ({}):",
                extra_in_generated.len()
            );
            for id in extra_in_generated.iter().take(20) {
                println!("   - {id}");
            }
            if extra_in_generated.len() > 20 {
                println!("   ... and {} more", extra_in_generated.len() - 20);
            }
            println!();
        }

        if !cardinality_mismatches.is_empty() {
            println!(
                "⚠️  Cardinality mismatches ({}):",
                cardinality_mismatches.len()
            );
            for (id, official, generated) in cardinality_mismatches.iter().take(10) {
                println!("   - {id}");
                println!("      Official:  {official}");
                println!("      Generated: {generated}");
            }
            if cardinality_mismatches.len() > 10 {
                println!("   ... and {} more", cardinality_mismatches.len() - 10);
            }
            println!();
        }
    }

    println!("🔍 Analysis of potential issues:\n");

    if !missing_in_generated.is_empty() {
        println!("1. Missing Elements:");
        println!("   - Check if element IDs are being generated correctly");
        println!("   - Verify slice name handling in element ID generation");
        println!("   - Review backbone elements and choice types\n");
    }

    if !extra_in_generated.is_empty() {
        println!("2. Extra Elements:");
        println!("   - May be creating duplicate slice children");
        println!("   - Check expand_slice_children logic");
        println!("   - Verify element filtering logic\n");
    }

    if !cardinality_mismatches.is_empty() {
        println!("3. Cardinality Mismatches:");
        println!("   - Verify cardinality merging logic");
        println!("   - Check inheritance chain processing");
        println!("   - Review differential override logic\n");
    }

    Ok(())
}
