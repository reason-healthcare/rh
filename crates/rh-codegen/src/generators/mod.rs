//! Code generation modules for FHIR types
//!
//! This module contains specialized generators for different aspects of FHIR code generation:
//! - `struct_generator`: Generate Rust structs from FHIR StructureDefinitions
//! - `trait_generator`: Generate Rust traits for FHIR resources
//! - `enum_generator`: Generate Rust enums from FHIR value sets
//! - `primitive_generator`: Generate Rust primitives for FHIR primitive types
//! - `field_generator`: Generate individual struct fields
//! - `documentation_generator`: Generate documentation for generated types
//! - `nested_struct_generator`: Generate nested structs within FHIR structures
//! - `file_generator`: Handle file generation and organization
//! - `file_io_manager`: Centralized file I/O operations and directory management
//! - `import_manager`: Manage imports and dependencies
//! - `type_utilities`: Utility functions for type classification and helpers

pub mod accessor_trait_generator;
pub mod crate_generator;
pub mod documentation_generator;
pub mod enum_generator;
pub mod existence_trait_generator;
pub mod field_generator;
pub mod file_generator;
pub mod file_io_manager;
pub mod import_manager;
// #[cfg(test)]
// pub mod import_manager_integration_test;
pub mod metadata_generator;
pub mod mutator_trait_generator;
pub mod naming_manager;
pub mod nested_struct_generator;
pub mod primitive_generator;
pub mod struct_generator;
pub mod token_generator;
pub mod trait_generator;
pub mod trait_impl_generator;
pub mod type_registry;
pub mod type_utilities;
pub mod utils;

// Re-export key functionality
pub use crate_generator::{
    generate_crate_structure, parse_package_metadata, CrateGenerationParams,
};
pub use documentation_generator::DocumentationGenerator;
pub use enum_generator::EnumGenerator;
pub use existence_trait_generator::ExistenceTraitGenerator;
pub use field_generator::FieldGenerator;
pub use file_generator::FileGenerator;
pub use file_io_manager::FileIoManager;
pub use import_manager::ImportManager;
pub use metadata_generator::{build_metadata_registry, generate_metadata_code};
pub use mutator_trait_generator::MutatorTraitGenerator;
pub use naming_manager::NamingManager;
pub use nested_struct_generator::NestedStructGenerator;
pub use primitive_generator::PrimitiveGenerator;
pub use struct_generator::StructGenerator;
pub use token_generator::TokenGenerator;
pub use trait_generator::TraitGenerator;
pub use trait_impl_generator::TraitImplGenerator;
pub use type_registry::TypeRegistry;
pub use type_utilities::TypeUtilities;
pub use utils::GeneratorUtils;
