use rh_codegen::generators::type_registry::TypeRegistry;

fn main() {
    println!("Testing AccountStatus classification:");
    
    // Test the enum types that should go to bindings
    let enum_types = vec![
        "AccountStatus",
        "PublicationStatus", 
        "AdministrativeGender",
        "ContactPointSystem",
    ];
    
    for type_name in enum_types {
        println!("\n=== {} ===", type_name);
        
        // Check if it's registered in TypeRegistry
        if let Some(classification) = TypeRegistry::get_classification(type_name) {
            println!("TypeRegistry classification: {:?}", classification);
        } else {
            println!("Not found in TypeRegistry - will use fallback logic");
        }
        
        // Get the actual import path
        let import_path = TypeRegistry::get_import_path_for_type(type_name);
        println!("Import path: {}", import_path);
        
        // Check if it's correctly routed to bindings
        if import_path.contains("crate::bindings::") {
            println!("✅ Correctly routed to bindings");
        } else if import_path.contains("crate::resources::") {
            println!("❌ Incorrectly routed to resources");
        } else {
            println!("⚠️  Unexpected routing");
        }
    }
}