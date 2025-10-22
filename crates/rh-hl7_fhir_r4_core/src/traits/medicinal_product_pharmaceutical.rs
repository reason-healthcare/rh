use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalCharacteristics;
use crate::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalRouteofadministration;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductPharmaceutical Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A pharmaceutical product described in terms of its composition and dose form.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPharmaceutical
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPharmaceutical
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPharmaceuticalAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the administrableDoseForm field.
    fn administrable_dose_form(&self) -> CodeableConcept;
    /// Returns a reference to the unitOfPresentation field.
    fn unit_of_presentation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[Reference];
    /// Returns a reference to the device field.
    fn device(&self) -> &[Reference];
    /// Returns a reference to the characteristics field.
    fn characteristics(&self) -> &[MedicinalProductPharmaceuticalCharacteristics];
    /// Returns a reference to the routeOfAdministration field.
    fn route_of_administration(&self) -> &[MedicinalProductPharmaceuticalRouteofadministration];
}
/// MedicinalProductPharmaceutical Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A pharmaceutical product described in terms of its composition and dose form.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPharmaceutical
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPharmaceutical
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPharmaceuticalMutators: DomainResourceMutators {
    /// Create a new MedicinalProductPharmaceutical with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_pharmaceutical::MedicinalProductPharmaceutical;
    /// use hl7_fhir_r4_core::traits::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalMutators;
    ///
    /// let resource = MedicinalProductPharmaceutical::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the administrableDoseForm field and returns self for chaining.
    fn set_administrable_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the unitOfPresentation field and returns self for chaining.
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the device field and returns self for chaining.
    fn add_device(self, item: Reference) -> Self;
    /// Sets the characteristics field and returns self for chaining.
    fn set_characteristics(self, value: Vec<MedicinalProductPharmaceuticalCharacteristics>)
        -> Self;
    /// Adds an item to the characteristics field and returns self for chaining.
    fn add_characteristics(self, item: MedicinalProductPharmaceuticalCharacteristics) -> Self;
    /// Sets the routeOfAdministration field and returns self for chaining.
    fn set_route_of_administration(
        self,
        value: Vec<MedicinalProductPharmaceuticalRouteofadministration>,
    ) -> Self;
    /// Adds an item to the routeOfAdministration field and returns self for chaining.
    fn add_route_of_administration(
        self,
        item: MedicinalProductPharmaceuticalRouteofadministration,
    ) -> Self;
}
/// MedicinalProductPharmaceutical Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A pharmaceutical product described in terms of its composition and dose form.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPharmaceutical
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPharmaceutical
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPharmaceuticalExistence: DomainResourceExistence {
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
    /// Returns true if the administrable_dose_form field is present (Some).
    fn has_administrable_dose_form(&self) -> bool;
    /// Returns true if the unit_of_presentation field is present (Some).
    fn has_unit_of_presentation(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the device field is not empty.
    fn has_device(&self) -> bool;
    /// Returns true if the characteristics field is not empty.
    fn has_characteristics(&self) -> bool;
    /// Returns true if the route_of_administration field is not empty.
    fn has_route_of_administration(&self) -> bool;
}
