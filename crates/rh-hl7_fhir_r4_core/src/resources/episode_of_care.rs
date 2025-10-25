use crate::bindings::episode_of_care_status::EpisodeOfCareStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::positive_int::PositiveIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EpisodeOfCare
///
/// An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EpisodeOfCare
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeOfCare {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier(s) relevant for this EpisodeOfCare
    pub identifier: Option<Vec<Identifier>>,
    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: EpisodeOfCareStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Past list of status codes (the current status may be included to cover the start date of the status)
    #[serde(rename = "statusHistory")]
    pub status_history: Option<Vec<EpisodeOfCareStatushistory>>,
    /// Type/class  - e.g. specialist referral, disease management
    ///
    /// Binding: example (The type of the episode of care.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/episodeofcare-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The list of diagnosis relevant to this episode of care
    pub diagnosis: Option<Vec<EpisodeOfCareDiagnosis>>,
    /// The patient who is the focus of this episode of care
    pub patient: Reference,
    /// Organization that assumes care
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Interval during responsibility is assumed
    pub period: Option<Period>,
    /// Originating Referral Request(s)
    #[serde(rename = "referralRequest")]
    pub referral_request: Option<Vec<Reference>>,
    /// Care manager/care coordinator for the patient
    #[serde(rename = "careManager")]
    pub care_manager: Option<Reference>,
    /// Other practitioners facilitating this episode of care
    pub team: Option<Vec<Reference>>,
    /// The set of accounts that may be used for billing for this EpisodeOfCare
    pub account: Option<Vec<Reference>>,
}
/// EpisodeOfCare nested structure for the 'statusHistory' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeOfCareStatushistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: EpisodeOfCareStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Duration the EpisodeOfCare was in the specified status
    pub period: Period,
}
/// EpisodeOfCare nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeOfCareDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Conditions/problems/diagnoses this episode of care is for
    pub condition: Reference,
    /// Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)
    ///
    /// Binding: preferred (The type of diagnosis this condition represents.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnosis-role
    pub role: Option<CodeableConcept>,
    /// Ranking of the diagnosis (for each role type)
    pub rank: Option<PositiveIntType>,
    /// Extension element for the 'rank' primitive field. Contains metadata and extensions.
    pub _rank: Option<Element>,
}

impl Default for EpisodeOfCare {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: EpisodeOfCareStatus::default(),
            _status: Default::default(),
            status_history: Default::default(),
            type_: Default::default(),
            diagnosis: Default::default(),
            patient: Reference::default(),
            managing_organization: Default::default(),
            period: Default::default(),
            referral_request: Default::default(),
            care_manager: Default::default(),
            team: Default::default(),
            account: Default::default(),
        }
    }
}

impl Default for EpisodeOfCareStatushistory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            status: Default::default(),
            _status: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for EpisodeOfCareDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            condition: Reference::default(),
            role: Default::default(),
            rank: Default::default(),
            _rank: Default::default(),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "EpisodeOfCare.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/episode-of-care-status|4.0.1",
            )
            .with_description("The status of the episode of care."),
            rh_foundation::ElementBinding::new(
                "EpisodeOfCare.statusHistory.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/episode-of-care-status|4.0.1",
            )
            .with_description("The status of the episode of care."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EpisodeOfCare.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.contained", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.extension", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.statusHistory", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.statusHistory.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.statusHistory.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.statusHistory.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.statusHistory.status",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.statusHistory.period",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.type", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.diagnosis.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis.condition", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.diagnosis.rank", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EpisodeOfCare.managingOrganization",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.referralRequest", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.careManager", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.team", 0, None),
            rh_foundation::ElementCardinality::new("EpisodeOfCare.account", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EpisodeOfCare {
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

impl crate::traits::resource::ResourceMutators for EpisodeOfCare {
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

impl crate::traits::resource::ResourceExistence for EpisodeOfCare {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EpisodeOfCare {
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

impl crate::traits::domain_resource::DomainResourceMutators for EpisodeOfCare {
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

impl crate::traits::domain_resource::DomainResourceExistence for EpisodeOfCare {
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

impl crate::traits::episode_of_care::EpisodeOfCareAccessors for EpisodeOfCare {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EpisodeOfCareStatus {
        self.status.clone()
    }
    fn status_history(&self) -> &[EpisodeOfCareStatushistory] {
        self.status_history.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn diagnosis(&self) -> &[EpisodeOfCareDiagnosis] {
        self.diagnosis.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn referral_request(&self) -> &[Reference] {
        self.referral_request.as_deref().unwrap_or(&[])
    }
    fn care_manager(&self) -> Option<Reference> {
        self.care_manager.clone()
    }
    fn team(&self) -> &[Reference] {
        self.team.as_deref().unwrap_or(&[])
    }
    fn account(&self) -> &[Reference] {
        self.account.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::episode_of_care::EpisodeOfCareMutators for EpisodeOfCare {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: EpisodeOfCareStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_history(self, value: Vec<EpisodeOfCareStatushistory>) -> Self {
        let mut resource = self.clone();
        resource.status_history = Some(value);
        resource
    }
    fn add_status_history(self, item: EpisodeOfCareStatushistory) -> Self {
        let mut resource = self.clone();
        resource
            .status_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_diagnosis(self, value: Vec<EpisodeOfCareDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = Some(value);
        resource
    }
    fn add_diagnosis(self, item: EpisodeOfCareDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_referral_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.referral_request = Some(value);
        resource
    }
    fn add_referral_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .referral_request
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_care_manager(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.care_manager = Some(value);
        resource
    }
    fn set_team(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.team = Some(value);
        resource
    }
    fn add_team(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.team.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_account(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.account = Some(value);
        resource
    }
    fn add_account(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.account.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::episode_of_care::EpisodeOfCareExistence for EpisodeOfCare {
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_history(&self) -> bool {
        self.status_history.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_diagnosis(&self) -> bool {
        self.diagnosis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_referral_request(&self) -> bool {
        self.referral_request
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_care_manager(&self) -> bool {
        self.care_manager.is_some()
    }
    fn has_team(&self) -> bool {
        self.team.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_account(&self) -> bool {
        self.account.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for EpisodeOfCare {
    fn resource_type(&self) -> &'static str {
        "EpisodeOfCare"
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
        Some("http://hl7.org/fhir/StructureDefinition/EpisodeOfCare")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::episode_of_care::{
    EpisodeOfCareAccessors, EpisodeOfCareExistence, EpisodeOfCareMutators,
};
