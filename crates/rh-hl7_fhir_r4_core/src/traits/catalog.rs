use crate::traits::resource::ResourceExistence;
/// Profile for Catalog Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of resources composed into a single coherent clinical statement with clinical attestation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/catalog
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait CatalogAccessors {}
/// Profile for Catalog Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of resources composed into a single coherent clinical statement with clinical attestation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/catalog
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait CatalogMutators {
    /// Create a new Catalog with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::catalog::Catalog;
    /// use hl7_fhir_r4_core::traits::catalog::CatalogMutators;
    ///
    /// let resource = Catalog::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// catalog Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A set of resources composed into a single coherent clinical statement with clinical attestation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/catalog
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait CatalogExistence: ResourceExistence {}
