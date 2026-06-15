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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// What was requested
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// registered | partial | preliminary | final +
    pub status: DiagnosticReportStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Service category
    ///
    /// Binding: example (Codes for diagnostic service sections.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnostic-service-sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<Reference>,
    /// Primary result interpreter
    #[serde(rename = "resultsInterpreter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results_interpreter: Vec<Reference>,
    /// Specimens this report is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<Reference>,
    /// Observations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<Reference>,
    /// Reference to full details of imaging associated with the diagnostic report
    #[serde(rename = "imagingStudy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imaging_study: Vec<Reference>,
    /// Key images associated with this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<DiagnosticReportMedia>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conclusion_code: Vec<CodeableConcept>,
    /// Entire report as issued
    #[serde(rename = "presentedForm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presented_form: Vec<Attachment>,
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
/// Extends
///
/// The report references related ("sibling") reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-extends
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportExtends {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Analysis
///
/// Knowledge-based comments on the effect of the sequence on patient's condition/medication reaction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsAnalysis
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsAnalysis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// AssessedCondition
///
/// Used to denote condition context for genetic testing, which may influence reported variants and interpretation for large genomic testing panels e.g. lung cancer or familial breast cancer.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsAssessedCondition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsAssessedCondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
/// locationPerformed
///
/// Facility location where this report was prepared.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-locationPerformed
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportLocationPerformed {
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
/// Replaces
///
/// The report replaces the target resource.  For example, when a final anatomic pathology report replaces a preliminary anatomic pathology report replaces  where the subsequent observation of case and report  may be on more or different material (specimen).  Note that  this is not same concept as` DiagnosticReport.status`  = preliminary of final, but industry definition of preliminary and final.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Risk
///
/// Provides a link to an assessment of prognosis or risk as informed by the diagnostic results (For example, genetic results and possibly by patient genetic family history information).  This extension is used when need RiskAssessment as an alternate choice  for `Observation.hasMember` or `DiagnosticReport.result`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-risk
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportRisk {
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

impl Default for DiagnosticReportExtends {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportGeneticsAnalysis {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportGeneticsAssessedCondition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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

impl Default for DiagnosticReportLocationPerformed {
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

impl Default for DiagnosticReportReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DiagnosticReportRisk {
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
            "DiagnosticReport.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/diagnostic-report-status|4.0.1",
        )
        .with_description("The status of the diagnostic report.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DiagnosticReport.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.contained", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.extension", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.category", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.effective[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.performer", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.resultsInterpreter", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.specimen", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.result", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.imagingStudy", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.media", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.media.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.media.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.media.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DiagnosticReport.media.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.media.link", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.conclusion", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DiagnosticReport.conclusionCode", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.presentedForm", 0, None),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for DiagnosticReport {
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

impl crate::traits::diagnostic_report::DiagnosticReportAccessors for DiagnosticReport {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn status(&self) -> DiagnosticReportStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
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
        self.performer.as_slice()
    }
    fn results_interpreter(&self) -> &[Reference] {
        self.results_interpreter.as_slice()
    }
    fn specimen(&self) -> &[Reference] {
        self.specimen.as_slice()
    }
    fn result(&self) -> &[Reference] {
        self.result.as_slice()
    }
    fn imaging_study(&self) -> &[Reference] {
        self.imaging_study.as_slice()
    }
    fn media(&self) -> &[DiagnosticReportMedia] {
        self.media.as_slice()
    }
    fn conclusion(&self) -> Option<StringType> {
        self.conclusion.clone()
    }
    fn conclusion_code(&self) -> &[CodeableConcept] {
        self.conclusion_code.as_slice()
    }
    fn presented_form(&self) -> &[Attachment] {
        self.presented_form.as_slice()
    }
}

impl crate::traits::diagnostic_report::DiagnosticReportMutators for DiagnosticReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
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
    fn set_status(self, value: DiagnosticReportStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
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
        resource.performer = value;
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.push(item);
        resource
    }
    fn set_results_interpreter(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.results_interpreter = value;
        resource
    }
    fn add_results_interpreter(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.results_interpreter.push(item);
        resource
    }
    fn set_specimen(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen = value;
        resource
    }
    fn add_specimen(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen.push(item);
        resource
    }
    fn set_result(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.result = value;
        resource
    }
    fn add_result(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.result.push(item);
        resource
    }
    fn set_imaging_study(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.imaging_study = value;
        resource
    }
    fn add_imaging_study(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.imaging_study.push(item);
        resource
    }
    fn set_media(self, value: Vec<DiagnosticReportMedia>) -> Self {
        let mut resource = self.clone();
        resource.media = value;
        resource
    }
    fn add_media(self, item: DiagnosticReportMedia) -> Self {
        let mut resource = self.clone();
        resource.media.push(item);
        resource
    }
    fn set_conclusion(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.conclusion = Some(value);
        resource
    }
    fn set_conclusion_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.conclusion_code = value;
        resource
    }
    fn add_conclusion_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.conclusion_code.push(item);
        resource
    }
    fn set_presented_form(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.presented_form = value;
        resource
    }
    fn add_presented_form(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.presented_form.push(item);
        resource
    }
}

impl crate::traits::diagnostic_report::DiagnosticReportExistence for DiagnosticReport {
    fn has_effective(&self) -> bool {
        self.effective_date_time.is_some() || self.effective_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
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
        !self.performer.is_empty()
    }
    fn has_results_interpreter(&self) -> bool {
        !self.results_interpreter.is_empty()
    }
    fn has_specimen(&self) -> bool {
        !self.specimen.is_empty()
    }
    fn has_result(&self) -> bool {
        !self.result.is_empty()
    }
    fn has_imaging_study(&self) -> bool {
        !self.imaging_study.is_empty()
    }
    fn has_media(&self) -> bool {
        !self.media.is_empty()
    }
    fn has_conclusion(&self) -> bool {
        self.conclusion.is_some()
    }
    fn has_conclusion_code(&self) -> bool {
        !self.conclusion_code.is_empty()
    }
    fn has_presented_form(&self) -> bool {
        !self.presented_form.is_empty()
    }
}

impl crate::validation::ValidatableResource for DiagnosticReport {
    fn resource_type(&self) -> &'static str {
        "DiagnosticReport"
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
        Some("http://hl7.org/fhir/StructureDefinition/DiagnosticReport")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::diagnostic_report::{
    DiagnosticReportAccessors, DiagnosticReportExistence, DiagnosticReportMutators,
};
