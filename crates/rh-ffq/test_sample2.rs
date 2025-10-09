use std::process;

fn main() {
    let input = "http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003";
    
    // Test URI parsing
    match parse_uri_for_system(input) {
        Ok((rest, uri)) => println!("URI: '{}', Rest: '{}'", uri, rest),
        Err(e) => println!("URI parse error: {:?}", e),
    }
    
    // Test system ref parsing  
    match parse_system_ref(input) {
        Ok((rest, sys)) => println!("System: '{:?}', Rest: '{}'", sys, rest),
        Err(e) => println!("System parse error: {:?}", e),
    }
    
    // Test clause parsing
    match parse_clause(input) {
        Ok((rest, clause)) => println!("Clause: '{:?}', Rest: '{}'", clause, rest),
        Err(e) => println!("Clause parse error: {:?}", e),
    }
}
