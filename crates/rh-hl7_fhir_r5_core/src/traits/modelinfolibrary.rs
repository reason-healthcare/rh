use crate::traits::resource::ResourceExistence;
/// Model Information Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable representation of a model information library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/modelinfolibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModelinfolibraryAccessors {}
/// Model Information Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a computable representation of a model information library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/modelinfolibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModelinfolibraryMutators {
    /// Create a new Modelinfolibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::modelinfolibrary::Modelinfolibrary;
    /// use rh_hl7_fhir_r5_core::traits::modelinfolibrary::ModelinfolibraryMutators;
    ///
    /// let resource = Modelinfolibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// modelinfolibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents a computable representation of a model information library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/modelinfolibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModelinfolibraryExistence: ResourceExistence {}
