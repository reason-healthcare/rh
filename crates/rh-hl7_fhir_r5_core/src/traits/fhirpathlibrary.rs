use crate::traits::resource::ResourceExistence;
/// FHIRPath Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable/executable FHIRPath logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/fhirpathlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait FhirpathlibraryAccessors {}
/// FHIRPath Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable/executable FHIRPath logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/fhirpathlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait FhirpathlibraryMutators {
    /// Create a new Fhirpathlibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::fhirpathlibrary::Fhirpathlibrary;
    /// use rh_hl7_fhir_r5_core::traits::fhirpathlibrary::FhirpathlibraryMutators;
    ///
    /// let resource = Fhirpathlibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// fhirpathlibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents a computable/executable FHIRPath logic library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/fhirpathlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/logiclibrary
pub trait FhirpathlibraryExistence: ResourceExistence {}
