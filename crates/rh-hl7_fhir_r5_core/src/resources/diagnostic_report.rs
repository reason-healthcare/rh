use crate::bindings::diagnostic_report_status::DiagnosticReportStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
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
/// The findings and interpretation of diagnostic tests performed on patients, groups of patients, products, substances, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports. The report also includes non-clinical context such as batch analysis and stability reporting of products and substances.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
/// - Version: 5.0.0
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
    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown
    pub status: DiagnosticReportStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Service category
    ///
    /// Binding: example (HL7 V2 table 0074)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnostic-service-sections
    pub category: Option<Vec<CodeableConcept>>,
    /// Name/Code for this diagnostic report
    ///
    /// Binding: preferred (LOINC Codes for Diagnostic Reports)
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
    /// Comments about the diagnostic report
    pub note: Option<Vec<Annotation>>,
    /// Reference to full details of an analysis associated with the diagnostic report
    pub study: Option<Vec<Reference>>,
    /// Additional information supporting the diagnostic report
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<DiagnosticReportSupportinginfo>>,
    /// Key images or data associated with this report
    pub media: Option<Vec<DiagnosticReportMedia>>,
    /// Reference to a Composition resource for the DiagnosticReport structure
    pub composition: Option<Reference>,
    /// Clinical conclusion (interpretation) of test results
    pub conclusion: Option<StringType>,
    /// Extension element for the 'conclusion' primitive field. Contains metadata and extensions.
    pub _conclusion: Option<Element>,
    /// Codes for the clinical conclusion of test results
    ///
    /// Binding: example (SNOMED CT Clinical Findings)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    #[serde(rename = "conclusionCode")]
    pub conclusion_code: Option<Vec<CodeableConcept>>,
    /// Entire report as issued
    #[serde(rename = "presentedForm")]
    pub presented_form: Option<Vec<Attachment>>,
}
/// DiagnosticReport nested structure for the 'media' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportMedia {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Comment about the image or data (e.g. explanation)
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Reference to the image or data source
    pub link: Reference,
}
/// DiagnosticReport nested structure for the 'supportingInfo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportSupportinginfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Supporting information role code
    ///
    /// Binding: example (The code value for the role of the supporting information in the diagnostic report.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0936
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Supporting information reference
    pub reference: Reference,
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
            note: Default::default(),
            study: Default::default(),
            supporting_info: Default::default(),
            media: Default::default(),
            composition: Default::default(),
            conclusion: Default::default(),
            _conclusion: Default::default(),
            conclusion_code: Default::default(),
            presented_form: Default::default(),
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

impl Default for DiagnosticReportSupportinginfo {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            reference: Default::default(),
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
    rh_foundation::Invariant::new("dgr-1", rh_foundation::Severity::Error, "When a Composition is referenced in `Diagnostic.composition`, all Observation resources referenced in `Composition.entry` must also be referenced in `Diagnostic.entry` or in the references Observations in `Observation.hasMember`", "composition.exists() implies (composition.resolve().section.entry.reference.where(resolve() is Observation) in (result.reference|result.reference.resolve().hasMember.reference))"),
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
                "DiagnosticReport.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DiagnosticReport.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/diagnostic-report-status|5.0.0",
            )
            .with_description("The status of the diagnostic report."),
        ]
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
            rh_foundation::ElementCardinality::new("DiagnosticReport.note", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.study", 0, None),
            rh_foundation::ElementCardinality::new("DiagnosticReport.supportingInfo", 0, None),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.supportingInfo.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.supportingInfo.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.supportingInfo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.supportingInfo.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DiagnosticReport.supportingInfo.reference",
                1,
                Some(1),
            ),
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
            rh_foundation::ElementCardinality::new("DiagnosticReport.composition", 0, Some(1)),
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
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn study(&self) -> &[Reference] {
        self.study.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[DiagnosticReportSupportinginfo] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn media(&self) -> &[DiagnosticReportMedia] {
        self.media.as_deref().unwrap_or(&[])
    }
    fn composition(&self) -> Option<Reference> {
        self.composition.clone()
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
    fn set_study(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.study = Some(value);
        resource
    }
    fn add_study(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.study.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<DiagnosticReportSupportinginfo>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: DiagnosticReportSupportinginfo) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
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
    fn set_composition(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.composition = Some(value);
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
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_study(&self) -> bool {
        self.study.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_media(&self) -> bool {
        self.media.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_composition(&self) -> bool {
        self.composition.is_some()
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
