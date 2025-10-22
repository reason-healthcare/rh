use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceReferenceInformation
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceReferenceInformation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceReferenceInformation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Todo
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Todo
    pub gene: Option<Vec<SubstanceReferenceInformationGene>>,
    /// Todo
    #[serde(rename = "geneElement")]
    pub gene_element: Option<Vec<SubstanceReferenceInformationGeneelement>>,
    /// Todo
    pub classification: Option<Vec<SubstanceReferenceInformationClassification>>,
    /// Todo
    pub target: Option<Vec<SubstanceReferenceInformationTarget>>,
}
/// SubstanceReferenceInformation nested structure for the 'classification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceReferenceInformationClassification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub domain: Option<CodeableConcept>,
    /// Todo
    pub classification: Option<CodeableConcept>,
    /// Todo
    pub subtype: Option<Vec<CodeableConcept>>,
    /// Todo
    pub source: Option<Vec<Reference>>,
}
/// SubstanceReferenceInformation nested structure for the 'geneElement' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceReferenceInformationGeneelement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    pub element: Option<Identifier>,
    /// Todo
    pub source: Option<Vec<Reference>>,
}
/// SubstanceReferenceInformation nested structure for the 'gene' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceReferenceInformationGene {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "geneSequenceOrigin")]
    pub gene_sequence_origin: Option<CodeableConcept>,
    /// Todo
    pub gene: Option<CodeableConcept>,
    /// Todo
    pub source: Option<Vec<Reference>>,
}
/// SubstanceReferenceInformation nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceReferenceInformationTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub target: Option<Identifier>,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    pub interaction: Option<CodeableConcept>,
    /// Todo
    pub organism: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "organismType")]
    pub organism_type: Option<CodeableConcept>,
    /// Todo (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// Todo (Range)
    #[serde(rename = "amountRange")]
    pub amount_range: Option<Range>,
    /// Todo (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
    /// Todo
    #[serde(rename = "amountType")]
    pub amount_type: Option<CodeableConcept>,
    /// Todo
    pub source: Option<Vec<Reference>>,
}

impl Default for SubstanceReferenceInformation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            comment: Default::default(),
            _comment: Default::default(),
            gene: Default::default(),
            gene_element: Default::default(),
            classification: Default::default(),
            target: Default::default(),
        }
    }
}

impl Default for SubstanceReferenceInformationClassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            domain: Default::default(),
            classification: Default::default(),
            subtype: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceReferenceInformationGeneelement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            element: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceReferenceInformationGene {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            gene_sequence_origin: Default::default(),
            gene: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceReferenceInformationTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target: Default::default(),
            type_: Default::default(),
            interaction: Default::default(),
            organism: Default::default(),
            organism_type: Default::default(),
            amount_quantity: Default::default(),
            amount_range: Default::default(),
            amount_string: Default::default(),
            amount_type: Default::default(),
            source: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstanceReferenceInformation {
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

impl crate::traits::resource::ResourceMutators for SubstanceReferenceInformation {
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

impl crate::traits::resource::ResourceExistence for SubstanceReferenceInformation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceReferenceInformation {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceReferenceInformation {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceReferenceInformation {
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

impl crate::traits::substance_reference_information::SubstanceReferenceInformationAccessors
    for SubstanceReferenceInformation
{
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn gene(&self) -> &[SubstanceReferenceInformationGene] {
        self.gene.as_deref().unwrap_or(&[])
    }
    fn gene_element(&self) -> &[SubstanceReferenceInformationGeneelement] {
        self.gene_element.as_deref().unwrap_or(&[])
    }
    fn classification(&self) -> &[SubstanceReferenceInformationClassification] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn target(&self) -> &[SubstanceReferenceInformationTarget] {
        self.target.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_reference_information::SubstanceReferenceInformationMutators
    for SubstanceReferenceInformation
{
    fn new() -> Self {
        Self::default()
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_gene(self, value: Vec<SubstanceReferenceInformationGene>) -> Self {
        let mut resource = self.clone();
        resource.gene = Some(value);
        resource
    }
    fn add_gene(self, item: SubstanceReferenceInformationGene) -> Self {
        let mut resource = self.clone();
        resource.gene.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_gene_element(self, value: Vec<SubstanceReferenceInformationGeneelement>) -> Self {
        let mut resource = self.clone();
        resource.gene_element = Some(value);
        resource
    }
    fn add_gene_element(self, item: SubstanceReferenceInformationGeneelement) -> Self {
        let mut resource = self.clone();
        resource
            .gene_element
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_classification(self, value: Vec<SubstanceReferenceInformationClassification>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: SubstanceReferenceInformationClassification) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_target(self, value: Vec<SubstanceReferenceInformationTarget>) -> Self {
        let mut resource = self.clone();
        resource.target = Some(value);
        resource
    }
    fn add_target(self, item: SubstanceReferenceInformationTarget) -> Self {
        let mut resource = self.clone();
        resource.target.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance_reference_information::SubstanceReferenceInformationExistence
    for SubstanceReferenceInformation
{
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
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_gene(&self) -> bool {
        self.gene.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_gene_element(&self) -> bool {
        self.gene_element.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_target(&self) -> bool {
        self.target.as_ref().is_some_and(|v| !v.is_empty())
    }
}
