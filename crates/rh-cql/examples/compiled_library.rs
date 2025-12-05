//! Example: Using CompiledLibrary
//!
//! This example demonstrates how to use CompiledLibrary to inspect
//! and query ELM library definitions.
//!
//! Run with: `cargo run --example compiled_library`

use rh_cql::elm::{
    AccessModifier, CodeDef, CodeDefRef, CodeDefs, CodeSystemDef, CodeSystemDefRef, CodeSystemDefs,
    ConceptDef, ConceptDefs, ContextDef, ContextDefs, ExpressionDef, ExpressionDefs, IncludeDef,
    IncludeDefs, Library, ParameterDef, ParameterDefs, UsingDef, UsingDefs, ValueSetDef,
    ValueSetDefs, VersionedIdentifier,
};
use rh_cql::library::{CompiledLibrary, DefinitionRef};

fn main() -> anyhow::Result<()> {
    println!("=== CompiledLibrary Example ===\n");

    // Create a sample ELM library representing a CQL quality measure
    let library = create_sample_library();
    let compiled = CompiledLibrary::with_source_location(library, "/cql/SampleMeasure.cql");

    // Example 1: Library metadata
    println!("1. Library Metadata:");
    println!("   Name: {:?}", compiled.name());
    println!("   Version: {:?}", compiled.version());
    println!("   Source: {:?}", compiled.source_location());

    let lib_id = compiled.to_library_identifier();
    println!("   As LibraryIdentifier: {lib_id}");

    // Example 2: Using declarations (data models)
    println!("\n2. Using Declarations:");
    for using in compiled.usings() {
        println!(
            "   using {} version '{}'",
            using.local_identifier.as_deref().unwrap_or("?"),
            using.version.as_deref().unwrap_or("?")
        );
    }

    // Look up specific using
    if let Some(fhir) = compiled.get_using("FHIR") {
        println!("   FHIR URI: {:?}", fhir.uri);
    }

    // Example 3: Include declarations (library dependencies)
    println!("\n3. Include Declarations:");
    for inc in compiled.includes() {
        println!(
            "   include {} version '{}' called {}",
            inc.path.as_deref().unwrap_or("?"),
            inc.version.as_deref().unwrap_or("?"),
            inc.local_identifier.as_deref().unwrap_or("(same)")
        );
    }

    // Get all included library identifiers
    let deps = compiled.include_identifiers();
    println!("   Dependencies: {deps:?}");

    // Example 4: Parameters
    println!("\n4. Parameters:");
    for param in compiled.parameters() {
        let access = if param.access_level == Some(AccessModifier::Private) {
            "private"
        } else {
            "public"
        };
        println!(
            "   {access} parameter {}",
            param.name.as_deref().unwrap_or("?")
        );
    }

    // Example 5: Terminology definitions
    println!("\n5. Terminology:");

    println!("   Code Systems:");
    for cs in compiled.code_systems() {
        println!(
            "      codesystem {}: '{}'",
            cs.name.as_deref().unwrap_or("?"),
            cs.id.as_deref().unwrap_or("?")
        );
    }

    println!("   Value Sets:");
    for vs in compiled.value_sets() {
        println!(
            "      valueset {}: '{}'",
            vs.name.as_deref().unwrap_or("?"),
            vs.id.as_deref().unwrap_or("?")
        );
    }

    println!("   Codes:");
    for code in compiled.codes() {
        println!(
            "      code {}: '{}' from {}",
            code.name.as_deref().unwrap_or("?"),
            code.id.as_deref().unwrap_or("?"),
            code.code_system
                .as_ref()
                .and_then(|cs| cs.name.as_deref())
                .unwrap_or("?")
        );
    }

    println!("   Concepts:");
    for concept in compiled.concepts() {
        let codes: Vec<_> = concept
            .code
            .iter()
            .filter_map(|c| c.name.as_deref())
            .collect();
        println!(
            "      concept {}: [{}]",
            concept.name.as_deref().unwrap_or("?"),
            codes.join(", ")
        );
    }

    // Example 6: Contexts
    println!("\n6. Contexts:");
    for ctx in compiled.contexts() {
        println!("   context {}", ctx.name.as_deref().unwrap_or("?"));
    }

    // Example 7: Expression definitions
    println!("\n7. Expression Definitions:");
    for expr in compiled.expressions() {
        let access = if expr.access_level == Some(AccessModifier::Private) {
            "private"
        } else {
            "public"
        };
        println!(
            "   {access} define {} (context: {})",
            expr.name.as_deref().unwrap_or("?"),
            expr.context.as_deref().unwrap_or("Unfiltered")
        );
    }

    // Example 8: Filtering expressions
    println!("\n8. Filtering:");

    let public = compiled.public_expressions();
    println!("   Public expressions: {}", public.len());

    let patient_exprs = compiled.expressions_for_context("Patient");
    println!("   Patient context expressions: {}", patient_exprs.len());

    // Example 9: Generic definition lookup
    println!("\n9. Definition Lookup:");

    let names_to_check = [
        "MeasurementPeriod",
        "InInitialPopulation",
        "LOINC",
        "DiabetesConditions",
        "NonExistent",
    ];

    for name in names_to_check {
        match compiled.get_definition(name) {
            Some(def) => {
                let kind = match def {
                    DefinitionRef::Expression(_) => "Expression",
                    DefinitionRef::Parameter(_) => "Parameter",
                    DefinitionRef::CodeSystem(_) => "CodeSystem",
                    DefinitionRef::ValueSet(_) => "ValueSet",
                    DefinitionRef::Code(_) => "Code",
                    DefinitionRef::Concept(_) => "Concept",
                    DefinitionRef::Context(_) => "Context",
                };
                let public = if def.is_public() { "public" } else { "private" };
                println!("   '{name}' -> {kind} ({public})");
            }
            None => println!("   '{name}' -> NOT FOUND"),
        }
    }

    // Example 10: All definition names
    println!("\n10. All Definition Names:");
    let all_names = compiled.definition_names();
    println!("   Total: {} definitions", all_names.len());
    for name in &all_names {
        println!("      - {name}");
    }

    // Example 11: From trait
    println!("\n11. Creating from Library:");
    let lib2 = Library {
        identifier: Some(VersionedIdentifier {
            id: Some("QuickExample".to_string()),
            version: Some("1.0".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let compiled2: CompiledLibrary = lib2.into();
    println!("   Created: {:?}", compiled2.name());

    println!("\n=== Example Complete ===");
    Ok(())
}

/// Create a sample ELM library for demonstration
fn create_sample_library() -> Library {
    Library {
        identifier: Some(VersionedIdentifier {
            id: Some("SampleMeasure".to_string()),
            version: Some("1.0.0".to_string()),
            system: Some("http://example.org/measures".to_string()),
        }),
        usings: Some(UsingDefs {
            defs: vec![
                UsingDef {
                    local_identifier: Some("System".to_string()),
                    uri: Some("urn:hl7-org:elm-types:r1".to_string()),
                    version: None,
                },
                UsingDef {
                    local_identifier: Some("FHIR".to_string()),
                    uri: Some("http://hl7.org/fhir".to_string()),
                    version: Some("4.0.1".to_string()),
                },
            ],
        }),
        includes: Some(IncludeDefs {
            defs: vec![
                IncludeDef {
                    local_identifier: Some("FHIRHelpers".to_string()),
                    path: Some("FHIRHelpers".to_string()),
                    version: Some("4.0.1".to_string()),
                },
                IncludeDef {
                    local_identifier: Some("Common".to_string()),
                    path: Some("MATGlobalCommonFunctions".to_string()),
                    version: Some("6.0.0".to_string()),
                },
            ],
        }),
        parameters: Some(ParameterDefs {
            defs: vec![
                ParameterDef {
                    name: Some("MeasurementPeriod".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                },
                ParameterDef {
                    name: Some("InternalThreshold".to_string()),
                    access_level: Some(AccessModifier::Private),
                    ..Default::default()
                },
            ],
        }),
        code_systems: Some(CodeSystemDefs {
            defs: vec![
                CodeSystemDef {
                    name: Some("LOINC".to_string()),
                    id: Some("http://loinc.org".to_string()),
                    version: Some("2.73".to_string()),
                    access_level: Some(AccessModifier::Public),
                },
                CodeSystemDef {
                    name: Some("SNOMEDCT".to_string()),
                    id: Some("http://snomed.info/sct".to_string()),
                    version: None,
                    access_level: Some(AccessModifier::Public),
                },
            ],
        }),
        value_sets: Some(ValueSetDefs {
            defs: vec![ValueSetDef {
                name: Some("DiabetesConditions".to_string()),
                id: Some(
                    "http://cts.nlm.nih.gov/fhir/ValueSet/2.16.840.1.113883.3.464.1003.103.12.1001"
                        .to_string(),
                ),
                version: None,
                access_level: Some(AccessModifier::Public),
                ..Default::default()
            }],
        }),
        codes: Some(CodeDefs {
            defs: vec![CodeDef {
                name: Some("HbA1c".to_string()),
                id: Some("4548-4".to_string()),
                display: Some("Hemoglobin A1c/Hemoglobin.total in Blood".to_string()),
                access_level: Some(AccessModifier::Public),
                code_system: Some(CodeSystemDefRef {
                    name: Some("LOINC".to_string()),
                    ..Default::default()
                }),
            }],
        }),
        concepts: Some(ConceptDefs {
            defs: vec![ConceptDef {
                name: Some("DiabetesMarkers".to_string()),
                display: Some("Diabetes laboratory markers".to_string()),
                access_level: Some(AccessModifier::Public),
                code: vec![CodeDefRef {
                    name: Some("HbA1c".to_string()),
                    library_name: None,
                }],
            }],
        }),
        contexts: Some(ContextDefs {
            defs: vec![ContextDef {
                name: Some("Patient".to_string()),
            }],
        }),
        statements: Some(ExpressionDefs {
            defs: vec![
                ExpressionDef {
                    name: Some("InInitialPopulation".to_string()),
                    context: Some("Patient".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                },
                ExpressionDef {
                    name: Some("InDenominator".to_string()),
                    context: Some("Patient".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                },
                ExpressionDef {
                    name: Some("InNumerator".to_string()),
                    context: Some("Patient".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                },
                ExpressionDef {
                    name: Some("HelperFunction".to_string()),
                    context: Some("Patient".to_string()),
                    access_level: Some(AccessModifier::Private),
                    ..Default::default()
                },
            ],
        }),
        ..Default::default()
    }
}
