use crate::bindings::biologicallyderivedproductdispense_status::BiologicallyderivedproductdispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispensePerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// BiologicallyDerivedProductDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of dispensation of a biologically derived product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProductDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductDispenseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> BiologicallyderivedproductdispenseStatus;
    /// Returns a reference to the originRelationshipType field.
    fn origin_relationship_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the product field.
    fn product(&self) -> Reference;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the matchStatus field.
    fn match_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[BiologicallyDerivedProductDispensePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the preparedDate field.
    fn prepared_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the whenHandedOver field.
    fn when_handed_over(&self) -> Option<DateTimeType>;
    /// Returns a reference to the destination field.
    fn destination(&self) -> Option<Reference>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the usageInstruction field.
    fn usage_instruction(&self) -> Option<StringType>;
}
/// BiologicallyDerivedProductDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of dispensation of a biologically derived product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProductDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductDispenseMutators: DomainResourceMutators {
    /// Create a new BiologicallyDerivedProductDispense with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::biologically_derived_product_dispense::BiologicallyDerivedProductDispense;
    /// use rh_hl7_fhir_r5_core::traits::biologically_derived_product_dispense::BiologicallyDerivedProductDispenseMutators;
    ///
    /// let resource = BiologicallyDerivedProductDispense::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: BiologicallyderivedproductdispenseStatus) -> Self;
    /// Sets the originRelationshipType field and returns self for chaining.
    fn set_origin_relationship_type(self, value: CodeableConcept) -> Self;
    /// Sets the product field and returns self for chaining.
    fn set_product(self, value: Reference) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the matchStatus field and returns self for chaining.
    fn set_match_status(self, value: CodeableConcept) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<BiologicallyDerivedProductDispensePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: BiologicallyDerivedProductDispensePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the preparedDate field and returns self for chaining.
    fn set_prepared_date(self, value: String) -> Self;
    /// Sets the whenHandedOver field and returns self for chaining.
    fn set_when_handed_over(self, value: String) -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the usageInstruction field and returns self for chaining.
    fn set_usage_instruction(self, value: String) -> Self;
}
/// BiologicallyDerivedProductDispense Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of dispensation of a biologically derived product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProductDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProductDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductDispenseExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the origin_relationship_type field is present (Some).
    fn has_origin_relationship_type(&self) -> bool;
    /// Returns true if the product field is present (Some).
    fn has_product(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the match_status field is present (Some).
    fn has_match_status(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the prepared_date field is present (Some).
    fn has_prepared_date(&self) -> bool;
    /// Returns true if the when_handed_over field is present (Some).
    fn has_when_handed_over(&self) -> bool;
    /// Returns true if the destination field is present (Some).
    fn has_destination(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the usage_instruction field is present (Some).
    fn has_usage_instruction(&self) -> bool;
}
