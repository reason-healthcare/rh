use crate::bindings::clinicalimpression_status::ClinicalimpressionStatus;
use crate::datatypes::annotation::Annotation;
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
/// ClinicalImpression
///
/// A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion with the recording of assessment tools such as Apgar score.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClinicalImpression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalImpression {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// in-progress | completed | entered-in-error
    pub status: ClinicalimpressionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Codes identifying the reason for the current state of a clinical impression.)
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Kind of assessment performed
    ///
    /// Binding: example (Identifies categories of clinical impressions.  This is a place-holder only.  It may be removed.)
    pub code: Option<CodeableConcept>,
    /// Why/how the assessment was performed
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Patient or group assessed
    pub subject: Reference,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// Time of assessment (dateTime)
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<DateTimeType>,
    /// Time of assessment (Period)
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// When the assessment was documented
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The clinician performing the assessment
    pub assessor: Option<Reference>,
    /// Reference to last assessment
    pub previous: Option<Reference>,
    /// Relevant impressions of patient state
    pub problem: Option<Vec<Reference>>,
    /// One or more sets of investigations (signs, symptoms, etc.)
    pub investigation: Option<Vec<ClinicalImpressionInvestigation>>,
    /// Clinical Protocol followed
    pub protocol: Option<Vec<StringType>>,
    /// Extension element for the 'protocol' primitive field. Contains metadata and extensions.
    pub _protocol: Option<Element>,
    /// Summary of the assessment
    pub summary: Option<StringType>,
    /// Extension element for the 'summary' primitive field. Contains metadata and extensions.
    pub _summary: Option<Element>,
    /// Possible or likely findings and diagnoses
    pub finding: Option<Vec<ClinicalImpressionFinding>>,
    /// Estimate of likely outcome
    ///
    /// Binding: example (Prognosis or outlook findings.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinicalimpression-prognosis
    #[serde(rename = "prognosisCodeableConcept")]
    pub prognosis_codeable_concept: Option<Vec<CodeableConcept>>,
    /// RiskAssessment expressing likely outcome
    #[serde(rename = "prognosisReference")]
    pub prognosis_reference: Option<Vec<Reference>>,
    /// Information supporting the clinical impression
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Comments made about the ClinicalImpression
    pub note: Option<Vec<Annotation>>,
}
/// ClinicalImpression nested structure for the 'finding' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalImpressionFinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What was found
    ///
    /// Binding: example (Identification of the Condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: Option<CodeableConcept>,
    /// What was found
    #[serde(rename = "itemReference")]
    pub item_reference: Option<Reference>,
    /// Which investigations support finding
    pub basis: Option<StringType>,
    /// Extension element for the 'basis' primitive field. Contains metadata and extensions.
    pub _basis: Option<Element>,
}
/// ClinicalImpression nested structure for the 'investigation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalImpressionInvestigation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A name/code for the set
    ///
    /// Binding: example (A name/code for a set of investigations.)
    ///
    /// Available values:
    /// - `271336007`: Examination / signs
    /// - `160237006`: History/symptoms
    pub code: CodeableConcept,
    /// Record of a specific investigation
    pub item: Option<Vec<Reference>>,
}

impl Default for ClinicalImpression {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ClinicalimpressionStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            effective_date_time: Default::default(),
            effective_period: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            assessor: Default::default(),
            previous: Default::default(),
            problem: Default::default(),
            investigation: Default::default(),
            protocol: Default::default(),
            _protocol: Default::default(),
            summary: Default::default(),
            _summary: Default::default(),
            finding: Default::default(),
            prognosis_codeable_concept: Default::default(),
            prognosis_reference: Default::default(),
            supporting_info: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for ClinicalImpressionFinding {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_codeable_concept: Default::default(),
            item_reference: Default::default(),
            basis: Default::default(),
            _basis: Default::default(),
        }
    }
}

impl Default for ClinicalImpressionInvestigation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            item: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for ClinicalImpression {
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

impl crate::traits::resource::ResourceMutators for ClinicalImpression {
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

impl crate::traits::resource::ResourceExistence for ClinicalImpression {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ClinicalImpression {
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

impl crate::traits::domain_resource::DomainResourceMutators for ClinicalImpression {
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

impl crate::traits::domain_resource::DomainResourceExistence for ClinicalImpression {
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

impl crate::traits::clinical_impression::ClinicalImpressionAccessors for ClinicalImpression {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ClinicalimpressionStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn assessor(&self) -> Option<Reference> {
        self.assessor.clone()
    }
    fn previous(&self) -> Option<Reference> {
        self.previous.clone()
    }
    fn problem(&self) -> &[Reference] {
        self.problem.as_deref().unwrap_or(&[])
    }
    fn investigation(&self) -> &[ClinicalImpressionInvestigation] {
        self.investigation.as_deref().unwrap_or(&[])
    }
    fn protocol(&self) -> &[StringType] {
        self.protocol.as_deref().unwrap_or(&[])
    }
    fn summary(&self) -> Option<StringType> {
        self.summary.clone()
    }
    fn finding(&self) -> &[ClinicalImpressionFinding] {
        self.finding.as_deref().unwrap_or(&[])
    }
    fn prognosis_codeable_concept(&self) -> &[CodeableConcept] {
        self.prognosis_codeable_concept.as_deref().unwrap_or(&[])
    }
    fn prognosis_reference(&self) -> &[Reference] {
        self.prognosis_reference.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::clinical_impression::ClinicalImpressionMutators for ClinicalImpression {
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
    fn set_status(self, value: ClinicalimpressionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_assessor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.assessor = Some(value);
        resource
    }
    fn set_previous(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.previous = Some(value);
        resource
    }
    fn set_problem(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.problem = Some(value);
        resource
    }
    fn add_problem(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.problem.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_investigation(self, value: Vec<ClinicalImpressionInvestigation>) -> Self {
        let mut resource = self.clone();
        resource.investigation = Some(value);
        resource
    }
    fn add_investigation(self, item: ClinicalImpressionInvestigation) -> Self {
        let mut resource = self.clone();
        resource
            .investigation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_protocol(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.protocol = Some(value);
        resource
    }
    fn add_protocol(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.protocol.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_summary(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.summary = Some(value);
        resource
    }
    fn set_finding(self, value: Vec<ClinicalImpressionFinding>) -> Self {
        let mut resource = self.clone();
        resource.finding = Some(value);
        resource
    }
    fn add_finding(self, item: ClinicalImpressionFinding) -> Self {
        let mut resource = self.clone();
        resource.finding.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_prognosis_codeable_concept(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.prognosis_codeable_concept = Some(value);
        resource
    }
    fn add_prognosis_codeable_concept(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .prognosis_codeable_concept
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_prognosis_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.prognosis_reference = Some(value);
        resource
    }
    fn add_prognosis_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .prognosis_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::clinical_impression::ClinicalImpressionExistence for ClinicalImpression {
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
    fn has_effective(&self) -> bool {
        self.effective_date_time.is_some() || self.effective_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_assessor(&self) -> bool {
        self.assessor.is_some()
    }
    fn has_previous(&self) -> bool {
        self.previous.is_some()
    }
    fn has_problem(&self) -> bool {
        self.problem.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_investigation(&self) -> bool {
        self.investigation.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_protocol(&self) -> bool {
        self.protocol.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_summary(&self) -> bool {
        self.summary.is_some()
    }
    fn has_finding(&self) -> bool {
        self.finding.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_prognosis_codeable_concept(&self) -> bool {
        self.prognosis_codeable_concept
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_prognosis_reference(&self) -> bool {
        self.prognosis_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ClinicalImpression {
    fn resource_type(&self) -> &'static str {
        "ClinicalImpression"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ClinicalImpression")
    }
}
