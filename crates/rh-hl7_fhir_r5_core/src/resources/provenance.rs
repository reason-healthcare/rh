use crate::bindings::provenance_entity_role::ProvenanceEntityRole;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::signature::Signature;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Provenance
///
/// Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Provenance
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Target Reference(s) (usually version specific)
    pub target: Vec<Reference>,
    /// When the activity occurred (Period)
    #[serde(rename = "occurredPeriod")]
    pub occurred_period: Option<Period>,
    /// When the activity occurred (dateTime)
    #[serde(rename = "occurredDateTime")]
    pub occurred_date_time: Option<DateTimeType>,
    /// When the activity was recorded / updated
    pub recorded: Option<InstantType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Policy or plan the activity was defined by
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<StringType>,
    /// Extension element for the 'policy' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _policy: Vec<Element>,
    /// Where the activity occurred, if relevant
    pub location: Option<Reference>,
    /// Authorization (purposeOfUse) related to the event
    ///
    /// Binding: example (The authorized purposeOfUse for the activity.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<CodeableReference>,
    /// Activity that occurred
    ///
    /// Binding: example (The activity that took place.)
    ///
    /// Available values:
    /// - `amend`
    /// - `originate`
    /// - `merge`
    /// - `deidentify`
    /// - `receive`
    /// - `transform`
    /// - `verify`
    pub activity: Option<CodeableConcept>,
    /// Workflow authorization within which this event occurred
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// The patient is the subject of the data created/updated (.target) by the activity
    pub patient: Option<Reference>,
    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<Reference>,
    /// Actor involved
    pub agent: Vec<ProvenanceAgent>,
    /// An entity used in this activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<ProvenanceEntity>,
    /// Signature on target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}
/// Provenance nested structure for the 'agent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceAgent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How the agent participated
    ///
    /// Binding: example (The type of participation that a provenance agent played with respect to the activity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participation-role-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// What the agents role was
    ///
    /// Binding: example (The role that a provenance agent played with respect to the activity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-role-type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<CodeableConcept>,
    /// The agent that participated in the event
    pub who: Reference,
    /// The agent that delegated
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
}
/// Provenance nested structure for the 'entity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// revision | quotation | source | instantiates | removal
    pub role: ProvenanceEntityRole,
    /// Extension element for the 'role' primitive field. Contains metadata and extensions.
    pub _role: Option<Element>,
    /// Identity of entity
    pub what: Reference,
    /// Entity is attributed to this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<StringType>,
}

impl Default for Provenance {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            target: Vec::new(),
            occurred_period: Default::default(),
            occurred_date_time: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            policy: Default::default(),
            _policy: Default::default(),
            location: Default::default(),
            authorization: Default::default(),
            activity: Default::default(),
            based_on: Default::default(),
            patient: Default::default(),
            encounter: Default::default(),
            agent: Vec::new(),
            entity: Default::default(),
            signature: Default::default(),
        }
    }
}

impl Default for ProvenanceAgent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            role: Default::default(),
            who: Reference::default(),
            on_behalf_of: Default::default(),
        }
    }
}

impl Default for ProvenanceEntity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: ProvenanceEntityRole::default(),
            _role: Default::default(),
            what: Reference::default(),
            agent: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("prov-1", rh_foundation::Severity::Error, "Who and onBehalfOf cannot be the same", "who.resolve().exists() and onBehalfOf.resolve().exists() implies who.resolve() != onBehalfOf.resolve()"),
    rh_foundation::Invariant::new("prov-2", rh_foundation::Severity::Error, "If who is a PractitionerRole, onBehalfOf can't reference the same Practitioner", "who.resolve().ofType(PractitionerRole).practitioner.resolve().exists() and onBehalfOf.resolve().ofType(Practitioner).exists() implies who.resolve().practitioner.resolve() != onBehalfOf.resolve()"),
    rh_foundation::Invariant::new("prov-3", rh_foundation::Severity::Error, "If who is an organization, onBehalfOf can't be a PractitionerRole within that organization", "who.resolve().ofType(Organization).exists() and onBehalfOf.resolve().ofType(PractitionerRole).organization.resolve().exists() implies who.resolve() != onBehalfOf.resolve().organization.resolve()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "Provenance.entity.role",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/provenance-entity-role|5.0.0",
            )
            .with_description("How an entity was used in an activity."),
            rh_foundation::ElementBinding::new(
                "Provenance.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Provenance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.contained", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.extension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.target", 1, None),
            rh_foundation::ElementCardinality::new("Provenance.occurred[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.recorded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.policy", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.authorization", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.activity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.patient", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.agent", 1, None),
            rh_foundation::ElementCardinality::new("Provenance.agent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.agent.extension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.agent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.agent.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.agent.role", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.agent.who", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.agent.onBehalfOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.entity", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.entity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.entity.extension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.entity.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.entity.role", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.entity.what", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Provenance.entity.agent", 0, None),
            rh_foundation::ElementCardinality::new("Provenance.signature", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Provenance {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for Provenance {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Provenance {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for Provenance {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for Provenance {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Provenance {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::provenance::ProvenanceAccessors for Provenance {
    fn target(&self) -> &[Reference] {
        &self.target
    }
    fn recorded(&self) -> Option<InstantType> {
        self.recorded.clone()
    }
    fn policy(&self) -> &[StringType] {
        self.policy.as_slice()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn authorization(&self) -> &[CodeableReference] {
        self.authorization.as_slice()
    }
    fn activity(&self) -> Option<CodeableConcept> {
        self.activity.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn agent(&self) -> &[ProvenanceAgent] {
        &self.agent
    }
    fn entity(&self) -> &[ProvenanceEntity] {
        self.entity.as_slice()
    }
    fn signature(&self) -> &[Signature] {
        self.signature.as_slice()
    }
}

impl crate::traits::provenance::ProvenanceMutators for Provenance {
    fn new() -> Self {
        Self::default()
    }
    fn set_target(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.target = value;
        resource
    }
    fn add_target(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.target.push(item);
        resource
    }
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
        resource
    }
    fn set_policy(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.policy = value;
        resource
    }
    fn add_policy(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.policy.push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_authorization(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.authorization = value;
        resource
    }
    fn add_authorization(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.authorization.push(item);
        resource
    }
    fn set_activity(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.activity = Some(value);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_agent(self, value: Vec<ProvenanceAgent>) -> Self {
        let mut resource = self.clone();
        resource.agent = value;
        resource
    }
    fn add_agent(self, item: ProvenanceAgent) -> Self {
        let mut resource = self.clone();
        resource.agent.push(item);
        resource
    }
    fn set_entity(self, value: Vec<ProvenanceEntity>) -> Self {
        let mut resource = self.clone();
        resource.entity = value;
        resource
    }
    fn add_entity(self, item: ProvenanceEntity) -> Self {
        let mut resource = self.clone();
        resource.entity.push(item);
        resource
    }
    fn set_signature(self, value: Vec<Signature>) -> Self {
        let mut resource = self.clone();
        resource.signature = value;
        resource
    }
    fn add_signature(self, item: Signature) -> Self {
        let mut resource = self.clone();
        resource.signature.push(item);
        resource
    }
}

impl crate::traits::provenance::ProvenanceExistence for Provenance {
    fn has_occurred(&self) -> bool {
        self.occurred_period.is_some() || self.occurred_date_time.is_some()
    }
    fn has_target(&self) -> bool {
        !self.target.is_empty()
    }
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
    }
    fn has_policy(&self) -> bool {
        !self.policy.is_empty()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_authorization(&self) -> bool {
        !self.authorization.is_empty()
    }
    fn has_activity(&self) -> bool {
        self.activity.is_some()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_agent(&self) -> bool {
        !self.agent.is_empty()
    }
    fn has_entity(&self) -> bool {
        !self.entity.is_empty()
    }
    fn has_signature(&self) -> bool {
        !self.signature.is_empty()
    }
}

impl crate::validation::ValidatableResource for Provenance {
    fn resource_type(&self) -> &'static str {
        "Provenance"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Provenance")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::provenance::{ProvenanceAccessors, ProvenanceExistence, ProvenanceMutators};
