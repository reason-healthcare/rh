use crate::bindings::detectedissue_severity::DetectedissueSeverity;
use crate::bindings::observation_status::ObservationStatus;
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
/// DetectedIssue
///
/// Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DetectedIssue
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIssue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique id for the detected issue
    pub identifier: Option<Vec<Identifier>>,
    /// registered | preliminary | final | amended +
    pub status: ObservationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    ///
    /// Binding: preferred (Codes identifying the general type of detected issue; e.g. Drug-drug interaction, Timing issue, Duplicate therapy, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/detectedissue-category
    pub code: Option<CodeableConcept>,
    /// high | moderate | low
    pub severity: Option<DetectedissueSeverity>,
    /// Extension element for the 'severity' primitive field. Contains metadata and extensions.
    pub _severity: Option<Element>,
    /// Associated patient
    pub patient: Option<Reference>,
    /// When identified (dateTime)
    #[serde(rename = "identifiedDateTime")]
    pub identified_date_time: Option<DateTimeType>,
    /// When identified (Period)
    #[serde(rename = "identifiedPeriod")]
    pub identified_period: Option<Period>,
    /// The provider or device that identified the issue
    pub author: Option<Reference>,
    /// Problem resource
    pub implicated: Option<Vec<Reference>>,
    /// Supporting evidence
    pub evidence: Option<Vec<DetectedIssueEvidence>>,
    /// Description and context
    pub detail: Option<StringType>,
    /// Extension element for the 'detail' primitive field. Contains metadata and extensions.
    pub _detail: Option<Element>,
    /// Authority for issue
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Step taken to address
    pub mitigation: Option<Vec<DetectedIssueMitigation>>,
}
/// DetectedIssue nested structure for the 'evidence' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIssueEvidence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Manifestation
    ///
    /// Binding: example (Codes that describes the types of evidence for a detected issue.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/manifestation-or-symptom
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting information
    pub detail: Option<Vec<Reference>>,
}
/// DetectedIssue nested structure for the 'mitigation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIssueMitigation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What mitigation?
    ///
    /// Binding: preferred (Codes describing steps taken to resolve the issue or other circumstances that mitigate the risk associated with the issue; e.g. 'added concurrent therapy', 'prior therapy documented', etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/detectedissue-mitigation-action
    pub action: CodeableConcept,
    /// Date committed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who is committing?
    pub author: Option<Reference>,
}

impl Default for DetectedIssue {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ObservationStatus::default(),
            _status: Default::default(),
            code: Default::default(),
            severity: Default::default(),
            _severity: Default::default(),
            patient: Default::default(),
            identified_date_time: Default::default(),
            identified_period: Default::default(),
            author: Default::default(),
            implicated: Default::default(),
            evidence: Default::default(),
            detail: Default::default(),
            _detail: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
            mitigation: Default::default(),
        }
    }
}

impl Default for DetectedIssueEvidence {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            detail: Default::default(),
        }
    }
}

impl Default for DetectedIssueMitigation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: CodeableConcept::default(),
            date: Default::default(),
            _date: Default::default(),
            author: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DetectedIssue {
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

impl crate::traits::resource::ResourceMutators for DetectedIssue {
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

impl crate::traits::resource::ResourceExistence for DetectedIssue {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DetectedIssue {
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

impl crate::traits::domain_resource::DomainResourceMutators for DetectedIssue {
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

impl crate::traits::domain_resource::DomainResourceExistence for DetectedIssue {
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

impl crate::traits::detected_issue::DetectedIssueAccessors for DetectedIssue {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ObservationStatus {
        self.status.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn severity(&self) -> Option<DetectedissueSeverity> {
        self.severity.clone()
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn implicated(&self) -> &[Reference] {
        self.implicated.as_deref().unwrap_or(&[])
    }
    fn evidence(&self) -> &[DetectedIssueEvidence] {
        self.evidence.as_deref().unwrap_or(&[])
    }
    fn detail(&self) -> Option<StringType> {
        self.detail.clone()
    }
    fn reference(&self) -> Option<StringType> {
        self.reference.clone()
    }
    fn mitigation(&self) -> &[DetectedIssueMitigation] {
        self.mitigation.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::detected_issue::DetectedIssueMutators for DetectedIssue {
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
    fn set_status(self, value: ObservationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_severity(self, value: DetectedissueSeverity) -> Self {
        let mut resource = self.clone();
        resource.severity = Some(value);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_implicated(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.implicated = Some(value);
        resource
    }
    fn add_implicated(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.implicated.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_evidence(self, value: Vec<DetectedIssueEvidence>) -> Self {
        let mut resource = self.clone();
        resource.evidence = Some(value);
        resource
    }
    fn add_evidence(self, item: DetectedIssueEvidence) -> Self {
        let mut resource = self.clone();
        resource.evidence.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_detail(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.detail = Some(value);
        resource
    }
    fn set_reference(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.reference = Some(value);
        resource
    }
    fn set_mitigation(self, value: Vec<DetectedIssueMitigation>) -> Self {
        let mut resource = self.clone();
        resource.mitigation = Some(value);
        resource
    }
    fn add_mitigation(self, item: DetectedIssueMitigation) -> Self {
        let mut resource = self.clone();
        resource.mitigation.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::detected_issue::DetectedIssueExistence for DetectedIssue {
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
    fn has_identified(&self) -> bool {
        self.identified_date_time.is_some() || self.identified_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_severity(&self) -> bool {
        self.severity.is_some()
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_implicated(&self) -> bool {
        self.implicated.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_evidence(&self) -> bool {
        self.evidence.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_detail(&self) -> bool {
        self.detail.is_some()
    }
    fn has_reference(&self) -> bool {
        self.reference.is_some()
    }
    fn has_mitigation(&self) -> bool {
        self.mitigation.as_ref().is_some_and(|v| !v.is_empty())
    }
}
