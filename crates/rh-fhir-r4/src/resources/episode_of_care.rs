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
