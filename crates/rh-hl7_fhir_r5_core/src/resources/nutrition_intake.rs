use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// NutritionIntake
///
/// A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionIntake
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionIntake {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
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
    /// Fulfils plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: EventStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (A coded concept indicating the reason for the status of the statement.)
    ///
    /// Available values:
    /// - `397709008`
    /// - `105480006`
    /// - `719500002`
    /// - `445060000`
    /// - `704273008`
    /// - `704274002`
    /// - `704458005`
    /// - `704275001`
    /// - `704276000`
    /// - `704277009`
    /// - ... and 1 more values
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// Code representing an overall type of nutrition intake
    ///
    /// Binding: example (A coded concept identifying an overall type of diet or nutrition that is represented by this intake.  See consumedItem for more details.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diet-type
    pub code: Option<CodeableConcept>,
    /// Who is/was consuming the food or fluid
    pub subject: Reference,
    /// Encounter associated with NutritionIntake
    pub encounter: Option<Reference>,
    /// The date/time or interval when the food or fluid is/was consumed (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// The date/time or interval when the food or fluid is/was consumed (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When the intake was recorded
    pub recorded: Option<DateTimeType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Person or organization that provided the information about the consumption of this food or fluid (boolean)
    #[serde(rename = "reportedBoolean")]
    pub reported_boolean: Option<BooleanType>,
    /// Person or organization that provided the information about the consumption of this food or fluid (Reference)
    #[serde(rename = "reportedReference")]
    pub reported_reference: Option<Reference>,
    /// What food or fluid product or item was consumed
    #[serde(rename = "consumedItem")]
    pub consumed_item: Vec<NutritionIntakeConsumeditem>,
    /// Total nutrient for the whole meal, product, serving
    #[serde(rename = "ingredientLabel")]
    pub ingredient_label: Option<Vec<NutritionIntakeIngredientlabel>>,
    /// Who was performed in the intake
    pub performer: Option<Vec<NutritionIntakePerformer>>,
    /// Where the intake occurred
    pub location: Option<Reference>,
    /// Additional supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// Reason for why the food or fluid is /was consumed
    ///
    /// Binding: example (Reason for why something was ingested.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub reason: Option<Vec<CodeableReference>>,
    /// Further information about the consumption
    pub note: Option<Vec<Annotation>>,
}
/// NutritionIntake nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionIntakePerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of performer
    ///
    /// Binding: example (Type of performance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/performer-role
    pub function: Option<CodeableConcept>,
    /// Who performed the intake
    pub actor: Reference,
}
/// NutritionIntake nested structure for the 'ingredientLabel' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionIntakeIngredientlabel {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Total nutrient consumed
    ///
    /// Binding: example (Types of nutrients that can be found in a nutrition product.)
    ///
    /// Available values:
    /// - `33463005`: Fluid
    /// - `39972003`: Sodium
    /// - `88480006`: Potassium
    pub nutrient: CodeableReference,
    /// Total amount of nutrient consumed
    pub amount: Quantity,
}
/// NutritionIntake nested structure for the 'consumedItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionIntakeConsumeditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of food or fluid product
    ///
    /// Binding: example (Types of food.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/edible-substance-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Code that identifies the food or fluid product that was consumed
    ///
    /// Binding: example (Specific food that can be consumed by a patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/food-type
    #[serde(rename = "nutritionProduct")]
    pub nutrition_product: CodeableReference,
    /// Scheduled frequency of consumption
    pub schedule: Option<Timing>,
    /// Quantity of the specified food
    pub amount: Option<Quantity>,
    /// Rate at which enteral feeding was administered
    pub rate: Option<Quantity>,
    /// Flag to indicate if the food or fluid item was refused or otherwise not consumed
    #[serde(rename = "notConsumed")]
    pub not_consumed: Option<BooleanType>,
    /// Extension element for the 'notConsumed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_notConsumed")]
    pub _not_consumed: Option<Element>,
    /// Reason food or fluid was not consumed
    ///
    /// Binding: example (Reasons for why something was not consumed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/not-consumed-reason
    #[serde(rename = "notConsumedReason")]
    pub not_consumed_reason: Option<CodeableConcept>,
}

impl Default for NutritionIntake {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: EventStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            code: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            reported_boolean: Default::default(),
            reported_reference: Default::default(),
            consumed_item: Vec::new(),
            ingredient_label: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            derived_from: Default::default(),
            reason: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for NutritionIntakePerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for NutritionIntakeIngredientlabel {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            nutrient: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for NutritionIntakeConsumeditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            nutrition_product: Default::default(),
            schedule: Default::default(),
            amount: Default::default(),
            rate: Default::default(),
            not_consumed: Default::default(),
            _not_consumed: Default::default(),
            not_consumed_reason: Default::default(),
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
                "NutritionIntake.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "NutritionIntake.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/event-status|5.0.0",
            )
            .with_description(
                "A coded concept indicating the current status of a NutritionIntake.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NutritionIntake.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.contained", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.extension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.identifier", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.partOf", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.statusReason", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.recorded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.reported[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.consumedItem", 1, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.consumedItem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.consumedItem.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.nutritionProduct",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.schedule",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.consumedItem.rate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.notConsumed",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.consumedItem.notConsumedReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.ingredientLabel", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.ingredientLabel.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.ingredientLabel.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.ingredientLabel.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.ingredientLabel.nutrient",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.ingredientLabel.amount",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.performer", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionIntake.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionIntake.performer.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionIntake.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.reason", 0, None),
            rh_foundation::ElementCardinality::new("NutritionIntake.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for NutritionIntake {
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

impl crate::traits::resource::ResourceMutators for NutritionIntake {
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

impl crate::traits::resource::ResourceExistence for NutritionIntake {
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

impl crate::traits::domain_resource::DomainResourceAccessors for NutritionIntake {
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

impl crate::traits::domain_resource::DomainResourceMutators for NutritionIntake {
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

impl crate::traits::domain_resource::DomainResourceExistence for NutritionIntake {
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

impl crate::traits::nutrition_intake::NutritionIntakeAccessors for NutritionIntake {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EventStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> &[CodeableConcept] {
        self.status_reason.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorded(&self) -> Option<DateTimeType> {
        self.recorded.clone()
    }
    fn consumed_item(&self) -> &[NutritionIntakeConsumeditem] {
        &self.consumed_item
    }
    fn ingredient_label(&self) -> &[NutritionIntakeIngredientlabel] {
        self.ingredient_label.as_deref().unwrap_or(&[])
    }
    fn performer(&self) -> &[NutritionIntakePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::nutrition_intake::NutritionIntakeMutators for NutritionIntake {
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
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: EventStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn add_status_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .status_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
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
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
        resource
    }
    fn set_consumed_item(self, value: Vec<NutritionIntakeConsumeditem>) -> Self {
        let mut resource = self.clone();
        resource.consumed_item = value;
        resource
    }
    fn add_consumed_item(self, item: NutritionIntakeConsumeditem) -> Self {
        let mut resource = self.clone();
        resource.consumed_item.push(item);
        resource
    }
    fn set_ingredient_label(self, value: Vec<NutritionIntakeIngredientlabel>) -> Self {
        let mut resource = self.clone();
        resource.ingredient_label = Some(value);
        resource
    }
    fn add_ingredient_label(self, item: NutritionIntakeIngredientlabel) -> Self {
        let mut resource = self.clone();
        resource
            .ingredient_label
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_performer(self, value: Vec<NutritionIntakePerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: NutritionIntakePerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_derived_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
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

impl crate::traits::nutrition_intake::NutritionIntakeExistence for NutritionIntake {
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some() || self.occurrence_period.is_some()
    }
    fn has_reported(&self) -> bool {
        self.reported_boolean.is_some() || self.reported_reference.is_some()
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
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
    }
    fn has_consumed_item(&self) -> bool {
        !self.consumed_item.is_empty()
    }
    fn has_ingredient_label(&self) -> bool {
        self.ingredient_label
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for NutritionIntake {
    fn resource_type(&self) -> &'static str {
        "NutritionIntake"
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
        Some("http://hl7.org/fhir/StructureDefinition/NutritionIntake")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::nutrition_intake::{
    NutritionIntakeAccessors, NutritionIntakeExistence, NutritionIntakeMutators,
};
