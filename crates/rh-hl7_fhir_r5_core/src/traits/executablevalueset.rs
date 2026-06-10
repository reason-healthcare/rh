use crate::traits::resource::ResourceExistence;
/// Executable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines an executable value set as one that SHALL have an expansion included, as well as a usage warning indicating the expansion is a point-in-time snapshot and must be maintained over time for production usage. The value set expansion specifies the timestamp when the expansion was produced, SHOULD contain the parameters used for the expansion, and SHALL contain the codes that are obtained by evaluating the value set definition. If this is ONLY an executable value set, a computable definition of the value set must be obtained to compute the updated expansion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/executablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ExecutablevaluesetAccessors {}
/// Executable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines an executable value set as one that SHALL have an expansion included, as well as a usage warning indicating the expansion is a point-in-time snapshot and must be maintained over time for production usage. The value set expansion specifies the timestamp when the expansion was produced, SHOULD contain the parameters used for the expansion, and SHALL contain the codes that are obtained by evaluating the value set definition. If this is ONLY an executable value set, a computable definition of the value set must be obtained to compute the updated expansion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/executablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ExecutablevaluesetMutators {
    /// Create a new Executablevalueset with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::executablevalueset::Executablevalueset;
    /// use rh_hl7_fhir_r5_core::traits::executablevalueset::ExecutablevaluesetMutators;
    ///
    /// let resource = Executablevalueset::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// executablevalueset Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines an executable value set as one that SHALL have an expansion included, as well as a usage warning indicating the expansion is a point-in-time snapshot and must be maintained over time for production usage. The value set expansion specifies the timestamp when the expansion was produced, SHOULD contain the parameters used for the expansion, and SHALL contain the codes that are obtained by evaluating the value set definition. If this is ONLY an executable value set, a computable definition of the value set must be obtained to compute the updated expansion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/executablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait ExecutablevaluesetExistence: ResourceExistence {}
