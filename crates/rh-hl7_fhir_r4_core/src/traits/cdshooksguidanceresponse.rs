use crate::traits::resource::ResourceExistence;
/// CDS Hooks GuidanceResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a GuidanceResponse that represents the response container for a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksguidanceresponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
pub trait CdshooksguidanceresponseAccessors {}
/// CDS Hooks GuidanceResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a GuidanceResponse that represents the response container for a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksguidanceresponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
pub trait CdshooksguidanceresponseMutators {
    /// Create a new Cdshooksguidanceresponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cdshooksguidanceresponse::Cdshooksguidanceresponse;
    /// use hl7_fhir_r4_core::traits::cdshooksguidanceresponse::CdshooksguidanceresponseMutators;
    ///
    /// let resource = Cdshooksguidanceresponse::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cdshooksguidanceresponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a GuidanceResponse that represents the response container for a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksguidanceresponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
pub trait CdshooksguidanceresponseExistence: ResourceExistence {}
