use crate::bindings::audit_event_action::AuditEventAction;
use crate::bindings::audit_event_severity::AuditEventSeverity;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::resources::audit_event::AuditEventAgent;
use crate::resources::audit_event::AuditEventEntity;
use crate::resources::audit_event::AuditEventOutcome;
use crate::resources::audit_event::AuditEventSource;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AuditEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AuditEventAccessors: DomainResourceAccessors {
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the action field.
    fn action(&self) -> Option<AuditEventAction>;
    /// Returns a reference to the severity field.
    fn severity(&self) -> Option<AuditEventSeverity>;
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> InstantType;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<AuditEventOutcome>;
    /// Returns a reference to the authorization field.
    fn authorization(&self) -> &[CodeableConcept];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the agent field.
    fn agent(&self) -> &[AuditEventAgent];
    /// Returns a reference to the source field.
    fn source(&self) -> AuditEventSource;
    /// Returns a reference to the entity field.
    fn entity(&self) -> &[AuditEventEntity];
}
/// AuditEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AuditEventMutators: DomainResourceMutators {
    /// Create a new AuditEvent with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::audit_event::AuditEvent;
    /// use rh_hl7_fhir_r5_core::traits::audit_event::AuditEventMutators;
    ///
    /// let resource = AuditEvent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the action field and returns self for chaining.
    fn set_action(self, value: AuditEventAction) -> Self;
    /// Sets the severity field and returns self for chaining.
    fn set_severity(self, value: AuditEventSeverity) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: AuditEventOutcome) -> Self;
    /// Sets the authorization field and returns self for chaining.
    fn set_authorization(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the authorization field and returns self for chaining.
    fn add_authorization(self, item: CodeableConcept) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the agent field and returns self for chaining.
    fn set_agent(self, value: Vec<AuditEventAgent>) -> Self;
    /// Adds an item to the agent field and returns self for chaining.
    fn add_agent(self, item: AuditEventAgent) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: AuditEventSource) -> Self;
    /// Sets the entity field and returns self for chaining.
    fn set_entity(self, value: Vec<AuditEventEntity>) -> Self;
    /// Adds an item to the entity field and returns self for chaining.
    fn add_entity(self, item: AuditEventEntity) -> Self;
}
/// AuditEvent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AuditEventExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the action field is present (Some).
    fn has_action(&self) -> bool;
    /// Returns true if the severity field is present (Some).
    fn has_severity(&self) -> bool;
    /// Returns true if the occurred field is present (Some).
    fn has_occurred(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the authorization field is not empty.
    fn has_authorization(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the agent field is not empty.
    fn has_agent(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the entity field is not empty.
    fn has_entity(&self) -> bool;
}
