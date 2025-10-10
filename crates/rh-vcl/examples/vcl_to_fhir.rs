//! Example demonstrating VCL to FHIR ValueSet translation
//!
//! This example shows how to translate various VCL expressions into FHIR ValueSet.compose structures.

use rh_vcl::{translate_vcl_string_to_fhir, VclTranslator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("VCL to FHIR ValueSet Translation Examples");
    println!("==========================================\n");

    // Example translations from VCL to FHIR ValueSet.compose
    let examples = vec![
        (
            "Simple code",
            "(http://snomed.info/sct)123456",
            "Single code from SNOMED CT"
        ),
        (
            "Quoted code with spaces",
            "(http://example.org)\"code with spaces\"",
            "Code containing spaces (must be quoted in VCL)"
        ),
        (
            "Wildcard - all codes",
            "(http://snomed.info/sct)*",
            "Include all codes from the code system"
        ),
        (
            "Property filter - equals",
            "(http://snomed.info/sct)status = \"active\"",
            "Filter by property equality"
        ),
        (
            "Property filter - is-a relationship",
            "(http://snomed.info/sct)category << 123456",
            "Filter by is-a (subsumption) relationship"
        ),
        (
            "Property filter - descendant-of",
            "(http://snomed.info/sct)parent < 456789",
            "Filter by descendant-of relationship"
        ),
        (
            "Include ValueSet",
            "^http://example.org/fhir/ValueSet/example-vs",
            "Include another ValueSet by reference"
        ),
        (
            "Conjunction (AND)",
            "(http://snomed.info/sct)code1, (http://snomed.info/sct)code2, (http://snomed.info/sct)code3",
            "Multiple codes - creates multiple includes"
        ),
        (
            "Disjunction (OR)",
            "(http://snomed.info/sct)code1; (http://snomed.info/sct)code2; (http://snomed.info/sct)code3",
            "Alternative codes - creates single include with multiple concepts"
        ),
        (
            "Exclusion (NOT)",
            "(http://snomed.info/sct)* - (http://snomed.info/sct)inactive",
            "All codes except excluded ones"
        ),
        (
            "Complex expression",
            "(http://snomed.info/sct)status = \"active\", (http://snomed.info/sct)category << 123456",
            "Combination of filters and relationships"
        ),
        (
            "Of operation",
            "(http://snomed.info/sct)*.category",
            "Property of all codes in system"
        ),
    ];

    for (i, (title, vcl_expr, description)) in examples.iter().enumerate() {
        println!("Example {}: {}", i + 1, title);
        println!("Description: {description}");
        println!("VCL: {vcl_expr}");

        match translate_vcl_string_to_fhir(vcl_expr) {
            Ok(compose) => {
                println!("✅ Translation successful!");

                // Display the resulting FHIR structure
                let json = serde_json::to_string_pretty(&compose).unwrap();
                println!("FHIR ValueSet.compose:");
                println!("{json}");

                // Provide analysis
                analyze_compose(&compose);
            }
            Err(e) => {
                println!("❌ Translation failed: {e}");
            }
        }

        println!("{}", "=".repeat(60));
        println!();
    }

    // Demonstrate custom translator settings
    println!("Custom Translator Configuration Examples");
    println!("=======================================\n");

    let snomed_translator =
        VclTranslator::with_default_system("http://snomed.info/sct".to_string());
    let loinc_translator = VclTranslator::with_default_system("http://loinc.org".to_string());

    // Test with default SNOMED CT system
    let vcl_without_system = "123456, 789012";
    println!("VCL without system URI: {vcl_without_system}");

    match snomed_translator.translate(&rh_vcl::parse_vcl(vcl_without_system)?) {
        Ok(compose) => {
            println!("✅ SNOMED CT translation:");
            let json = serde_json::to_string_pretty(&compose)?;
            println!("{json}");
        }
        Err(e) => println!("❌ SNOMED CT translation failed: {e}"),
    }

    println!();

    // Test with default LOINC system
    match loinc_translator.translate(&rh_vcl::parse_vcl(vcl_without_system)?) {
        Ok(compose) => {
            println!("✅ LOINC translation:");
            let json = serde_json::to_string_pretty(&compose)?;
            println!("{json}");
        }
        Err(e) => println!("❌ LOINC translation failed: {e}"),
    }

    println!("\n🎉 All translation examples completed!");

    Ok(())
}

/// Analyze and display information about a FHIR ValueSet compose structure
fn analyze_compose(compose: &rh_vcl::ValueSetCompose) {
    if compose.is_empty() {
        println!("📋 Compose is empty");
        return;
    }

    if !compose.include.is_empty() {
        println!("📋 Analysis:");
        println!("   - Include entries: {}", compose.include.len());

        let total_concepts: usize = compose.include.iter().map(|inc| inc.concept.len()).sum();
        if total_concepts > 0 {
            println!("   - Total concepts: {total_concepts}");
        }

        let total_filters: usize = compose.include.iter().map(|inc| inc.filter.len()).sum();
        if total_filters > 0 {
            println!("   - Total filters: {total_filters}");
        }

        let total_valuesets: usize = compose.include.iter().map(|inc| inc.value_set.len()).sum();
        if total_valuesets > 0 {
            println!("   - ValueSet references: {total_valuesets}");
        }

        // List unique systems
        let systems: Vec<_> = compose
            .include
            .iter()
            .filter_map(|inc| inc.system.as_ref())
            .collect();
        if !systems.is_empty() {
            println!("   - Code systems: {}", systems.len());
            for system in systems.iter().take(3) {
                println!("     • {system}");
            }
            if systems.len() > 3 {
                println!("     • ... and {} more", systems.len() - 3);
            }
        }
    }

    if !compose.exclude.is_empty() {
        println!("   - Exclude entries: {}", compose.exclude.len());
        let excluded_concepts: usize = compose.exclude.iter().map(|exc| exc.concept.len()).sum();
        if excluded_concepts > 0 {
            println!("   - Excluded concepts: {excluded_concepts}");
        }
    }
}
