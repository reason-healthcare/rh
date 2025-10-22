use crate::traits::resource::ResourceExistence;
/// CQL Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Library
pub trait CqllibraryAccessors {}
/// CQL Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Library
pub trait CqllibraryMutators {
    /// Create a new Cqllibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cqllibrary::Cqllibrary;
    /// use hl7_fhir_r4_core::traits::cqllibrary::CqllibraryMutators;
    ///
    /// let resource = Cqllibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cqllibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents a CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Library
pub trait CqllibraryExistence: ResourceExistence {}
