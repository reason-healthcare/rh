use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir);

    antlr_rust::process_grammar(
        "fhirpath.g4",
        destination,
        antlr_rust::ProcessorBuilder::new()
            .visitor(true)
            .listener(true)
            .build(),
    )
    .expect("Failed to process ANTLR grammar");

    println!("cargo:rerun-if-changed=fhirpath.g4");
}
