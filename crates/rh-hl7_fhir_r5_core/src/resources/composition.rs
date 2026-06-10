use crate::bindings::composition_status::CompositionStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::narrative::Narrative;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this Composition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Version-independent identifier for the Composition
    pub identifier: Option<Vec<Identifier>>,
    /// An explicitly assigned identifer of a variation of the content in the Composition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
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
    /// ValueSet: http://hl7.org/fhir/ValueSet/referenced-item-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Who and/or what the composition is about
    pub subject: Option<Vec<Reference>>,
    /// Context of the Composition
    pub encounter: Option<Reference>,
    /// Composition editing time
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Who and/or what authored the composition
    pub author: Vec<Reference>,
    /// Name for this Composition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Human Readable name/title
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// For any additional notes
    pub note: Option<Vec<Annotation>>,
    /// Attests to accuracy of composition
    pub attester: Option<Vec<CompositionAttester>>,
    /// Organization which maintains the composition
    pub custodian: Option<Reference>,
    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<RelatedArtifact>>,
    /// The clinical service(s) being documented
    pub event: Option<Vec<CompositionEvent>>,
    /// Composition is broken into sections
    pub section: Option<Vec<CompositionSection>>,
}
/// Composition nested structure for the 'attester' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionAttester {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// personal | professional | legal | official
    ///
    /// Binding: preferred (The way in which a person authenticated a composition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/composition-attestation-mode
    pub mode: CodeableConcept,
    /// When the composition was attested
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// Who attested the composition
    pub party: Option<Reference>,
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
    pub author: Option<Vec<Reference>>,
    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<Reference>,
    /// Text summary of the section, for human interpretation
    pub text: Option<Narrative>,
    /// Order of section entries
    ///
    /// Binding: preferred (What order applies to the items in the entry.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-order
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,
    /// A reference to data that supports this section
    pub entry: Option<Vec<Reference>>,
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
/// Composition nested structure for the 'event' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The period covered by the documentation
    pub period: Option<Period>,
    /// The event(s) being documented, as code(s), reference(s), or both
    ///
    /// Binding: example (This list of codes represents the main clinical acts being documented.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    pub detail: Option<Vec<CodeableReference>>,
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            status: CompositionStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            use_context: Default::default(),
            author: Vec::new(),
            name: Default::default(),
            _name: Default::default(),
            title: StringType::default(),
            _title: Default::default(),
            note: Default::default(),
            attester: Default::default(),
            custodian: Default::default(),
            relates_to: Default::default(),
            event: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for CompositionAttester {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: CodeableConcept::default(),
            time: Default::default(),
            _time: Default::default(),
            party: Default::default(),
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
            period: Default::default(),
            detail: Default::default(),
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
    rh_foundation::Invariant::new("cmp-1", rh_foundation::Severity::Error, "A section must contain at least one of text, entries, or sub-sections", "text.exists() or entry.exists() or section.exists()"),
    rh_foundation::Invariant::new("cmp-2", rh_foundation::Severity::Error, "A section can only have an emptyReason if it is empty", "emptyReason.empty() or entry.empty()"),
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
                "Composition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Composition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/composition-status|5.0.0",
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
            rh_foundation::ElementCardinality::new("Composition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Composition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.category", 0, None),
            rh_foundation::ElementCardinality::new("Composition.subject", 0, None),
            rh_foundation::ElementCardinality::new("Composition.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Composition.author", 1, None),
            rh_foundation::ElementCardinality::new("Composition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.note", 0, None),
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
            rh_foundation::ElementCardinality::new("Composition.event", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Composition.event.extension", 0, None),
            rh_foundation::ElementCardinality::new("Composition.event.modifierExtension", 0, None),
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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for Composition {
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

impl crate::traits::composition::CompositionAccessors for Composition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn status(&self) -> CompositionStatus {
        self.status.clone()
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn author(&self) -> &[Reference] {
        &self.author
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> StringType {
        self.title.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn attester(&self) -> &[CompositionAttester] {
        self.attester.as_deref().unwrap_or(&[])
    }
    fn custodian(&self) -> Option<Reference> {
        self.custodian.clone()
    }
    fn relates_to(&self) -> &[RelatedArtifact] {
        self.relates_to.as_deref().unwrap_or(&[])
    }
    fn event(&self) -> &[CompositionEvent] {
        self.event.as_deref().unwrap_or(&[])
    }
    fn section(&self) -> &[CompositionSection] {
        self.section.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::composition::CompositionMutators for Composition {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
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
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
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
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn add_subject(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject.get_or_insert_with(Vec::new).push(item);
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
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = value;
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
    fn set_attester(self, value: Vec<CompositionAttester>) -> Self {
        let mut resource = self.clone();
        resource.attester = Some(value);
        resource
    }
    fn add_attester(self, item: CompositionAttester) -> Self {
        let mut resource = self.clone();
        resource.attester.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_custodian(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.custodian = Some(value);
        resource
    }
    fn set_relates_to(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = Some(value);
        resource
    }
    fn add_relates_to(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource.relates_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_event(self, value: Vec<CompositionEvent>) -> Self {
        let mut resource = self.clone();
        resource.event = Some(value);
        resource
    }
    fn add_event(self, item: CompositionEvent) -> Self {
        let mut resource = self.clone();
        resource.event.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_section(self, value: Vec<CompositionSection>) -> Self {
        let mut resource = self.clone();
        resource.section = Some(value);
        resource
    }
    fn add_section(self, item: CompositionSection) -> Self {
        let mut resource = self.clone();
        resource.section.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::composition::CompositionExistence for Composition {
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
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        true
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_attester(&self) -> bool {
        self.attester.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_custodian(&self) -> bool {
        self.custodian.is_some()
    }
    fn has_relates_to(&self) -> bool {
        self.relates_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_event(&self) -> bool {
        self.event.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_section(&self) -> bool {
        self.section.as_ref().is_some_and(|v| !v.is_empty())
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
