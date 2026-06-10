use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::administrable_product_definition::AdministrableProductDefinitionProperty;
use crate::resources::administrable_product_definition::AdministrableProductDefinitionRouteofadministration;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AdministrableProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdministrableProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdministrableProductDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the formOf field.
    fn form_of(&self) -> &[Reference];
    /// Returns a reference to the administrableDoseForm field.
    fn administrable_dose_form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the unitOfPresentation field.
    fn unit_of_presentation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the producedFrom field.
    fn produced_from(&self) -> &[Reference];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[CodeableConcept];
    /// Returns a reference to the device field.
    fn device(&self) -> Option<Reference>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the property field.
    fn property(&self) -> &[AdministrableProductDefinitionProperty];
    /// Returns a reference to the routeOfAdministration field.
    fn route_of_administration(&self) -> &[AdministrableProductDefinitionRouteofadministration];
}
/// AdministrableProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdministrableProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdministrableProductDefinitionMutators: DomainResourceMutators {
    /// Create a new AdministrableProductDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::administrable_product_definition::AdministrableProductDefinition;
    /// use rh_hl7_fhir_r5_core::traits::administrable_product_definition::AdministrableProductDefinitionMutators;
    ///
    /// let resource = AdministrableProductDefinition::new();
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
    /// Sets the formOf field and returns self for chaining.
    fn set_form_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the formOf field and returns self for chaining.
    fn add_form_of(self, item: Reference) -> Self;
    /// Sets the administrableDoseForm field and returns self for chaining.
    fn set_administrable_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the unitOfPresentation field and returns self for chaining.
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self;
    /// Sets the producedFrom field and returns self for chaining.
    fn set_produced_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the producedFrom field and returns self for chaining.
    fn add_produced_from(self, item: Reference) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: CodeableConcept) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<AdministrableProductDefinitionProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: AdministrableProductDefinitionProperty) -> Self;
    /// Sets the routeOfAdministration field and returns self for chaining.
    fn set_route_of_administration(
        self,
        value: Vec<AdministrableProductDefinitionRouteofadministration>,
    ) -> Self;
    /// Adds an item to the routeOfAdministration field and returns self for chaining.
    fn add_route_of_administration(
        self,
        item: AdministrableProductDefinitionRouteofadministration,
    ) -> Self;
}
/// AdministrableProductDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdministrableProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdministrableProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdministrableProductDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the form_of field is not empty.
    fn has_form_of(&self) -> bool;
    /// Returns true if the administrable_dose_form field is present (Some).
    fn has_administrable_dose_form(&self) -> bool;
    /// Returns true if the unit_of_presentation field is present (Some).
    fn has_unit_of_presentation(&self) -> bool;
    /// Returns true if the produced_from field is not empty.
    fn has_produced_from(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the route_of_administration field is not empty.
    fn has_route_of_administration(&self) -> bool;
}
