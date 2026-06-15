use crate::bindings::composition_attestation_mode::CompositionAttestationMode;
use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_relationship_type::DocumentRelationshipType;
use crate::bindings::list_mode::ListMode;
use crate::bindings::v3confidentiality_classification::V3ConfidentialityClassification;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::narrative::Narrative;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Composition
///
/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Composition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Version-independent identifier for the Composition
    pub identifier: Option<Identifier>,
    /// preliminary | final | amended | entered-in-error
    pub status: CompositionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Kind of composition (LOINC if possible)
    ///
    /// Binding: preferred (Type of a composition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/doc-typecodes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Categorization of Composition
    ///
    /// Binding: example (High-level kind of a clinical document at a macro level.)
    ///
    /// Available values:
    /// - `11369-6`: History of Immunization
    /// - `11485-0`: Anesthesia records
    /// - `11486-8`: Chemotherapy records
    /// - `11488-4`: Consult Note
    /// - `11506-3`: Provider-unspecified progress note
    /// - `11543-6`: Nursery records
    /// - `15508-5`: Labor and delivery records
    /// - `18726-0`: Radiology studies (set)
    /// - `18761-7`: Provider-unspecified transfer summary
    /// - `18842-5`: Discharge summary
    /// - ... and 35 more values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// Who and/or what the composition is about
    pub subject: Option<Reference>,
    /// Context of the Composition
    pub encounter: Option<Reference>,
    /// Composition editing time
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who and/or what authored the composition
    pub author: Vec<Reference>,
    /// Human Readable name/title
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// As defined by affinity domain
    pub confidentiality: Option<V3ConfidentialityClassification>,
    /// Extension element for the 'confidentiality' primitive field. Contains metadata and extensions.
    pub _confidentiality: Option<Element>,
    /// Attests to accuracy of composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<CompositionAttester>,
    /// Organization which maintains the composition
    pub custodian: Option<Reference>,
    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<CompositionRelatesto>,
    /// The clinical service(s) being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CompositionEvent>,
    /// Composition is broken into sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<CompositionSection>,
}
/// otherConfidentiality
///
/// Carries additional confidentiality codes beyond the base fixed code specified in the CDA document.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-clinicaldocument-otherConfidentiality
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionClinicaldocumentOtherConfidentiality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Overrides Composition.subject
///
/// Specifies that the section has a different subject that the Composition, or it's container section.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-section-subject
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionSectionSubject {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Composition nested structure for the 'section' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionSection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for section (e.g. for ToC)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Classification of section (recommended)
    ///
    /// Binding: example (Classification of a section of a composition/document.)
    ///
    /// Available values:
    /// - `10154-3`
    /// - `10157-6`
    /// - `10160-0`
    /// - `10164-2`
    /// - `10183-2`
    /// - `10184-0`
    /// - `10187-3`
    /// - `10210-3`
    /// - `10216-0`
    /// - `10218-6`
    /// - ... and 47 more values
    pub code: Option<CodeableConcept>,
    /// Who and/or what authored the section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<Reference>,
    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<Reference>,
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
    /// A reference to data that supports this section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<Reference>,
    /// Why the section is empty
    ///
    /// Binding: preferred (If a section is empty, why it is empty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-empty-reason
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,
    /// Nested Section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<StringType>,
}
/// Composition nested structure for the 'event' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code(s) that apply to the event being documented
    ///
    /// Binding: example (This list of codes represents the main clinical acts being documented.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<CodeableConcept>,
    /// The period covered by the documentation
    pub period: Option<Period>,
    /// The event(s) being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<Reference>,
}
/// Composition nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// replaces | transforms | signs | appends
    pub code: DocumentRelationshipType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Target of the relationship (Identifier)
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: Identifier,
    /// Target of the relationship (Reference)
    #[serde(rename = "targetReference")]
    pub target_reference: Reference,
}
/// versionNumber
///
/// Version specific identifier for the composition, assigned when each version is created/updated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-clinicaldocument-versionNumber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionClinicaldocumentVersionNumber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Composition nested structure for the 'attester' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionAttester {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// personal | professional | legal | official
    pub mode: CompositionAttestationMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// When the composition was attested
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// Who attested the composition
    pub party: Option<Reference>,
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: CompositionStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            author: Vec::new(),
            title: StringType::default(),
            _title: Default::default(),
            confidentiality: Default::default(),
            _confidentiality: Default::default(),
            attester: Default::default(),
            custodian: Default::default(),
            relates_to: Default::default(),
            event: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for CompositionClinicaldocumentOtherConfidentiality {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for CompositionSectionSubject {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for CompositionSection {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            code: Default::default(),
            author: Default::default(),
            focus: Default::default(),
            text: Default::default(),
            mode: Default::default(),
            _mode: Default::default(),
            ordered_by: Default::default(),
            entry: Default::default(),
            empty_reason: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for CompositionEvent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            period: Default::default(),
            detail: Default::default(),
        }
    }
}

impl Default for CompositionRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            target_identifier: Default::default(),
            target_reference: Default::default(),
        }
    }
}

impl Default for CompositionClinicaldocumentVersionNumber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for CompositionAttester {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: CompositionAttestationMode::default(),
            _mode: Default::default(),
            time: Default::default(),
            _time: Default::default(),
            party: Default::default(),
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
    rh_foundation::Invariant::new("cmp-1", rh_foundation::Severity::Error, "A section must contain at least one of text, entries, or sub-sections", "text.exists() or entry.exists() or section.exists()").with_xpath("exists(f:text) or exists(f:entry) or exists(f:section)"),
    rh_foundation::Invariant::new("cmp-2", rh_foundation::Severity::Error, "A section can only have an emptyReason if it is empty", "emptyReason.empty() or entry.empty()").with_xpath("not(exists(f:emptyReason) and exists(f:entry))"),
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
        vec![
            rh_foundation::ElementBinding::new(
                "Composition.attester.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/composition-attestation-mode|4.0.1",
            )
            .with_description("The way in which a person authenticated a composition."),
            rh_foundation::ElementBinding::new(
                "Composition.confidentiality",
                rh_foundation::BindingStrength::Required,
                "http://terminology.hl7.org/ValueSet/v3-ConfidentialityClassification|2014-03-26",
            )
            .with_description("Codes specifying the level of confidentiality of the composition."),
            rh_foundation::ElementBinding::new(
                "Composition.relatesTo.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/document-relationship-type|4.0.1",
            )
            .with_description("The type of relationship between documents."),
            rh_foundation::ElementBinding::new(
                "Composition.section.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/list-mode|4.0.1",
            )
            .with_description("The processing mode that applies to this section."),
            rh_foundation::ElementBinding::new(
                "Composition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/composition-status|4.0.1",
            )
            .with_description("The workflow/clinical status of the composition."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Composition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.contained", 0, None),
            rh_foundation::ElementCardinality::new("Composition.extension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.category", 0, None),
            rh_foundation::ElementCardinality::new("Composition.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.author", 1, None),
            rh_foundation::ElementCardinality::new("Composition.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.confidentiality", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester", 0, None),
            rh_foundation::ElementCardinality::new("Composition.attester.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Composition.attester.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Composition.attester.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.time", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.attester.party", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.custodian", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.relatesTo", 0, None),
            rh_foundation::ElementCardinality::new("Composition.relatesTo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.relatesTo.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Composition.relatesTo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Composition.relatesTo.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.relatesTo.target[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event.extension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.code", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event.detail", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Composition.section.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Composition.section.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.author", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.focus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.orderedBy", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.entry", 0, None),
            rh_foundation::ElementCardinality::new("Composition.section.emptyReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.section.section", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Composition {
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

impl crate::traits::resource::ResourceMutators for Composition {
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

impl crate::traits::resource::ResourceExistence for Composition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Composition {
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

impl crate::traits::domain_resource::DomainResourceMutators for Composition {
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

impl crate::traits::domain_resource::DomainResourceExistence for Composition {
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

impl crate::traits::composition::CompositionAccessors for Composition {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn status(&self) -> CompositionStatus {
        self.status.clone()
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn author(&self) -> &[Reference] {
        &self.author
    }
    fn title(&self) -> StringType {
        self.title.clone()
    }
    fn confidentiality(&self) -> Option<V3ConfidentialityClassification> {
        self.confidentiality.clone()
    }
    fn attester(&self) -> &[CompositionAttester] {
        self.attester.as_slice()
    }
    fn custodian(&self) -> Option<Reference> {
        self.custodian.clone()
    }
    fn relates_to(&self) -> &[CompositionRelatesto] {
        self.relates_to.as_slice()
    }
    fn event(&self) -> &[CompositionEvent] {
        self.event.as_slice()
    }
    fn section(&self) -> &[CompositionSection] {
        self.section.as_slice()
    }
}

impl crate::traits::composition::CompositionMutators for Composition {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_status(self, value: CompositionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
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
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
        resource
    }
    fn set_author(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.author = value;
        resource
    }
    fn add_author(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.author.push(item);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = value;
        resource
    }
    fn set_confidentiality(self, value: V3ConfidentialityClassification) -> Self {
        let mut resource = self.clone();
        resource.confidentiality = Some(value);
        resource
    }
    fn set_attester(self, value: Vec<CompositionAttester>) -> Self {
        let mut resource = self.clone();
        resource.attester = value;
        resource
    }
    fn add_attester(self, item: CompositionAttester) -> Self {
        let mut resource = self.clone();
        resource.attester.push(item);
        resource
    }
    fn set_custodian(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.custodian = Some(value);
        resource
    }
    fn set_relates_to(self, value: Vec<CompositionRelatesto>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = value;
        resource
    }
    fn add_relates_to(self, item: CompositionRelatesto) -> Self {
        let mut resource = self.clone();
        resource.relates_to.push(item);
        resource
    }
    fn set_event(self, value: Vec<CompositionEvent>) -> Self {
        let mut resource = self.clone();
        resource.event = value;
        resource
    }
    fn add_event(self, item: CompositionEvent) -> Self {
        let mut resource = self.clone();
        resource.event.push(item);
        resource
    }
    fn set_section(self, value: Vec<CompositionSection>) -> Self {
        let mut resource = self.clone();
        resource.section = value;
        resource
    }
    fn add_section(self, item: CompositionSection) -> Self {
        let mut resource = self.clone();
        resource.section.push(item);
        resource
    }
}

impl crate::traits::composition::CompositionExistence for Composition {
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_title(&self) -> bool {
        true
    }
    fn has_confidentiality(&self) -> bool {
        self.confidentiality.is_some()
    }
    fn has_attester(&self) -> bool {
        !self.attester.is_empty()
    }
    fn has_custodian(&self) -> bool {
        self.custodian.is_some()
    }
    fn has_relates_to(&self) -> bool {
        !self.relates_to.is_empty()
    }
    fn has_event(&self) -> bool {
        !self.event.is_empty()
    }
    fn has_section(&self) -> bool {
        !self.section.is_empty()
    }
}

impl crate::validation::ValidatableResource for Composition {
    fn resource_type(&self) -> &'static str {
        "Composition"
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
        Some("http://hl7.org/fhir/StructureDefinition/Composition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::composition::{
    CompositionAccessors, CompositionExistence, CompositionMutators,
};
