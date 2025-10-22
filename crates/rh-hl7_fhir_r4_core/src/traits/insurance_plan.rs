use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::insurance_plan::InsurancePlanContact;
use crate::resources::insurance_plan::InsurancePlanCoverage;
use crate::resources::insurance_plan::InsurancePlanPlan;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// InsurancePlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Details of a Health Insurance product/plan provided by an organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: InsurancePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InsurancePlanAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<PublicationStatus>;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the alias field.
    fn alias(&self) -> &[StringType];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the ownedBy field.
    fn owned_by(&self) -> Option<Reference>;
    /// Returns a reference to the administeredBy field.
    fn administered_by(&self) -> Option<Reference>;
    /// Returns a reference to the coverageArea field.
    fn coverage_area(&self) -> &[Reference];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[InsurancePlanContact];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
    /// Returns a reference to the network field.
    fn network(&self) -> &[Reference];
    /// Returns a reference to the coverage field.
    fn coverage(&self) -> &[InsurancePlanCoverage];
    /// Returns a reference to the plan field.
    fn plan(&self) -> &[InsurancePlanPlan];
}
/// InsurancePlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Details of a Health Insurance product/plan provided by an organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: InsurancePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InsurancePlanMutators: DomainResourceMutators {
    /// Create a new InsurancePlan with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::insurance_plan::InsurancePlan;
    /// use hl7_fhir_r4_core::traits::insurance_plan::InsurancePlanMutators;
    ///
    /// let resource = InsurancePlan::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the alias field and returns self for chaining.
    fn set_alias(self, value: Vec<String>) -> Self;
    /// Adds an item to the alias field and returns self for chaining.
    fn add_alias(self, item: String) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the ownedBy field and returns self for chaining.
    fn set_owned_by(self, value: Reference) -> Self;
    /// Sets the administeredBy field and returns self for chaining.
    fn set_administered_by(self, value: Reference) -> Self;
    /// Sets the coverageArea field and returns self for chaining.
    fn set_coverage_area(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the coverageArea field and returns self for chaining.
    fn add_coverage_area(self, item: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<InsurancePlanContact>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: InsurancePlanContact) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
    /// Sets the network field and returns self for chaining.
    fn set_network(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the network field and returns self for chaining.
    fn add_network(self, item: Reference) -> Self;
    /// Sets the coverage field and returns self for chaining.
    fn set_coverage(self, value: Vec<InsurancePlanCoverage>) -> Self;
    /// Adds an item to the coverage field and returns self for chaining.
    fn add_coverage(self, item: InsurancePlanCoverage) -> Self;
    /// Sets the plan field and returns self for chaining.
    fn set_plan(self, value: Vec<InsurancePlanPlan>) -> Self;
    /// Adds an item to the plan field and returns self for chaining.
    fn add_plan(self, item: InsurancePlanPlan) -> Self;
}
/// InsurancePlan Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Details of a Health Insurance product/plan provided by an organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: InsurancePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InsurancePlanExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the alias field is not empty.
    fn has_alias(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the owned_by field is present (Some).
    fn has_owned_by(&self) -> bool;
    /// Returns true if the administered_by field is present (Some).
    fn has_administered_by(&self) -> bool;
    /// Returns true if the coverage_area field is not empty.
    fn has_coverage_area(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
    /// Returns true if the network field is not empty.
    fn has_network(&self) -> bool;
    /// Returns true if the coverage field is not empty.
    fn has_coverage(&self) -> bool;
    /// Returns true if the plan field is not empty.
    fn has_plan(&self) -> bool;
}
