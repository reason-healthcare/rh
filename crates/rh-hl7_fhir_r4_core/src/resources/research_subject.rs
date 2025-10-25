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
        vec![rh_foundation::ElementBinding::new(
            "ResearchSubject.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/research-subject-status|4.0.1",
        )
        .with_description("Indicates the progression of a study subject through a study.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ResearchSubject.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.contained", 0, None),
            rh_foundation::ElementCardinality::new("ResearchSubject.extension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchSubject.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ResearchSubject.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ResearchSubject.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.study", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.individual", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.assignedArm", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.actualArm", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.consent", 0, Some(1)),
        ]
    });

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

impl crate::validation::ValidatableResource for ResearchSubject {
    fn resource_type(&self) -> &'static str {
        "ResearchSubject"
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
        Some("http://hl7.org/fhir/StructureDefinition/ResearchSubject")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::research_subject::{
    ResearchSubjectAccessors, ResearchSubjectExistence, ResearchSubjectMutators,
};
