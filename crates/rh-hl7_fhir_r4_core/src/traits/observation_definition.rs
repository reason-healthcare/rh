use crate::bindings::permitted_data_type::PermittedDataType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::observation_definition::ObservationDefinitionQualifiedinterval;
use crate::resources::observation_definition::ObservationDefinitionQuantitativedetails;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ObservationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the permittedDataType field.
    fn permitted_data_type(&self) -> &[PermittedDataType];
    /// Returns a reference to the multipleResultsAllowed field.
    fn multiple_results_allowed(&self) -> Option<BooleanType>;
    /// Returns a reference to the method field.
    fn method(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the preferredReportName field.
    fn preferred_report_name(&self) -> Option<StringType>;
    /// Returns a reference to the quantitativeDetails field.
    fn quantitative_details(&self) -> Option<ObservationDefinitionQuantitativedetails>;
    /// Returns a reference to the qualifiedInterval field.
    fn qualified_interval(&self) -> &[ObservationDefinitionQualifiedinterval];
    /// Returns a reference to the validCodedValueSet field.
    fn valid_coded_value_set(&self) -> Option<Reference>;
    /// Returns a reference to the normalCodedValueSet field.
    fn normal_coded_value_set(&self) -> Option<Reference>;
    /// Returns a reference to the abnormalCodedValueSet field.
    fn abnormal_coded_value_set(&self) -> Option<Reference>;
    /// Returns a reference to the criticalCodedValueSet field.
    fn critical_coded_value_set(&self) -> Option<Reference>;
}
/// ObservationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionMutators: DomainResourceMutators {
    /// Create a new ObservationDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::observation_definition::ObservationDefinition;
    /// use hl7_fhir_r4_core::traits::observation_definition::ObservationDefinitionMutators;
    ///
    /// let resource = ObservationDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the permittedDataType field and returns self for chaining.
    fn set_permitted_data_type(self, value: Vec<PermittedDataType>) -> Self;
    /// Adds an item to the permittedDataType field and returns self for chaining.
    fn add_permitted_data_type(self, item: PermittedDataType) -> Self;
    /// Sets the multipleResultsAllowed field and returns self for chaining.
    fn set_multiple_results_allowed(self, value: bool) -> Self;
    /// Sets the method field and returns self for chaining.
    fn set_method(self, value: CodeableConcept) -> Self;
    /// Sets the preferredReportName field and returns self for chaining.
    fn set_preferred_report_name(self, value: String) -> Self;
    /// Sets the quantitativeDetails field and returns self for chaining.
    fn set_quantitative_details(self, value: ObservationDefinitionQuantitativedetails) -> Self;
    /// Sets the qualifiedInterval field and returns self for chaining.
    fn set_qualified_interval(self, value: Vec<ObservationDefinitionQualifiedinterval>) -> Self;
    /// Adds an item to the qualifiedInterval field and returns self for chaining.
    fn add_qualified_interval(self, item: ObservationDefinitionQualifiedinterval) -> Self;
    /// Sets the validCodedValueSet field and returns self for chaining.
    fn set_valid_coded_value_set(self, value: Reference) -> Self;
    /// Sets the normalCodedValueSet field and returns self for chaining.
    fn set_normal_coded_value_set(self, value: Reference) -> Self;
    /// Sets the abnormalCodedValueSet field and returns self for chaining.
    fn set_abnormal_coded_value_set(self, value: Reference) -> Self;
    /// Sets the criticalCodedValueSet field and returns self for chaining.
    fn set_critical_coded_value_set(self, value: Reference) -> Self;
}
/// ObservationDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the permitted_data_type field is not empty.
    fn has_permitted_data_type(&self) -> bool;
    /// Returns true if the multiple_results_allowed field is present (Some).
    fn has_multiple_results_allowed(&self) -> bool;
    /// Returns true if the method field is present (Some).
    fn has_method(&self) -> bool;
    /// Returns true if the preferred_report_name field is present (Some).
    fn has_preferred_report_name(&self) -> bool;
    /// Returns true if the quantitative_details field is present (Some).
    fn has_quantitative_details(&self) -> bool;
    /// Returns true if the qualified_interval field is not empty.
    fn has_qualified_interval(&self) -> bool;
    /// Returns true if the valid_coded_value_set field is present (Some).
    fn has_valid_coded_value_set(&self) -> bool;
    /// Returns true if the normal_coded_value_set field is present (Some).
    fn has_normal_coded_value_set(&self) -> bool;
    /// Returns true if the abnormal_coded_value_set field is present (Some).
    fn has_abnormal_coded_value_set(&self) -> bool;
    /// Returns true if the critical_coded_value_set field is present (Some).
    fn has_critical_coded_value_set(&self) -> bool;
}
