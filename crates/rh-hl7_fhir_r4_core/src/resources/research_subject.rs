use crate::bindings::research_subject_status::ResearchSubjectStatus;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ResearchSubject
///
/// A physical entity which is the primary unit of operational and/or administrative interest in a study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSubject {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for research subject in a study
    pub identifier: Option<Vec<Identifier>>,
    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    pub status: ResearchSubjectStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Start and end of participation
    pub period: Option<Period>,
    /// Study subject is part of
    pub study: Reference,
    /// Who is part of study
    pub individual: Reference,
    /// What path should be followed
    #[serde(rename = "assignedArm")]
    pub assigned_arm: Option<StringType>,
    /// Extension element for the 'assignedArm' primitive field. Contains metadata and extensions.
    #[serde(rename = "_assignedArm")]
    pub _assigned_arm: Option<Element>,
    /// What path was followed
    #[serde(rename = "actualArm")]
    pub actual_arm: Option<StringType>,
    /// Extension element for the 'actualArm' primitive field. Contains metadata and extensions.
    #[serde(rename = "_actualArm")]
    pub _actual_arm: Option<Element>,
    /// Agreement to participate in study
    pub consent: Option<Reference>,
}

impl Default for ResearchSubject {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ResearchSubjectStatus::default(),
            _status: Default::default(),
            period: Default::default(),
            study: Reference::default(),
            individual: Reference::default(),
            assigned_arm: Default::default(),
            _assigned_arm: Default::default(),
            actual_arm: Default::default(),
            _actual_arm: Default::default(),
            consent: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ResearchSubject {
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

impl crate::traits::resource::ResourceMutators for ResearchSubject {
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

impl crate::traits::resource::ResourceExistence for ResearchSubject {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ResearchSubject {
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

impl crate::traits::domain_resource::DomainResourceMutators for ResearchSubject {
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

impl crate::traits::domain_resource::DomainResourceExistence for ResearchSubject {
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

impl crate::traits::research_subject::ResearchSubjectAccessors for ResearchSubject {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ResearchSubjectStatus {
        self.status.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn study(&self) -> Reference {
        self.study.clone()
    }
    fn individual(&self) -> Reference {
        self.individual.clone()
    }
    fn assigned_arm(&self) -> Option<StringType> {
        self.assigned_arm.clone()
    }
    fn actual_arm(&self) -> Option<StringType> {
        self.actual_arm.clone()
    }
    fn consent(&self) -> Option<Reference> {
        self.consent.clone()
    }
}

impl crate::traits::research_subject::ResearchSubjectMutators for ResearchSubject {
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
    fn set_status(self, value: ResearchSubjectStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_study(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.study = value;
        resource
    }
    fn set_individual(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.individual = value;
        resource
    }
    fn set_assigned_arm(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.assigned_arm = Some(value);
        resource
    }
    fn set_actual_arm(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.actual_arm = Some(value);
        resource
    }
    fn set_consent(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.consent = Some(value);
        resource
    }
}

impl crate::traits::research_subject::ResearchSubjectExistence for ResearchSubject {
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
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_study(&self) -> bool {
        true
    }
    fn has_individual(&self) -> bool {
        true
    }
    fn has_assigned_arm(&self) -> bool {
        self.assigned_arm.is_some()
    }
    fn has_actual_arm(&self) -> bool {
        self.actual_arm.is_some()
    }
    fn has_consent(&self) -> bool {
        self.consent.is_some()
    }
}
