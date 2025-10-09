use rh_ffq::{parse_start, translate_to_fhir};

fn main() {
    let samples = &[
        r#"http://snomed.info/sct: < 22298006"#,
        r#"http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003"#,
        r#"http://loinc.org: component = "Glucose" & method in ("Automated count","Manual count")"#,
        r#"
        @alias sct = http://snomed.info/sct|20250131
        @alias dm  = vs(https://example.org/fhir/ValueSet/diabetes)
        sct: << 73211009 | sct: in #dm - sct: << 44054006
        "#,
    ];

    for (i, s) in samples.iter().enumerate() {
        println!("\n=== SAMPLE {} ===\n{}", i + 1, s.trim());
        match parse_start(s) {
            Ok((rest, ast)) => {
                println!("\nAST:\n{ast:#?}");
                if !rest.trim().is_empty() {
                    println!("\n[WARN] Unparsed tail: {rest:?}");
                }

                // Translate to FHIR ValueSet.compose
                let compose = translate_to_fhir(&ast);
                println!("\nFHIR ValueSet.compose:");
                match serde_json::to_string_pretty(&compose) {
                    Ok(json) => println!("{json}"),
                    Err(e) => eprintln!("JSON serialization error: {e}"),
                }
            }
            Err(e) => {
                eprintln!("Parse error: {e:?}");
            }
        }
    }
}
