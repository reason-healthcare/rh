use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cql-file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)?;

    let result = rh_cql::compile(&source, None)?;

    if result.is_success() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
