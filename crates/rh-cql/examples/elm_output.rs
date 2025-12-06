//! Example demonstrating ELM output generation.
//!
//! Shows how to serialize ELM libraries to JSON with various options.

use rh_cql::elm::{
    Expression, ExpressionDef, ExpressionDefs, Library, Literal, UsingDef, UsingDefs,
    VersionedIdentifier,
};
use rh_cql::options::{CompilerOption, CompilerOptions, SignatureLevel};
use rh_cql::output::{library_to_json, ElmWriter, TRANSLATOR_VERSION};

fn main() {
    println!("=== ELM Output Generation Example ===\n");
    println!("Translator Version: {TRANSLATOR_VERSION}\n");

    // Create a sample ELM library
    let library = create_sample_library();

    // 1. Output with default options (annotations enabled)
    println!("1. JSON Output with Default Options:");
    println!("   (Includes translator info and schema identifier)");
    let json = library_to_json(&library).unwrap();
    println!("{json}\n");

    // 2. Output with no annotations
    println!("2. JSON Output with No Annotations:");
    let options = CompilerOptions::new();
    let writer = ElmWriter::new(&options);
    let json = writer.to_json(&library).unwrap();
    println!("{json}\n");

    // 3. Compact output (no pretty printing)
    println!("3. Compact JSON Output:");
    let options = CompilerOptions::new();
    let writer = ElmWriter::new(&options).with_pretty_print(false);
    let json = writer.to_json(&library).unwrap();
    println!("{json}\n");

    // 4. Output with all annotation options
    println!("4. JSON Output with All Annotation Options:");
    let options = CompilerOptions::debug()
        .with_option(CompilerOption::DisableListDemotion)
        .with_option(CompilerOption::DisableListPromotion)
        .with_signature_level(SignatureLevel::All);
    let writer = ElmWriter::new(&options);
    let json = writer.to_json(&library).unwrap();
    println!("{json}\n");

    // 5. Write to a buffer
    println!("5. Write JSON to Buffer:");
    let options = CompilerOptions::default();
    let writer = ElmWriter::new(&options);
    let mut buffer = Vec::new();
    writer.write_json(&library, &mut buffer).unwrap();
    println!("   Buffer size: {} bytes", buffer.len());
    println!();

    // 6. Get bytes directly
    println!("6. Get JSON as Bytes:");
    let bytes = writer.to_json_bytes(&library).unwrap();
    println!("   Byte array size: {} bytes", bytes.len());
    println!();

    println!("=== Example Complete ===");
}

fn create_sample_library() -> Library {
    Library {
        identifier: Some(VersionedIdentifier {
            id: Some("ExampleLibrary".to_string()),
            system: None,
            version: Some("1.0.0".to_string()),
        }),
        usings: Some(UsingDefs {
            defs: vec![UsingDef {
                local_identifier: Some("FHIR".to_string()),
                uri: Some("http://hl7.org/fhir".to_string()),
                version: Some("4.0.1".to_string()),
            }],
        }),
        statements: Some(ExpressionDefs {
            defs: vec![
                ExpressionDef {
                    name: Some("IsAdult".to_string()),
                    context: Some("Patient".to_string()),
                    expression: Some(Box::new(Expression::Literal(Literal {
                        value: Some("true".to_string()),
                        value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                },
                ExpressionDef {
                    name: Some("Age".to_string()),
                    context: Some("Patient".to_string()),
                    expression: Some(Box::new(Expression::Literal(Literal {
                        value: Some("42".to_string()),
                        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                },
            ],
        }),
        ..Default::default()
    }
}
