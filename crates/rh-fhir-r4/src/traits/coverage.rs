use crate::bindings::fm_status::FmStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::coverage::CoverageClass;
use crate::resources::coverage::CoverageCosttobeneficiary;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Coverage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Coverage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Coverage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the policyHolder field.
    fn policy_holder(&self) -> Option<Reference>;
    /// Returns a reference to the subscriber field.
    fn subscriber(&self) -> Option<Reference>;
    /// Returns a reference to the subscriberId field.
    fn subscriber_id(&self) -> Option<StringType>;
    /// Returns a reference to the beneficiary field.
    fn beneficiary(&self) -> Reference;
    /// Returns a reference to the dependent field.
    fn dependent(&self) -> Option<StringType>;
    /// Returns a reference to the relationship field.
    fn relationship(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the payor field.
    fn payor(&self) -> &[Reference];
    /// Returns a reference to the class field.
    fn class(&self) -> &[CoverageClass];
    /// Returns a reference to the order field.
    fn order(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the network field.
    fn network(&self) -> Option<StringType>;
    /// Returns a reference to the costToBeneficiary field.
    fn cost_to_beneficiary(&self) -> &[CoverageCosttobeneficiary];
    /// Returns a reference to the subrogation field.
    fn subrogation(&self) -> Option<BooleanType>;
    /// Returns a reference to the contract field.
    fn contract(&self) -> &[Reference];
}
/// Coverage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Coverage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Coverage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageMutators: DomainResourceMutators {
    /// Create a new Coverage with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::coverage::Coverage;
    /// use hl7_fhir_r4_core::traits::coverage::CoverageMutators;
    ///
    /// let resource = Coverage::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the policyHolder field and returns self for chaining.
    fn set_policy_holder(self, value: Reference) -> Self;
    /// Sets the subscriber field and returns self for chaining.
    fn set_subscriber(self, value: Reference) -> Self;
    /// Sets the subscriberId field and returns self for chaining.
    fn set_subscriber_id(self, value: String) -> Self;
    /// Sets the beneficiary field and returns self for chaining.
    fn set_beneficiary(self, value: Reference) -> Self;
    /// Sets the dependent field and returns self for chaining.
    fn set_dependent(self, value: String) -> Self;
    /// Sets the relationship field and returns self for chaining.
    fn set_relationship(self, value: CodeableConcept) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the payor field and returns self for chaining.
    fn set_payor(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the payor field and returns self for chaining.
    fn add_payor(self, item: Reference) -> Self;
    /// Sets the class field and returns self for chaining.
    fn set_class(self, value: Vec<CoverageClass>) -> Self;
    /// Adds an item to the class field and returns self for chaining.
    fn add_class(self, item: CoverageClass) -> Self;
    /// Sets the order field and returns self for chaining.
    fn set_order(self, value: i32) -> Self;
    /// Sets the network field and returns self for chaining.
    fn set_network(self, value: String) -> Self;
    /// Sets the costToBeneficiary field and returns self for chaining.
    fn set_cost_to_beneficiary(self, value: Vec<CoverageCosttobeneficiary>) -> Self;
    /// Adds an item to the costToBeneficiary field and returns self for chaining.
    fn add_cost_to_beneficiary(self, item: CoverageCosttobeneficiary) -> Self;
    /// Sets the subrogation field and returns self for chaining.
    fn set_subrogation(self, value: bool) -> Self;
    /// Sets the contract field and returns self for chaining.
    fn set_contract(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the contract field and returns self for chaining.
    fn add_contract(self, item: Reference) -> Self;
}
/// Coverage Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Coverage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Coverage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the policy_holder field is present (Some).
    fn has_policy_holder(&self) -> bool;
    /// Returns true if the subscriber field is present (Some).
    fn has_subscriber(&self) -> bool;
    /// Returns true if the subscriber_id field is present (Some).
    fn has_subscriber_id(&self) -> bool;
    /// Returns true if the beneficiary field is present (Some).
    fn has_beneficiary(&self) -> bool;
    /// Returns true if the dependent field is present (Some).
    fn has_dependent(&self) -> bool;
    /// Returns true if the relationship field is present (Some).
    fn has_relationship(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the payor field is not empty.
    fn has_payor(&self) -> bool;
    /// Returns true if the class field is not empty.
    fn has_class(&self) -> bool;
    /// Returns true if the order field is present (Some).
    fn has_order(&self) -> bool;
    /// Returns true if the network field is present (Some).
    fn has_network(&self) -> bool;
    /// Returns true if the cost_to_beneficiary field is not empty.
    fn has_cost_to_beneficiary(&self) -> bool;
    /// Returns true if the subrogation field is present (Some).
    fn has_subrogation(&self) -> bool;
    /// Returns true if the contract field is not empty.
    fn has_contract(&self) -> bool;
}
