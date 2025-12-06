//! ELM output generation module.
//!
//! This module provides functionality to generate JSON (and optionally XML)
//! output from translated ELM libraries. It handles:
//! - Adding translator metadata (version, options)
//! - JSON serialization with proper formatting
//! - Output format selection

use crate::elm::{CqlToElmInfo, Library, LibraryAnnotation, VersionedIdentifier};
use crate::options::{CompilerOptions, OutputFormat};

/// Version of this translator implementation.
pub const TRANSLATOR_VERSION: &str = env!("CARGO_PKG_VERSION");

/// ELM output writer for generating serialized ELM.
///
/// This struct handles the generation of ELM output in various formats,
/// including adding translator metadata and applying compiler options.
///
/// # Example
///
/// ```
/// use rh_cql::output::{ElmWriter, TRANSLATOR_VERSION};
/// use rh_cql::options::CompilerOptions;
/// use rh_cql::elm::Library;
///
/// let library = Library::default();
/// let options = CompilerOptions::default();
///
/// let writer = ElmWriter::new(&options);
/// let json = writer.to_json(&library).unwrap();
///
/// // Output includes schema identifier and translator info
/// assert!(json.contains("schemaIdentifier"));
/// assert!(json.contains("translatorVersion"));
/// ```
pub struct ElmWriter<'a> {
    options: &'a CompilerOptions,
    pretty_print: bool,
}

impl<'a> ElmWriter<'a> {
    /// Create a new ELM writer with the given compiler options.
    pub fn new(options: &'a CompilerOptions) -> Self {
        Self {
            options,
            pretty_print: true,
        }
    }

    /// Set whether to pretty-print the output.
    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }

    /// Generate JSON output from an ELM library.
    ///
    /// This adds translator metadata if annotations are enabled.
    pub fn to_json(&self, library: &Library) -> Result<String, OutputError> {
        let library = self.prepare_library(library);

        if self.pretty_print {
            serde_json::to_string_pretty(&library).map_err(OutputError::JsonSerialization)
        } else {
            serde_json::to_string(&library).map_err(OutputError::JsonSerialization)
        }
    }

    /// Generate JSON output as bytes.
    pub fn to_json_bytes(&self, library: &Library) -> Result<Vec<u8>, OutputError> {
        let library = self.prepare_library(library);

        if self.pretty_print {
            serde_json::to_vec_pretty(&library).map_err(OutputError::JsonSerialization)
        } else {
            serde_json::to_vec(&library).map_err(OutputError::JsonSerialization)
        }
    }

    /// Write JSON output to a writer.
    pub fn write_json<W: std::io::Write>(
        &self,
        library: &Library,
        writer: W,
    ) -> Result<(), OutputError> {
        let library = self.prepare_library(library);

        if self.pretty_print {
            serde_json::to_writer_pretty(writer, &library).map_err(OutputError::JsonSerialization)
        } else {
            serde_json::to_writer(writer, &library).map_err(OutputError::JsonSerialization)
        }
    }

    /// Generate output in the specified format.
    pub fn to_format(
        &self,
        library: &Library,
        format: OutputFormat,
    ) -> Result<String, OutputError> {
        match format {
            OutputFormat::Json => self.to_json(library),
            OutputFormat::Xml => Err(OutputError::UnsupportedFormat("XML".to_string())),
        }
    }

    /// Prepare a library for output by adding translator metadata.
    fn prepare_library(&self, library: &Library) -> Library {
        let mut output = library.clone();

        // Add translator info if annotations are enabled
        if self.options.annotations_enabled() {
            let cql_to_elm_info = CqlToElmInfo {
                translator_version: Some(TRANSLATOR_VERSION.to_string()),
                translator_options: Some(self.options.options_to_string()),
            };

            // Check if CqlToElmInfo already exists
            let has_info = output
                .annotation
                .iter()
                .any(|a| matches!(a, LibraryAnnotation::CqlToElmInfo(_)));

            if !has_info {
                output
                    .annotation
                    .insert(0, LibraryAnnotation::CqlToElmInfo(cql_to_elm_info));
            }
        }

        // Ensure schema identifier is set
        if output.schema_identifier.is_none() {
            output.schema_identifier = Some(VersionedIdentifier {
                id: Some("urn:hl7-org:elm".to_string()),
                system: None,
                version: Some("r1".to_string()),
            });
        }

        output
    }
}

/// Errors that can occur during output generation.
#[derive(Debug)]
pub enum OutputError {
    /// JSON serialization error.
    JsonSerialization(serde_json::Error),
    /// Unsupported output format.
    UnsupportedFormat(String),
    /// IO error.
    Io(std::io::Error),
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputError::JsonSerialization(e) => write!(f, "JSON serialization error: {e}"),
            OutputError::UnsupportedFormat(fmt) => write!(f, "Unsupported output format: {fmt}"),
            OutputError::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for OutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OutputError::JsonSerialization(e) => Some(e),
            OutputError::Io(e) => Some(e),
            OutputError::UnsupportedFormat(_) => None,
        }
    }
}

impl From<serde_json::Error> for OutputError {
    fn from(e: serde_json::Error) -> Self {
        OutputError::JsonSerialization(e)
    }
}

impl From<std::io::Error> for OutputError {
    fn from(e: std::io::Error) -> Self {
        OutputError::Io(e)
    }
}

/// Convenience function to convert an ELM library to JSON with default options.
pub fn library_to_json(library: &Library) -> Result<String, OutputError> {
    let options = CompilerOptions::default();
    ElmWriter::new(&options).to_json(library)
}

/// Convenience function to convert an ELM library to JSON with specific options.
pub fn library_to_json_with_options(
    library: &Library,
    options: &CompilerOptions,
) -> Result<String, OutputError> {
    ElmWriter::new(options).to_json(library)
}

/// Convenience function to convert an ELM library to compact JSON (no pretty printing).
pub fn library_to_compact_json(library: &Library) -> Result<String, OutputError> {
    let options = CompilerOptions::default();
    ElmWriter::new(&options)
        .with_pretty_print(false)
        .to_json(library)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{Expression, ExpressionDef, ExpressionDefs, Literal, UsingDef, UsingDefs};

    fn create_test_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLibrary".to_string()),
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
                defs: vec![ExpressionDef {
                    name: Some("TestExpression".to_string()),
                    expression: Some(Box::new(Expression::Literal(Literal {
                        value: Some("42".to_string()),
                        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                }],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_to_json_basic() {
        let library = create_test_library();
        let options = CompilerOptions::new(); // No annotations
        let writer = ElmWriter::new(&options);

        let json = writer.to_json(&library).unwrap();
        assert!(json.contains("TestLibrary"));
        assert!(json.contains("TestExpression"));
    }

    #[test]
    fn test_to_json_with_annotations() {
        let library = create_test_library();
        let options = CompilerOptions::default(); // Annotations enabled
        let writer = ElmWriter::new(&options);

        let json = writer.to_json(&library).unwrap();
        assert!(json.contains("translatorVersion"));
        assert!(json.contains(TRANSLATOR_VERSION));
        assert!(json.contains("translatorOptions"));
    }

    #[test]
    fn test_to_json_compact() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options).with_pretty_print(false);

        let json = writer.to_json(&library).unwrap();
        // Compact JSON should not have newlines in the middle
        assert!(!json.contains("\n  "));
    }

    #[test]
    fn test_to_json_pretty() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options).with_pretty_print(true);

        let json = writer.to_json(&library).unwrap();
        // Pretty JSON should have indented lines
        assert!(json.contains("\n  "));
    }

    #[test]
    fn test_schema_identifier_added() {
        let library = Library::default();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options);

        let json = writer.to_json(&library).unwrap();
        assert!(json.contains("schemaIdentifier"));
        assert!(json.contains("urn:hl7-org:elm"));
    }

    #[test]
    fn test_translator_info_not_duplicated() {
        let mut library = create_test_library();
        // Pre-add CqlToElmInfo
        library
            .annotation
            .push(LibraryAnnotation::CqlToElmInfo(CqlToElmInfo {
                translator_version: Some("existing".to_string()),
                translator_options: None,
            }));

        let options = CompilerOptions::default();
        let writer = ElmWriter::new(&options);

        let json = writer.to_json(&library).unwrap();
        // Should still have "existing" version, not duplicated
        assert!(json.contains("existing"));
        // Count occurrences of translatorVersion
        let count = json.matches("translatorVersion").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_to_json_bytes() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options);

        let bytes = writer.to_json_bytes(&library).unwrap();
        let json = String::from_utf8(bytes).unwrap();
        assert!(json.contains("TestLibrary"));
    }

    #[test]
    fn test_write_json() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options);

        let mut buffer = Vec::new();
        writer.write_json(&library, &mut buffer).unwrap();
        let json = String::from_utf8(buffer).unwrap();
        assert!(json.contains("TestLibrary"));
    }

    #[test]
    fn test_to_format_json() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options);

        let json = writer.to_format(&library, OutputFormat::Json).unwrap();
        assert!(json.contains("TestLibrary"));
    }

    #[test]
    fn test_to_format_xml_unsupported() {
        let library = create_test_library();
        let options = CompilerOptions::new();
        let writer = ElmWriter::new(&options);

        let result = writer.to_format(&library, OutputFormat::Xml);
        assert!(result.is_err());
        if let Err(OutputError::UnsupportedFormat(fmt)) = result {
            assert_eq!(fmt, "XML");
        } else {
            panic!("Expected UnsupportedFormat error");
        }
    }

    #[test]
    fn test_convenience_functions() {
        let library = create_test_library();

        let json1 = library_to_json(&library).unwrap();
        assert!(json1.contains("translatorVersion"));

        let options = CompilerOptions::new();
        let json2 = library_to_json_with_options(&library, &options).unwrap();
        assert!(!json2.contains("translatorVersion")); // No annotations

        let json3 = library_to_compact_json(&library).unwrap();
        assert!(!json3.contains("\n  ")); // Compact
    }

    #[test]
    fn test_output_error_display() {
        let err = OutputError::UnsupportedFormat("XML".to_string());
        assert_eq!(err.to_string(), "Unsupported output format: XML");
    }

    #[test]
    fn test_translator_version() {
        // Ensure version is set from Cargo.toml
        // Using explicit check since is_empty() on const triggers clippy
        assert!(TRANSLATOR_VERSION.chars().count() > 0);
    }

    #[test]
    fn test_options_in_output() {
        let library = create_test_library();
        let options = CompilerOptions::default()
            .with_option(crate::options::CompilerOption::EnableResultTypes);

        let writer = ElmWriter::new(&options);
        let json = writer.to_json(&library).unwrap();

        // Should contain the options string
        assert!(json.contains("EnableAnnotations"));
        assert!(json.contains("EnableLocators"));
        assert!(json.contains("EnableResultTypes"));
    }
}
