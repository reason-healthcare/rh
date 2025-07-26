/// FHIR Code Generation - Package Management Example
/// 
/// This example demonstrates how to download and install FHIR packages
/// from npm-style registries and generate Rust types.

use codegen::{PackageDownloader, PackageDownloadConfig, CodeGenerator, CodegenConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Download a FHIR package
    println!("📦 Setting up FHIR package download...");
    
    let download_config = PackageDownloadConfig {
        registry_url: "https://packages.fhir.org".to_string(),
        auth_token: None,
        timeout_seconds: 30,
    };
    
    let _downloader = PackageDownloader::new(download_config)?;
    
    // Note: In a real application, you would download actual packages
    // For this example, we'll show the configuration
    println!("✅ Package downloader configured for registry: https://packages.fhir.org");

    // Example 2: Set up code generation configuration
    println!("🔧 Setting up code generation...");
    
    let codegen_config = CodegenConfig {
        output_dir: "generated/r4_core".to_string(),
        module_name: "fhir_types".to_string(),
        with_serde: true,
        with_docs: true,
        type_mappings: HashMap::new(),
    };
    
    let _generator = CodeGenerator::new(codegen_config);
    
    println!("✅ Code generator configured!");
    println!("📁 Output directory: generated/r4_core/");

    // Example 3: Working with custom registries
    println!("🔐 Example: Custom registry configuration");
    
    let custom_config = PackageDownloadConfig {
        registry_url: "https://my-custom-fhir-registry.com".to_string(),
        auth_token: Some("your-auth-token".to_string()),
        timeout_seconds: 60,
    };
    
    println!("Configuration for custom registry:");
    println!("  Registry URL: {}", custom_config.registry_url);
    println!("  Timeout: {} seconds", custom_config.timeout_seconds);
    println!("  Auth: {}", if custom_config.auth_token.is_some() { "Configured" } else { "None" });
    
    println!("\n💡 Use environment variables for sensitive tokens:");
    println!("export FHIR_AUTH_TOKEN=your-token");
    
    // Example 4: Demonstrate actual download (commented out to avoid network calls)
    /*
    println!("📥 Downloading FHIR R4 core package...");
    let extract_path = Path::new("./packages");
    downloader.download_package(
        "hl7.fhir.r4.core",
        "4.0.1", 
        extract_path
    ).await?;
    println!("✅ Package downloaded to: {}", extract_path.display());
    */

    Ok(())
}
