use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AdministrableProductDefinition
///
/// A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdministrableProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrableProductDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// An identifier for the administrable product
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product
    #[serde(rename = "formOf")]
    pub form_of: Option<Vec<Reference>>,
    /// The dose form of the final product after necessary reconstitution or processing
    ///
    /// Binding: example (Dose form for a medication, in the form suitable for administering to the patient, after mixing, where necessary.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/administrable-dose-form
    #[serde(rename = "administrableDoseForm")]
    pub administrable_dose_form: Option<CodeableConcept>,
    /// The presentation type in which this item is given to a patient. e.g. for a spray - 'puff'
    ///
    /// Binding: example (The presentation type in which an administrable medicinal product is given to a patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/unit-of-presentation
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form
    #[serde(rename = "producedFrom")]
    pub produced_from: Option<Vec<Reference>>,
    /// The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton, or using by incoming references from the Ingredient resource
    ///
    /// Binding: example (This value set includes all substance codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-codes
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product
    pub device: Option<Reference>,
    /// A general description of the product, when in its final form, suitable for administration e.g. effervescent blue liquid, to be swallowed
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Characteristics e.g. a product's onset of action
    pub property: Option<Vec<AdministrableProductDefinitionProperty>>,
    /// The path by which the product is taken into or makes contact with the body
    #[serde(rename = "routeOfAdministration")]
    pub route_of_administration: Vec<AdministrableProductDefinitionRouteofadministration>,
}
/// AdministrableProductDefinition nested structure for the 'routeOfAdministration' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrableProductDefinitionRouteofadministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A species for which this route applies
    #[serde(rename = "targetSpecies")]
    pub target_species:
        Option<Vec<AdministrableProductDefinitionRouteofadministrationTargetspecies>>,
    /// Coded expression for the route
    ///
    /// Binding: example (A code specifying the route or physiological path of administration of a therapeutic agent into or onto a patient's body.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/route-codes
    pub code: CodeableConcept,
    /// The first dose (dose quantity) administered can be specified for the product
    #[serde(rename = "firstDose")]
    pub first_dose: Option<Quantity>,
    /// The maximum single dose that can be administered
    #[serde(rename = "maxSingleDose")]
    pub max_single_dose: Option<Quantity>,
    /// The maximum dose quantity to be administered in any one 24-h period
    #[serde(rename = "maxDosePerDay")]
    pub max_dose_per_day: Option<Quantity>,
    /// The maximum dose per treatment period that can be administered
    #[serde(rename = "maxDosePerTreatmentPeriod")]
    pub max_dose_per_treatment_period: Option<Ratio>,
    /// The maximum treatment period during which the product can be administered
    #[serde(rename = "maxTreatmentPeriod")]
    pub max_treatment_period: Option<Duration>,
}
/// AdministrableProductDefinitionRouteofadministration nested structure for the 'targetSpecies' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrableProductDefinitionRouteofadministrationTargetspecies {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coded expression for the species
    ///
    /// Binding: example (A tissue type of an animal.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/target-species
    pub code: CodeableConcept,
}
/// AdministrableProductDefinition nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrableProductDefinitionProperty {
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
    /// The status of characteristic e.g. assigned or pending
    pub status: Option<CodeableConcept>,
}

impl Default for AdministrableProductDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            form_of: Default::default(),
            administrable_dose_form: Default::default(),
            unit_of_presentation: Default::default(),
            produced_from: Default::default(),
            ingredient: Default::default(),
            device: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            property: Default::default(),
            route_of_administration: Vec::new(),
        }
    }
}

impl Default for AdministrableProductDefinitionRouteofadministration {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target_species: Default::default(),
            code: Default::default(),
            first_dose: Default::default(),
            max_single_dose: Default::default(),
            max_dose_per_day: Default::default(),
            max_dose_per_treatment_period: Default::default(),
            max_treatment_period: Default::default(),
        }
    }
}

impl Default for AdministrableProductDefinitionRouteofadministrationTargetspecies {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
        }
    }
}

impl Default for AdministrableProductDefinitionProperty {
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
            status: Default::default(),
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
    rh_foundation::Invariant::new("apd-1", rh_foundation::Severity::Error, "RouteOfAdministration cannot be used when the 'formOf' product already uses MedicinalProductDefinition.route (and vice versa)", "AdministrableProductDefinition.formOf.resolve().route.empty()"),
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
                "AdministrableProductDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "AdministrableProductDefinition.property.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
            rh_foundation::ElementBinding::new(
                "AdministrableProductDefinition.status",
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
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.meta", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.implicitRules", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.text", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.contained", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.extension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.identifier", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.status", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.formOf", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.administrableDoseForm", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.unitOfPresentation", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.producedFrom", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.ingredient", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.device", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.description", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.extension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.value[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.property.status", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration", 1, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.extension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.code", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.firstDose", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.maxSingleDose", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.maxDosePerDay", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.maxDosePerTreatmentPeriod", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.maxTreatmentPeriod", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.extension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.code", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.extension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.tissue", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.value", 1, Some(1)),
    rh_foundation::ElementCardinality::new("AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod.supportingInformation", 0, Some(1)),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for AdministrableProductDefinition {
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

impl crate::traits::resource::ResourceMutators for AdministrableProductDefinition {
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

impl crate::traits::resource::ResourceExistence for AdministrableProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for AdministrableProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for AdministrableProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for AdministrableProductDefinition {
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

impl crate::traits::administrable_product_definition::AdministrableProductDefinitionAccessors
    for AdministrableProductDefinition
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn form_of(&self) -> &[Reference] {
        self.form_of.as_deref().unwrap_or(&[])
    }
    fn administrable_dose_form(&self) -> Option<CodeableConcept> {
        self.administrable_dose_form.clone()
    }
    fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        self.unit_of_presentation.clone()
    }
    fn produced_from(&self) -> &[Reference] {
        self.produced_from.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[CodeableConcept] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn device(&self) -> Option<Reference> {
        self.device.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn property(&self) -> &[AdministrableProductDefinitionProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn route_of_administration(&self) -> &[AdministrableProductDefinitionRouteofadministration] {
        &self.route_of_administration
    }
}

impl crate::traits::administrable_product_definition::AdministrableProductDefinitionMutators
    for AdministrableProductDefinition
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
    fn set_form_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.form_of = Some(value);
        resource
    }
    fn add_form_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.form_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_administrable_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.administrable_dose_form = Some(value);
        resource
    }
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.unit_of_presentation = Some(value);
        resource
    }
    fn set_produced_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.produced_from = Some(value);
        resource
    }
    fn add_produced_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .produced_from
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
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_property(self, value: Vec<AdministrableProductDefinitionProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: AdministrableProductDefinitionProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_route_of_administration(
        self,
        value: Vec<AdministrableProductDefinitionRouteofadministration>,
    ) -> Self {
        let mut resource = self.clone();
        resource.route_of_administration = value;
        resource
    }
    fn add_route_of_administration(
        self,
        item: AdministrableProductDefinitionRouteofadministration,
    ) -> Self {
        let mut resource = self.clone();
        resource.route_of_administration.push(item);
        resource
    }
}

impl crate::traits::administrable_product_definition::AdministrableProductDefinitionExistence
    for AdministrableProductDefinition
{
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_form_of(&self) -> bool {
        self.form_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_administrable_dose_form(&self) -> bool {
        self.administrable_dose_form.is_some()
    }
    fn has_unit_of_presentation(&self) -> bool {
        self.unit_of_presentation.is_some()
    }
    fn has_produced_from(&self) -> bool {
        self.produced_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_device(&self) -> bool {
        self.device.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_route_of_administration(&self) -> bool {
        !self.route_of_administration.is_empty()
    }
}

impl crate::validation::ValidatableResource for AdministrableProductDefinition {
    fn resource_type(&self) -> &'static str {
        "AdministrableProductDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::administrable_product_definition::{
    AdministrableProductDefinitionAccessors, AdministrableProductDefinitionExistence,
    AdministrableProductDefinitionMutators,
};
