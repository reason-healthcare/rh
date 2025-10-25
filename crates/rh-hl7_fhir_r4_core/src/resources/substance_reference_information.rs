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

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SubstanceReferenceInformation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.meta",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.text",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.contained",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.comment",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceReferenceInformation.gene", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.geneSequenceOrigin",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.gene",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.gene.source",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.element",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.geneElement.source",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.domain",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.classification",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.subtype",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.classification.source",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceReferenceInformation.target", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.target",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.interaction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.organism",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.organismType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.amount[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.amountType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceReferenceInformation.target.source",
                0,
                None,
            ),
        ]
    });

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

impl crate::validation::ValidatableResource for SubstanceReferenceInformation {
    fn resource_type(&self) -> &'static str {
        "SubstanceReferenceInformation"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::substance_reference_information::{
    SubstanceReferenceInformationAccessors, SubstanceReferenceInformationExistence,
    SubstanceReferenceInformationMutators,
};
