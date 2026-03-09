//! Library source providers and compiled library management.
//!
//! This module provides infrastructure for resolving CQL library dependencies:
//!
//! - [`LibrarySourceProvider`] trait for locating and loading CQL source code
//! - [`MemoryLibrarySourceProvider`] for in-memory storage (WASM-compatible)
//! - [`FileLibrarySourceProvider`] for filesystem-based loading (non-WASM)
//! - [`CompiledLibrary`] for efficient, indexed access to ELM library definitions
//! - [`LibraryManager`] for caching, dependency resolution, and cycle detection
//!
//! # Example
//!
//! ```
//! use rh_cql::library::{LibrarySourceProvider, MemoryLibrarySourceProvider, LibraryIdentifier};
//!
//! // Create a memory provider
//! let provider = MemoryLibrarySourceProvider::new();
//!
//! // Register a library
//! let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
//! provider.register_source(id.clone(), r#"
//!     library FHIRHelpers version '4.0.1'
//!     using FHIR version '4.0.1'
//! "#.to_string());
//!
//! // Look up the library
//! let source = provider.get_source(&id);
//! assert!(source.is_some());
//! ```

pub mod compiled;
pub mod identifiers;
pub mod manager;
pub mod providers;
pub mod sources;

pub use compiled::{CompiledLibrary, DefinitionRef, FunctionRef};
pub use identifiers::LibraryIdentifier;
pub use manager::{LibraryError, LibraryManager, LibraryResult};
pub use providers::{
    CompositeLibrarySourceProvider, FileLibrarySourceProvider, LibrarySourceProvider,
    MemoryLibrarySourceProvider,
};
pub use sources::LibrarySource;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::elm::{
        AccessModifier, CodeDef, CodeDefs, CodeSystemDef, CodeSystemDefs, ConceptDef, ConceptDefs,
        ExpressionDef, ExpressionDefs, FunctionDef, IncludeDef, IncludeDefs, Library, OperandDef,
        ParameterDef, ParameterDefs, StatementDef, UsingDef, UsingDefs, ValueSetDef, ValueSetDefs,
        VersionedIdentifier,
    };

    // ===========================================
    // LibraryIdentifier tests
    // ===========================================

    #[test]
    fn test_library_identifier_new() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(id.name, "FHIRHelpers");
        assert_eq!(id.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_library_identifier_unversioned() {
        let id = LibraryIdentifier::unversioned("Common");
        assert_eq!(id.name, "Common");
        assert_eq!(id.version, None);
    }

    #[test]
    fn test_library_identifier_key() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(id.to_key(), "FHIRHelpers|4.0.1");
        assert_eq!(LibraryIdentifier::from_key("FHIRHelpers|4.0.1"), id);

        let id2 = LibraryIdentifier::unversioned("Common");
        assert_eq!(id2.to_key(), "Common");

        assert_eq!(LibraryIdentifier::from_key("Common"), id2);
    }

    #[test]
    fn test_library_identifier_key_with_special_chars() {
        // Version can have dots, dashes, etc.
        let id = LibraryIdentifier::new("MyLib", Some("1.2.3-beta.1"));
        assert_eq!(id.to_key(), "MyLib|1.2.3-beta.1");
        let parsed = LibraryIdentifier::from_key("MyLib|1.2.3-beta.1");
        assert_eq!(parsed.name, "MyLib");
        assert_eq!(parsed.version, Some("1.2.3-beta.1".to_string()));
    }

    #[test]
    fn test_library_identifier_display() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(format!("{id}"), "FHIRHelpers version '4.0.1'");

        let id2 = LibraryIdentifier::unversioned("Common");
        assert_eq!(format!("{id2}"), "Common");
    }

    #[test]
    fn test_library_identifier_matches() {
        let versioned = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let unversioned = LibraryIdentifier::unversioned("FHIRHelpers");
        let different_version = LibraryIdentifier::new("FHIRHelpers", Some("4.0.0"));
        let different_name = LibraryIdentifier::new("Common", Some("4.0.1"));

        // Unversioned matches any version of same library
        assert!(unversioned.matches(&versioned));
        assert!(unversioned.matches(&different_version));
        assert!(!unversioned.matches(&different_name));

        // Versioned only matches exact version
        assert!(versioned.matches(&versioned));
        assert!(!versioned.matches(&different_version));
        assert!(!versioned.matches(&different_name));

        // Versioned doesn't match unversioned candidate
        assert!(!versioned.matches(&unversioned));
    }

    #[test]
    fn test_library_identifier_equality() {
        let id1 = LibraryIdentifier::new("Test", Some("1.0"));
        let id2 = LibraryIdentifier::new("Test", Some("1.0"));
        let id3 = LibraryIdentifier::new("Test", Some("2.0"));

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_library_identifier_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(LibraryIdentifier::new("A", Some("1.0")));
        set.insert(LibraryIdentifier::new("B", Some("1.0")));
        set.insert(LibraryIdentifier::new("A", Some("1.0"))); // duplicate

        assert_eq!(set.len(), 2);
    }

    // ===========================================
    // LibrarySource tests
    // ===========================================

    #[test]
    fn test_library_source_new() {
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        let source = LibrarySource::new(id.clone(), "library Test", None::<String>);

        assert_eq!(source.identifier, id);
        assert_eq!(source.source, "library Test");
        assert!(source.location.is_none());
    }

    #[test]
    fn test_library_source_with_location() {
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        let source = LibrarySource::new(id, "library Test", Some("/path/to/file.cql"));

        assert_eq!(source.location, Some("/path/to/file.cql".to_string()));
    }

    // ===========================================
    // MemoryLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_memory_provider_new() {
        let provider = MemoryLibrarySourceProvider::new();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_default() {
        let provider = MemoryLibrarySourceProvider::default();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_with_max_libraries() {
        let provider = MemoryLibrarySourceProvider::with_max_libraries(10);
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_with_stats() {
        let provider = MemoryLibrarySourceProvider::with_stats();
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        provider.register_source(id.clone(), "test".to_string());

        let _ = provider.get_source(&id);
        let stats = provider.stats();
        assert!(stats.hits > 0 || stats.insertions > 0);
    }

    #[test]
    fn test_memory_provider_register_and_get() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id.clone(), "library TestLib version '1.0'".to_string());

        assert!(provider.has_library(&id));
        assert_eq!(provider.library_count(), 1);

        let source = provider.get_source(&id).unwrap();
        assert_eq!(source.source, "library TestLib version '1.0'");
        assert!(source.location.is_none());
    }

    #[test]
    fn test_memory_provider_get_unversioned() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id, "library TestLib version '1.0'".to_string());

        // Look up without version
        let unversioned_id = LibraryIdentifier::unversioned("TestLib");
        let source = provider.get_source(&unversioned_id).unwrap();
        assert_eq!(source.source, "library TestLib version '1.0'");
    }

    #[test]
    fn test_memory_provider_get_nonexistent() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("NotFound", Some("1.0"));
        assert!(provider.get_source(&id).is_none());
        assert!(!provider.has_library(&id));
    }

    #[test]
    fn test_memory_provider_with_location() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source_with_location(
            id.clone(),
            "library TestLib version '1.0'".to_string(),
            "/path/to/TestLib-1.0.cql".to_string(),
        );

        let source = provider.get_source(&id).unwrap();
        assert_eq!(
            source.location,
            Some("/path/to/TestLib-1.0.cql".to_string())
        );
    }

    #[test]
    fn test_memory_provider_register_library_source() {
        let provider = MemoryLibrarySourceProvider::new();

        let source = LibrarySource::new(
            LibraryIdentifier::new("Direct", Some("1.0")),
            "library Direct",
            Some("/path/file.cql"),
        );
        provider.register(source);

        let id = LibraryIdentifier::new("Direct", Some("1.0"));
        let retrieved = provider.get_source(&id).unwrap();
        assert_eq!(retrieved.source, "library Direct");
        assert_eq!(retrieved.location, Some("/path/file.cql".to_string()));
    }

    #[test]
    fn test_memory_provider_list_libraries() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(
            LibraryIdentifier::new("LibA", Some("1.0")),
            "library LibA".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("LibB", Some("2.0")),
            "library LibB".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::unversioned("LibC"),
            "library LibC".to_string(),
        );

        let libraries = provider.list_libraries();
        assert_eq!(libraries.len(), 3);
    }

    #[test]
    fn test_memory_provider_find_by_name() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(
            LibraryIdentifier::new("Common", Some("1.0")),
            "library Common v1".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("Common", Some("2.0")),
            "library Common v2".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("Other", Some("1.0")),
            "library Other".to_string(),
        );

        let common_libs = provider.find_by_name("Common");
        assert_eq!(common_libs.len(), 2);
        assert!(common_libs.iter().all(|id| id.name == "Common"));

        let other_libs = provider.find_by_name("Other");
        assert_eq!(other_libs.len(), 1);

        let none_libs = provider.find_by_name("NotFound");
        assert_eq!(none_libs.len(), 0);
    }

    #[test]
    fn test_memory_provider_remove() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id.clone(), "library TestLib".to_string());
        assert!(provider.has_library(&id));

        let removed = provider.remove(&id);
        assert!(removed.is_some());
        assert!(!provider.has_library(&id));

        // Remove again returns None
        let removed_again = provider.remove(&id);
        assert!(removed_again.is_none());
    }

    #[test]
    fn test_memory_provider_clear() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(LibraryIdentifier::new("A", Some("1.0")), "a".to_string());
        provider.register_source(LibraryIdentifier::new("B", Some("1.0")), "b".to_string());
        assert_eq!(provider.library_count(), 2);

        provider.clear();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_overwrite() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("Test", Some("1.0"));
        provider.register_source(id.clone(), "original".to_string());
        provider.register_source(id.clone(), "updated".to_string());

        let source = provider.get_source(&id).unwrap();
        assert_eq!(source.source, "updated");
        assert_eq!(provider.library_count(), 1);
    }

    // ===========================================
    // FileLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_file_provider_new() {
        let provider = FileLibrarySourceProvider::new();
        assert!(provider.paths().is_empty());
    }

    #[test]
    fn test_file_provider_default() {
        let provider = FileLibrarySourceProvider::default();
        assert!(provider.paths().is_empty());
    }

    #[test]
    fn test_file_provider_with_path() {
        let provider = FileLibrarySourceProvider::new()
            .with_path("./cql")
            .with_path("./libs");

        assert_eq!(provider.paths().len(), 2);
    }

    #[test]
    fn test_file_provider_with_paths() {
        let provider = FileLibrarySourceProvider::new().with_paths(["./cql", "./libs", "./other"]);

        assert_eq!(provider.paths().len(), 3);
    }

    #[test]
    fn test_file_provider_possible_filenames() {
        let provider = FileLibrarySourceProvider::new();

        let versioned = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let filenames = provider.possible_filenames(&versioned);
        assert_eq!(filenames, vec!["FHIRHelpers-4.0.1.cql", "FHIRHelpers.cql"]);

        let unversioned = LibraryIdentifier::unversioned("Common");
        let filenames = provider.possible_filenames(&unversioned);
        assert_eq!(filenames, vec!["Common.cql"]);
    }

    #[test]
    fn test_file_provider_custom_extension() {
        let provider = FileLibrarySourceProvider::new().with_extension("txt");

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        let filenames = provider.possible_filenames(&id);
        assert_eq!(filenames, vec!["TestLib-1.0.txt", "TestLib.txt"]);
    }

    #[test]
    fn test_file_provider_nonexistent_path() {
        let provider = FileLibrarySourceProvider::new().with_path("/nonexistent/path");

        let id = LibraryIdentifier::new("Test", Some("1.0"));
        assert!(!provider.has_library(&id));
        assert!(provider.get_source(&id).is_none());
    }

    // ===========================================
    // CompositeLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_composite_provider_new() {
        let composite = CompositeLibrarySourceProvider::new();
        assert!(composite.list_libraries().is_empty());
    }

    #[test]
    fn test_composite_provider_default() {
        let composite = CompositeLibrarySourceProvider::default();
        assert!(composite.list_libraries().is_empty());
    }

    #[test]
    fn test_composite_provider_single() {
        let memory = MemoryLibrarySourceProvider::new();
        memory.register_source(
            LibraryIdentifier::new("MemoryLib", Some("1.0")),
            "from memory".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new().add_provider(memory);

        let id = LibraryIdentifier::new("MemoryLib", Some("1.0"));
        let source = composite.get_source(&id).unwrap();
        assert_eq!(source.source, "from memory");

        let libraries = composite.list_libraries();
        assert_eq!(libraries.len(), 1);
    }

    #[test]
    fn test_composite_provider_priority() {
        // First provider takes precedence
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from first".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from second".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        let id = LibraryIdentifier::new("Shared", Some("1.0"));
        let source = composite.get_source(&id).unwrap();
        assert_eq!(source.source, "from first");
    }

    #[test]
    fn test_composite_provider_fallback() {
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("OnlyInFirst", Some("1.0")),
            "from first".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("OnlyInSecond", Some("1.0")),
            "from second".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        // Found in first
        let id1 = LibraryIdentifier::new("OnlyInFirst", Some("1.0"));
        assert!(composite.has_library(&id1));

        // Found in second (fallback)
        let id2 = LibraryIdentifier::new("OnlyInSecond", Some("1.0"));
        assert!(composite.has_library(&id2));

        // Not found anywhere
        let id3 = LibraryIdentifier::new("NotFound", Some("1.0"));
        assert!(!composite.has_library(&id3));
    }

    #[test]
    fn test_composite_provider_list_deduplicates() {
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from first".to_string(),
        );
        memory1.register_source(
            LibraryIdentifier::new("OnlyFirst", Some("1.0")),
            "first only".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from second".to_string(),
        );
        memory2.register_source(
            LibraryIdentifier::new("OnlySecond", Some("1.0")),
            "second only".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        let libraries = composite.list_libraries();
        // Should have 3 unique: Shared, OnlyFirst, OnlySecond
        assert_eq!(libraries.len(), 3);
    }

    // ===========================================
    // CompiledLibrary tests
    // ===========================================

    fn create_test_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLibrary".to_string()),
                version: Some("1.0.0".to_string()),
                system: None,
            }),
            usings: Some(UsingDefs {
                defs: vec![UsingDef {
                    local_identifier: Some("FHIR".to_string()),
                    uri: Some("http://hl7.org/fhir".to_string()),
                    version: Some("4.0.1".to_string()),
                }],
            }),
            includes: Some(IncludeDefs {
                defs: vec![IncludeDef {
                    local_identifier: Some("FHIRHelpers".to_string()),
                    path: Some("FHIRHelpers".to_string()),
                    version: Some("4.0.1".to_string()),
                }],
            }),
            parameters: Some(ParameterDefs {
                defs: vec![ParameterDef {
                    name: Some("MeasurementPeriod".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                }],
            }),
            statements: Some(ExpressionDefs {
                defs: vec![
                    StatementDef::Expression(ExpressionDef {
                        name: Some("InPopulation".to_string()),
                        context: Some("Patient".to_string()),
                        access_level: Some(AccessModifier::Public),
                        ..Default::default()
                    }),
                    StatementDef::Expression(ExpressionDef {
                        name: Some("PrivateHelper".to_string()),
                        context: Some("Patient".to_string()),
                        access_level: Some(AccessModifier::Private),
                        ..Default::default()
                    }),
                    StatementDef::Expression(ExpressionDef {
                        name: Some("UnfilteredExpression".to_string()),
                        context: Some("Unfiltered".to_string()),
                        access_level: Some(AccessModifier::Public),
                        ..Default::default()
                    }),
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_compiled_library_new() {
        let library = Library::default();
        let compiled = CompiledLibrary::new(library);
        assert!(compiled.source_location().is_none());
    }

    #[test]
    fn test_compiled_library_with_source_location() {
        let library = Library::default();
        let compiled = CompiledLibrary::with_source_location(library, "/path/to/file.cql");
        assert_eq!(compiled.source_location(), Some("/path/to/file.cql"));
    }

    #[test]
    fn test_compiled_library_identifier() {
        let compiled = CompiledLibrary::new(create_test_library());
        assert_eq!(compiled.name(), Some("TestLibrary"));
        assert_eq!(compiled.version(), Some("1.0.0"));

        let lib_id = compiled.to_library_identifier();
        assert_eq!(lib_id.name, "TestLibrary");
        assert_eq!(lib_id.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_compiled_library_usings() {
        let compiled = CompiledLibrary::new(create_test_library());

        let usings = compiled.usings();
        assert_eq!(usings.len(), 1);

        let fhir = compiled.get_using("FHIR");
        assert!(fhir.is_some());
        assert_eq!(fhir.unwrap().uri, Some("http://hl7.org/fhir".to_string()));

        assert!(compiled.get_using("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_includes() {
        let compiled = CompiledLibrary::new(create_test_library());

        let includes = compiled.includes();
        assert_eq!(includes.len(), 1);

        let fhir_helpers = compiled.get_include("FHIRHelpers");
        assert!(fhir_helpers.is_some());

        let include_ids = compiled.include_identifiers();
        assert_eq!(include_ids.len(), 1);
        assert_eq!(include_ids[0].name, "FHIRHelpers");
        assert_eq!(include_ids[0].version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_compiled_library_parameters() {
        let compiled = CompiledLibrary::new(create_test_library());

        let params = compiled.parameters();
        assert_eq!(params.len(), 1);

        let mp = compiled.get_parameter("MeasurementPeriod");
        assert!(mp.is_some());

        assert!(compiled.get_parameter("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_expressions() {
        let compiled = CompiledLibrary::new(create_test_library());

        let exprs = compiled.expressions();
        assert_eq!(exprs.len(), 3);

        let in_pop = compiled.get_expression("InPopulation");
        assert!(in_pop.is_some());
        assert_eq!(in_pop.unwrap().context, Some("Patient".to_string()));

        assert!(compiled.get_expression("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_public_expressions() {
        let compiled = CompiledLibrary::new(create_test_library());

        let public = compiled.public_expressions();
        assert_eq!(public.len(), 2); // InPopulation and UnfilteredExpression
        assert!(public
            .iter()
            .any(|e| e.name.as_deref() == Some("InPopulation")));
        assert!(!public
            .iter()
            .any(|e| e.name.as_deref() == Some("PrivateHelper")));
    }

    #[test]
    fn test_compiled_library_expressions_for_context() {
        let compiled = CompiledLibrary::new(create_test_library());

        let patient_exprs = compiled.expressions_for_context("Patient");
        assert_eq!(patient_exprs.len(), 2);

        let unfiltered_exprs = compiled.expressions_for_context("Unfiltered");
        assert_eq!(unfiltered_exprs.len(), 1);

        let other_exprs = compiled.expressions_for_context("Other");
        assert_eq!(other_exprs.len(), 0);
    }

    #[test]
    fn test_compiled_library_get_definition() {
        let compiled = CompiledLibrary::new(create_test_library());

        // Expression
        let def = compiled.get_definition("InPopulation");
        assert!(matches!(def, Some(DefinitionRef::Expression(_))));

        // Parameter
        let def = compiled.get_definition("MeasurementPeriod");
        assert!(matches!(def, Some(DefinitionRef::Parameter(_))));

        // Not found
        assert!(compiled.get_definition("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_has_definition() {
        let compiled = CompiledLibrary::new(create_test_library());

        assert!(compiled.has_definition("InPopulation"));
        assert!(compiled.has_definition("MeasurementPeriod"));
        assert!(!compiled.has_definition("NonExistent"));
    }

    #[test]
    fn test_compiled_library_definition_names() {
        let compiled = CompiledLibrary::new(create_test_library());

        let names = compiled.definition_names();
        assert!(names.contains(&"InPopulation"));
        assert!(names.contains(&"PrivateHelper"));
        assert!(names.contains(&"MeasurementPeriod"));
    }

    #[test]
    fn test_definition_ref_name() {
        let compiled = CompiledLibrary::new(create_test_library());

        let def = compiled.get_definition("InPopulation").unwrap();
        assert_eq!(def.name(), Some("InPopulation"));
    }

    #[test]
    fn test_definition_ref_access_level() {
        let compiled = CompiledLibrary::new(create_test_library());

        let public_def = compiled.get_definition("InPopulation").unwrap();
        assert!(public_def.is_public());

        let private_def = compiled.get_definition("PrivateHelper").unwrap();
        assert!(!private_def.is_public());
    }

    #[test]
    fn test_compiled_library_from_library() {
        let library = create_test_library();
        let compiled: CompiledLibrary = library.into();
        assert_eq!(compiled.name(), Some("TestLibrary"));
    }

    #[test]
    fn test_compiled_library_empty() {
        let compiled = CompiledLibrary::new(Library::default());

        assert!(compiled.name().is_none());
        assert!(compiled.usings().is_empty());
        assert!(compiled.includes().is_empty());
        assert!(compiled.parameters().is_empty());
        assert!(compiled.expressions().is_empty());
        assert!(compiled.definition_names().is_empty());
    }

    // ===========================================
    // Function lookup tests
    // ===========================================

    fn create_function_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("FuncLib".to_string()),
                version: Some("1.0.0".to_string()),
                system: None,
            }),
            statements: Some(ExpressionDefs {
                defs: vec![
                    StatementDef::Expression(ExpressionDef {
                        name: Some("Simple".to_string()),
                        access_level: Some(AccessModifier::Public),
                        ..Default::default()
                    }),
                    StatementDef::Function(FunctionDef {
                        name: Some("Add".to_string()),
                        access_level: Some(AccessModifier::Public),
                        operand: vec![
                            OperandDef {
                                name: Some("a".to_string()),
                                operand_type_name: Some(
                                    "{urn:hl7-org:elm-types:r1}Integer".to_string(),
                                ),
                                ..Default::default()
                            },
                            OperandDef {
                                name: Some("b".to_string()),
                                operand_type_name: Some(
                                    "{urn:hl7-org:elm-types:r1}Integer".to_string(),
                                ),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }),
                    // Second overload of Add with String operands
                    StatementDef::Function(FunctionDef {
                        name: Some("Add".to_string()),
                        access_level: Some(AccessModifier::Public),
                        operand: vec![
                            OperandDef {
                                name: Some("a".to_string()),
                                operand_type_name: Some(
                                    "{urn:hl7-org:elm-types:r1}String".to_string(),
                                ),
                                ..Default::default()
                            },
                            OperandDef {
                                name: Some("b".to_string()),
                                operand_type_name: Some(
                                    "{urn:hl7-org:elm-types:r1}String".to_string(),
                                ),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }),
                    StatementDef::Function(FunctionDef {
                        name: Some("Greet".to_string()),
                        access_level: Some(AccessModifier::Private),
                        operand: vec![OperandDef {
                            name: Some("name".to_string()),
                            operand_type_name: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_compiled_library_functions_all() {
        let compiled = CompiledLibrary::new(create_function_library());
        let funcs = compiled.functions();
        // Add (×2) + Greet = 3 functions
        assert_eq!(funcs.len(), 3);
        let names: Vec<&str> = funcs.iter().map(|f| f.name).collect();
        assert_eq!(names.iter().filter(|&&n| n == "Add").count(), 2);
        assert!(names.contains(&"Greet"));
    }

    #[test]
    fn test_compiled_library_get_function_first_overload() {
        let compiled = CompiledLibrary::new(create_function_library());
        let func = compiled.get_function("Add");
        assert!(func.is_some());
        let fr = func.unwrap();
        assert_eq!(fr.name, "Add");
        assert_eq!(fr.operands.len(), 2);
        assert!(fr.def.is_some());
    }

    #[test]
    fn test_compiled_library_get_function_not_found() {
        let compiled = CompiledLibrary::new(create_function_library());
        assert!(compiled.get_function("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_get_function_overloads() {
        let compiled = CompiledLibrary::new(create_function_library());
        let overloads = compiled.get_function_overloads("Add");
        assert_eq!(overloads.len(), 2);
        // First overload has Integer operands
        assert_eq!(
            overloads[0].operands[0].operand_type_name.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}Integer")
        );
        // Second overload has String operands
        assert_eq!(
            overloads[1].operands[0].operand_type_name.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}String")
        );
    }

    #[test]
    fn test_compiled_library_get_function_overloads_not_found() {
        let compiled = CompiledLibrary::new(create_function_library());
        let overloads = compiled.get_function_overloads("NonExistent");
        assert!(overloads.is_empty());
    }

    #[test]
    fn test_compiled_library_get_function_by_signature() {
        let compiled = CompiledLibrary::new(create_function_library());

        // Match Integer overload
        let int_add = compiled.get_function_by_signature(
            "Add",
            &[
                "{urn:hl7-org:elm-types:r1}Integer",
                "{urn:hl7-org:elm-types:r1}Integer",
            ],
        );
        assert!(int_add.is_some());
        assert_eq!(
            int_add.unwrap().operands[0].operand_type_name.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}Integer")
        );

        // Match String overload
        let str_add = compiled.get_function_by_signature(
            "Add",
            &[
                "{urn:hl7-org:elm-types:r1}String",
                "{urn:hl7-org:elm-types:r1}String",
            ],
        );
        assert!(str_add.is_some());
        assert_eq!(
            str_add.unwrap().operands[0].operand_type_name.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}String")
        );

        // Wrong arity
        let wrong_arity =
            compiled.get_function_by_signature("Add", &["{urn:hl7-org:elm-types:r1}Integer"]);
        assert!(wrong_arity.is_none());

        // Not found
        assert!(compiled
            .get_function_by_signature("NoSuchFn", &[])
            .is_none());
    }

    #[test]
    fn test_compiled_library_functions_empty() {
        let compiled = CompiledLibrary::new(Library::default());
        assert!(compiled.functions().is_empty());
        assert!(compiled.get_function("Any").is_none());
        assert!(compiled.get_function_overloads("Any").is_empty());
    }

    #[test]
    fn test_compiled_library_index_repeated_lookup() {
        // Verify that repeated calls to get_* return consistent results,
        // confirming the index doesn't get corrupted across lookups.
        let compiled = CompiledLibrary::new(create_test_library());

        for _ in 0..10 {
            assert_eq!(
                compiled
                    .get_expression("InPopulation")
                    .unwrap()
                    .name
                    .as_deref(),
                Some("InPopulation")
            );
            assert_eq!(
                compiled
                    .get_parameter("MeasurementPeriod")
                    .unwrap()
                    .name
                    .as_deref(),
                Some("MeasurementPeriod")
            );
            assert!(compiled.get_expression("NonExistent").is_none());
            assert!(compiled.get_parameter("NonExistent").is_none());
        }
    }

    #[test]
    fn test_compiled_library_functions_not_in_expressions() {
        // Expressions and functions should be disjoint sets in the index.
        let compiled = CompiledLibrary::new(create_function_library());
        // "Add" is a function, not an expression
        assert!(compiled.get_expression("Add").is_none());
        assert!(compiled.get_function("Add").is_some());
        // "Simple" is an expression, not a function
        assert!(compiled.get_expression("Simple").is_some());
        assert!(compiled.get_function("Simple").is_none());
    }

    // ===========================================
    // Terminology definition tests
    // ===========================================

    fn create_terminology_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TerminologyLibrary".to_string()),
                version: Some("1.0.0".to_string()),
                system: None,
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
                        name: Some("SNOMED".to_string()),
                        id: Some("http://snomed.info/sct".to_string()),
                        version: Some("2023-09".to_string()),
                        access_level: Some(AccessModifier::Public),
                    },
                    CodeSystemDef {
                        name: Some("InternalCodes".to_string()),
                        id: Some("http://example.org/internal".to_string()),
                        version: None,
                        access_level: Some(AccessModifier::Private),
                    },
                ],
            }),
            value_sets: Some(ValueSetDefs {
                defs: vec![
                    ValueSetDef {
                        name: Some("DiabetesCodes".to_string()),
                        id: Some("http://cts.nlm.nih.gov/fhir/ValueSet/2.16.840.1.113883.3.464.1003.103.12.1001".to_string()),
                        version: Some("20230101".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: vec![],
                    },
                    ValueSetDef {
                        name: Some("PrivateValueSet".to_string()),
                        id: Some("http://example.org/private".to_string()),
                        version: None,
                        access_level: Some(AccessModifier::Private),
                        code_system: vec![],
                    },
                ],
            }),
            codes: Some(CodeDefs {
                defs: vec![
                    CodeDef {
                        name: Some("HbA1c".to_string()),
                        id: Some("4548-4".to_string()),
                        display: Some("Hemoglobin A1c".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: None,
                    },
                    CodeDef {
                        name: Some("GlucoseLevel".to_string()),
                        id: Some("2339-0".to_string()),
                        display: Some("Glucose [Mass/volume] in Blood".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: None,
                    },
                    CodeDef {
                        name: Some("InternalCode".to_string()),
                        id: Some("INT-001".to_string()),
                        display: None,
                        access_level: Some(AccessModifier::Private),
                        code_system: None,
                    },
                ],
            }),
            concepts: Some(ConceptDefs {
                defs: vec![
                    ConceptDef {
                        name: Some("DiabetesLabTests".to_string()),
                        display: Some("Diabetes-related laboratory tests".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code: vec![],
                    },
                    ConceptDef {
                        name: Some("PrivateConcept".to_string()),
                        display: None,
                        access_level: Some(AccessModifier::Private),
                        code: vec![],
                    },
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_compiled_library_code_systems() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let code_systems = compiled.code_systems();
        assert_eq!(code_systems.len(), 3);

        let loinc = compiled.get_code_system("LOINC");
        assert!(loinc.is_some());
        assert_eq!(loinc.unwrap().id, Some("http://loinc.org".to_string()));
        assert_eq!(loinc.unwrap().version, Some("2.73".to_string()));

        let snomed = compiled.get_code_system("SNOMED");
        assert!(snomed.is_some());

        assert!(compiled.get_code_system("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_value_sets() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let value_sets = compiled.value_sets();
        assert_eq!(value_sets.len(), 2);

        let diabetes = compiled.get_value_set("DiabetesCodes");
        assert!(diabetes.is_some());
        assert!(diabetes.unwrap().id.as_ref().unwrap().contains("2.16.840"));

        assert!(compiled.get_value_set("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_codes() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let codes = compiled.codes();
        assert_eq!(codes.len(), 3);

        let hba1c = compiled.get_code("HbA1c");
        assert!(hba1c.is_some());
        assert_eq!(hba1c.unwrap().id, Some("4548-4".to_string()));
        assert_eq!(hba1c.unwrap().display, Some("Hemoglobin A1c".to_string()));

        assert!(compiled.get_code("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_concepts() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let concepts = compiled.concepts();
        assert_eq!(concepts.len(), 2);

        let diabetes_labs = compiled.get_concept("DiabetesLabTests");
        assert!(diabetes_labs.is_some());
        assert_eq!(
            diabetes_labs.unwrap().display,
            Some("Diabetes-related laboratory tests".to_string())
        );

        assert!(compiled.get_concept("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_get_definition_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        // CodeSystem
        let def = compiled.get_definition("LOINC");
        assert!(matches!(def, Some(DefinitionRef::CodeSystem(_))));

        // ValueSet
        let def = compiled.get_definition("DiabetesCodes");
        assert!(matches!(def, Some(DefinitionRef::ValueSet(_))));

        // Code
        let def = compiled.get_definition("HbA1c");
        assert!(matches!(def, Some(DefinitionRef::Code(_))));

        // Concept
        let def = compiled.get_definition("DiabetesLabTests");
        assert!(matches!(def, Some(DefinitionRef::Concept(_))));
    }

    #[test]
    fn test_compiled_library_has_definition_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        assert!(compiled.has_definition("LOINC"));
        assert!(compiled.has_definition("DiabetesCodes"));
        assert!(compiled.has_definition("HbA1c"));
        assert!(compiled.has_definition("DiabetesLabTests"));
        assert!(!compiled.has_definition("NonExistent"));
    }

    #[test]
    fn test_compiled_library_definition_names_includes_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let names = compiled.definition_names();
        assert!(names.contains(&"LOINC"));
        assert!(names.contains(&"SNOMED"));
        assert!(names.contains(&"DiabetesCodes"));
        assert!(names.contains(&"HbA1c"));
        assert!(names.contains(&"GlucoseLevel"));
        assert!(names.contains(&"DiabetesLabTests"));
    }

    #[test]
    fn test_definition_ref_access_level_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        // Public code system
        let def = compiled.get_definition("LOINC").unwrap();
        assert!(def.is_public());
        assert_eq!(def.access_level(), Some(AccessModifier::Public));

        // Private code system
        let def = compiled.get_definition("InternalCodes").unwrap();
        assert!(!def.is_public());
        assert_eq!(def.access_level(), Some(AccessModifier::Private));

        // Public value set
        let def = compiled.get_definition("DiabetesCodes").unwrap();
        assert!(def.is_public());

        // Private value set
        let def = compiled.get_definition("PrivateValueSet").unwrap();
        assert!(!def.is_public());

        // Public code
        let def = compiled.get_definition("HbA1c").unwrap();
        assert!(def.is_public());

        // Private code
        let def = compiled.get_definition("InternalCode").unwrap();
        assert!(!def.is_public());

        // Public concept
        let def = compiled.get_definition("DiabetesLabTests").unwrap();
        assert!(def.is_public());

        // Private concept
        let def = compiled.get_definition("PrivateConcept").unwrap();
        assert!(!def.is_public());
    }

    #[test]
    fn test_definition_ref_name_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let cs = compiled.get_definition("LOINC").unwrap();
        assert_eq!(cs.name(), Some("LOINC"));

        let vs = compiled.get_definition("DiabetesCodes").unwrap();
        assert_eq!(vs.name(), Some("DiabetesCodes"));

        let code = compiled.get_definition("HbA1c").unwrap();
        assert_eq!(code.name(), Some("HbA1c"));

        let concept = compiled.get_definition("DiabetesLabTests").unwrap();
        assert_eq!(concept.name(), Some("DiabetesLabTests"));
    }

    #[test]
    fn test_compiled_library_empty_terminology() {
        let compiled = CompiledLibrary::new(Library::default());

        assert!(compiled.code_systems().is_empty());
        assert!(compiled.value_sets().is_empty());
        assert!(compiled.codes().is_empty());
        assert!(compiled.concepts().is_empty());
    }

    #[test]
    fn test_compiled_library_mixed_definitions() {
        // Test a library with both expressions/parameters and terminology
        let mut library = create_test_library();
        library.code_systems = Some(CodeSystemDefs {
            defs: vec![CodeSystemDef {
                name: Some("ICD10".to_string()),
                id: Some("http://hl7.org/fhir/sid/icd-10".to_string()),
                version: None,
                access_level: Some(AccessModifier::Public),
            }],
        });

        let compiled = CompiledLibrary::new(library);

        // Check expressions still work
        assert!(compiled.get_expression("InPopulation").is_some());
        assert!(compiled.get_parameter("MeasurementPeriod").is_some());

        // Check code system also works
        assert!(compiled.get_code_system("ICD10").is_some());

        // Check definition_names includes all types
        let names = compiled.definition_names();
        assert!(names.contains(&"InPopulation"));
        assert!(names.contains(&"MeasurementPeriod"));
        assert!(names.contains(&"ICD10"));
    }

    // ===========================================
    // LibraryManager tests
    // ===========================================

    #[test]
    fn test_library_manager_new() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);
        assert_eq!(manager.cached_libraries().len(), 0);
    }

    #[test]
    fn test_library_manager_has_source() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("TestLib", Some("1.0")),
            "library TestLib version '1.0'".to_string(),
        );

        let manager = LibraryManager::new(provider);
        assert!(manager.has_source(&LibraryIdentifier::new("TestLib", Some("1.0"))));
        assert!(!manager.has_source(&LibraryIdentifier::new("NonExistent", Some("1.0"))));
    }

    #[test]
    fn test_library_manager_get_source() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("TestLib", Some("1.0")),
            "library TestLib version '1.0'".to_string(),
        );

        let manager = LibraryManager::new(provider);
        let source = manager.get_source(&LibraryIdentifier::new("TestLib", Some("1.0")));
        assert!(source.is_some());
        assert!(source.unwrap().source.contains("TestLib"));
    }

    #[test]
    fn test_library_manager_cache_operations() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        let library = CompiledLibrary::new(Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLib".to_string()),
                version: Some("1.0".to_string()),
                system: None,
            }),
            ..Default::default()
        });

        // Not cached initially
        assert!(!manager.is_cached(&id));
        assert!(manager.get_cached(&id).is_none());

        // Cache it
        manager.cache_library(&id, library);
        assert!(manager.is_cached(&id));

        // Retrieve from cache
        let cached = manager.get_cached(&id);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().name(), Some("TestLib"));
    }

    #[test]
    fn test_library_manager_invalidate() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        let library = CompiledLibrary::new(Library::default());

        manager.cache_library(&id, library);
        assert!(manager.is_cached(&id));

        manager.invalidate(&id);
        assert!(!manager.is_cached(&id));
    }

    #[test]
    fn test_library_manager_clear_cache() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        manager.cache_library(
            &LibraryIdentifier::new("Lib1", Some("1.0")),
            CompiledLibrary::new(Library::default()),
        );
        manager.cache_library(
            &LibraryIdentifier::new("Lib2", Some("1.0")),
            CompiledLibrary::new(Library::default()),
        );

        assert_eq!(manager.cached_libraries().len(), 2);
        manager.clear_cache();
        assert_eq!(manager.cached_libraries().len(), 0);
    }

    #[test]
    fn test_library_manager_cached_libraries() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        let id1 = LibraryIdentifier::new("Lib1", Some("1.0"));
        let id2 = LibraryIdentifier::new("Lib2", Some("2.0"));

        manager.cache_library(&id1, CompiledLibrary::new(Library::default()));
        manager.cache_library(&id2, CompiledLibrary::new(Library::default()));

        let cached = manager.cached_libraries();
        assert_eq!(cached.len(), 2);
        assert!(cached.iter().any(|id| id.name == "Lib1"));
        assert!(cached.iter().any(|id| id.name == "Lib2"));
    }

    #[test]
    fn test_library_manager_resolve_not_found() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        let id = LibraryIdentifier::new("NonExistent", Some("1.0"));
        let result = manager.resolve(&id);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), LibraryError::NotFound(_)));
    }

    #[test]
    fn test_library_manager_resolve_from_source() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("TestLib", Some("1.0")),
            "library TestLib version '1.0'".to_string(),
        );

        let manager = LibraryManager::new(provider);
        let id = LibraryIdentifier::new("TestLib", Some("1.0"));

        // Not cached initially
        assert!(!manager.is_cached(&id));

        // Resolve
        let result = manager.resolve(&id);
        assert!(result.is_ok());

        // Now cached
        assert!(manager.is_cached(&id));

        // Second resolve uses cache
        let result2 = manager.resolve(&id);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_library_manager_resolve_cached() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        let library = CompiledLibrary::new(Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLib".to_string()),
                version: Some("1.0".to_string()),
                system: None,
            }),
            ..Default::default()
        });

        // Pre-cache
        manager.cache_library(&id, library);

        // Resolve should return cached (even without source)
        let result = manager.resolve(&id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name(), Some("TestLib"));
    }

    #[test]
    fn test_library_manager_resolve_all() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("Lib1", Some("1.0")),
            "library Lib1 version '1.0'".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("Lib2", Some("1.0")),
            "library Lib2 version '1.0'".to_string(),
        );

        let manager = LibraryManager::new(provider);

        let ids = vec![
            LibraryIdentifier::new("Lib1", Some("1.0")),
            LibraryIdentifier::new("Lib2", Some("1.0")),
        ];

        let result = manager.resolve_all(&ids);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_library_manager_resolve_all_partial_failure() {
        let provider = MemoryLibrarySourceProvider::new();
        provider.register_source(
            LibraryIdentifier::new("Lib1", Some("1.0")),
            "library Lib1 version '1.0'".to_string(),
        );

        let manager = LibraryManager::new(provider);

        let ids = vec![
            LibraryIdentifier::new("Lib1", Some("1.0")),
            LibraryIdentifier::new("NonExistent", Some("1.0")),
        ];

        let result = manager.resolve_all(&ids);
        assert!(result.is_err());
    }

    #[test]
    fn test_library_error_display() {
        let not_found = LibraryError::NotFound(LibraryIdentifier::new("MyLib", Some("1.0")));
        assert!(not_found.to_string().contains("MyLib"));

        let cycle = LibraryError::CircularDependency(vec![
            LibraryIdentifier::new("A", Some("1.0")),
            LibraryIdentifier::new("B", Some("1.0")),
            LibraryIdentifier::new("A", Some("1.0")),
        ]);
        let msg = cycle.to_string();
        assert!(msg.contains("Circular"));
        assert!(msg.contains("A"));
        assert!(msg.contains("B"));

        let parse_err = LibraryError::ParseError {
            library: LibraryIdentifier::new("BadLib", Some("1.0")),
            message: "syntax error".to_string(),
        };
        assert!(parse_err.to_string().contains("BadLib"));
        assert!(parse_err.to_string().contains("syntax error"));

        let version_conflict = LibraryError::VersionConflict {
            library: "ConflictLib".to_string(),
            requested: Some("1.0".to_string()),
            found: Some("2.0".to_string()),
        };
        assert!(version_conflict.to_string().contains("ConflictLib"));
        assert!(version_conflict.to_string().contains("1.0"));
        assert!(version_conflict.to_string().contains("2.0"));
    }

    #[test]
    fn test_library_manager_circular_dependency_self() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // Create a library that includes itself (using cached include_identifiers)
        let id = LibraryIdentifier::new("SelfRef", Some("1.0"));
        let library = CompiledLibrary::new(Library {
            identifier: Some(VersionedIdentifier {
                id: Some("SelfRef".to_string()),
                version: Some("1.0".to_string()),
                system: None,
            }),
            includes: Some(IncludeDefs {
                defs: vec![IncludeDef {
                    local_identifier: Some("SelfRef".to_string()),
                    path: Some("SelfRef".to_string()),
                    version: Some("1.0".to_string()),
                }],
            }),
            ..Default::default()
        });

        manager.cache_library(&id, library);

        // detect_cycle should find the cycle
        let cycle = manager.detect_cycle(&id);
        assert!(cycle.is_some());
        let cycle_path = cycle.unwrap();
        assert!(cycle_path.iter().any(|i| i.name == "SelfRef"));
    }

    #[test]
    fn test_library_manager_circular_dependency_chain() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // Create a cycle: A -> B -> C -> A
        let a = LibraryIdentifier::new("A", Some("1.0"));
        let b = LibraryIdentifier::new("B", Some("1.0"));
        let c = LibraryIdentifier::new("C", Some("1.0"));

        manager.cache_library(
            &a,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("A".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("B".to_string()),
                        path: Some("B".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &b,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("B".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("C".to_string()),
                        path: Some("C".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &c,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("C".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("A".to_string()),
                        path: Some("A".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        // detect_cycle from A should find the cycle
        let cycle = manager.detect_cycle(&a);
        assert!(cycle.is_some());
    }

    #[test]
    fn test_library_manager_no_cycle() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // A -> B -> C (no cycle)
        let a = LibraryIdentifier::new("A", Some("1.0"));
        let b = LibraryIdentifier::new("B", Some("1.0"));
        let c = LibraryIdentifier::new("C", Some("1.0"));

        manager.cache_library(
            &a,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("A".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("B".to_string()),
                        path: Some("B".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &b,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("B".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("C".to_string()),
                        path: Some("C".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &c,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("C".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                ..Default::default()
            }),
        );

        // No cycle
        assert!(manager.detect_cycle(&a).is_none());
    }

    #[test]
    fn test_library_manager_dependency_graph() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // A -> B, C
        // B -> C
        let a = LibraryIdentifier::new("A", Some("1.0"));
        let b = LibraryIdentifier::new("B", Some("1.0"));
        let c = LibraryIdentifier::new("C", Some("1.0"));

        manager.cache_library(
            &a,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("A".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![
                        IncludeDef {
                            local_identifier: Some("B".to_string()),
                            path: Some("B".to_string()),
                            version: Some("1.0".to_string()),
                        },
                        IncludeDef {
                            local_identifier: Some("C".to_string()),
                            path: Some("C".to_string()),
                            version: Some("1.0".to_string()),
                        },
                    ],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &b,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("B".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("C".to_string()),
                        path: Some("C".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &c,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("C".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                ..Default::default()
            }),
        );

        let graph = manager.dependency_graph(&a);
        assert_eq!(graph.len(), 3);
        assert!(graph.get(&a).unwrap().len() == 2);
        assert!(graph.get(&b).unwrap().len() == 1);
        assert!(graph.get(&c).unwrap().is_empty());
    }

    #[test]
    fn test_library_manager_topological_sort() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // A -> B, C
        // B -> C
        let a = LibraryIdentifier::new("A", Some("1.0"));
        let b = LibraryIdentifier::new("B", Some("1.0"));
        let c = LibraryIdentifier::new("C", Some("1.0"));

        manager.cache_library(
            &a,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("A".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![
                        IncludeDef {
                            local_identifier: Some("B".to_string()),
                            path: Some("B".to_string()),
                            version: Some("1.0".to_string()),
                        },
                        IncludeDef {
                            local_identifier: Some("C".to_string()),
                            path: Some("C".to_string()),
                            version: Some("1.0".to_string()),
                        },
                    ],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &b,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("B".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("C".to_string()),
                        path: Some("C".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &c,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("C".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                ..Default::default()
            }),
        );

        let sorted = manager.topological_sort(&a).unwrap();

        // C should come before B, and both before A
        let c_pos = sorted.iter().position(|x| x.name == "C").unwrap();
        let b_pos = sorted.iter().position(|x| x.name == "B").unwrap();
        let a_pos = sorted.iter().position(|x| x.name == "A").unwrap();

        assert!(c_pos < b_pos);
        assert!(c_pos < a_pos);
        assert!(b_pos < a_pos);
    }

    #[test]
    fn test_library_manager_topological_sort_cycle() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);

        // A -> B -> A (cycle)
        let a = LibraryIdentifier::new("A", Some("1.0"));
        let b = LibraryIdentifier::new("B", Some("1.0"));

        manager.cache_library(
            &a,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("A".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("B".to_string()),
                        path: Some("B".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        manager.cache_library(
            &b,
            CompiledLibrary::new(Library {
                identifier: Some(VersionedIdentifier {
                    id: Some("B".to_string()),
                    version: Some("1.0".to_string()),
                    system: None,
                }),
                includes: Some(IncludeDefs {
                    defs: vec![IncludeDef {
                        local_identifier: Some("A".to_string()),
                        path: Some("A".to_string()),
                        version: Some("1.0".to_string()),
                    }],
                }),
                ..Default::default()
            }),
        );

        let result = manager.topological_sort(&a);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LibraryError::CircularDependency(_)
        ));
    }

    #[test]
    fn test_library_manager_default() {
        let manager: LibraryManager<MemoryLibrarySourceProvider> = LibraryManager::default();
        assert_eq!(manager.cached_libraries().len(), 0);
    }

    #[test]
    fn test_library_manager_debug() {
        let provider = MemoryLibrarySourceProvider::new();
        let manager = LibraryManager::new(provider);
        let debug_str = format!("{manager:?}");
        assert!(debug_str.contains("LibraryManager"));
    }
}
