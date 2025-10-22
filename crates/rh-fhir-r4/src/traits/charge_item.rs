use crate::bindings::chargeitem_status::ChargeitemStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::charge_item::ChargeItemPerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ChargeItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the definitionUri field.
    fn definition_uri(&self) -> &[StringType];
    /// Returns a reference to the definitionCanonical field.
    fn definition_canonical(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> ChargeitemStatus;
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the context field.
    fn context(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[ChargeItemPerformer];
    /// Returns a reference to the performingOrganization field.
    fn performing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the requestingOrganization field.
    fn requesting_organization(&self) -> Option<Reference>;
    /// Returns a reference to the costCenter field.
    fn cost_center(&self) -> Option<Reference>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the bodysite field.
    fn bodysite(&self) -> &[CodeableConcept];
    /// Returns a reference to the factorOverride field.
    fn factor_override(&self) -> Option<DecimalType>;
    /// Returns a reference to the priceOverride field.
    fn price_override(&self) -> Option<Money>;
    /// Returns a reference to the overrideReason field.
    fn override_reason(&self) -> Option<StringType>;
    /// Returns a reference to the enterer field.
    fn enterer(&self) -> Option<Reference>;
    /// Returns a reference to the enteredDate field.
    fn entered_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the service field.
    fn service(&self) -> &[Reference];
    /// Returns a reference to the account field.
    fn account(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
}
/// ChargeItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemMutators: DomainResourceMutators {
    /// Create a new ChargeItem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::charge_item::ChargeItem;
    /// use hl7_fhir_r4_core::traits::charge_item::ChargeItemMutators;
    ///
    /// let resource = ChargeItem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the definitionUri field and returns self for chaining.
    fn set_definition_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the definitionUri field and returns self for chaining.
    fn add_definition_uri(self, item: String) -> Self;
    /// Sets the definitionCanonical field and returns self for chaining.
    fn set_definition_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the definitionCanonical field and returns self for chaining.
    fn add_definition_canonical(self, item: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ChargeitemStatus) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<ChargeItemPerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: ChargeItemPerformer) -> Self;
    /// Sets the performingOrganization field and returns self for chaining.
    fn set_performing_organization(self, value: Reference) -> Self;
    /// Sets the requestingOrganization field and returns self for chaining.
    fn set_requesting_organization(self, value: Reference) -> Self;
    /// Sets the costCenter field and returns self for chaining.
    fn set_cost_center(self, value: Reference) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the bodysite field and returns self for chaining.
    fn set_bodysite(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the bodysite field and returns self for chaining.
    fn add_bodysite(self, item: CodeableConcept) -> Self;
    /// Sets the factorOverride field and returns self for chaining.
    fn set_factor_override(self, value: f64) -> Self;
    /// Sets the priceOverride field and returns self for chaining.
    fn set_price_override(self, value: Money) -> Self;
    /// Sets the overrideReason field and returns self for chaining.
    fn set_override_reason(self, value: String) -> Self;
    /// Sets the enterer field and returns self for chaining.
    fn set_enterer(self, value: Reference) -> Self;
    /// Sets the enteredDate field and returns self for chaining.
    fn set_entered_date(self, value: String) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableConcept) -> Self;
    /// Sets the service field and returns self for chaining.
    fn set_service(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the service field and returns self for chaining.
    fn add_service(self, item: Reference) -> Self;
    /// Sets the account field and returns self for chaining.
    fn set_account(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the account field and returns self for chaining.
    fn add_account(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the supportingInformation field and returns self for chaining.
    fn set_supporting_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInformation field and returns self for chaining.
    fn add_supporting_information(self, item: Reference) -> Self;
}
/// ChargeItem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemExistence: DomainResourceExistence {
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
    /// Returns true if the definition_uri field is not empty.
    fn has_definition_uri(&self) -> bool;
    /// Returns true if the definition_canonical field is not empty.
    fn has_definition_canonical(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the context field is present (Some).
    fn has_context(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the performing_organization field is present (Some).
    fn has_performing_organization(&self) -> bool;
    /// Returns true if the requesting_organization field is present (Some).
    fn has_requesting_organization(&self) -> bool;
    /// Returns true if the cost_center field is present (Some).
    fn has_cost_center(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the bodysite field is not empty.
    fn has_bodysite(&self) -> bool;
    /// Returns true if the factor_override field is present (Some).
    fn has_factor_override(&self) -> bool;
    /// Returns true if the price_override field is present (Some).
    fn has_price_override(&self) -> bool;
    /// Returns true if the override_reason field is present (Some).
    fn has_override_reason(&self) -> bool;
    /// Returns true if the enterer field is present (Some).
    fn has_enterer(&self) -> bool;
    /// Returns true if the entered_date field is present (Some).
    fn has_entered_date(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the service field is not empty.
    fn has_service(&self) -> bool;
    /// Returns true if the product field is present (Some).
    fn has_product(&self) -> bool;
    /// Returns true if the account field is not empty.
    fn has_account(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
}
