use crate::traits::resource::ResourceExistence;
/// EHRS FM Record Lifecycle Event - Audit Event Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the elements to be supported within the AuditEvent resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-auditevent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/AuditEvent
pub trait EhrsrleAuditeventAccessors {}
/// EHRS FM Record Lifecycle Event - Audit Event Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the elements to be supported within the AuditEvent resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-auditevent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/AuditEvent
pub trait EhrsrleAuditeventMutators {
    /// Create a new EhrsrleAuditevent with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::ehrsrle_auditevent::EhrsrleAuditevent;
    /// use hl7_fhir_r4_core::traits::ehrsrle_auditevent::EhrsrleAuditeventMutators;
    ///
    /// let resource = EhrsrleAuditevent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// ehrsrle-auditevent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines the elements to be supported within the AuditEvent resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-auditevent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/AuditEvent
pub trait EhrsrleAuditeventExistence: ResourceExistence {}
