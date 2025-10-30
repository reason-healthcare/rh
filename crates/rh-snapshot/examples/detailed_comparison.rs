use anyhow::Result;
use rh_snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use serde_json::Value;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🔬 Detailed Comparison: rh-snapshot vs Official QI-Core Patient\n");

    let packages_dir = PathBuf::from(std::env::var("HOME")?)
        .join(".fhir")
        .join("packages");

    println!("📦 Loading FHIR packages...");
    let mut generator = SnapshotGenerator::new();

    for (name, version) in &[
        ("hl7.fhir.r4.core", "4.0.1"),
        ("hl7.fhir.us.core", "6.1.0"),
        ("hl7.fhir.us.qicore", "6.0.0"),
    ] {
        let structure_defs =
            StructureDefinitionLoader::load_from_package(name, version, &packages_dir)?;
        for sd in structure_defs {
            generator.load_structure_definition(sd);
        }
    }

    let qi_core_url = "http://hl7.org/fhir/us/qicore/StructureDefinition/qicore-patient";
    let generated_snapshot = generator.generate_snapshot(qi_core_url)?;

    let qi_core_file = packages_dir
        .join("hl7.fhir.us.qicore#6.0.0")
        .join("package")
        .join("StructureDefinition-qicore-patient.json");

    let official_json = std::fs::read_to_string(&qi_core_file)?;
    let official_sd: Value = serde_json::from_str(&official_json)?;
    let official_snapshot = official_sd["snapshot"]["element"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("No snapshot in official"))?;

    println!("\n📊 Element Count Comparison:");
    println!("   Official:  {} elements", official_snapshot.len());
    println!(
        "   Generated: {} elements",
        generated_snapshot.element.len()
    );

    println!("\n🔍 Detailed Element-by-Element Comparison:\n");

    let mut differences = Vec::new();
    let mut perfect_matches = 0;
    let mut property_mismatches = 0;

    for (i, official_elem) in official_snapshot.iter().enumerate() {
        let official_id = official_elem["id"].as_str().unwrap_or("?");
        let official_path = official_elem["path"].as_str().unwrap_or("?");

        if let Some(generated_elem) = generated_snapshot.element.get(i) {
            let generated_id = generated_elem.id.as_deref().unwrap_or(&generated_elem.path);
            let generated_path = &generated_elem.path;

            if official_id != generated_id || official_path != generated_path {
                differences.push(format!(
                    "Element #{i}: ID mismatch\n   Official:  {official_id} ({official_path})\n   Generated: {generated_id} ({generated_path})"
                ));
                continue;
            }

            let mut element_diffs = Vec::new();

            let off_min = official_elem["min"].as_u64();
            let gen_min = generated_elem.min.map(|m| m as u64);
            if off_min != gen_min {
                element_diffs.push(format!("     min: {off_min:?} vs {gen_min:?}"));
            }

            let off_max = official_elem["max"].as_str();
            let gen_max = generated_elem.max.as_deref();
            if off_max != gen_max {
                element_diffs.push(format!("     max: {off_max:?} vs {gen_max:?}"));
            }

            let off_must_support = official_elem["mustSupport"].as_bool();
            let gen_must_support = generated_elem.must_support;
            if off_must_support != gen_must_support {
                element_diffs.push(format!(
                    "     mustSupport: {off_must_support:?} vs {gen_must_support:?}"
                ));
            }

            if official_elem.get("binding").is_some() != generated_elem.binding.is_some() {
                element_diffs.push(format!(
                    "     binding presence: {} vs {}",
                    official_elem.get("binding").is_some(),
                    generated_elem.binding.is_some()
                ));
            }

            if let (Some(off_binding), Some(gen_binding)) =
                (official_elem.get("binding"), &generated_elem.binding)
            {
                let off_strength = off_binding["strength"].as_str();
                let gen_strength = Some(gen_binding.strength.as_str());
                if off_strength != gen_strength {
                    element_diffs.push(format!(
                        "     binding.strength: {off_strength:?} vs {gen_strength:?}"
                    ));
                }
            }

            let off_constraint_count = official_elem
                .get("constraint")
                .and_then(|c| c.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let gen_constraint_count = generated_elem
                .constraint
                .as_ref()
                .map(|c| c.len())
                .unwrap_or(0);
            if off_constraint_count != gen_constraint_count {
                element_diffs.push(format!(
                    "     constraint count: {off_constraint_count} vs {gen_constraint_count}"
                ));
            }

            if !element_diffs.is_empty() {
                property_mismatches += 1;
                differences.push(format!(
                    "Element #{}: {} ({})\n{}",
                    i,
                    official_id,
                    official_path,
                    element_diffs.join("\n")
                ));
            } else {
                perfect_matches += 1;
            }
        } else {
            differences.push(format!(
                "Element #{i}: Missing in generated ({official_id})"
            ));
        }
    }

    println!("✅ Perfect matches: {perfect_matches}");
    println!("⚠️  Elements with property differences: {property_mismatches}");
    println!(
        "❌ Structural differences: {}",
        differences.len() - property_mismatches
    );

    if !differences.is_empty() {
        println!("\n📋 Detailed Differences (first 20):\n");
        for diff in differences.iter().take(20) {
            println!("{diff}\n");
        }
        if differences.len() > 20 {
            println!("... and {} more differences\n", differences.len() - 20);
        }
    } else {
        println!("\n🎉 Perfect match! All {perfect_matches} elements match exactly!\n");
    }

    println!("\n📈 Summary:");
    println!("   Total elements checked: {}", official_snapshot.len());
    println!("   Exact matches: {perfect_matches}");
    println!("   Property mismatches: {property_mismatches}");
    println!(
        "   Match rate: {:.1}%",
        (perfect_matches as f64 / official_snapshot.len() as f64) * 100.0
    );

    if differences.is_empty() {
        println!("\n✅ CONCLUSION: rh-snapshot generates snapshots that perfectly match");
        println!("   the official FHIR implementation for QI-Core Patient!");
    } else {
        println!("\n⚠️  CONCLUSION: Some differences found. Review above for details.");
    }

    Ok(())
}
