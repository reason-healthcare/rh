use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use crate::resources::substance_polymer::SubstancePolymerMonomerset;
use crate::resources::substance_polymer::SubstancePolymerRepeat;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstancePolymer Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Properties of a substance specific to it being a polymer.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstancePolymer
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstancePolymerAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the class field.
    fn class(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the geometry field.
    fn geometry(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the copolymerConnectivity field.
    fn copolymer_connectivity(&self) -> &[CodeableConcept];
    /// Returns a reference to the modification field.
    fn modification(&self) -> Option<StringType>;
    /// Returns a reference to the monomerSet field.
    fn monomer_set(&self) -> &[SubstancePolymerMonomerset];
    /// Returns a reference to the repeat field.
    fn repeat(&self) -> &[SubstancePolymerRepeat];
}
/// SubstancePolymer Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Properties of a substance specific to it being a polymer.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstancePolymer
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstancePolymerMutators: DomainResourceMutators {
    /// Create a new SubstancePolymer with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::substance_polymer::SubstancePolymer;
    /// use rh_hl7_fhir_r5_core::traits::substance_polymer::SubstancePolymerMutators;
    ///
    /// let resource = SubstancePolymer::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the class field and returns self for chaining.
    fn set_class(self, value: CodeableConcept) -> Self;
    /// Sets the geometry field and returns self for chaining.
    fn set_geometry(self, value: CodeableConcept) -> Self;
    /// Sets the copolymerConnectivity field and returns self for chaining.
    fn set_copolymer_connectivity(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the copolymerConnectivity field and returns self for chaining.
    fn add_copolymer_connectivity(self, item: CodeableConcept) -> Self;
    /// Sets the modification field and returns self for chaining.
    fn set_modification(self, value: String) -> Self;
    /// Sets the monomerSet field and returns self for chaining.
    fn set_monomer_set(self, value: Vec<SubstancePolymerMonomerset>) -> Self;
    /// Adds an item to the monomerSet field and returns self for chaining.
    fn add_monomer_set(self, item: SubstancePolymerMonomerset) -> Self;
    /// Sets the repeat field and returns self for chaining.
    fn set_repeat(self, value: Vec<SubstancePolymerRepeat>) -> Self;
    /// Adds an item to the repeat field and returns self for chaining.
    fn add_repeat(self, item: SubstancePolymerRepeat) -> Self;
}
/// SubstancePolymer Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Properties of a substance specific to it being a polymer.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstancePolymer
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstancePolymerExistence: DomainResourceExistence {
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the class field is present (Some).
    fn has_class(&self) -> bool;
    /// Returns true if the geometry field is present (Some).
    fn has_geometry(&self) -> bool;
    /// Returns true if the copolymer_connectivity field is not empty.
    fn has_copolymer_connectivity(&self) -> bool;
    /// Returns true if the modification field is present (Some).
    fn has_modification(&self) -> bool;
    /// Returns true if the monomer_set field is not empty.
    fn has_monomer_set(&self) -> bool;
    /// Returns true if the repeat field is not empty.
    fn has_repeat(&self) -> bool;
}
