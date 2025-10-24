use crate::bindings::provenance_entity_role::ProvenanceEntityRole;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
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
/// - Version: 4.0.1
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
    pub recorded: InstantType,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Policy or plan the activity was defined by
    pub policy: Option<Vec<StringType>>,
    /// Extension element for the 'policy' primitive field. Contains metadata and extensions.
    pub _policy: Option<Element>,
    /// Where the activity occurred, if relevant
    pub location: Option<Reference>,
    /// Reason the activity is occurring
    ///
    /// Binding: extensible (The reason the activity took place.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub reason: Option<Vec<CodeableConcept>>,
    /// Activity that occurred
    ///
    /// Binding: extensible (The activity that took place.)
    ///
    /// Available values:
    /// - `LA`
    /// - `ANONY`
    /// - `DEID`
    /// - `MASK`
    /// - `LABEL`
    /// - `PSEUD`
    /// - `CREATE`
    /// - `DELETE`
    /// - `UPDATE`
    /// - `APPEND`
    /// - ... and 1 more values
    pub activity: Option<CodeableConcept>,
    /// Actor involved
    pub agent: Vec<ProvenanceAgent>,
    /// An entity used in this activity
    pub entity: Option<Vec<ProvenanceEntity>>,
    /// Signature on target
    pub signature: Option<Vec<Signature>>,
}
/// Provenance nested structure for the 'entity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// derivation | revision | quotation | source | removal
    pub role: ProvenanceEntityRole,
    /// Extension element for the 'role' primitive field. Contains metadata and extensions.
    pub _role: Option<Element>,
    /// Identity of entity
    pub what: Reference,
    /// Entity is attributed to this agent
    pub agent: Option<Vec<StringType>>,
}
/// Provenance nested structure for the 'agent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceAgent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How the agent participated
    ///
    /// Binding: extensible (The type of participation that a provenance agent played with respect to the activity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/provenance-agent-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// What the agents role was
    ///
    /// Binding: example (The role that a provenance agent played with respect to the activity.)
    ///
    /// Available values:
    /// - `AMENDER`
    /// - `COAUTH`
    /// - `CONT`
    /// - `EVTWIT`
    /// - `PRIMAUTH`
    /// - `REVIEWER`
    /// - `SOURCE`
    /// - `TRANS`
    /// - `VALID`
    /// - `VERF`
    /// - ... and 53 more values
    pub role: Option<Vec<CodeableConcept>>,
    /// Who participated
    pub who: Reference,
    /// Who the agent is representing
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
}

impl Default for Provenance {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            target: Vec::new(),
            occurred_period: Default::default(),
            occurred_date_time: Default::default(),
            recorded: InstantType::default(),
            _recorded: Default::default(),
            policy: Default::default(),
            _policy: Default::default(),
            location: Default::default(),
            reason: Default::default(),
            activity: Default::default(),
            agent: Vec::new(),
            entity: Default::default(),
            signature: Default::default(),
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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Provenance {
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
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::provenance::ProvenanceAccessors for Provenance {
    fn target(&self) -> &[Reference] {
        &self.target
    }
    fn recorded(&self) -> InstantType {
        self.recorded.clone()
    }
    fn policy(&self) -> &[StringType] {
        self.policy.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason(&self) -> &[CodeableConcept] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn activity(&self) -> Option<CodeableConcept> {
        self.activity.clone()
    }
    fn agent(&self) -> &[ProvenanceAgent] {
        &self.agent
    }
    fn entity(&self) -> &[ProvenanceEntity] {
        self.entity.as_deref().unwrap_or(&[])
    }
    fn signature(&self) -> &[Signature] {
        self.signature.as_deref().unwrap_or(&[])
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
        resource.recorded = value;
        resource
    }
    fn set_policy(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.policy = Some(value);
        resource
    }
    fn add_policy(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.policy.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_activity(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.activity = Some(value);
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
        resource.entity = Some(value);
        resource
    }
    fn add_entity(self, item: ProvenanceEntity) -> Self {
        let mut resource = self.clone();
        resource.entity.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_signature(self, value: Vec<Signature>) -> Self {
        let mut resource = self.clone();
        resource.signature = Some(value);
        resource
    }
    fn add_signature(self, item: Signature) -> Self {
        let mut resource = self.clone();
        resource.signature.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::provenance::ProvenanceExistence for Provenance {
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
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_occurred(&self) -> bool {
        self.occurred_period.is_some() || self.occurred_date_time.is_some()
    }
    fn has_target(&self) -> bool {
        !self.target.is_empty()
    }
    fn has_recorded(&self) -> bool {
        true
    }
    fn has_policy(&self) -> bool {
        self.policy.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_activity(&self) -> bool {
        self.activity.is_some()
    }
    fn has_agent(&self) -> bool {
        !self.agent.is_empty()
    }
    fn has_entity(&self) -> bool {
        self.entity.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_signature(&self) -> bool {
        self.signature.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Provenance {
    fn resource_type(&self) -> &'static str {
        "Provenance"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Provenance")
    }
}
