use anyhow::Result;
use rh_snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

struct ComparisonResult {
    profile_name: String,
    profile_url: String,
    official_count: usize,
    generated_count: usize,
    id_mismatches: Vec<String>,
    count_mismatch: bool,
    cardinality_diffs: usize,
    property_diffs: usize,
}

impl ComparisonResult {
    fn is_perfect(&self) -> bool {
        self.official_count == self.generated_count
            && self.id_mismatches.is_empty()
            && self.cardinality_diffs == 0
            && self.property_diffs == 0
    }

    fn total_issues(&self) -> usize {
        let count_issue = if self.count_mismatch { 1 } else { 0 };
        count_issue + self.id_mismatches.len() + self.cardinality_diffs + self.property_diffs
    }
}

fn compare_snapshot(
    generator: &mut SnapshotGenerator,
    packages_dir: &Path,
    profile_url: &str,
    json_filename: &str,
) -> Result<ComparisonResult> {
    let generated_snapshot = generator.generate_snapshot(profile_url)?;

    let official_file = packages_dir
        .join("hl7.fhir.us.qicore#6.0.0")
        .join("package")
        .join(json_filename);

    let official_json = std::fs::read_to_string(&official_file)?;
    let official_sd: Value = serde_json::from_str(&official_json)?;

    let profile_name = official_sd["name"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string();

    let official_snapshot = official_sd["snapshot"]["element"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("No snapshot in official"))?;

    let official_count = official_snapshot.len();
    let generated_count = generated_snapshot.element.len();

    let mut id_mismatches = Vec::new();
    let mut cardinality_diffs = 0;
    let mut property_diffs = 0;

    let max_len = official_count.max(generated_count);

    for i in 0..max_len {
        if let (Some(official_elem), Some(generated_elem)) =
            (official_snapshot.get(i), generated_snapshot.element.get(i))
        {
            let official_id = official_elem["id"].as_str().unwrap_or("?");
            let generated_id = generated_elem.id.as_deref().unwrap_or(&generated_elem.path);

            if official_id != generated_id {
                id_mismatches.push(format!("#{i}: {official_id} vs {generated_id}"));
                continue;
            }

            let off_min = official_elem["min"].as_u64();
            let gen_min = generated_elem.min.map(|m| m as u64);
            if off_min != gen_min {
                cardinality_diffs += 1;
            }

            let off_max = official_elem["max"].as_str();
            let gen_max = generated_elem.max.as_deref();
            if off_max != gen_max {
                cardinality_diffs += 1;
            }

            let off_must_support = official_elem.get("mustSupport").and_then(|v| v.as_bool());
            let gen_must_support = generated_elem.must_support;
            if off_must_support != gen_must_support {
                property_diffs += 1;
            }
        }
    }

    Ok(ComparisonResult {
        profile_name,
        profile_url: profile_url.to_string(),
        official_count,
        generated_count,
        id_mismatches,
        count_mismatch: official_count != generated_count,
        cardinality_diffs,
        property_diffs,
    })
}

fn main() -> Result<()> {
    println!("🧪 Comprehensive QI-Core Validation Report\n");
    println!("Testing all QI-Core StructureDefinitions against official snapshots\n");

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
        print!("   Loading {name}@{version}...");
        let structure_defs =
            StructureDefinitionLoader::load_from_package(name, version, &packages_dir)?;
        println!(" {}", structure_defs.len());
        for sd in structure_defs {
            generator.load_structure_definition(sd);
        }
    }

    println!("\n📋 Finding all QI-Core profiles...");
    let qi_core_dir = packages_dir
        .join("hl7.fhir.us.qicore#6.0.0")
        .join("package");

    let mut profiles = Vec::new();
    for entry in std::fs::read_dir(&qi_core_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("StructureDefinition-") && filename.ends_with(".json") {
                let content = std::fs::read_to_string(&path)?;
                let sd: Value = serde_json::from_str(&content)?;
                if let Some(url) = sd.get("url").and_then(|u| u.as_str()) {
                    if sd.get("snapshot").is_some() && sd.get("derivation").is_some() {
                        profiles.push((url.to_string(), filename.to_string()));
                    }
                }
            }
        }
    }

    println!(
        "   Found {} QI-Core profiles with snapshots\n",
        profiles.len()
    );

    println!("🔄 Testing each profile...\n");

    let mut results = Vec::new();
    let mut errors = HashMap::new();

    for (i, (url, filename)) in profiles.iter().enumerate() {
        print!("   [{}/{}] Testing {}...", i + 1, profiles.len(), filename);
        match compare_snapshot(&mut generator, &packages_dir, url, filename) {
            Ok(result) => {
                if result.is_perfect() {
                    println!(" ✅");
                } else {
                    println!(" ⚠️  {} issues", result.total_issues());
                }
                results.push(result);
            }
            Err(e) => {
                println!(" ❌ Error");
                errors.insert(filename.clone(), e.to_string());
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("\n📊 VALIDATION REPORT\n");

    let perfect = results.iter().filter(|r| r.is_perfect()).count();
    let issues = results.iter().filter(|r| !r.is_perfect()).count();

    println!("Overall Results:");
    println!("   Total profiles tested: {}", results.len());
    println!(
        "   Perfect matches: {} ({}%)",
        perfect,
        (perfect * 100) / results.len()
    );
    println!(
        "   Profiles with issues: {} ({}%)",
        issues,
        (issues * 100) / results.len()
    );
    println!("   Errors: {}", errors.len());

    if !errors.is_empty() {
        println!("\n❌ Profiles with Errors:");
        for (filename, error) in &errors {
            println!("   {filename}");
            println!("      Error: {error}");
        }
    }

    if issues > 0 {
        println!("\n⚠️  Profiles with Issues:\n");

        let mut issues_by_type: HashMap<String, Vec<&ComparisonResult>> = HashMap::new();

        for result in &results {
            if !result.is_perfect() {
                if result.count_mismatch {
                    issues_by_type
                        .entry("Element Count Mismatch".to_string())
                        .or_default()
                        .push(result);
                }
                if !result.id_mismatches.is_empty() {
                    issues_by_type
                        .entry("Element ID Mismatch".to_string())
                        .or_default()
                        .push(result);
                }
                if result.cardinality_diffs > 0 {
                    issues_by_type
                        .entry("Cardinality Differences".to_string())
                        .or_default()
                        .push(result);
                }
                if result.property_diffs > 0 {
                    issues_by_type
                        .entry("Property Differences".to_string())
                        .or_default()
                        .push(result);
                }
            }
        }

        for (issue_type, profiles) in &issues_by_type {
            println!("   {issue_type}: {} profiles", profiles.len());
            for result in profiles.iter().take(5) {
                println!(
                    "      - {} ({} vs {} elements)",
                    result.profile_name, result.official_count, result.generated_count
                );
            }
            if profiles.len() > 5 {
                println!("      ... and {} more", profiles.len() - 5);
            }
            println!();
        }

        println!("\n📋 Detailed Issue Summary (Top 10):\n");
        let mut sorted_results: Vec<_> = results.iter().filter(|r| !r.is_perfect()).collect();
        sorted_results.sort_by_key(|r| std::cmp::Reverse(r.total_issues()));

        for (i, result) in sorted_results.iter().take(10).enumerate() {
            println!(
                "   {}. {} ({} issues)",
                i + 1,
                result.profile_name,
                result.total_issues()
            );
            println!("      URL: {}", result.profile_url);
            println!(
                "      Elements: {} official, {} generated",
                result.official_count, result.generated_count
            );

            if result.count_mismatch {
                println!("      ⚠️  Element count mismatch");
            }
            if !result.id_mismatches.is_empty() {
                println!(
                    "      ⚠️  {} ID mismatches (first 3):",
                    result.id_mismatches.len()
                );
                for id_mismatch in result.id_mismatches.iter().take(3) {
                    println!("         {id_mismatch}");
                }
            }
            if result.cardinality_diffs > 0 {
                println!(
                    "      ⚠️  {} cardinality differences",
                    result.cardinality_diffs
                );
            }
            if result.property_diffs > 0 {
                println!("      ⚠️  {} property differences", result.property_diffs);
            }
            println!();
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("\n🎯 Conclusion:\n");

    if perfect == results.len() && errors.is_empty() {
        println!(
            "   ✅ PERFECT! All {} QI-Core profiles match official snapshots exactly!",
            results.len()
        );
        println!("   rh-snapshot is fully compliant with QI-Core Implementation Guide");
    } else {
        println!(
            "   Overall match rate: {}%",
            (perfect * 100) / results.len()
        );
        if issues > 0 {
            println!("   {issues} profiles need investigation");
        }
        if !errors.is_empty() {
            let error_count = errors.len();
            println!("   {error_count} profiles had generation errors");
        }
    }

    println!(
        "\n   Cache performance: {} snapshots cached",
        generator.cache_size()
    );

    Ok(())
}
