use crate::resources::parameters::ParametersParameter;
use crate::traits::resource::ResourceAccessors;
use crate::traits::resource::ResourceExistence;
use crate::traits::resource::ResourceMutators;
/// Parameters Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource is a non-persisted resource used to pass information into and back from an [operation](operations.html). It has no other use, and there is no RESTful endpoint associated with it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Parameters
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Parameters
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait ParametersAccessors: ResourceAccessors {
    /// Returns a reference to the parameter field.
    fn parameter(&self) -> &[ParametersParameter];
}
/// Parameters Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource is a non-persisted resource used to pass information into and back from an [operation](operations.html). It has no other use, and there is no RESTful endpoint associated with it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Parameters
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Parameters
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait ParametersMutators: ResourceMutators {
    /// Create a new Parameters with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::parameters::Parameters;
    /// use hl7_fhir_r4_core::traits::parameters::ParametersMutators;
    ///
    /// let resource = Parameters::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the parameter field and returns self for chaining.
    fn set_parameter(self, value: Vec<ParametersParameter>) -> Self;
    /// Adds an item to the parameter field and returns self for chaining.
    fn add_parameter(self, item: ParametersParameter) -> Self;
}
/// Parameters Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource is a non-persisted resource used to pass information into and back from an [operation](operations.html). It has no other use, and there is no RESTful endpoint associated with it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Parameters
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Parameters
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait ParametersExistence: ResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the parameter field is not empty.
    fn has_parameter(&self) -> bool;
}
