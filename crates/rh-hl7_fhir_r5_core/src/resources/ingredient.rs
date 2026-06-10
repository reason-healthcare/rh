use crate::bindings::ingredient_manufacturer_role::IngredientManufacturerRole;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::ratio_range::RatioRange;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Ingredient
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Ingredient
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Ingredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// An identifier or code by which the ingredient can be referenced
    pub identifier: Option<Identifier>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The product which this ingredient is a constituent part of
    #[serde(rename = "for")]
    pub for_: Option<Vec<Reference>>,
    /// Purpose of the ingredient within the product, e.g. active, inactive
    ///
    /// Binding: example (A classification of the ingredient identifying its purpose within the product, e.g. active, inactive.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ingredient-role
    pub role: CodeableConcept,
    /// Precise action within the drug product, e.g. antioxidant, alkalizing agent
    ///
    /// Binding: example (A classification of the ingredient identifying its precise purpose(s) in the drug product (beyond e.g. active/inactive).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ingredient-function
    pub function: Option<Vec<CodeableConcept>>,
    /// A classification of the ingredient according to where in the physical item it tends to be used, such the outer shell of a tablet, inner body or ink
    pub group: Option<CodeableConcept>,
    /// If the ingredient is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<BooleanType>,
    /// Extension element for the 'allergenicIndicator' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allergenicIndicator")]
    pub _allergenic_indicator: Option<Element>,
    /// A place for providing any notes that are relevant to the component, e.g. removed during process, adjusted for loss on drying
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// An organization that manufactures this ingredient
    pub manufacturer: Option<Vec<IngredientManufacturer>>,
    /// The substance that comprises this ingredient
    pub substance: IngredientSubstance,
}
/// Ingredient nested structure for the 'substance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientSubstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The quantity of substance, per presentation, or per volume or mass, and type of quantity
    pub strength: Option<Vec<IngredientSubstanceStrength>>,
    /// A code or full resource that represents the ingredient substance
    ///
    /// Binding: example (This value set includes all substance codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-codes
    pub code: CodeableReference,
}
/// IngredientSubstance nested structure for the 'strength' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientSubstanceStrength {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The quantity of substance in the unit of presentation (Ratio)
    #[serde(rename = "presentationRatio")]
    pub presentation_ratio: Option<Ratio>,
    /// The quantity of substance in the unit of presentation (RatioRange)
    #[serde(rename = "presentationRatioRange")]
    pub presentation_ratio_range: Option<RatioRange>,
    /// The quantity of substance in the unit of presentation (CodeableConcept)
    #[serde(rename = "presentationCodeableConcept")]
    pub presentation_codeable_concept: Option<CodeableConcept>,
    /// The quantity of substance in the unit of presentation (Quantity)
    #[serde(rename = "presentationQuantity")]
    pub presentation_quantity: Option<Quantity>,
    /// Text of either the whole presentation strength or a part of it (rest being in Strength.presentation as a ratio)
    #[serde(rename = "textPresentation")]
    pub text_presentation: Option<StringType>,
    /// Extension element for the 'textPresentation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_textPresentation")]
    pub _text_presentation: Option<Element>,
    /// The strength per unitary volume (or mass) (Ratio)
    #[serde(rename = "concentrationRatio")]
    pub concentration_ratio: Option<Ratio>,
    /// The strength per unitary volume (or mass) (RatioRange)
    #[serde(rename = "concentrationRatioRange")]
    pub concentration_ratio_range: Option<RatioRange>,
    /// The strength per unitary volume (or mass) (CodeableConcept)
    #[serde(rename = "concentrationCodeableConcept")]
    pub concentration_codeable_concept: Option<CodeableConcept>,
    /// The strength per unitary volume (or mass) (Quantity)
    #[serde(rename = "concentrationQuantity")]
    pub concentration_quantity: Option<Quantity>,
    /// Text of either the whole concentration strength or a part of it (rest being in Strength.concentration as a ratio)
    #[serde(rename = "textConcentration")]
    pub text_concentration: Option<StringType>,
    /// Extension element for the 'textConcentration' primitive field. Contains metadata and extensions.
    #[serde(rename = "_textConcentration")]
    pub _text_concentration: Option<Element>,
    /// A code that indicates if the strength is, for example, based on the ingredient substance as stated or on the substance base (when the ingredient is a salt)
    pub basis: Option<CodeableConcept>,
    /// When strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<StringType>,
    /// Extension element for the 'measurementPoint' primitive field. Contains metadata and extensions.
    #[serde(rename = "_measurementPoint")]
    pub _measurement_point: Option<Element>,
    /// Where the strength range applies
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/country
    pub country: Option<Vec<CodeableConcept>>,
}
/// Ingredient nested structure for the 'manufacturer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientManufacturer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// allowed | possible | actual
    pub role: Option<IngredientManufacturerRole>,
    /// Extension element for the 'role' primitive field. Contains metadata and extensions.
    pub _role: Option<Element>,
    /// An organization that manufactures this ingredient
    pub manufacturer: Reference,
}

impl Default for Ingredient {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            for_: Default::default(),
            role: CodeableConcept::default(),
            function: Default::default(),
            group: Default::default(),
            allergenic_indicator: Default::default(),
            _allergenic_indicator: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            manufacturer: Default::default(),
            substance: IngredientSubstance::default(),
        }
    }
}

impl Default for IngredientSubstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            strength: Default::default(),
            code: CodeableReference::default(),
        }
    }
}

impl Default for IngredientSubstanceStrength {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            presentation_ratio: Default::default(),
            presentation_ratio_range: Default::default(),
            presentation_codeable_concept: Default::default(),
            presentation_quantity: Default::default(),
            text_presentation: Default::default(),
            _text_presentation: Default::default(),
            concentration_ratio: Default::default(),
            concentration_ratio_range: Default::default(),
            concentration_codeable_concept: Default::default(),
            concentration_quantity: Default::default(),
            text_concentration: Default::default(),
            _text_concentration: Default::default(),
            basis: Default::default(),
            measurement_point: Default::default(),
            _measurement_point: Default::default(),
            country: Default::default(),
        }
    }
}

impl Default for IngredientManufacturer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            _role: Default::default(),
            manufacturer: Reference::default(),
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
    rh_foundation::Invariant::new("ing-1", rh_foundation::Severity::Error, "If an ingredient is noted as an allergen (allergenicIndicator) then its substance should be a code. If the substance is a SubstanceDefinition, then the allegen information should be documented in that resource", "Ingredient.where(allergenicIndicator=true).count() + Ingredient.substance.code.reference.count()  < 2"),
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
                "Ingredient.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Ingredient.manufacturer.role",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/ingredient-manufacturer-role|5.0.0",
            )
            .with_description(
                "The way in which this manufacturer is associated with the ingredient.",
            ),
            rh_foundation::ElementBinding::new(
                "Ingredient.status",
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
            rh_foundation::ElementCardinality::new("Ingredient.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.contained", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.extension", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.for", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.role", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.function", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.group", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.allergenicIndicator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.manufacturer", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.manufacturer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.manufacturer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Ingredient.manufacturer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Ingredient.manufacturer.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Ingredient.manufacturer.manufacturer",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Ingredient.substance", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.substance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.substance.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Ingredient.substance.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Ingredient.substance.strength", 0, None),
            rh_foundation::ElementCardinality::new("Ingredient.substance.strength.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.presentation[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.textPresentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.concentration[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.textConcentration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.basis",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.measurementPoint",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.country",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.substance",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.strength[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.measurementPoint",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Ingredient.substance.strength.referenceStrength.country",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Ingredient {
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

impl crate::traits::resource::ResourceMutators for Ingredient {
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

impl crate::traits::resource::ResourceExistence for Ingredient {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Ingredient {
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

impl crate::traits::domain_resource::DomainResourceMutators for Ingredient {
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

impl crate::traits::domain_resource::DomainResourceExistence for Ingredient {
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

impl crate::traits::ingredient::IngredientAccessors for Ingredient {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn for_(&self) -> &[Reference] {
        self.for_.as_deref().unwrap_or(&[])
    }
    fn role(&self) -> CodeableConcept {
        self.role.clone()
    }
    fn function(&self) -> &[CodeableConcept] {
        self.function.as_deref().unwrap_or(&[])
    }
    fn group(&self) -> Option<CodeableConcept> {
        self.group.clone()
    }
    fn allergenic_indicator(&self) -> Option<BooleanType> {
        self.allergenic_indicator
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn manufacturer(&self) -> &[IngredientManufacturer] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn substance(&self) -> IngredientSubstance {
        self.substance.clone()
    }
}

impl crate::traits::ingredient::IngredientMutators for Ingredient {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_for_(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.for_ = Some(value);
        resource
    }
    fn add_for_(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.for_.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_role(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.role = value;
        resource
    }
    fn set_function(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.function = Some(value);
        resource
    }
    fn add_function(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.function.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_group(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.group = Some(value);
        resource
    }
    fn set_allergenic_indicator(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.allergenic_indicator = Some(value);
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Vec<IngredientManufacturer>) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn add_manufacturer(self, item: IngredientManufacturer) -> Self {
        let mut resource = self.clone();
        resource
            .manufacturer
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_substance(self, value: IngredientSubstance) -> Self {
        let mut resource = self.clone();
        resource.substance = value;
        resource
    }
}

impl crate::traits::ingredient::IngredientExistence for Ingredient {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_for_(&self) -> bool {
        self.for_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_role(&self) -> bool {
        true
    }
    fn has_function(&self) -> bool {
        self.function.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group(&self) -> bool {
        self.group.is_some()
    }
    fn has_allergenic_indicator(&self) -> bool {
        self.allergenic_indicator.is_some()
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_substance(&self) -> bool {
        true
    }
}

impl crate::validation::ValidatableResource for Ingredient {
    fn resource_type(&self) -> &'static str {
        "Ingredient"
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
        Some("http://hl7.org/fhir/StructureDefinition/Ingredient")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::ingredient::{IngredientAccessors, IngredientExistence, IngredientMutators};
