use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ResearchSubject
///
/// A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 5.0.0
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
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Subject status
    pub progress: Option<Vec<ResearchSubjectProgress>>,
    /// Start and end of participation
    pub period: Option<Period>,
    /// Study subject is part of
    pub study: Reference,
    /// Who or what is part of study
    pub subject: Reference,
    /// What path should be followed
    #[serde(rename = "assignedComparisonGroup")]
    pub assigned_comparison_group: Option<StringType>,
    /// Extension element for the 'assignedComparisonGroup' primitive field. Contains metadata and extensions.
    #[serde(rename = "_assignedComparisonGroup")]
    pub _assigned_comparison_group: Option<Element>,
    /// What path was followed
    #[serde(rename = "actualComparisonGroup")]
    pub actual_comparison_group: Option<StringType>,
    /// Extension element for the 'actualComparisonGroup' primitive field. Contains metadata and extensions.
    #[serde(rename = "_actualComparisonGroup")]
    pub _actual_comparison_group: Option<Element>,
    /// Agreement to participate in study
    pub consent: Option<Vec<Reference>>,
}
/// ResearchSubject nested structure for the 'progress' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSubjectProgress {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// state | milestone
    ///
    /// Binding: example (Identifies the kind of state being refered to.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-subject-state-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    #[serde(rename = "subjectState")]
    pub subject_state: Option<CodeableConcept>,
    /// SignedUp | Screened | Randomized
    ///
    /// Binding: example (Indicates the progression of a study subject through the study milestones.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/research-subject-milestone
    pub milestone: Option<CodeableConcept>,
    /// State change reason
    ///
    /// Binding: example (Indicates why the state of the subject changed.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/state-change-reason
    pub reason: Option<CodeableConcept>,
    /// State change date
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTimeType>,
    /// Extension element for the 'startDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_startDate")]
    pub _start_date: Option<Element>,
    /// State change date
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTimeType>,
    /// Extension element for the 'endDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_endDate")]
    pub _end_date: Option<Element>,
}

impl Default for ResearchSubject {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            progress: Default::default(),
            period: Default::default(),
            study: Reference::default(),
            subject: Reference::default(),
            assigned_comparison_group: Default::default(),
            _assigned_comparison_group: Default::default(),
            actual_comparison_group: Default::default(),
            _actual_comparison_group: Default::default(),
            consent: Default::default(),
        }
    }
}

impl Default for ResearchSubjectProgress {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            subject_state: Default::default(),
            milestone: Default::default(),
            reason: Default::default(),
            start_date: Default::default(),
            _start_date: Default::default(),
            end_date: Default::default(),
            _end_date: Default::default(),
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
                "ResearchSubject.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ResearchSubject.progress.subjectState",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/research-subject-state|5.0.0",
            )
            .with_description("Indicates the progression of a study subject through a study."),
            rh_foundation::ElementBinding::new(
                "ResearchSubject.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description(
                "Codes that convey the current publication status of the research study resource.",
            ),
        ]
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
            rh_foundation::ElementCardinality::new("ResearchSubject.progress", 0, None),
            rh_foundation::ElementCardinality::new("ResearchSubject.progress.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.progress.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.progress.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchSubject.progress.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.progress.subjectState",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.progress.milestone",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchSubject.progress.reason", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.progress.startDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchSubject.progress.endDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.study", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchSubject.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.assignedComparisonGroup",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchSubject.actualComparisonGroup",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchSubject.consent", 0, None),
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
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn progress(&self) -> &[ResearchSubjectProgress] {
        self.progress.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn study(&self) -> Reference {
        self.study.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn assigned_comparison_group(&self) -> Option<StringType> {
        self.assigned_comparison_group.clone()
    }
    fn actual_comparison_group(&self) -> Option<StringType> {
        self.actual_comparison_group.clone()
    }
    fn consent(&self) -> &[Reference] {
        self.consent.as_deref().unwrap_or(&[])
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
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_progress(self, value: Vec<ResearchSubjectProgress>) -> Self {
        let mut resource = self.clone();
        resource.progress = Some(value);
        resource
    }
    fn add_progress(self, item: ResearchSubjectProgress) -> Self {
        let mut resource = self.clone();
        resource.progress.get_or_insert_with(Vec::new).push(item);
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
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_assigned_comparison_group(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.assigned_comparison_group = Some(value);
        resource
    }
    fn set_actual_comparison_group(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.actual_comparison_group = Some(value);
        resource
    }
    fn set_consent(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.consent = Some(value);
        resource
    }
    fn add_consent(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.consent.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::research_subject::ResearchSubjectExistence for ResearchSubject {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_progress(&self) -> bool {
        self.progress.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_study(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_assigned_comparison_group(&self) -> bool {
        self.assigned_comparison_group.is_some()
    }
    fn has_actual_comparison_group(&self) -> bool {
        self.actual_comparison_group.is_some()
    }
    fn has_consent(&self) -> bool {
        self.consent.as_ref().is_some_and(|v| !v.is_empty())
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
