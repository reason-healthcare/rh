use crate::bindings::list_mode::ListMode;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::report_relation_type::ReportRelationType;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::narrative::Narrative;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EvidenceReport
///
/// The EvidenceReport Resource is a specialized container for a collection of resources and codeable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EvidenceReport
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EvidenceReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReport {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this EvidenceReport, represented as a globally unique URI
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Unique identifier for the evidence report
    pub identifier: Option<Vec<Identifier>>,
    /// Identifiers for articles that may relate to more than one evidence report
    #[serde(rename = "relatedIdentifier")]
    pub related_identifier: Option<Vec<Identifier>>,
    /// Citation for this report (Reference)
    #[serde(rename = "citeAsReference")]
    pub cite_as_reference: Option<Reference>,
    /// Citation for this report (markdown)
    #[serde(rename = "citeAsMarkdown")]
    pub cite_as_markdown: Option<StringType>,
    /// Kind of report
    ///
    /// Binding: example (The kind of report, such as grouping of classifiers, search results, or human-compiled expression.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-report-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Used for footnotes and annotations
    pub note: Option<Vec<Annotation>>,
    /// Link, description or reference to artifact associated with the report
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Focus of the report
    pub subject: EvidenceReportSubject,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,
    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<EvidenceReportRelatesto>>,
    /// Composition is broken into sections
    pub section: Option<Vec<EvidenceReportSection>>,
}
/// EvidenceReport nested structure for the 'subject' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReportSubject {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Characteristic
    pub characteristic: Option<Vec<EvidenceReportSubjectCharacteristic>>,
    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,
}
/// EvidenceReport nested structure for the 'section' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReportSection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for section (e.g. for ToC)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Classification of section (recommended)
    ///
    /// Binding: extensible (Evidence Report Section Type.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-report-section
    pub focus: Option<CodeableConcept>,
    /// Classification of section by Resource
    #[serde(rename = "focusReference")]
    pub focus_reference: Option<Reference>,
    /// Who and/or what authored the section
    pub author: Option<Vec<Reference>>,
    /// Text summary of the section, for human interpretation
    pub text: Option<Narrative>,
    /// working | snapshot | changes
    pub mode: Option<ListMode>,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Order of section entries
    ///
    /// Binding: preferred (What order applies to the items in the entry.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-order
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,
    /// Extensible classifiers as content
    ///
    /// Binding: extensible (Commonly used classifiers for evidence sets.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-classifier-code
    #[serde(rename = "entryClassifier")]
    pub entry_classifier: Option<Vec<CodeableConcept>>,
    /// Reference to resources as content
    #[serde(rename = "entryReference")]
    pub entry_reference: Option<Vec<Reference>>,
    /// Quantity as content
    #[serde(rename = "entryQuantity")]
    pub entry_quantity: Option<Vec<Quantity>>,
    /// Why the section is empty
    ///
    /// Binding: preferred (If a section is empty, why it is empty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-empty-reason
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,
    /// Nested Section
    pub section: Option<Vec<StringType>>,
}
/// EvidenceReport nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReportRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Target of the relationship
    pub target: EvidenceReportRelatestoTarget,
    /// replaces | amends | appends | transforms | replacedWith | amendedWith | appendedWith | transformedWith
    pub code: ReportRelationType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
}
/// EvidenceReportRelatesto nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReportRelatestoTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Target of the relationship URL
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Target of the relationship Identifier
    pub identifier: Option<Identifier>,
    /// Target of the relationship Display
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Target of the relationship Resource reference
    pub resource: Option<Reference>,
}
/// EvidenceReportSubject nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReportSubjectCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Characteristic code
    ///
    /// Binding: extensible (Evidence focus characteristic code.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/focus-characteristic-code
    pub code: CodeableConcept,
    /// Characteristic value (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Characteristic value (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Characteristic value (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Characteristic value (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Characteristic value (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Is used to express not the characteristic
    pub exclude: Option<BooleanType>,
    /// Extension element for the 'exclude' primitive field. Contains metadata and extensions.
    pub _exclude: Option<Element>,
    /// Timeframe for the characteristic
    pub period: Option<Period>,
}

impl Default for EvidenceReport {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            use_context: Default::default(),
            identifier: Default::default(),
            related_identifier: Default::default(),
            cite_as_reference: Default::default(),
            cite_as_markdown: Default::default(),
            type_: Default::default(),
            note: Default::default(),
            related_artifact: Default::default(),
            subject: EvidenceReportSubject::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            relates_to: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for EvidenceReportSubject {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            characteristic: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for EvidenceReportSection {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            focus: Default::default(),
            focus_reference: Default::default(),
            author: Default::default(),
            text: Default::default(),
            mode: Default::default(),
            _mode: Default::default(),
            ordered_by: Default::default(),
            entry_classifier: Default::default(),
            entry_reference: Default::default(),
            entry_quantity: Default::default(),
            empty_reason: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for EvidenceReportRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target: Default::default(),
            code: Default::default(),
            _code: Default::default(),
        }
    }
}

impl Default for EvidenceReportRelatestoTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            resource: Default::default(),
        }
    }
}

impl Default for EvidenceReportSubjectCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value_reference: Default::default(),
            value_codeable_concept: Default::default(),
            value_boolean: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            exclude: Default::default(),
            _exclude: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
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
                "EvidenceReport.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "EvidenceReport.relatesTo.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-relation-type|5.0.0",
            )
            .with_description("The type of relationship between reports."),
            rh_foundation::ElementBinding::new(
                "EvidenceReport.section.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/list-mode|5.0.0",
            )
            .with_description("The processing mode that applies to this section."),
            rh_foundation::ElementBinding::new(
                "EvidenceReport.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EvidenceReport.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.contained", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.extension", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.useContext", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatedIdentifier", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.citeAs[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.note", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.subject.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.subject.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.exclude",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.subject.characteristic.period",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.subject.note", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.contact", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.author", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.editor", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.endorser", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatesTo", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatesTo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatesTo.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatesTo.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.relatesTo.target", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.url",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.identifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.relatesTo.target.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.section", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.section.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.focus", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.section.focusReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.author", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.orderedBy", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.section.entryClassifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.section.entryReference",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.entryQuantity", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceReport.section.emptyReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceReport.section.section", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EvidenceReport {
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

impl crate::traits::resource::ResourceMutators for EvidenceReport {
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

impl crate::traits::resource::ResourceExistence for EvidenceReport {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EvidenceReport {
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

impl crate::traits::domain_resource::DomainResourceMutators for EvidenceReport {
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

impl crate::traits::domain_resource::DomainResourceExistence for EvidenceReport {
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

impl crate::traits::evidence_report::EvidenceReportAccessors for EvidenceReport {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn related_identifier(&self) -> &[Identifier] {
        self.related_identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> EvidenceReportSubject {
        self.subject.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn author(&self) -> &[ContactDetail] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_deref().unwrap_or(&[])
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_deref().unwrap_or(&[])
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_deref().unwrap_or(&[])
    }
    fn relates_to(&self) -> &[EvidenceReportRelatesto] {
        self.relates_to.as_deref().unwrap_or(&[])
    }
    fn section(&self) -> &[EvidenceReportSection] {
        self.section.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::evidence_report::EvidenceReportMutators for EvidenceReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
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
    fn set_related_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.related_identifier = Some(value);
        resource
    }
    fn add_related_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource
            .related_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
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
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = Some(value);
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource
            .related_artifact
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject(self, value: EvidenceReportSubject) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = Some(value);
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = Some(value);
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = Some(value);
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_relates_to(self, value: Vec<EvidenceReportRelatesto>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = Some(value);
        resource
    }
    fn add_relates_to(self, item: EvidenceReportRelatesto) -> Self {
        let mut resource = self.clone();
        resource.relates_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_section(self, value: Vec<EvidenceReportSection>) -> Self {
        let mut resource = self.clone();
        resource.section = Some(value);
        resource
    }
    fn add_section(self, item: EvidenceReportSection) -> Self {
        let mut resource = self.clone();
        resource.section.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::evidence_report::EvidenceReportExistence for EvidenceReport {
    fn has_cite_as(&self) -> bool {
        self.cite_as_reference.is_some() || self.cite_as_markdown.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_identifier(&self) -> bool {
        self.related_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_artifact(&self) -> bool {
        self.related_artifact
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_editor(&self) -> bool {
        self.editor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reviewer(&self) -> bool {
        self.reviewer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endorser(&self) -> bool {
        self.endorser.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_relates_to(&self) -> bool {
        self.relates_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_section(&self) -> bool {
        self.section.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for EvidenceReport {
    fn resource_type(&self) -> &'static str {
        "EvidenceReport"
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
        Some("http://hl7.org/fhir/StructureDefinition/EvidenceReport")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::evidence_report::{
    EvidenceReportAccessors, EvidenceReportExistence, EvidenceReportMutators,
};
