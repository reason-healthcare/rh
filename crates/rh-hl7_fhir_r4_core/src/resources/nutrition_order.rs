use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// NutritionOrder
///
/// A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NutritionOrder
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers assigned to this order
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Instantiates protocol or definition
    pub instantiates: Option<Vec<StringType>>,
    /// Extension element for the 'instantiates' primitive field. Contains metadata and extensions.
    pub _instantiates: Option<Element>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// The person who requires the diet, formula or nutritional supplement
    pub patient: Reference,
    /// The encounter associated with this nutrition order
    pub encounter: Option<Reference>,
    /// Date and time the nutrition order was requested
    #[serde(rename = "dateTime")]
    pub date_time: DateTimeType,
    /// Extension element for the 'dateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateTime")]
    pub _date_time: Option<Element>,
    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<Reference>,
    /// List of the patient's food and nutrition-related allergies and intolerances
    #[serde(rename = "allergyIntolerance")]
    pub allergy_intolerance: Option<Vec<Reference>>,
    /// Order-specific modifier about the type of food that should be given
    ///
    /// Binding: example (Medical, cultural or ethical food preferences to help with catering requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diet
    #[serde(rename = "foodPreferenceModifier")]
    pub food_preference_modifier: Option<Vec<CodeableConcept>>,
    /// Order-specific modifier about the type of food that should not be given
    ///
    /// Binding: example (Codes used to indicate the type of food that should NOT be given to the patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/food-type
    #[serde(rename = "excludeFoodModifier")]
    pub exclude_food_modifier: Option<Vec<CodeableConcept>>,
    /// Oral diet components
    #[serde(rename = "oralDiet")]
    pub oral_diet: Option<NutritionOrderOraldiet>,
    /// Supplement components
    pub supplement: Option<Vec<NutritionOrderSupplement>>,
    /// Enteral formula components
    #[serde(rename = "enteralFormula")]
    pub enteral_formula: Option<NutritionOrderEnteralformula>,
    /// Comments
    pub note: Option<Vec<Annotation>>,
}
/// NutritionOrder nested structure for the 'enteralFormula' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformula {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Formula feeding instruction as structured data
    pub administration: Option<Vec<NutritionOrderEnteralformulaAdministration>>,
    /// Type of enteral or infant formula
    ///
    /// Binding: example (Codes for type of enteral formula to be administered to patient.)
    ///
    /// Available values:
    /// - `443031000124106`: Adult critical care formula
    /// - `443051000124104`: Adult diabetes specialty formula
    /// - `442911000124109`: Adult elemental formula
    /// - `443021000124108`: Adult hepatic specialty formula
    /// - `442971000124100`: Adult high energy formula
    /// - `442981000124102`: Adult hydrolyzed protein formula
    /// - `442991000124104`: Adult high protein formula
    /// - `443011000124100`: Adult high protein high fiber formula
    /// - `442961000124107`: Adult low carbohydrate formula
    /// - `442951000124105`: Adult pulmonary specialty formula
    /// - ... and 26 more values
    #[serde(rename = "baseFormulaType")]
    pub base_formula_type: Option<CodeableConcept>,
    /// Product or brand name of the enteral or infant formula
    #[serde(rename = "baseFormulaProductName")]
    pub base_formula_product_name: Option<StringType>,
    /// Extension element for the 'baseFormulaProductName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_baseFormulaProductName")]
    pub _base_formula_product_name: Option<Element>,
    /// Type of modular component to add to the feeding
    ///
    /// Binding: example (Codes for the type of modular component such as protein, carbohydrate or fiber to be provided in addition to or mixed with the base formula.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/entformula-additive
    #[serde(rename = "additiveType")]
    pub additive_type: Option<CodeableConcept>,
    /// Product or brand name of the modular additive
    #[serde(rename = "additiveProductName")]
    pub additive_product_name: Option<StringType>,
    /// Extension element for the 'additiveProductName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_additiveProductName")]
    pub _additive_product_name: Option<Element>,
    /// Amount of energy per specified volume that is required
    #[serde(rename = "caloricDensity")]
    pub caloric_density: Option<Quantity>,
    /// How the formula should enter the patient's gastrointestinal tract
    ///
    /// Binding: extensible (Codes specifying the route of administration of enteral formula.)
    ///
    /// Available values:
    /// - `PO`
    /// - `EFT`
    /// - `ENTINSTL`
    /// - `GT`
    /// - `NGT`
    /// - `OGT`
    /// - `GJT`
    /// - `JJTINSTL`
    /// - `OJJ`
    #[serde(rename = "routeofAdministration")]
    pub routeof_administration: Option<CodeableConcept>,
    /// Upper limit on formula volume per unit of time
    #[serde(rename = "maxVolumeToDeliver")]
    pub max_volume_to_deliver: Option<Quantity>,
    /// Formula feeding instructions expressed as text
    #[serde(rename = "administrationInstruction")]
    pub administration_instruction: Option<StringType>,
    /// Extension element for the 'administrationInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_administrationInstruction")]
    pub _administration_instruction: Option<Element>,
}
/// NutritionOrderOraldiet nested structure for the 'nutrient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderOraldietNutrient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of nutrient that is being modified
    ///
    /// Binding: example (Codes for types of nutrients that are being modified such as carbohydrate or sodium.)
    ///
    /// Available values:
    /// - `33463005`: Fluid
    /// - `39972003`: Sodium
    /// - `88480006`: Potassium
    pub modifier: Option<CodeableConcept>,
    /// Quantity of the specified nutrient
    pub amount: Option<Quantity>,
}
/// NutritionOrder nested structure for the 'oralDiet' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderOraldiet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Required  texture modifications
    pub texture: Option<Vec<NutritionOrderOraldietTexture>>,
    /// Required  nutrient modifications
    pub nutrient: Option<Vec<NutritionOrderOraldietNutrient>>,
    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    ///
    /// Binding: example (Codes used to indicate the type of diet being ordered for a patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diet-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Scheduled frequency of diet
    pub schedule: Option<Vec<Timing>>,
    /// The required consistency of fluids and liquids provided to the patient
    ///
    /// Binding: example (Codes used to represent the consistency of fluids and liquids provided to the patient.)
    ///
    /// Available values:
    /// - `439031000124108`: honey thick liquid
    /// - `439021000124105`: nectar thick liquid
    /// - `439041000124103`: spoon thick liquid
    /// - `439081000124109`: thin liquid
    #[serde(rename = "fluidConsistencyType")]
    pub fluid_consistency_type: Option<Vec<CodeableConcept>>,
    /// Instructions or additional information about the oral diet
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
}
/// NutritionOrder nested structure for the 'supplement' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderSupplement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of supplement product requested
    ///
    /// Binding: example (Codes for nutritional supplements to be provided to the patient.)
    ///
    /// Available values:
    /// - `442901000124106`: Adult clear liquid supplement
    /// - `443031000124106`: Adult critical care formula
    /// - `443051000124104`: Adult diabetes specialty formula
    /// - `442911000124109`: Adult elemental formula
    /// - `443021000124108`: Adult hepatic specialty formula
    /// - `442971000124100`: Adult high energy formula
    /// - `442981000124102`: Adult hydrolyzed protein formula
    /// - `442991000124104`: Adult high protein formula
    /// - `443011000124100`: Adult high protein high fiber formula
    /// - `442961000124107`: Adult low carbohydrate formula
    /// - ... and 35 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Product or brand name of the nutritional supplement
    #[serde(rename = "productName")]
    pub product_name: Option<StringType>,
    /// Extension element for the 'productName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productName")]
    pub _product_name: Option<Element>,
    /// Scheduled frequency of supplement
    pub schedule: Option<Vec<Timing>>,
    /// Amount of the nutritional supplement
    pub quantity: Option<Quantity>,
    /// Instructions or additional information about the oral supplement
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
}
/// NutritionOrderEnteralformula nested structure for the 'administration' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformulaAdministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Scheduled frequency of enteral feeding
    pub schedule: Option<Timing>,
    /// The volume of formula to provide
    pub quantity: Option<Quantity>,
    /// Speed with which the formula is provided per period of time (Quantity)
    #[serde(rename = "rateQuantity")]
    pub rate_quantity: Option<Quantity>,
    /// Speed with which the formula is provided per period of time (Ratio)
    #[serde(rename = "rateRatio")]
    pub rate_ratio: Option<Ratio>,
}
/// NutritionOrderOraldiet nested structure for the 'texture' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderOraldietTexture {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code to indicate how to alter the texture of the foods, e.g. pureed
    ///
    /// Binding: example (Codes for food consistency types or texture modifications to apply to foods.)
    ///
    /// Available values:
    /// - `228053002`: Cut-up food
    /// - `439091000124107`: Easy to chew food
    /// - `228049004`: Chopped food
    /// - `441881000124103`: Ground food
    /// - `441761000124103`: Minced food
    /// - `441751000124100`: Mashed food
    /// - `228059003`: Soft food
    /// - `441791000124106`: Strained food
    /// - `228055009`: Liquidized food
    /// - `228056005`: Lumpy food
    /// - ... and 4 more values
    pub modifier: Option<CodeableConcept>,
    /// Concepts that are used to identify an entity that is ingested for nutritional purposes
    ///
    /// Binding: example (Codes for types of foods that are texture-modified.)
    ///
    /// Available values:
    /// - `255620007`: Foods
    /// - `28647000`: Meat
    /// - `22836000`: Vegetables
    /// - `72511004`: Fruit
    /// - `226760005`: Dairy foods
    /// - `226887002`: Dietary Fats and Oils
    /// - `102263004`: Eggs
    /// - `74242007`: Food Starch
    /// - `227415002`: Fruit Nuts and Seeds
    /// - `264331002`: Grain
    /// - ... and 4 more values
    #[serde(rename = "foodType")]
    pub food_type: Option<CodeableConcept>,
}

impl Default for NutritionOrder {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            instantiates: Default::default(),
            _instantiates: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            patient: Reference::default(),
            encounter: Default::default(),
            date_time: DateTimeType::default(),
            _date_time: Default::default(),
            orderer: Default::default(),
            allergy_intolerance: Default::default(),
            food_preference_modifier: Default::default(),
            exclude_food_modifier: Default::default(),
            oral_diet: Default::default(),
            supplement: Default::default(),
            enteral_formula: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for NutritionOrderEnteralformula {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            administration: Default::default(),
            base_formula_type: Default::default(),
            base_formula_product_name: Default::default(),
            _base_formula_product_name: Default::default(),
            additive_type: Default::default(),
            additive_product_name: Default::default(),
            _additive_product_name: Default::default(),
            caloric_density: Default::default(),
            routeof_administration: Default::default(),
            max_volume_to_deliver: Default::default(),
            administration_instruction: Default::default(),
            _administration_instruction: Default::default(),
        }
    }
}

impl Default for NutritionOrderOraldietNutrient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            modifier: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for NutritionOrderOraldiet {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            texture: Default::default(),
            nutrient: Default::default(),
            type_: Default::default(),
            schedule: Default::default(),
            fluid_consistency_type: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
        }
    }
}

impl Default for NutritionOrderSupplement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            product_name: Default::default(),
            _product_name: Default::default(),
            schedule: Default::default(),
            quantity: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
        }
    }
}

impl Default for NutritionOrderEnteralformulaAdministration {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            schedule: Default::default(),
            quantity: Default::default(),
            rate_quantity: Default::default(),
            rate_ratio: Default::default(),
        }
    }
}

impl Default for NutritionOrderOraldietTexture {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            modifier: Default::default(),
            food_type: Default::default(),
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
    rh_foundation::Invariant::new("nor-1", rh_foundation::Severity::Warning, "Nutrition Order SHALL contain either Oral Diet , Supplement, or Enteral Formula class", "oralDiet.exists() or supplement.exists() or enteralFormula.exists()").with_xpath("exists(f:oralDiet) or exists(f:supplement) or exists(f:enteralFormula)"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for NutritionOrder {
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

impl crate::traits::resource::ResourceMutators for NutritionOrder {
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

impl crate::traits::resource::ResourceExistence for NutritionOrder {
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

impl crate::traits::domain_resource::DomainResourceAccessors for NutritionOrder {
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

impl crate::traits::domain_resource::DomainResourceMutators for NutritionOrder {
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

impl crate::traits::domain_resource::DomainResourceExistence for NutritionOrder {
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

impl crate::traits::nutrition_order::NutritionOrderAccessors for NutritionOrder {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn instantiates(&self) -> &[StringType] {
        self.instantiates.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date_time(&self) -> DateTimeType {
        self.date_time.clone()
    }
    fn orderer(&self) -> Option<Reference> {
        self.orderer.clone()
    }
    fn allergy_intolerance(&self) -> &[Reference] {
        self.allergy_intolerance.as_deref().unwrap_or(&[])
    }
    fn food_preference_modifier(&self) -> &[CodeableConcept] {
        self.food_preference_modifier.as_deref().unwrap_or(&[])
    }
    fn exclude_food_modifier(&self) -> &[CodeableConcept] {
        self.exclude_food_modifier.as_deref().unwrap_or(&[])
    }
    fn oral_diet(&self) -> Option<NutritionOrderOraldiet> {
        self.oral_diet.clone()
    }
    fn supplement(&self) -> &[NutritionOrderSupplement] {
        self.supplement.as_deref().unwrap_or(&[])
    }
    fn enteral_formula(&self) -> Option<NutritionOrderEnteralformula> {
        self.enteral_formula.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::nutrition_order::NutritionOrderMutators for NutritionOrder {
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
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates = Some(value);
        resource
    }
    fn add_instantiates(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_date_time(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_time = value;
        resource
    }
    fn set_orderer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.orderer = Some(value);
        resource
    }
    fn set_allergy_intolerance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.allergy_intolerance = Some(value);
        resource
    }
    fn add_allergy_intolerance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .allergy_intolerance
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_food_preference_modifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.food_preference_modifier = Some(value);
        resource
    }
    fn add_food_preference_modifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .food_preference_modifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_exclude_food_modifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.exclude_food_modifier = Some(value);
        resource
    }
    fn add_exclude_food_modifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .exclude_food_modifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_oral_diet(self, value: NutritionOrderOraldiet) -> Self {
        let mut resource = self.clone();
        resource.oral_diet = Some(value);
        resource
    }
    fn set_supplement(self, value: Vec<NutritionOrderSupplement>) -> Self {
        let mut resource = self.clone();
        resource.supplement = Some(value);
        resource
    }
    fn add_supplement(self, item: NutritionOrderSupplement) -> Self {
        let mut resource = self.clone();
        resource.supplement.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_enteral_formula(self, value: NutritionOrderEnteralformula) -> Self {
        let mut resource = self.clone();
        resource.enteral_formula = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::nutrition_order::NutritionOrderExistence for NutritionOrder {
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
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates(&self) -> bool {
        self.instantiates.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date_time(&self) -> bool {
        true
    }
    fn has_orderer(&self) -> bool {
        self.orderer.is_some()
    }
    fn has_allergy_intolerance(&self) -> bool {
        self.allergy_intolerance
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_food_preference_modifier(&self) -> bool {
        self.food_preference_modifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_exclude_food_modifier(&self) -> bool {
        self.exclude_food_modifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_oral_diet(&self) -> bool {
        self.oral_diet.is_some()
    }
    fn has_supplement(&self) -> bool {
        self.supplement.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_enteral_formula(&self) -> bool {
        self.enteral_formula.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for NutritionOrder {
    fn resource_type(&self) -> &'static str {
        "NutritionOrder"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/NutritionOrder")
    }
}
