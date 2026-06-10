use crate::bindings::clinical_use_definition_type::ClinicalUseDefinitionType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::clinical_use_definition::ClinicalUseDefinitionContraindication;
use crate::resources::clinical_use_definition::ClinicalUseDefinitionIndication;
use crate::resources::clinical_use_definition::ClinicalUseDefinitionInteraction;
use crate::resources::clinical_use_definition::ClinicalUseDefinitionUndesirableeffect;
use crate::resources::clinical_use_definition::ClinicalUseDefinitionWarning;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ClinicalUseDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ClinicalUseDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalUseDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> ClinicalUseDefinitionType;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the contraindication field.
    fn contraindication(&self) -> Option<ClinicalUseDefinitionContraindication>;
    /// Returns a reference to the indication field.
    fn indication(&self) -> Option<ClinicalUseDefinitionIndication>;
    /// Returns a reference to the interaction field.
    fn interaction(&self) -> Option<ClinicalUseDefinitionInteraction>;
    /// Returns a reference to the population field.
    fn population(&self) -> &[Reference];
    /// Returns a reference to the library field.
    fn library(&self) -> &[StringType];
    /// Returns a reference to the undesirableEffect field.
    fn undesirable_effect(&self) -> Option<ClinicalUseDefinitionUndesirableeffect>;
    /// Returns a reference to the warning field.
    fn warning(&self) -> Option<ClinicalUseDefinitionWarning>;
}
/// ClinicalUseDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ClinicalUseDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalUseDefinitionMutators: DomainResourceMutators {
    /// Create a new ClinicalUseDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::clinical_use_definition::ClinicalUseDefinition;
    /// use rh_hl7_fhir_r5_core::traits::clinical_use_definition::ClinicalUseDefinitionMutators;
    ///
    /// let resource = ClinicalUseDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: ClinicalUseDefinitionType) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the contraindication field and returns self for chaining.
    fn set_contraindication(self, value: ClinicalUseDefinitionContraindication) -> Self;
    /// Sets the indication field and returns self for chaining.
    fn set_indication(self, value: ClinicalUseDefinitionIndication) -> Self;
    /// Sets the interaction field and returns self for chaining.
    fn set_interaction(self, value: ClinicalUseDefinitionInteraction) -> Self;
    /// Sets the population field and returns self for chaining.
    fn set_population(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the population field and returns self for chaining.
    fn add_population(self, item: Reference) -> Self;
    /// Sets the library field and returns self for chaining.
    fn set_library(self, value: Vec<String>) -> Self;
    /// Adds an item to the library field and returns self for chaining.
    fn add_library(self, item: String) -> Self;
    /// Sets the undesirableEffect field and returns self for chaining.
    fn set_undesirable_effect(self, value: ClinicalUseDefinitionUndesirableeffect) -> Self;
    /// Sets the warning field and returns self for chaining.
    fn set_warning(self, value: ClinicalUseDefinitionWarning) -> Self;
}
/// ClinicalUseDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ClinicalUseDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalUseDefinitionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the contraindication field is present (Some).
    fn has_contraindication(&self) -> bool;
    /// Returns true if the indication field is present (Some).
    fn has_indication(&self) -> bool;
    /// Returns true if the interaction field is present (Some).
    fn has_interaction(&self) -> bool;
    /// Returns true if the population field is not empty.
    fn has_population(&self) -> bool;
    /// Returns true if the library field is not empty.
    fn has_library(&self) -> bool;
    /// Returns true if the undesirable_effect field is present (Some).
    fn has_undesirable_effect(&self) -> bool;
    /// Returns true if the warning field is present (Some).
    fn has_warning(&self) -> bool;
}
