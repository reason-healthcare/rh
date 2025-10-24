use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceNucleicAcid
///
/// Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceNucleicAcid
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceNucleicAcid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The type of the sequence shall be specified based on a controlled vocabulary
    #[serde(rename = "sequenceType")]
    pub sequence_type: Option<CodeableConcept>,
    /// The number of linear sequences of nucleotides linked through phosphodiester bonds shall be described. Subunits would be strands of nucleic acids that are tightly associated typically through Watson-Crick base pairing. NOTE: If not specified in the reference source, the assumption is that there is 1 subunit
    #[serde(rename = "numberOfSubunits")]
    pub number_of_subunits: Option<IntegerType>,
    /// Extension element for the 'numberOfSubunits' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfSubunits")]
    pub _number_of_subunits: Option<Element>,
    /// The area of hybridisation shall be described if applicable for double stranded RNA or DNA. The number associated with the subunit followed by the number associated to the residue shall be specified in increasing order. The underscore “” shall be used as separator as follows: “Subunitnumber Residue”
    #[serde(rename = "areaOfHybridisation")]
    pub area_of_hybridisation: Option<StringType>,
    /// Extension element for the 'areaOfHybridisation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_areaOfHybridisation")]
    pub _area_of_hybridisation: Option<Element>,
    /// (TBC)
    #[serde(rename = "oligoNucleotideType")]
    pub oligo_nucleotide_type: Option<CodeableConcept>,
    /// Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times
    pub subunit: Option<Vec<SubstanceNucleicAcidSubunit>>,
}
/// SubstanceNucleicAcidSubunit nested structure for the 'linkage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceNucleicAcidSubunitLinkage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The entity that links the sugar residues together should also be captured for nearly all naturally occurring nucleic acid the linkage is a phosphate group. For many synthetic oligonucleotides phosphorothioate linkages are often seen. Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or 5’-5’ this should be specified
    pub connectivity: Option<StringType>,
    /// Extension element for the 'connectivity' primitive field. Contains metadata and extensions.
    pub _connectivity: Option<Element>,
    /// Each linkage will be registered as a fragment and have an ID
    pub identifier: Option<Identifier>,
    /// Each linkage will be registered as a fragment and have at least one name. A single name shall be assigned to each linkage
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Residues shall be captured as described in 5.3.6.8.3
    #[serde(rename = "residueSite")]
    pub residue_site: Option<StringType>,
    /// Extension element for the 'residueSite' primitive field. Contains metadata and extensions.
    #[serde(rename = "_residueSite")]
    pub _residue_site: Option<Element>,
}
/// SubstanceNucleicAcid nested structure for the 'subunit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceNucleicAcidSubunit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The linkages between sugar residues will also be captured
    pub linkage: Option<Vec<SubstanceNucleicAcidSubunitLinkage>>,
    /// 5.3.6.8.1 Sugar ID (Mandatory)
    pub sugar: Option<Vec<SubstanceNucleicAcidSubunitSugar>>,
    /// Index of linear sequences of nucleic acids in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts
    pub subunit: Option<IntegerType>,
    /// Extension element for the 'subunit' primitive field. Contains metadata and extensions.
    pub _subunit: Option<Element>,
    /// Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes. In addition to the base sequence, sugar and type of phosphate or non-phosphate linkage should also be captured
    pub sequence: Option<StringType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// The length of the sequence shall be captured
    pub length: Option<IntegerType>,
    /// Extension element for the 'length' primitive field. Contains metadata and extensions.
    pub _length: Option<Element>,
    /// (TBC)
    #[serde(rename = "sequenceAttachment")]
    pub sequence_attachment: Option<Attachment>,
    /// The nucleotide present at the 5’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the first position in the sequence. A separate representation would be redundant
    #[serde(rename = "fivePrime")]
    pub five_prime: Option<CodeableConcept>,
    /// The nucleotide present at the 3’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the last position in the sequence. A separate representation would be redundant
    #[serde(rename = "threePrime")]
    pub three_prime: Option<CodeableConcept>,
}
/// SubstanceNucleicAcidSubunit nested structure for the 'sugar' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceNucleicAcidSubunitSugar {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The Substance ID of the sugar or sugar-like component that make up the nucleotide
    pub identifier: Option<Identifier>,
    /// The name of the sugar or sugar-like component that make up the nucleotide
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// The residues that contain a given sugar will be captured. The order of given residues will be captured in the 5‘-3‘direction consistent with the base sequences listed above
    #[serde(rename = "residueSite")]
    pub residue_site: Option<StringType>,
    /// Extension element for the 'residueSite' primitive field. Contains metadata and extensions.
    #[serde(rename = "_residueSite")]
    pub _residue_site: Option<Element>,
}

impl Default for SubstanceNucleicAcid {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            sequence_type: Default::default(),
            number_of_subunits: Default::default(),
            _number_of_subunits: Default::default(),
            area_of_hybridisation: Default::default(),
            _area_of_hybridisation: Default::default(),
            oligo_nucleotide_type: Default::default(),
            subunit: Default::default(),
        }
    }
}

impl Default for SubstanceNucleicAcidSubunitLinkage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            connectivity: Default::default(),
            _connectivity: Default::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            residue_site: Default::default(),
            _residue_site: Default::default(),
        }
    }
}

impl Default for SubstanceNucleicAcidSubunit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            linkage: Default::default(),
            sugar: Default::default(),
            subunit: Default::default(),
            _subunit: Default::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            length: Default::default(),
            _length: Default::default(),
            sequence_attachment: Default::default(),
            five_prime: Default::default(),
            three_prime: Default::default(),
        }
    }
}

impl Default for SubstanceNucleicAcidSubunitSugar {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            residue_site: Default::default(),
            _residue_site: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for SubstanceNucleicAcid {
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

impl crate::traits::resource::ResourceMutators for SubstanceNucleicAcid {
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

impl crate::traits::resource::ResourceExistence for SubstanceNucleicAcid {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceNucleicAcid {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceNucleicAcid {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceNucleicAcid {
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

impl crate::traits::substance_nucleic_acid::SubstanceNucleicAcidAccessors for SubstanceNucleicAcid {
    fn sequence_type(&self) -> Option<CodeableConcept> {
        self.sequence_type.clone()
    }
    fn number_of_subunits(&self) -> Option<IntegerType> {
        self.number_of_subunits
    }
    fn area_of_hybridisation(&self) -> Option<StringType> {
        self.area_of_hybridisation.clone()
    }
    fn oligo_nucleotide_type(&self) -> Option<CodeableConcept> {
        self.oligo_nucleotide_type.clone()
    }
    fn subunit(&self) -> &[SubstanceNucleicAcidSubunit] {
        self.subunit.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_nucleic_acid::SubstanceNucleicAcidMutators for SubstanceNucleicAcid {
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
    fn set_area_of_hybridisation(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.area_of_hybridisation = Some(value);
        resource
    }
    fn set_oligo_nucleotide_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.oligo_nucleotide_type = Some(value);
        resource
    }
    fn set_subunit(self, value: Vec<SubstanceNucleicAcidSubunit>) -> Self {
        let mut resource = self.clone();
        resource.subunit = Some(value);
        resource
    }
    fn add_subunit(self, item: SubstanceNucleicAcidSubunit) -> Self {
        let mut resource = self.clone();
        resource.subunit.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance_nucleic_acid::SubstanceNucleicAcidExistence for SubstanceNucleicAcid {
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
    fn has_area_of_hybridisation(&self) -> bool {
        self.area_of_hybridisation.is_some()
    }
    fn has_oligo_nucleotide_type(&self) -> bool {
        self.oligo_nucleotide_type.is_some()
    }
    fn has_subunit(&self) -> bool {
        self.subunit.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SubstanceNucleicAcid {
    fn resource_type(&self) -> &'static str {
        "SubstanceNucleicAcid"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid")
    }
}
