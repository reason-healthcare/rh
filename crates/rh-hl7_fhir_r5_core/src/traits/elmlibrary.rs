use crate::traits::resource::ResourceExistence;
/// ELM Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents an executable CQL logic library in translated ELM format
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elmlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait ElmlibraryAccessors {}
/// ELM Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents an executable CQL logic library in translated ELM format
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elmlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait ElmlibraryMutators {
    /// Create a new Elmlibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::elmlibrary::Elmlibrary;
    /// use rh_hl7_fhir_r5_core::traits::elmlibrary::ElmlibraryMutators;
    ///
    /// let resource = Elmlibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// elmlibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents an executable CQL logic library in translated ELM format
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elmlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait ElmlibraryExistence: ResourceExistence {}
