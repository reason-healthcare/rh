use crate::traits::resource::ResourceExistence;
/// Module Definition Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The module definition library profile sets the expectations for module definition libraries, including support for terminology and dependency declaration, parameters, and data requirements
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/moduledefinitionlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModuledefinitionlibraryAccessors {}
/// Module Definition Library Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The module definition library profile sets the expectations for module definition libraries, including support for terminology and dependency declaration, parameters, and data requirements
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/moduledefinitionlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModuledefinitionlibraryMutators {
    /// Create a new Moduledefinitionlibrary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::moduledefinitionlibrary::Moduledefinitionlibrary;
    /// use rh_hl7_fhir_r5_core::traits::moduledefinitionlibrary::ModuledefinitionlibraryMutators;
    ///
    /// let resource = Moduledefinitionlibrary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// moduledefinitionlibrary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The module definition library profile sets the expectations for module definition libraries, including support for terminology and dependency declaration, parameters, and data requirements
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/moduledefinitionlibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
pub trait ModuledefinitionlibraryExistence: ResourceExistence {}
