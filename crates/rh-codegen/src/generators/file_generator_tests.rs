#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CodegenConfig;
    use crate::generators::token_generator::TokenGenerator;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_macros_file() {
        let temp_dir = TempDir::new().unwrap();
        let macros_path = temp_dir.path().join("macros.rs");
        
        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new(&config);
        let file_generator = FileGenerator::new(&config, &token_generator);
        
        file_generator.generate_macros_file(&macros_path).unwrap();
        
        assert!(macros_path.exists());
        let content = fs::read_to_string(&macros_path).unwrap();
        
        // Check that the file contains our macro definitions
        assert!(content.contains("macro_rules! primitive_string"));
        assert!(content.contains("macro_rules! primitive_boolean"));
        assert!(content.contains("primitive_id!"));
    }

    #[test]
    fn test_generate_lib_file() {
        let temp_dir = TempDir::new().unwrap();
        let lib_path = temp_dir.path().join("lib.rs");
        
        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new(&config);
        let file_generator = FileGenerator::new(&config, &token_generator);
        
        file_generator.generate_lib_file(&lib_path).unwrap();
        
        assert!(lib_path.exists());
        let content = fs::read_to_string(&lib_path).unwrap();
        
        // Check that the file contains module declarations
        assert!(content.contains("pub mod macros;"));
        assert!(content.contains("pub mod primitives;"));
        assert!(content.contains("pub mod datatypes;"));
        assert!(content.contains("pub mod resource;"));
        assert!(content.contains("pub mod traits;"));
        
        // Check re-exports
        assert!(content.contains("pub use primitives::*;"));
        assert!(content.contains("pub use macros::*;"));
    }

    #[test]
    fn test_generate_complete_crate() {
        let temp_dir = TempDir::new().unwrap();
        let crate_path = temp_dir.path().join("test-crate");
        
        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new(&config);
        let file_generator = FileGenerator::new(&config, &token_generator);
        
        file_generator.generate_complete_crate(
            &crate_path,
            "test-crate",
            &[], // Empty structure definitions
        ).unwrap();
        
        // Check that all required files and directories exist
        assert!(crate_path.join("Cargo.toml").exists());
        assert!(crate_path.join("src").is_dir());
        assert!(crate_path.join("src/lib.rs").exists());
        assert!(crate_path.join("src/macros.rs").exists());
        assert!(crate_path.join("src/primitives").is_dir());
        assert!(crate_path.join("src/primitives/mod.rs").exists());
        assert!(crate_path.join("src/datatypes").is_dir());
        assert!(crate_path.join("src/datatypes/mod.rs").exists());
        assert!(crate_path.join("src/resource").is_dir());
        assert!(crate_path.join("src/resource/mod.rs").exists());
        assert!(crate_path.join("src/traits").is_dir());
        assert!(crate_path.join("src/traits/mod.rs").exists());
        
        // Check Cargo.toml content
        let cargo_content = fs::read_to_string(crate_path.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("name = \"test-crate\""));
        assert!(cargo_content.contains("edition = \"2021\""));
        assert!(cargo_content.contains("serde"));
        assert!(cargo_content.contains("paste"));
    }
}
