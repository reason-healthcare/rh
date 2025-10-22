use crate::traits::resource::ResourceExistence;
/// Group Definition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces a descriptive group that can be used in definitional resources
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/groupdefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait GroupdefinitionAccessors {}
/// Group Definition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces a descriptive group that can be used in definitional resources
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/groupdefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait GroupdefinitionMutators {
    /// Create a new Groupdefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::groupdefinition::Groupdefinition;
    /// use hl7_fhir_r4_core::traits::groupdefinition::GroupdefinitionMutators;
    ///
    /// let resource = Groupdefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// groupdefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces a descriptive group that can be used in definitional resources
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/groupdefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait GroupdefinitionExistence: ResourceExistence {}
