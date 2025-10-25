use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::prod_characteristic::ProdCharacteristic;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductManufactured
///
/// The manufactured item as contained in the packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductManufactured
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductManufactured
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductManufactured {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Dose form as manufactured and before any transformation into the pharmaceutical product
    #[serde(rename = "manufacturedDoseForm")]
    pub manufactured_dose_form: CodeableConcept,
    /// The “real world” units in which the quantity of the manufactured item is described
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// The quantity or "count number" of the manufactured item
    pub quantity: Quantity,
    /// Manufacturer of the item (Note that this should be named "manufacturer" but it currently causes technical issues)
    pub manufacturer: Option<Vec<Reference>>,
    /// Ingredient
    pub ingredient: Option<Vec<Reference>>,
    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,
    /// Other codeable characteristics
    #[serde(rename = "otherCharacteristics")]
    pub other_characteristics: Option<Vec<CodeableConcept>>,
}

impl Default for MedicinalProductManufactured {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            manufactured_dose_form: CodeableConcept::default(),
            unit_of_presentation: Default::default(),
            quantity: Quantity::default(),
            manufacturer: Default::default(),
            ingredient: Default::default(),
            physical_characteristics: Default::default(),
            other_characteristics: Default::default(),
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
            rh_foundation::ElementCardinality::new("MedicinalProductManufactured.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicinalProductManufactured.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicinalProductManufactured.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.contained",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.manufacturedDoseForm",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.unitOfPresentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.quantity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.manufacturer",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.ingredient",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.physicalCharacteristics",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductManufactured.otherCharacteristics",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductManufactured {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductManufactured {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductManufactured {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductManufactured {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductManufactured {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductManufactured {
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

impl crate::traits::medicinal_product_manufactured::MedicinalProductManufacturedAccessors
    for MedicinalProductManufactured
{
    fn manufactured_dose_form(&self) -> CodeableConcept {
        self.manufactured_dose_form.clone()
    }
    fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        self.unit_of_presentation.clone()
    }
    fn quantity(&self) -> Quantity {
        self.quantity.clone()
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[Reference] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn physical_characteristics(&self) -> Option<ProdCharacteristic> {
        self.physical_characteristics.clone()
    }
    fn other_characteristics(&self) -> &[CodeableConcept] {
        self.other_characteristics.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medicinal_product_manufactured::MedicinalProductManufacturedMutators
    for MedicinalProductManufactured
{
    fn new() -> Self {
        Self::default()
    }
    fn set_manufactured_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.manufactured_dose_form = value;
        resource
    }
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.unit_of_presentation = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = value;
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
    fn set_ingredient(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_physical_characteristics(self, value: ProdCharacteristic) -> Self {
        let mut resource = self.clone();
        resource.physical_characteristics = Some(value);
        resource
    }
    fn set_other_characteristics(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.other_characteristics = Some(value);
        resource
    }
    fn add_other_characteristics(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .other_characteristics
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medicinal_product_manufactured::MedicinalProductManufacturedExistence
    for MedicinalProductManufactured
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
    fn has_manufactured_dose_form(&self) -> bool {
        true
    }
    fn has_unit_of_presentation(&self) -> bool {
        self.unit_of_presentation.is_some()
    }
    fn has_quantity(&self) -> bool {
        true
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_physical_characteristics(&self) -> bool {
        self.physical_characteristics.is_some()
    }
    fn has_other_characteristics(&self) -> bool {
        self.other_characteristics
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicinalProductManufactured {
    fn resource_type(&self) -> &'static str {
        "MedicinalProductManufactured"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/MedicinalProductManufactured")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medicinal_product_manufactured::{
    MedicinalProductManufacturedAccessors, MedicinalProductManufacturedExistence,
    MedicinalProductManufacturedMutators,
};
