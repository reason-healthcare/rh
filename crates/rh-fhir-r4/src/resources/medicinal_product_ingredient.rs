use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductIngredient
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIngredient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIngredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier for the ingredient
    pub identifier: Option<Identifier>,
    /// Ingredient role e.g. Active ingredient, excipient
    pub role: CodeableConcept,
    /// If the ingredient is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<BooleanType>,
    /// Extension element for the 'allergenicIndicator' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allergenicIndicator")]
    pub _allergenic_indicator: Option<Element>,
    /// Manufacturer of this Ingredient
    pub manufacturer: Option<Vec<Reference>>,
    /// A specified substance that comprises this ingredient
    #[serde(rename = "specifiedSubstance")]
    pub specified_substance: Option<Vec<MedicinalProductIngredientSpecifiedsubstance>>,
    /// The ingredient substance
    pub substance: Option<MedicinalProductIngredientSubstance>,
}
/// MedicinalProductIngredient nested structure for the 'substance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductIngredientSubstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The ingredient substance
    pub code: CodeableConcept,
    /// Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product
    pub strength: Option<Vec<StringType>>,
}
/// MedicinalProductIngredientSpecifiedsubstanceStrength nested structure for the 'referenceStrength' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductIngredientSpecifiedsubstanceStrengthReferencestrength {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Relevant reference substance
    pub substance: Option<CodeableConcept>,
    /// Strength expressed in terms of a reference substance
    pub strength: Ratio,
    /// Strength expressed in terms of a reference substance
    #[serde(rename = "strengthLowLimit")]
    pub strength_low_limit: Option<Ratio>,
    /// For when strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<StringType>,
    /// Extension element for the 'measurementPoint' primitive field. Contains metadata and extensions.
    #[serde(rename = "_measurementPoint")]
    pub _measurement_point: Option<Element>,
    /// The country or countries for which the strength range applies
    pub country: Option<Vec<CodeableConcept>>,
}
/// MedicinalProductIngredient nested structure for the 'specifiedSubstance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductIngredientSpecifiedsubstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product
    pub strength: Option<Vec<MedicinalProductIngredientSpecifiedsubstanceStrength>>,
    /// The specified substance
    pub code: CodeableConcept,
    /// The group of specified substance, e.g. group 1 to 4
    pub group: CodeableConcept,
    /// Confidentiality level of the specified substance as the ingredient
    pub confidentiality: Option<CodeableConcept>,
}
/// MedicinalProductIngredientSpecifiedsubstance nested structure for the 'strength' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductIngredientSpecifiedsubstanceStrength {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item
    pub presentation: Ratio,
    /// A lower limit for the quantity of substance in the unit of presentation. For use when there is a range of strengths, this is the lower limit, with the presentation attribute becoming the upper limit
    #[serde(rename = "presentationLowLimit")]
    pub presentation_low_limit: Option<Ratio>,
    /// The strength per unitary volume (or mass)
    pub concentration: Option<Ratio>,
    /// A lower limit for the strength per unitary volume (or mass), for when there is a range. The concentration attribute then becomes the upper limit
    #[serde(rename = "concentrationLowLimit")]
    pub concentration_low_limit: Option<Ratio>,
    /// For when strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<StringType>,
    /// Extension element for the 'measurementPoint' primitive field. Contains metadata and extensions.
    #[serde(rename = "_measurementPoint")]
    pub _measurement_point: Option<Element>,
    /// The country or countries for which the strength range applies
    pub country: Option<Vec<CodeableConcept>>,
}

impl Default for MedicinalProductIngredient {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            role: CodeableConcept::default(),
            allergenic_indicator: Default::default(),
            _allergenic_indicator: Default::default(),
            manufacturer: Default::default(),
            specified_substance: Default::default(),
            substance: Default::default(),
        }
    }
}

impl Default for MedicinalProductIngredientSubstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            strength: Default::default(),
        }
    }
}

impl Default for MedicinalProductIngredientSpecifiedsubstanceStrengthReferencestrength {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance: Default::default(),
            strength: Default::default(),
            strength_low_limit: Default::default(),
            measurement_point: Default::default(),
            _measurement_point: Default::default(),
            country: Default::default(),
        }
    }
}

impl Default for MedicinalProductIngredientSpecifiedsubstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            strength: Default::default(),
            code: Default::default(),
            group: Default::default(),
            confidentiality: Default::default(),
        }
    }
}

impl Default for MedicinalProductIngredientSpecifiedsubstanceStrength {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            presentation: Default::default(),
            presentation_low_limit: Default::default(),
            concentration: Default::default(),
            concentration_low_limit: Default::default(),
            measurement_point: Default::default(),
            _measurement_point: Default::default(),
            country: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductIngredient {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductIngredient {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductIngredient {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductIngredient {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductIngredient {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductIngredient {
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

impl crate::traits::medicinal_product_ingredient::MedicinalProductIngredientAccessors
    for MedicinalProductIngredient
{
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn role(&self) -> CodeableConcept {
        self.role.clone()
    }
    fn allergenic_indicator(&self) -> Option<BooleanType> {
        self.allergenic_indicator
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn specified_substance(&self) -> &[MedicinalProductIngredientSpecifiedsubstance] {
        self.specified_substance.as_deref().unwrap_or(&[])
    }
    fn substance(&self) -> Option<MedicinalProductIngredientSubstance> {
        self.substance.clone()
    }
}

impl crate::traits::medicinal_product_ingredient::MedicinalProductIngredientMutators
    for MedicinalProductIngredient
{
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_role(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.role = value;
        resource
    }
    fn set_allergenic_indicator(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.allergenic_indicator = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn add_manufacturer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .manufacturer
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specified_substance(
        self,
        value: Vec<MedicinalProductIngredientSpecifiedsubstance>,
    ) -> Self {
        let mut resource = self.clone();
        resource.specified_substance = Some(value);
        resource
    }
    fn add_specified_substance(self, item: MedicinalProductIngredientSpecifiedsubstance) -> Self {
        let mut resource = self.clone();
        resource
            .specified_substance
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_substance(self, value: MedicinalProductIngredientSubstance) -> Self {
        let mut resource = self.clone();
        resource.substance = Some(value);
        resource
    }
}

impl crate::traits::medicinal_product_ingredient::MedicinalProductIngredientExistence
    for MedicinalProductIngredient
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
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_role(&self) -> bool {
        true
    }
    fn has_allergenic_indicator(&self) -> bool {
        self.allergenic_indicator.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specified_substance(&self) -> bool {
        self.specified_substance
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_substance(&self) -> bool {
        self.substance.is_some()
    }
}
