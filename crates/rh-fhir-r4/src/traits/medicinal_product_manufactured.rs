use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::prod_characteristic::ProdCharacteristic;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductManufactured Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The manufactured item as contained in the packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductManufactured
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductManufactured
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductManufacturedAccessors: DomainResourceAccessors {
    /// Returns a reference to the manufacturedDoseForm field.
    fn manufactured_dose_form(&self) -> CodeableConcept;
    /// Returns a reference to the unitOfPresentation field.
    fn unit_of_presentation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Quantity;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[Reference];
    /// Returns a reference to the physicalCharacteristics field.
    fn physical_characteristics(&self) -> Option<ProdCharacteristic>;
    /// Returns a reference to the otherCharacteristics field.
    fn other_characteristics(&self) -> &[CodeableConcept];
}
/// MedicinalProductManufactured Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The manufactured item as contained in the packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductManufactured
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductManufactured
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductManufacturedMutators: DomainResourceMutators {
    /// Create a new MedicinalProductManufactured with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_manufactured::MedicinalProductManufactured;
    /// use hl7_fhir_r4_core::traits::medicinal_product_manufactured::MedicinalProductManufacturedMutators;
    ///
    /// let resource = MedicinalProductManufactured::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the manufacturedDoseForm field and returns self for chaining.
    fn set_manufactured_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the unitOfPresentation field and returns self for chaining.
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: Reference) -> Self;
    /// Sets the physicalCharacteristics field and returns self for chaining.
    fn set_physical_characteristics(self, value: ProdCharacteristic) -> Self;
    /// Sets the otherCharacteristics field and returns self for chaining.
    fn set_other_characteristics(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the otherCharacteristics field and returns self for chaining.
    fn add_other_characteristics(self, item: CodeableConcept) -> Self;
}
/// MedicinalProductManufactured Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The manufactured item as contained in the packaged medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductManufactured
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductManufactured
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductManufacturedExistence: DomainResourceExistence {
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
    /// Returns true if the manufactured_dose_form field is present (Some).
    fn has_manufactured_dose_form(&self) -> bool;
    /// Returns true if the unit_of_presentation field is present (Some).
    fn has_unit_of_presentation(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the physical_characteristics field is present (Some).
    fn has_physical_characteristics(&self) -> bool;
    /// Returns true if the other_characteristics field is not empty.
    fn has_other_characteristics(&self) -> bool;
}
