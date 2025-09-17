use anyhow::Result;
use rh_codegen::{CodeGenerator, CodegenConfig};
use std::path::Path;

fn main() -> Result<()> {
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    let output_dir = Path::new("./test_array_fix");
    let src_dir = output_dir.join("src");

    // Clean up any existing output
    if output_dir.exists() {
        std::fs::remove_dir_all(output_dir)?;
    }
    std::fs::create_dir_all(&src_dir)?;

    println!("Testing array handling fix...");

    // Generate a simple resource file to test array handling
    generator.generate_crate(output_dir, "test_array_fix")?;

    println!("✅ Generated test files for array handling verification");
    println!("📁 Files generated in: ./test_array_fix/");

    Ok(())
}
