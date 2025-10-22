use crate::traits::resource::ResourceExistence;
/// Family member history for genetics analysis Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Adds additional information to a family member history supporting both the capture of mother/father relationships as well as additional observations necessary to enable genetics-based risk analysis for patients
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-genetic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
pub trait FamilymemberhistoryGeneticAccessors {}
/// Family member history for genetics analysis Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Adds additional information to a family member history supporting both the capture of mother/father relationships as well as additional observations necessary to enable genetics-based risk analysis for patients
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-genetic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
pub trait FamilymemberhistoryGeneticMutators {
    /// Create a new FamilymemberhistoryGenetic with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::familymemberhistory_genetic::FamilymemberhistoryGenetic;
    /// use hl7_fhir_r4_core::traits::familymemberhistory_genetic::FamilymemberhistoryGeneticMutators;
    ///
    /// let resource = FamilymemberhistoryGenetic::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// familymemberhistory-genetic Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Adds additional information to a family member history supporting both the capture of mother/father relationships as well as additional observations necessary to enable genetics-based risk analysis for patients
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-genetic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
pub trait FamilymemberhistoryGeneticExistence: ResourceExistence {}
