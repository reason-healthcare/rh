use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionOrder
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<StringType>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _instantiates_canonical: Vec<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _instantiates_uri: Vec<Element>,
    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates: Vec<StringType>,
    /// Extension element for the 'instantiates' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _instantiates: Vec<Element>,
    /// What this order fulfills
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// Composite Request ID
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Who requires the diet, formula or nutritional supplement
    pub subject: Reference,
    /// The encounter associated with this nutrition order
    pub encounter: Option<Reference>,
    /// Information to support fulfilling of the nutrition order
    #[serde(rename = "supportingInformation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<Reference>,
    /// Date and time the nutrition order was requested
    #[serde(rename = "dateTime")]
    pub date_time: DateTimeType,
    /// Extension element for the 'dateTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateTime")]
    pub _date_time: Option<Element>,
    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<Reference>,
    /// Who is desired to perform the administration of what is being ordered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<CodeableReference>,
    /// List of the patient's food and nutrition-related allergies and intolerances
    #[serde(rename = "allergyIntolerance")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergy_intolerance: Vec<Reference>,
    /// Order-specific modifier about the type of food that should be given
    ///
    /// Binding: example (Medical, cultural or ethical food preferences to help with catering requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diet
    #[serde(rename = "foodPreferenceModifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub food_preference_modifier: Vec<CodeableConcept>,
    /// Order-specific modifier about the type of food that should not be given
    ///
    /// Binding: example (Codes used to indicate the type of food that should NOT be given to the patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/food-type
    #[serde(rename = "excludeFoodModifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_food_modifier: Vec<CodeableConcept>,
    /// Capture when a food item is brought in by the patient and/or family
    #[serde(rename = "outsideFoodAllowed")]
    pub outside_food_allowed: Option<BooleanType>,
    /// Extension element for the 'outsideFoodAllowed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_outsideFoodAllowed")]
    pub _outside_food_allowed: Option<Element>,
    /// Oral diet components
    #[serde(rename = "oralDiet")]
    pub oral_diet: Option<NutritionOrderOraldiet>,
    /// Supplement components
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplement: Vec<NutritionOrderSupplement>,
    /// Enteral formula components
    #[serde(rename = "enteralFormula")]
    pub enteral_formula: Option<NutritionOrderEnteralformula>,
    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// NutritionOrder nested structure for the 'enteralFormula' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformula {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Components to add to the feeding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<NutritionOrderEnteralformulaAdditive>,
    /// Formula feeding instruction as structured data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administration: Vec<NutritionOrderEnteralformulaAdministration>,
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
    pub base_formula_type: Option<CodeableReference>,
    /// Product or brand name of the enteral or infant formula
    #[serde(rename = "baseFormulaProductName")]
    pub base_formula_product_name: Option<StringType>,
    /// Extension element for the 'baseFormulaProductName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_baseFormulaProductName")]
    pub _base_formula_product_name: Option<Element>,
    /// Intended type of device for the administration
    #[serde(rename = "deliveryDevice")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_device: Vec<CodeableReference>,
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
    #[serde(rename = "routeOfAdministration")]
    pub route_of_administration: Option<CodeableConcept>,
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
/// NutritionOrderEnteralformula nested structure for the 'additive' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformulaAdditive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of modular component to add to the feeding
    ///
    /// Binding: example (Codes for the type of modular component such as protein, carbohydrate or fiber to be provided in addition to or mixed with the base formula.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/entformula-additive
    #[serde(rename = "type")]
    pub type_: Option<CodeableReference>,
    /// Product or brand name of the modular additive
    #[serde(rename = "productName")]
    pub product_name: Option<StringType>,
    /// Extension element for the 'productName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productName")]
    pub _product_name: Option<Element>,
    /// Amount of additive to be given or mixed in
    pub quantity: Option<Quantity>,
}
/// NutritionOrderEnteralformula nested structure for the 'administration' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformulaAdministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The volume of formula to provide
    pub quantity: Option<Quantity>,
    /// Speed with which the formula is provided per period of time (Quantity)
    #[serde(rename = "rateQuantity")]
    pub rate_quantity: Option<Quantity>,
    /// Speed with which the formula is provided per period of time (Ratio)
    #[serde(rename = "rateRatio")]
    pub rate_ratio: Option<Ratio>,
}
/// NutritionOrderEnteralformulaAdministration nested structure for the 'schedule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderEnteralformulaAdministrationSchedule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Scheduled frequency of enteral formula
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<Timing>,
    /// Take 'as needed'
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<BooleanType>,
    /// Extension element for the 'asNeeded' primitive field. Contains metadata and extensions.
    #[serde(rename = "_asNeeded")]
    pub _as_needed: Option<Element>,
    /// Take 'as needed' for x
    ///
    /// Binding: example (A coded concept identifying the precondition that should be met or evaluated prior to       consuming an enteral formula.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-as-needed-reason
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Option<CodeableConcept>,
}
/// NutritionOrder nested structure for the 'oralDiet' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderOraldiet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Required  nutrient modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrient: Vec<NutritionOrderOraldietNutrient>,
    /// Scheduling information for oral diets
    pub schedule: Option<NutritionOrderOraldietSchedule>,
    /// Required  texture modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texture: Vec<NutritionOrderOraldietTexture>,
    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    ///
    /// Binding: example (Codes used to indicate the type of diet being ordered for a patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diet-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fluid_consistency_type: Vec<CodeableConcept>,
    /// Instructions or additional information about the oral diet
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
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
/// NutritionOrderOraldiet nested structure for the 'schedule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderOraldietSchedule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Scheduled frequency of diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<Timing>,
    /// Take 'as needed'
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<BooleanType>,
    /// Extension element for the 'asNeeded' primitive field. Contains metadata and extensions.
    #[serde(rename = "_asNeeded")]
    pub _as_needed: Option<Element>,
    /// Take 'as needed' for x
    ///
    /// Binding: example (A coded concept identifying the precondition that should be met or evaluated prior to       consuming a nutrition product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-as-needed-reason
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Option<CodeableConcept>,
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
/// NutritionOrder nested structure for the 'supplement' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderSupplement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Scheduling information for supplements
    pub schedule: Option<NutritionOrderSupplementSchedule>,
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
    pub type_: Option<CodeableReference>,
    /// Product or brand name of the nutritional supplement
    #[serde(rename = "productName")]
    pub product_name: Option<StringType>,
    /// Extension element for the 'productName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productName")]
    pub _product_name: Option<Element>,
    /// Amount of the nutritional supplement
    pub quantity: Option<Quantity>,
    /// Instructions or additional information about the oral supplement
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
}
/// NutritionOrderSupplement nested structure for the 'schedule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionOrderSupplementSchedule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Scheduled frequency of diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<Timing>,
    /// Take 'as needed'
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<BooleanType>,
    /// Extension element for the 'asNeeded' primitive field. Contains metadata and extensions.
    #[serde(rename = "_asNeeded")]
    pub _as_needed: Option<Element>,
    /// Take 'as needed' for x
    ///
    /// Binding: example (A coded concept identifying the precondition that should be met or evaluated prior to       consuming a supplement.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-as-needed-reason
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Option<CodeableConcept>,
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
            based_on: Default::default(),
            group_identifier: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            date_time: DateTimeType::default(),
            _date_time: Default::default(),
            orderer: Default::default(),
            performer: Default::default(),
            allergy_intolerance: Default::default(),
            food_preference_modifier: Default::default(),
            exclude_food_modifier: Default::default(),
            outside_food_allowed: Default::default(),
            _outside_food_allowed: Default::default(),
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
            additive: Default::default(),
            administration: Default::default(),
            base_formula_type: Default::default(),
            base_formula_product_name: Default::default(),
            _base_formula_product_name: Default::default(),
            delivery_device: Default::default(),
            caloric_density: Default::default(),
            route_of_administration: Default::default(),
            max_volume_to_deliver: Default::default(),
            administration_instruction: Default::default(),
            _administration_instruction: Default::default(),
        }
    }
}

impl Default for NutritionOrderEnteralformulaAdditive {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            product_name: Default::default(),
            _product_name: Default::default(),
            quantity: Default::default(),
        }
    }
}

impl Default for NutritionOrderEnteralformulaAdministration {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            rate_quantity: Default::default(),
            rate_ratio: Default::default(),
        }
    }
}

impl Default for NutritionOrderEnteralformulaAdministrationSchedule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            timing: Default::default(),
            as_needed: Default::default(),
            _as_needed: Default::default(),
            as_needed_for: Default::default(),
        }
    }
}

impl Default for NutritionOrderOraldiet {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            nutrient: Default::default(),
            schedule: Default::default(),
            texture: Default::default(),
            type_: Default::default(),
            fluid_consistency_type: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
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

impl Default for NutritionOrderOraldietSchedule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            timing: Default::default(),
            as_needed: Default::default(),
            _as_needed: Default::default(),
            as_needed_for: Default::default(),
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

impl Default for NutritionOrderSupplement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            schedule: Default::default(),
            type_: Default::default(),
            product_name: Default::default(),
            _product_name: Default::default(),
            quantity: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
        }
    }
}

impl Default for NutritionOrderSupplementSchedule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            timing: Default::default(),
            as_needed: Default::default(),
            _as_needed: Default::default(),
            as_needed_for: Default::default(),
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
    rh_foundation::Invariant::new("nor-1", rh_foundation::Severity::Warning, "Nutrition Order SHALL contain either Oral Diet , Supplement, or Enteral Formula class", "oralDiet.exists() or supplement.exists() or enteralFormula.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("NutritionOrder.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|5.0.0").with_description("Codes indicating the degree of authority/intentionality associated with a nutrition order."),
    rh_foundation::ElementBinding::new("NutritionOrder.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("NutritionOrder.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("NutritionOrder.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|5.0.0").with_description("Codes identifying the lifecycle stage of the nutrition order."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NutritionOrder.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.contained", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.extension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.identifier", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.instantiates", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.groupIdentifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.supportingInformation", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.dateTime", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.orderer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.performer", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.allergyIntolerance", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.foodPreferenceModifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.excludeFoodModifier", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.outsideFoodAllowed", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.type", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.schedule", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.timing",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.asNeeded",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.schedule.asNeededFor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.nutrient", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.nutrient.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.nutrient.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.nutrient.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.nutrient.modifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.nutrient.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.oralDiet.texture", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.texture.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.texture.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.texture.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.texture.modifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.texture.foodType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.fluidConsistencyType",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.oralDiet.instruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.supplement", 0, None),
            rh_foundation::ElementCardinality::new("NutritionOrder.supplement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.supplement.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.supplement.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.productName",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.timing",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.asNeeded",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.schedule.asNeededFor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.supplement.instruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.enteralFormula", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionOrder.enteralFormula.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.baseFormulaType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.baseFormulaProductName",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.deliveryDevice",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.productName",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.additive.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.caloricDensity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.routeOfAdministration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.timing",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.asNeeded",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.schedule.asNeededFor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administration.rate[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.maxVolumeToDeliver",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionOrder.enteralFormula.administrationInstruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionOrder.note", 0, None),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for NutritionOrder {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::nutrition_order::NutritionOrderAccessors for NutritionOrder {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_slice()
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_slice()
    }
    fn instantiates(&self) -> &[StringType] {
        self.instantiates.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_slice()
    }
    fn date_time(&self) -> DateTimeType {
        self.date_time.clone()
    }
    fn orderer(&self) -> Option<Reference> {
        self.orderer.clone()
    }
    fn performer(&self) -> &[CodeableReference] {
        self.performer.as_slice()
    }
    fn allergy_intolerance(&self) -> &[Reference] {
        self.allergy_intolerance.as_slice()
    }
    fn food_preference_modifier(&self) -> &[CodeableConcept] {
        self.food_preference_modifier.as_slice()
    }
    fn exclude_food_modifier(&self) -> &[CodeableConcept] {
        self.exclude_food_modifier.as_slice()
    }
    fn outside_food_allowed(&self) -> Option<BooleanType> {
        self.outside_food_allowed
    }
    fn oral_diet(&self) -> Option<NutritionOrderOraldiet> {
        self.oral_diet.clone()
    }
    fn supplement(&self) -> &[NutritionOrderSupplement] {
        self.supplement.as_slice()
    }
    fn enteral_formula(&self) -> Option<NutritionOrderEnteralformula> {
        self.enteral_formula.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
}

impl crate::traits::nutrition_order::NutritionOrderMutators for NutritionOrder {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = value;
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical.push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = value;
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri.push(item);
        resource
    }
    fn set_instantiates(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates = value;
        resource
    }
    fn add_instantiates(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates.push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
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
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = value;
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.supporting_information.push(item);
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
    fn set_performer(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.performer = value;
        resource
    }
    fn add_performer(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.performer.push(item);
        resource
    }
    fn set_allergy_intolerance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.allergy_intolerance = value;
        resource
    }
    fn add_allergy_intolerance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.allergy_intolerance.push(item);
        resource
    }
    fn set_food_preference_modifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.food_preference_modifier = value;
        resource
    }
    fn add_food_preference_modifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.food_preference_modifier.push(item);
        resource
    }
    fn set_exclude_food_modifier(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.exclude_food_modifier = value;
        resource
    }
    fn add_exclude_food_modifier(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.exclude_food_modifier.push(item);
        resource
    }
    fn set_outside_food_allowed(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.outside_food_allowed = Some(value);
        resource
    }
    fn set_oral_diet(self, value: NutritionOrderOraldiet) -> Self {
        let mut resource = self.clone();
        resource.oral_diet = Some(value);
        resource
    }
    fn set_supplement(self, value: Vec<NutritionOrderSupplement>) -> Self {
        let mut resource = self.clone();
        resource.supplement = value;
        resource
    }
    fn add_supplement(self, item: NutritionOrderSupplement) -> Self {
        let mut resource = self.clone();
        resource.supplement.push(item);
        resource
    }
    fn set_enteral_formula(self, value: NutritionOrderEnteralformula) -> Self {
        let mut resource = self.clone();
        resource.enteral_formula = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
}

impl crate::traits::nutrition_order::NutritionOrderExistence for NutritionOrder {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_instantiates_canonical(&self) -> bool {
        !self.instantiates_canonical.is_empty()
    }
    fn has_instantiates_uri(&self) -> bool {
        !self.instantiates_uri.is_empty()
    }
    fn has_instantiates(&self) -> bool {
        !self.instantiates.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        !self.supporting_information.is_empty()
    }
    fn has_date_time(&self) -> bool {
        true
    }
    fn has_orderer(&self) -> bool {
        self.orderer.is_some()
    }
    fn has_performer(&self) -> bool {
        !self.performer.is_empty()
    }
    fn has_allergy_intolerance(&self) -> bool {
        !self.allergy_intolerance.is_empty()
    }
    fn has_food_preference_modifier(&self) -> bool {
        !self.food_preference_modifier.is_empty()
    }
    fn has_exclude_food_modifier(&self) -> bool {
        !self.exclude_food_modifier.is_empty()
    }
    fn has_outside_food_allowed(&self) -> bool {
        self.outside_food_allowed.is_some()
    }
    fn has_oral_diet(&self) -> bool {
        self.oral_diet.is_some()
    }
    fn has_supplement(&self) -> bool {
        !self.supplement.is_empty()
    }
    fn has_enteral_formula(&self) -> bool {
        self.enteral_formula.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
}

impl crate::validation::ValidatableResource for NutritionOrder {
    fn resource_type(&self) -> &'static str {
        "NutritionOrder"
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
        Some("http://hl7.org/fhir/StructureDefinition/NutritionOrder")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::nutrition_order::{
    NutritionOrderAccessors, NutritionOrderExistence, NutritionOrderMutators,
};
