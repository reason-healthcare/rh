use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::manufactured_item_definition::ManufacturedItemDefinitionComponent;
use crate::resources::manufactured_item_definition::ManufacturedItemDefinitionProperty;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ManufacturedItemDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ManufacturedItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ManufacturedItemDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the manufacturedDoseForm field.
    fn manufactured_dose_form(&self) -> CodeableConcept;
    /// Returns a reference to the unitOfPresentation field.
    fn unit_of_presentation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the marketingStatus field.
    fn marketing_status(&self) -> &[MarketingStatus];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[CodeableConcept];
    /// Returns a reference to the property field.
    fn property(&self) -> &[ManufacturedItemDefinitionProperty];
    /// Returns a reference to the component field.
    fn component(&self) -> &[ManufacturedItemDefinitionComponent];
}
/// ManufacturedItemDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ManufacturedItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ManufacturedItemDefinitionMutators: DomainResourceMutators {
    /// Create a new ManufacturedItemDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::manufactured_item_definition::ManufacturedItemDefinition;
    /// use rh_hl7_fhir_r5_core::traits::manufactured_item_definition::ManufacturedItemDefinitionMutators;
    ///
    /// let resource = ManufacturedItemDefinition::new();
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
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the manufacturedDoseForm field and returns self for chaining.
    fn set_manufactured_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the unitOfPresentation field and returns self for chaining.
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the marketingStatus field and returns self for chaining.
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self;
    /// Adds an item to the marketingStatus field and returns self for chaining.
    fn add_marketing_status(self, item: MarketingStatus) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: CodeableConcept) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<ManufacturedItemDefinitionProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: ManufacturedItemDefinitionProperty) -> Self;
    /// Sets the component field and returns self for chaining.
    fn set_component(self, value: Vec<ManufacturedItemDefinitionComponent>) -> Self;
    /// Adds an item to the component field and returns self for chaining.
    fn add_component(self, item: ManufacturedItemDefinitionComponent) -> Self;
}
/// ManufacturedItemDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ManufacturedItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ManufacturedItemDefinitionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the manufactured_dose_form field is present (Some).
    fn has_manufactured_dose_form(&self) -> bool;
    /// Returns true if the unit_of_presentation field is present (Some).
    fn has_unit_of_presentation(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the marketing_status field is not empty.
    fn has_marketing_status(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the component field is not empty.
    fn has_component(&self) -> bool;
}
