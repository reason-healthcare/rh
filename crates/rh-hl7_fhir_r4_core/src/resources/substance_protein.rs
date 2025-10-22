use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceProtein
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceProtein
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceProtein {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence
    #[serde(rename = "sequenceType")]
    pub sequence_type: Option<CodeableConcept>,
    /// Number of linear sequences of amino acids linked through peptide bonds. The number of subunits constituting the SubstanceProtein shall be described. It is possible that the number of subunits can be variable
    #[serde(rename = "numberOfSubunits")]
    pub number_of_subunits: Option<IntegerType>,
    /// Extension element for the 'numberOfSubunits' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfSubunits")]
    pub _number_of_subunits: Option<Element>,
    /// The disulphide bond between two cysteine residues either on the same subunit or on two different subunits shall be described. The position of the disulfide bonds in the SubstanceProtein shall be listed in increasing order of subunit number and position within subunit followed by the abbreviation of the amino acids involved. The disulfide linkage positions shall actually contain the amino acid Cysteine at the respective positions
    #[serde(rename = "disulfideLinkage")]
    pub disulfide_linkage: Option<Vec<StringType>>,
    /// Extension element for the 'disulfideLinkage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_disulfideLinkage")]
    pub _disulfide_linkage: Option<Element>,
    /// This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall be provided when the finished SubstanceProtein is a complex of multiple sequences; subunits are not used to delineate domains within a single sequence. Subunits are listed in order of decreasing length; sequences of the same length will be ordered by decreasing molecular weight; subunits that have identical sequences will be repeated multiple times
    pub subunit: Option<Vec<SubstanceProteinSubunit>>,
}
/// SubstanceProtein nested structure for the 'subunit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceProteinSubunit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts
    pub subunit: Option<IntegerType>,
    /// Extension element for the 'subunit' primitive field. Contains metadata and extensions.
    pub _subunit: Option<Element>,
    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence
    pub sequence: Option<StringType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Length of linear sequences of amino acids contained in the subunit
    pub length: Option<IntegerType>,
    /// Extension element for the 'length' primitive field. Contains metadata and extensions.
    pub _length: Option<Element>,
    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence
    #[serde(rename = "sequenceAttachment")]
    pub sequence_attachment: Option<Attachment>,
    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[serde(rename = "nTerminalModificationId")]
    pub n_terminal_modification_id: Option<Identifier>,
    /// The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified
    #[serde(rename = "nTerminalModification")]
    pub n_terminal_modification: Option<StringType>,
    /// Extension element for the 'nTerminalModification' primitive field. Contains metadata and extensions.
    #[serde(rename = "_nTerminalModification")]
    pub _n_terminal_modification: Option<Element>,
    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[serde(rename = "cTerminalModificationId")]
    pub c_terminal_modification_id: Option<Identifier>,
    /// The modification at the C-terminal shall be specified
    #[serde(rename = "cTerminalModification")]
    pub c_terminal_modification: Option<StringType>,
    /// Extension element for the 'cTerminalModification' primitive field. Contains metadata and extensions.
    #[serde(rename = "_cTerminalModification")]
    pub _c_terminal_modification: Option<Element>,
}

impl Default for SubstanceProtein {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            sequence_type: Default::default(),
            number_of_subunits: Default::default(),
            _number_of_subunits: Default::default(),
            disulfide_linkage: Default::default(),
            _disulfide_linkage: Default::default(),
            subunit: Default::default(),
        }
    }
}

impl Default for SubstanceProteinSubunit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            subunit: Default::default(),
            _subunit: Default::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            length: Default::default(),
            _length: Default::default(),
            sequence_attachment: Default::default(),
            n_terminal_modification_id: Default::default(),
            n_terminal_modification: Default::default(),
            _n_terminal_modification: Default::default(),
            c_terminal_modification_id: Default::default(),
            c_terminal_modification: Default::default(),
            _c_terminal_modification: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstanceProtein {
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

impl crate::traits::resource::ResourceMutators for SubstanceProtein {
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

impl crate::traits::resource::ResourceExistence for SubstanceProtein {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceProtein {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceProtein {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceProtein {
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

impl crate::traits::substance_protein::SubstanceProteinAccessors for SubstanceProtein {
    fn sequence_type(&self) -> Option<CodeableConcept> {
        self.sequence_type.clone()
    }
    fn number_of_subunits(&self) -> Option<IntegerType> {
        self.number_of_subunits
    }
    fn disulfide_linkage(&self) -> &[StringType] {
        self.disulfide_linkage.as_deref().unwrap_or(&[])
    }
    fn subunit(&self) -> &[SubstanceProteinSubunit] {
        self.subunit.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_protein::SubstanceProteinMutators for SubstanceProtein {
    fn new() -> Self {
        Self::default()
    }
    fn set_sequence_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.sequence_type = Some(value);
        resource
    }
    fn set_number_of_subunits(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.number_of_subunits = Some(value);
        resource
    }
    fn set_disulfide_linkage(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.disulfide_linkage = Some(value);
        resource
    }
    fn add_disulfide_linkage(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .disulfide_linkage
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subunit(self, value: Vec<SubstanceProteinSubunit>) -> Self {
        let mut resource = self.clone();
        resource.subunit = Some(value);
        resource
    }
    fn add_subunit(self, item: SubstanceProteinSubunit) -> Self {
        let mut resource = self.clone();
        resource.subunit.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance_protein::SubstanceProteinExistence for SubstanceProtein {
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
    fn has_sequence_type(&self) -> bool {
        self.sequence_type.is_some()
    }
    fn has_number_of_subunits(&self) -> bool {
        self.number_of_subunits.is_some()
    }
    fn has_disulfide_linkage(&self) -> bool {
        self.disulfide_linkage
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_subunit(&self) -> bool {
        self.subunit.as_ref().is_some_and(|v| !v.is_empty())
    }
}
