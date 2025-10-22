use crate::bindings::audit_event_action::AuditEventAction;
use crate::bindings::audit_event_outcome::AuditEventOutcome;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::period::Period;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::audit_event::AuditEventAgent;
use crate::resources::audit_event::AuditEventEntity;
use crate::resources::audit_event::AuditEventSource;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AuditEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AuditEventAccessors: DomainResourceAccessors {
    /// Returns a reference to the type field.
    fn type_(&self) -> Coding;
    /// Returns a reference to the subtype field.
    fn subtype(&self) -> &[Coding];
    /// Returns a reference to the action field.
    fn action(&self) -> Option<AuditEventAction>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> InstantType;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<AuditEventOutcome>;
    /// Returns a reference to the outcomeDesc field.
    fn outcome_desc(&self) -> Option<StringType>;
    /// Returns a reference to the purposeOfEvent field.
    fn purpose_of_event(&self) -> &[CodeableConcept];
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
/// A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::audit_event::AuditEvent;
    /// use hl7_fhir_r4_core::traits::audit_event::AuditEventMutators;
    ///
    /// let resource = AuditEvent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Coding) -> Self;
    /// Sets the subtype field and returns self for chaining.
    fn set_subtype(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the subtype field and returns self for chaining.
    fn add_subtype(self, item: Coding) -> Self;
    /// Sets the action field and returns self for chaining.
    fn set_action(self, value: AuditEventAction) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: AuditEventOutcome) -> Self;
    /// Sets the outcomeDesc field and returns self for chaining.
    fn set_outcome_desc(self, value: String) -> Self;
    /// Sets the purposeOfEvent field and returns self for chaining.
    fn set_purpose_of_event(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the purposeOfEvent field and returns self for chaining.
    fn add_purpose_of_event(self, item: CodeableConcept) -> Self;
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
/// A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 4.0.1
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the subtype field is not empty.
    fn has_subtype(&self) -> bool;
    /// Returns true if the action field is present (Some).
    fn has_action(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the outcome_desc field is present (Some).
    fn has_outcome_desc(&self) -> bool;
    /// Returns true if the purpose_of_event field is not empty.
    fn has_purpose_of_event(&self) -> bool;
    /// Returns true if the agent field is not empty.
    fn has_agent(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the entity field is not empty.
    fn has_entity(&self) -> bool;
}
