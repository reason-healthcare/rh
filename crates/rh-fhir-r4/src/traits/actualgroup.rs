use crate::traits::resource::ResourceExistence;
/// Actual Group Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces an actual group, rather than a definitional group
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/actualgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait ActualgroupAccessors {}
/// Actual Group Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces an actual group, rather than a definitional group
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/actualgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait ActualgroupMutators {
    /// Create a new Actualgroup with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::actualgroup::Actualgroup;
    /// use hl7_fhir_r4_core::traits::actualgroup::ActualgroupMutators;
    ///
    /// let resource = Actualgroup::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// actualgroup Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces an actual group, rather than a definitional group
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/actualgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Group
pub trait ActualgroupExistence: ResourceExistence {}
