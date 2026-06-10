use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::nutrition_intake::NutritionIntakeConsumeditem;
use crate::resources::nutrition_intake::NutritionIntakeIngredientlabel;
use crate::resources::nutrition_intake::NutritionIntakePerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// NutritionIntake Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionIntake
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionIntakeAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> EventStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> Option<DateTimeType>;
    /// Returns a reference to the consumedItem field.
    fn consumed_item(&self) -> &[NutritionIntakeConsumeditem];
    /// Returns a reference to the ingredientLabel field.
    fn ingredient_label(&self) -> &[NutritionIntakeIngredientlabel];
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[NutritionIntakePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// NutritionIntake Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionIntake
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionIntakeMutators: DomainResourceMutators {
    /// Create a new NutritionIntake with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::nutrition_intake::NutritionIntake;
    /// use rh_hl7_fhir_r5_core::traits::nutrition_intake::NutritionIntakeMutators;
    ///
    /// let resource = NutritionIntake::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesCanonical field and returns self for chaining.
    fn add_instantiates_canonical(self, item: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesUri field and returns self for chaining.
    fn add_instantiates_uri(self, item: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EventStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the statusReason field and returns self for chaining.
    fn add_status_reason(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the consumedItem field and returns self for chaining.
    fn set_consumed_item(self, value: Vec<NutritionIntakeConsumeditem>) -> Self;
    /// Adds an item to the consumedItem field and returns self for chaining.
    fn add_consumed_item(self, item: NutritionIntakeConsumeditem) -> Self;
    /// Sets the ingredientLabel field and returns self for chaining.
    fn set_ingredient_label(self, value: Vec<NutritionIntakeIngredientlabel>) -> Self;
    /// Adds an item to the ingredientLabel field and returns self for chaining.
    fn add_ingredient_label(self, item: NutritionIntakeIngredientlabel) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<NutritionIntakePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: NutritionIntakePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// NutritionIntake Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionIntake
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionIntakeExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the instantiates_canonical field is not empty.
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is not empty.
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is not empty.
    fn has_status_reason(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the reported field is present (Some).
    fn has_reported(&self) -> bool;
    /// Returns true if the consumed_item field is not empty.
    fn has_consumed_item(&self) -> bool;
    /// Returns true if the ingredient_label field is not empty.
    fn has_ingredient_label(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
