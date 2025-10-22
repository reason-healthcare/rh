use crate::bindings::diagnostic_report_status::DiagnosticReportStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DiagnosticReport
///
/// The findings and interpretation of diagnostic  tests performed on patients, groups of patients, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting and provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for report
    pub identifier: Option<Vec<Identifier>>,
    /// What was requested
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// registered | partial | preliminary | final +
    pub status: DiagnosticReportStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Service category
    ///
    /// Binding: example (Codes for diagnostic service sections.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnostic-service-sections
    pub category: Option<Vec<CodeableConcept>>,
    /// Name/Code for this diagnostic report
    ///
    /// Binding: preferred (Codes that describe Diagnostic Reports.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/report-codes
    pub code: CodeableConcept,
    /// The subject of the report - usually, but not always, the patient
    pub subject: Option<Reference>,
    /// Health care event when test ordered
    pub encounter: Option<Reference>,
    /// Clinically relevant time/time-period for report (dateTime)
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<DateTimeType>,
    /// Clinically relevant time/time-period for report (Period)
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// DateTime this version was made
    pub issued: Option<InstantType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// Responsible Diagnostic Service
    pub performer: Option<Vec<Reference>>,
    /// Primary result interpreter
    #[serde(rename = "resultsInterpreter")]
    pub results_interpreter: Option<Vec<Reference>>,
    /// Specimens this report is based on
    pub specimen: Option<Vec<Reference>>,
    /// Observations
    pub result: Option<Vec<Reference>>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[serde(rename = "imagingStudy")]
    pub imaging_study: Option<Vec<Reference>>,
    /// Key images associated with this report
    pub media: Option<Vec<DiagnosticReportMedia>>,
    /// Clinical conclusion (interpretation) of test results
    pub conclusion: Option<StringType>,
    /// Extension element for the 'conclusion' primitive field. Contains metadata and extensions.
    pub _conclusion: Option<Element>,
    /// Codes for the clinical conclusion of test results
    ///
    /// Binding: example (Diagnosis codes provided as adjuncts to the report.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    #[serde(rename = "conclusionCode")]
    pub conclusion_code: Option<Vec<CodeableConcept>>,
    /// Entire report as issued
    #[serde(rename = "presentedForm")]
    pub presented_form: Option<Vec<Attachment>>,
}
/// Addendum Of
///
/// The supplements or provides additional information for the target report.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-addendumOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportAddendumOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Summary Of
///
/// A summary report that points to subordinate target reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-summaryOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportSummaryOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// DiagnosticReport nested structure for the 'media' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportMedia {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Comment about the image (e.g. explanation)
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Reference to the image source
    pub link: Reference,
}
/// FamilyMemberHistory
///
/// Significant health events and conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsFamilyMemberHistory
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsFamilyMemberHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// References
///
/// Additional bibliographic reference information about genetics, medications, clinical trials, etc. associated with knowledge-based information on genetics/genetic condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsReferences
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsReferences {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReport {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            status: DiagnosticReportStatus::default(),
            _status: Default::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            subject: Default::default(),
            encounter: Default::default(),
            effective_date_time: Default::default(),
            effective_period: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            performer: Default::default(),
            results_interpreter: Default::default(),
            specimen: Default::default(),
            result: Default::default(),
            imaging_study: Default::default(),
            media: Default::default(),
            conclusion: Default::default(),
            _conclusion: Default::default(),
            conclusion_code: Default::default(),
            presented_form: Default::default(),
        }
    }
}

impl Default for DiagnosticReportAddendumOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportSummaryOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportMedia {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            comment: Default::default(),
            _comment: Default::default(),
            link: Reference::default(),
        }
    }
}

impl Default for DiagnosticReportGeneticsFamilyMemberHistory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportGeneticsReferences {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DiagnosticReport {
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

impl crate::traits::resource::ResourceMutators for DiagnosticReport {
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

impl crate::traits::resource::ResourceExistence for DiagnosticReport {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DiagnosticReport {
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

impl crate::traits::domain_resource::DomainResourceMutators for DiagnosticReport {
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

impl crate::traits::domain_resource::DomainResourceExistence for DiagnosticReport {
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

impl crate::traits::diagnostic_report::DiagnosticReportAccessors for DiagnosticReport {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DiagnosticReportStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn issued(&self) -> Option<InstantType> {
        self.issued.clone()
    }
    fn performer(&self) -> &[Reference] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn results_interpreter(&self) -> &[Reference] {
        self.results_interpreter.as_deref().unwrap_or(&[])
    }
    fn specimen(&self) -> &[Reference] {
        self.specimen.as_deref().unwrap_or(&[])
    }
    fn result(&self) -> &[Reference] {
        self.result.as_deref().unwrap_or(&[])
    }
    fn imaging_study(&self) -> &[Reference] {
        self.imaging_study.as_deref().unwrap_or(&[])
    }
    fn media(&self) -> &[DiagnosticReportMedia] {
        self.media.as_deref().unwrap_or(&[])
    }
    fn conclusion(&self) -> Option<StringType> {
        self.conclusion.clone()
    }
    fn conclusion_code(&self) -> &[CodeableConcept] {
        self.conclusion_code.as_deref().unwrap_or(&[])
    }
    fn presented_form(&self) -> &[Attachment] {
        self.presented_form.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::diagnostic_report::DiagnosticReportMutators for DiagnosticReport {
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
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: DiagnosticReportStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_results_interpreter(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.results_interpreter = Some(value);
        resource
    }
    fn add_results_interpreter(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .results_interpreter
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specimen(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn add_specimen(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_result(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.result = Some(value);
        resource
    }
    fn add_result(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.result.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_imaging_study(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.imaging_study = Some(value);
        resource
    }
    fn add_imaging_study(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .imaging_study
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_media(self, value: Vec<DiagnosticReportMedia>) -> Self {
        let mut resource = self.clone();
        resource.media = Some(value);
        resource
    }
    fn add_media(self, item: DiagnosticReportMedia) -> Self {
        let mut resource = self.clone();
        resource.media.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_conclusion(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.conclusion = Some(value);
        resource
    }
    fn set_conclusion_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.conclusion_code = Some(value);
        resource
    }
    fn add_conclusion_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .conclusion_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_presented_form(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.presented_form = Some(value);
        resource
    }
    fn add_presented_form(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource
            .presented_form
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::diagnostic_report::DiagnosticReportExistence for DiagnosticReport {
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
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_results_interpreter(&self) -> bool {
        self.results_interpreter
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_specimen(&self) -> bool {
        self.specimen.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_result(&self) -> bool {
        self.result.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_imaging_study(&self) -> bool {
        self.imaging_study.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_media(&self) -> bool {
        self.media.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_conclusion(&self) -> bool {
        self.conclusion.is_some()
    }
    fn has_conclusion_code(&self) -> bool {
        self.conclusion_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_presented_form(&self) -> bool {
        self.presented_form.as_ref().is_some_and(|v| !v.is_empty())
    }
}
