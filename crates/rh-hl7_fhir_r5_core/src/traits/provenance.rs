use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::reference::Reference;
use crate::datatypes::signature::Signature;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::provenance::ProvenanceAgent;
use crate::resources::provenance::ProvenanceEntity;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Provenance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Provenance
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProvenanceAccessors: DomainResourceAccessors {
    /// Returns a reference to the target field.
    fn target(&self) -> &[Reference];
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> Option<InstantType>;
    /// Returns a reference to the policy field.
    fn policy(&self) -> &[StringType];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the authorization field.
    fn authorization(&self) -> &[CodeableReference];
    /// Returns a reference to the activity field.
    fn activity(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the agent field.
    fn agent(&self) -> &[ProvenanceAgent];
    /// Returns a reference to the entity field.
    fn entity(&self) -> &[ProvenanceEntity];
    /// Returns a reference to the signature field.
    fn signature(&self) -> &[Signature];
}
/// Provenance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Provenance
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProvenanceMutators: DomainResourceMutators {
    /// Create a new Provenance with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::provenance::Provenance;
    /// use rh_hl7_fhir_r5_core::traits::provenance::ProvenanceMutators;
    ///
    /// let resource = Provenance::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the target field and returns self for chaining.
    fn set_target(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the target field and returns self for chaining.
    fn add_target(self, item: Reference) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the policy field and returns self for chaining.
    fn set_policy(self, value: Vec<String>) -> Self;
    /// Adds an item to the policy field and returns self for chaining.
    fn add_policy(self, item: String) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the authorization field and returns self for chaining.
    fn set_authorization(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the authorization field and returns self for chaining.
    fn add_authorization(self, item: CodeableReference) -> Self;
    /// Sets the activity field and returns self for chaining.
    fn set_activity(self, value: CodeableConcept) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the agent field and returns self for chaining.
    fn set_agent(self, value: Vec<ProvenanceAgent>) -> Self;
    /// Adds an item to the agent field and returns self for chaining.
    fn add_agent(self, item: ProvenanceAgent) -> Self;
    /// Sets the entity field and returns self for chaining.
    fn set_entity(self, value: Vec<ProvenanceEntity>) -> Self;
    /// Adds an item to the entity field and returns self for chaining.
    fn add_entity(self, item: ProvenanceEntity) -> Self;
    /// Sets the signature field and returns self for chaining.
    fn set_signature(self, value: Vec<Signature>) -> Self;
    /// Adds an item to the signature field and returns self for chaining.
    fn add_signature(self, item: Signature) -> Self;
}
/// Provenance Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Provenance
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProvenanceExistence: DomainResourceExistence {
    /// Returns true if the target field is not empty.
    fn has_target(&self) -> bool;
    /// Returns true if the occurred field is present (Some).
    fn has_occurred(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the policy field is not empty.
    fn has_policy(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the authorization field is not empty.
    fn has_authorization(&self) -> bool;
    /// Returns true if the activity field is present (Some).
    fn has_activity(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the agent field is not empty.
    fn has_agent(&self) -> bool;
    /// Returns true if the entity field is not empty.
    fn has_entity(&self) -> bool;
    /// Returns true if the signature field is not empty.
    fn has_signature(&self) -> bool;
}
