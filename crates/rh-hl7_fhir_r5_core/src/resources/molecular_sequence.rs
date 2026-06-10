use crate::bindings::orientation_type::OrientationType;
use crate::bindings::sequence_type::SequenceType;
use crate::bindings::strand_type::StrandType;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MolecularSequence
///
/// Representation of a molecular sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique ID for this particular sequence
    pub identifier: Option<Vec<Identifier>>,
    /// aa | dna | rna
    #[serde(rename = "type")]
    pub type_: Option<SequenceType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Subject this sequence is associated too
    pub subject: Option<Reference>,
    /// What the molecular sequence is about, when it is not about the subject of record
    pub focus: Option<Vec<Reference>>,
    /// Specimen used for sequencing
    pub specimen: Option<Reference>,
    /// The method for sequencing
    pub device: Option<Reference>,
    /// Who should be responsible for test result
    pub performer: Option<Reference>,
    /// Sequence that was observed
    pub literal: Option<StringType>,
    /// Extension element for the 'literal' primitive field. Contains metadata and extensions.
    pub _literal: Option<Element>,
    /// Embedded file or a link (URL) which contains content to represent the sequence
    pub formatted: Option<Vec<Attachment>>,
    /// A sequence defined relative to another sequence
    pub relative: Option<Vec<MolecularSequenceRelative>>,
}
/// MolecularSequenceRelative nested structure for the 'edit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceRelativeEdit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Start position of the edit on the starting sequence
    pub start: Option<IntegerType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// End position of the edit on the starting sequence
    pub end: Option<IntegerType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Allele that was observed
    #[serde(rename = "replacementSequence")]
    pub replacement_sequence: Option<StringType>,
    /// Extension element for the 'replacementSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_replacementSequence")]
    pub _replacement_sequence: Option<Element>,
    /// Allele in the starting sequence
    #[serde(rename = "replacedSequence")]
    pub replaced_sequence: Option<StringType>,
    /// Extension element for the 'replacedSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_replacedSequence")]
    pub _replaced_sequence: Option<Element>,
}
/// MolecularSequence nested structure for the 'relative' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceRelative {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A sequence used as starting sequence
    #[serde(rename = "startingSequence")]
    pub starting_sequence: Option<MolecularSequenceRelativeStartingsequence>,
    /// Changes in sequence from the starting sequence
    pub edit: Option<Vec<MolecularSequenceRelativeEdit>>,
    /// Ways of identifying nucleotides or amino acids within a sequence
    ///
    /// Binding: extensible (Genomic coordinate system.)
    ///
    /// ValueSet: http://loinc.org/LL5323-2/
    #[serde(rename = "coordinateSystem")]
    pub coordinate_system: CodeableConcept,
    /// Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together
    #[serde(rename = "ordinalPosition")]
    pub ordinal_position: Option<IntegerType>,
    /// Extension element for the 'ordinalPosition' primitive field. Contains metadata and extensions.
    #[serde(rename = "_ordinalPosition")]
    pub _ordinal_position: Option<Element>,
    /// Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together
    #[serde(rename = "sequenceRange")]
    pub sequence_range: Option<Range>,
}
/// MolecularSequenceRelative nested structure for the 'startingSequence' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceRelativeStartingsequence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The genome assembly used for starting sequence, e.g. GRCh38
    ///
    /// Binding: extensible (Human reference sequence NCBI build ID.)
    ///
    /// ValueSet: http://loinc.org/LL1040-6/
    #[serde(rename = "genomeAssembly")]
    pub genome_assembly: Option<CodeableConcept>,
    /// Chromosome Identifier
    pub chromosome: Option<CodeableConcept>,
    /// The reference sequence that represents the starting sequence (CodeableConcept)
    #[serde(rename = "sequenceCodeableConcept")]
    pub sequence_codeable_concept: Option<CodeableConcept>,
    /// The reference sequence that represents the starting sequence (string)
    #[serde(rename = "sequenceString")]
    pub sequence_string: Option<StringType>,
    /// The reference sequence that represents the starting sequence (Reference)
    #[serde(rename = "sequenceReference")]
    pub sequence_reference: Option<Reference>,
    /// Start position of the window on the starting sequence
    #[serde(rename = "windowStart")]
    pub window_start: Option<IntegerType>,
    /// Extension element for the 'windowStart' primitive field. Contains metadata and extensions.
    #[serde(rename = "_windowStart")]
    pub _window_start: Option<Element>,
    /// End position of the window on the starting sequence
    #[serde(rename = "windowEnd")]
    pub window_end: Option<IntegerType>,
    /// Extension element for the 'windowEnd' primitive field. Contains metadata and extensions.
    #[serde(rename = "_windowEnd")]
    pub _window_end: Option<Element>,
    /// sense | antisense
    pub orientation: Option<OrientationType>,
    /// Extension element for the 'orientation' primitive field. Contains metadata and extensions.
    pub _orientation: Option<Element>,
    /// watson | crick
    pub strand: Option<StrandType>,
    /// Extension element for the 'strand' primitive field. Contains metadata and extensions.
    pub _strand: Option<Element>,
}

impl Default for MolecularSequence {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            subject: Default::default(),
            focus: Default::default(),
            specimen: Default::default(),
            device: Default::default(),
            performer: Default::default(),
            literal: Default::default(),
            _literal: Default::default(),
            formatted: Default::default(),
            relative: Default::default(),
        }
    }
}

impl Default for MolecularSequenceRelativeEdit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            replacement_sequence: Default::default(),
            _replacement_sequence: Default::default(),
            replaced_sequence: Default::default(),
            _replaced_sequence: Default::default(),
        }
    }
}

impl Default for MolecularSequenceRelative {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            starting_sequence: Default::default(),
            edit: Default::default(),
            coordinate_system: CodeableConcept::default(),
            ordinal_position: Default::default(),
            _ordinal_position: Default::default(),
            sequence_range: Default::default(),
        }
    }
}

impl Default for MolecularSequenceRelativeStartingsequence {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            genome_assembly: Default::default(),
            chromosome: Default::default(),
            sequence_codeable_concept: Default::default(),
            sequence_string: Default::default(),
            sequence_reference: Default::default(),
            window_start: Default::default(),
            _window_start: Default::default(),
            window_end: Default::default(),
            _window_end: Default::default(),
            orientation: Default::default(),
            _orientation: Default::default(),
            strand: Default::default(),
            _strand: Default::default(),
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
    rh_foundation::Invariant::new("msq-5", rh_foundation::Severity::Error, "Both genomeAssembly and chromosome must be both contained if either one of them is contained", "chromosome.exists() = genomeAssembly.exists()"),
    rh_foundation::Invariant::new("msq-6", rh_foundation::Severity::Error, "Have and only have one of the following elements in startingSequence: 1. genomeAssembly; 2 sequence", "genomeAssembly.exists() xor sequence.exists()"),
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
                "MolecularSequence.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "MolecularSequence.relative.startingSequence.chromosome",
                rh_foundation::BindingStrength::Required,
                "http://loinc.org/LL2938-0/|5.0.0",
            )
            .with_description("The chromosome containing the sequence."),
            rh_foundation::ElementBinding::new(
                "MolecularSequence.relative.startingSequence.orientation",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/orientation-type|5.0.0",
            )
            .with_description("Type for orientation"),
            rh_foundation::ElementBinding::new(
                "MolecularSequence.relative.startingSequence.strand",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/strand-type|5.0.0",
            )
            .with_description("Type for strand"),
            rh_foundation::ElementBinding::new(
                "MolecularSequence.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/sequence-type|5.0.0",
            )
            .with_description("Type if a sequence -- DNA, RNA, or amino acid sequence."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MolecularSequence.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.contained", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.extension", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.focus", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.specimen", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.device", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.performer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.literal", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.formatted", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.relative", 0, None),
            rh_foundation::ElementCardinality::new("MolecularSequence.relative.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MolecularSequence.relative.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.coordinateSystem",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.ordinalPosition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.sequenceRange",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.genomeAssembly",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.chromosome",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.sequence[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.windowStart",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.windowEnd",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.orientation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.startingSequence.strand",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MolecularSequence.relative.edit", 0, None),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.start",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.end",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.replacementSequence",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MolecularSequence.relative.edit.replacedSequence",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MolecularSequence {
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

impl crate::traits::resource::ResourceMutators for MolecularSequence {
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

impl crate::traits::resource::ResourceExistence for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceMutators for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceExistence for MolecularSequence {
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

impl crate::traits::molecular_sequence::MolecularSequenceAccessors for MolecularSequence {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<SequenceType> {
        self.type_.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn focus(&self) -> &[Reference] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn specimen(&self) -> Option<Reference> {
        self.specimen.clone()
    }
    fn device(&self) -> Option<Reference> {
        self.device.clone()
    }
    fn performer(&self) -> Option<Reference> {
        self.performer.clone()
    }
    fn literal(&self) -> Option<StringType> {
        self.literal.clone()
    }
    fn formatted(&self) -> &[Attachment] {
        self.formatted.as_deref().unwrap_or(&[])
    }
    fn relative(&self) -> &[MolecularSequenceRelative] {
        self.relative.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::molecular_sequence::MolecularSequenceMutators for MolecularSequence {
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
    fn set_type_(self, value: SequenceType) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_focus(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_specimen(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn set_performer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn set_literal(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.literal = Some(value);
        resource
    }
    fn set_formatted(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.formatted = Some(value);
        resource
    }
    fn add_formatted(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.formatted.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_relative(self, value: Vec<MolecularSequenceRelative>) -> Self {
        let mut resource = self.clone();
        resource.relative = Some(value);
        resource
    }
    fn add_relative(self, item: MolecularSequenceRelative) -> Self {
        let mut resource = self.clone();
        resource.relative.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::molecular_sequence::MolecularSequenceExistence for MolecularSequence {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specimen(&self) -> bool {
        self.specimen.is_some()
    }
    fn has_device(&self) -> bool {
        self.device.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.is_some()
    }
    fn has_literal(&self) -> bool {
        self.literal.is_some()
    }
    fn has_formatted(&self) -> bool {
        self.formatted.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_relative(&self) -> bool {
        self.relative.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MolecularSequence {
    fn resource_type(&self) -> &'static str {
        "MolecularSequence"
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
        Some("http://hl7.org/fhir/StructureDefinition/MolecularSequence")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::molecular_sequence::{
    MolecularSequenceAccessors, MolecularSequenceExistence, MolecularSequenceMutators,
};
