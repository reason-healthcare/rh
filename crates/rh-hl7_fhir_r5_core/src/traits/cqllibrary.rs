use crate::traits::resource::ResourceExistence;
/// CQL Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait CqllibraryAccessors {}
/// CQL Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait CqllibraryMutators {
    /// Create a new Cqllibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::cqllibrary::Cqllibrary;
    /// use rh_hl7_fhir_r5_core::traits::cqllibrary::CqllibraryMutators;
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
/// Represents a computable CQL logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqllibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait CqllibraryExistence: ResourceExistence {}
