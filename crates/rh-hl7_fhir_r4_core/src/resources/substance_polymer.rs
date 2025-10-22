use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::substance_amount::SubstanceAmount;
use crate::primitives::boolean::BooleanType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstancePolymer
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstancePolymer
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Todo
    pub class: Option<CodeableConcept>,
    /// Todo
    pub geometry: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "copolymerConnectivity")]
    pub copolymer_connectivity: Option<Vec<CodeableConcept>>,
    /// Todo
    pub modification: Option<Vec<StringType>>,
    /// Extension element for the 'modification' primitive field. Contains metadata and extensions.
    pub _modification: Option<Element>,
    /// Todo
    #[serde(rename = "monomerSet")]
    pub monomer_set: Option<Vec<SubstancePolymerMonomerset>>,
    /// Todo
    pub repeat: Option<Vec<SubstancePolymerRepeat>>,
}
/// SubstancePolymer nested structure for the 'monomerSet' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerMonomerset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "startingMaterial")]
    pub starting_material: Option<Vec<SubstancePolymerMonomersetStartingmaterial>>,
    /// Todo
    #[serde(rename = "ratioType")]
    pub ratio_type: Option<CodeableConcept>,
}
/// SubstancePolymerMonomerset nested structure for the 'startingMaterial' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerMonomersetStartingmaterial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub material: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "isDefining")]
    pub is_defining: Option<BooleanType>,
    /// Extension element for the 'isDefining' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDefining")]
    pub _is_defining: Option<Element>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}
/// SubstancePolymerRepeatRepeatunit nested structure for the 'degreeOfPolymerisation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunitDegreeofpolymerisation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    pub degree: Option<CodeableConcept>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}
/// SubstancePolymer nested structure for the 'repeat' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeat {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatunit>>,
    /// Todo
    #[serde(rename = "numberOfUnits")]
    pub number_of_units: Option<IntegerType>,
    /// Extension element for the 'numberOfUnits' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfUnits")]
    pub _number_of_units: Option<Element>,
    /// Todo
    #[serde(rename = "averageMolecularFormula")]
    pub average_molecular_formula: Option<StringType>,
    /// Extension element for the 'averageMolecularFormula' primitive field. Contains metadata and extensions.
    #[serde(rename = "_averageMolecularFormula")]
    pub _average_molecular_formula: Option<Element>,
    /// Todo
    #[serde(rename = "repeatUnitAmountType")]
    pub repeat_unit_amount_type: Option<CodeableConcept>,
}
/// SubstancePolymerRepeat nested structure for the 'repeatUnit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "orientationOfPolymerisation")]
    pub orientation_of_polymerisation: Option<CodeableConcept>,
    /// Todo
    #[serde(rename = "repeatUnit")]
    pub repeat_unit: Option<StringType>,
    /// Extension element for the 'repeatUnit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_repeatUnit")]
    pub _repeat_unit: Option<Element>,
    /// Todo
    pub amount: Option<SubstanceAmount>,
}
/// SubstancePolymerRepeatRepeatunit nested structure for the 'structuralRepresentation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstancePolymerRepeatRepeatunitStructuralrepresentation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Todo
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    pub representation: Option<StringType>,
    /// Extension element for the 'representation' primitive field. Contains metadata and extensions.
    pub _representation: Option<Element>,
    /// Todo
    pub attachment: Option<Attachment>,
}

impl Default for SubstancePolymer {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            class: Default::default(),
            geometry: Default::default(),
            copolymer_connectivity: Default::default(),
            modification: Default::default(),
            _modification: Default::default(),
            monomer_set: Default::default(),
            repeat: Default::default(),
        }
    }
}

impl Default for SubstancePolymerMonomerset {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            starting_material: Default::default(),
            ratio_type: Default::default(),
        }
    }
}

impl Default for SubstancePolymerMonomersetStartingmaterial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            material: Default::default(),
            type_: Default::default(),
            is_defining: Default::default(),
            _is_defining: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunitDegreeofpolymerisation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            degree: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeat {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            repeat_unit: Default::default(),
            number_of_units: Default::default(),
            _number_of_units: Default::default(),
            average_molecular_formula: Default::default(),
            _average_molecular_formula: Default::default(),
            repeat_unit_amount_type: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            orientation_of_polymerisation: Default::default(),
            repeat_unit: Default::default(),
            _repeat_unit: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstancePolymerRepeatRepeatunitStructuralrepresentation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            representation: Default::default(),
            _representation: Default::default(),
            attachment: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstancePolymer {
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

impl crate::traits::resource::ResourceMutators for SubstancePolymer {
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

impl crate::traits::resource::ResourceExistence for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstancePolymer {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstancePolymer {
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

impl crate::traits::substance_polymer::SubstancePolymerAccessors for SubstancePolymer {
    fn class(&self) -> Option<CodeableConcept> {
        self.class.clone()
    }
    fn geometry(&self) -> Option<CodeableConcept> {
        self.geometry.clone()
    }
    fn copolymer_connectivity(&self) -> &[CodeableConcept] {
        self.copolymer_connectivity.as_deref().unwrap_or(&[])
    }
    fn modification(&self) -> &[StringType] {
        self.modification.as_deref().unwrap_or(&[])
    }
    fn monomer_set(&self) -> &[SubstancePolymerMonomerset] {
        self.monomer_set.as_deref().unwrap_or(&[])
    }
    fn repeat(&self) -> &[SubstancePolymerRepeat] {
        self.repeat.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_polymer::SubstancePolymerMutators for SubstancePolymer {
    fn new() -> Self {
        Self::default()
    }
    fn set_class(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.class = Some(value);
        resource
    }
    fn set_geometry(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.geometry = Some(value);
        resource
    }
    fn set_copolymer_connectivity(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.copolymer_connectivity = Some(value);
        resource
    }
    fn add_copolymer_connectivity(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .copolymer_connectivity
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modification(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.modification = Some(value);
        resource
    }
    fn add_modification(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .modification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_monomer_set(self, value: Vec<SubstancePolymerMonomerset>) -> Self {
        let mut resource = self.clone();
        resource.monomer_set = Some(value);
        resource
    }
    fn add_monomer_set(self, item: SubstancePolymerMonomerset) -> Self {
        let mut resource = self.clone();
        resource.monomer_set.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_repeat(self, value: Vec<SubstancePolymerRepeat>) -> Self {
        let mut resource = self.clone();
        resource.repeat = Some(value);
        resource
    }
    fn add_repeat(self, item: SubstancePolymerRepeat) -> Self {
        let mut resource = self.clone();
        resource.repeat.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::substance_polymer::SubstancePolymerExistence for SubstancePolymer {
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
    fn has_class(&self) -> bool {
        self.class.is_some()
    }
    fn has_geometry(&self) -> bool {
        self.geometry.is_some()
    }
    fn has_copolymer_connectivity(&self) -> bool {
        self.copolymer_connectivity
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_modification(&self) -> bool {
        self.modification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_monomer_set(&self) -> bool {
        self.monomer_set.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_repeat(&self) -> bool {
        self.repeat.as_ref().is_some_and(|v| !v.is_empty())
    }
}
