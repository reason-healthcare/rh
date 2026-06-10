use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ManufacturedItemDefinition
///
/// The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ManufacturedItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturedItemDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// A descriptive name applied to this item
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Dose form as manufactured (before any necessary transformation)
    ///
    /// Binding: example (Dose form for a medication, in the form suitable for administering to the patient, after mixing, where necessary.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/manufactured-dose-form
    #[serde(rename = "manufacturedDoseForm")]
    pub manufactured_dose_form: CodeableConcept,
    /// The “real-world” units in which the quantity of the item is described
    ///
    /// Binding: example (The presentation type in which an administrable medicinal product is given to a patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/unit-of-presentation
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// Manufacturer of the item, one of several possible
    pub manufacturer: Option<Vec<Reference>>,
    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// The ingredients of this manufactured item. Only needed if these are not specified by incoming references from the Ingredient resource
    ///
    /// Binding: example (This value set includes all substance codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-codes
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// General characteristics of this item
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,
    /// Physical parts of the manufactured item, that it is intrisically made from. This is distinct from the ingredients that are part of its chemical makeup
    pub component: Option<Vec<ManufacturedItemDefinitionComponent>>,
}
/// ManufacturedItemDefinition nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturedItemDefinitionProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code expressing the type of characteristic
    ///
    /// Binding: example (This value set includes all observable entity codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-characteristic-codes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// A value for the characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// A value for the characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// A value for the characteristic (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// A value for the characteristic (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// A value for the characteristic (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<StringType>,
    /// A value for the characteristic (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// A value for the characteristic (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
}
/// ManufacturedItemDefinition nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturedItemDefinitionComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A reference to a constituent of the manufactured item as a whole, linked here so that its component location within the item can be indicated. This not where the item's ingredient are primarily stated (for which see Ingredient.for or ManufacturedItemDefinition.ingredient)
    pub constituent: Option<Vec<ManufacturedItemDefinitionComponentConstituent>>,
    /// Defining type of the component e.g. shell, layer, ink
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The function of this component within the item e.g. delivers active ingredient, masks taste
    pub function: Option<Vec<CodeableConcept>>,
    /// The measurable amount of total quantity of all substances in the component, expressable in different ways (e.g. by mass or volume)
    pub amount: Option<Vec<Quantity>>,
    /// General characteristics of this component
    pub property: Option<Vec<StringType>>,
    /// A component that this component contains or is made from
    pub component: Option<Vec<StringType>>,
}
/// ManufacturedItemDefinitionComponent nested structure for the 'constituent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturedItemDefinitionComponentConstituent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The measurable amount of the substance, expressable in different ways (e.g. by mass or volume)
    pub amount: Option<Vec<Quantity>>,
    /// The physical location of the constituent/ingredient within the component
    pub location: Option<Vec<CodeableConcept>>,
    /// The function of this constituent within the component e.g. binder
    pub function: Option<Vec<CodeableConcept>>,
    /// The ingredient that is the constituent of the given component
    #[serde(rename = "hasIngredient")]
    pub has_ingredient: Option<Vec<CodeableReference>>,
}

impl Default for ManufacturedItemDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            manufactured_dose_form: CodeableConcept::default(),
            unit_of_presentation: Default::default(),
            manufacturer: Default::default(),
            marketing_status: Default::default(),
            ingredient: Default::default(),
            property: Default::default(),
            component: Default::default(),
        }
    }
}

impl Default for ManufacturedItemDefinitionProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_date: Default::default(),
            value_boolean: Default::default(),
            value_markdown: Default::default(),
            value_attachment: Default::default(),
            value_reference: Default::default(),
        }
    }
}

impl Default for ManufacturedItemDefinitionComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            constituent: Default::default(),
            type_: Default::default(),
            function: Default::default(),
            amount: Default::default(),
            property: Default::default(),
            component: Default::default(),
        }
    }
}

impl Default for ManufacturedItemDefinitionComponentConstituent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            amount: Default::default(),
            location: Default::default(),
            function: Default::default(),
            has_ingredient: Default::default(),
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
                "ManufacturedItemDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ManufacturedItemDefinition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.manufacturedDoseForm",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.unitOfPresentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.manufacturer",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.marketingStatus",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.ingredient",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.property", 0, None),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.property.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.property.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ManufacturedItemDefinition.component", 0, None),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.function",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.amount",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.amount",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.location",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.function",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.constituent.hasIngredient",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.property",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ManufacturedItemDefinition.component.component",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ManufacturedItemDefinition {
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

impl crate::traits::resource::ResourceMutators for ManufacturedItemDefinition {
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

impl crate::traits::resource::ResourceExistence for ManufacturedItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ManufacturedItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ManufacturedItemDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ManufacturedItemDefinition {
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

impl crate::traits::manufactured_item_definition::ManufacturedItemDefinitionAccessors
    for ManufacturedItemDefinition
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn manufactured_dose_form(&self) -> CodeableConcept {
        self.manufactured_dose_form.clone()
    }
    fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        self.unit_of_presentation.clone()
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn marketing_status(&self) -> &[MarketingStatus] {
        self.marketing_status.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[CodeableConcept] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[ManufacturedItemDefinitionProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn component(&self) -> &[ManufacturedItemDefinitionComponent] {
        self.component.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::manufactured_item_definition::ManufacturedItemDefinitionMutators
    for ManufacturedItemDefinition
{
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
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
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
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self {
        let mut resource = self.clone();
        resource.marketing_status = Some(value);
        resource
    }
    fn add_marketing_status(self, item: MarketingStatus) -> Self {
        let mut resource = self.clone();
        resource
            .marketing_status
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_ingredient(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<ManufacturedItemDefinitionProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: ManufacturedItemDefinitionProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_component(self, value: Vec<ManufacturedItemDefinitionComponent>) -> Self {
        let mut resource = self.clone();
        resource.component = Some(value);
        resource
    }
    fn add_component(self, item: ManufacturedItemDefinitionComponent) -> Self {
        let mut resource = self.clone();
        resource.component.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::manufactured_item_definition::ManufacturedItemDefinitionExistence
    for ManufacturedItemDefinition
{
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_manufactured_dose_form(&self) -> bool {
        true
    }
    fn has_unit_of_presentation(&self) -> bool {
        self.unit_of_presentation.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_marketing_status(&self) -> bool {
        self.marketing_status
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_component(&self) -> bool {
        self.component.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ManufacturedItemDefinition {
    fn resource_type(&self) -> &'static str {
        "ManufacturedItemDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::manufactured_item_definition::{
    ManufacturedItemDefinitionAccessors, ManufacturedItemDefinitionExistence,
    ManufacturedItemDefinitionMutators,
};
