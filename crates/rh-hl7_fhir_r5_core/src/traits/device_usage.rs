use crate::bindings::deviceusage_status::DeviceusageStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::device_usage::DeviceUsageAdherence;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceUsage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceUsage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUsageAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> DeviceusageStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the context field.
    fn context(&self) -> Option<Reference>;
    /// Returns a reference to the dateAsserted field.
    fn date_asserted(&self) -> Option<DateTimeType>;
    /// Returns a reference to the usageStatus field.
    fn usage_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the usageReason field.
    fn usage_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the adherence field.
    fn adherence(&self) -> Option<DeviceUsageAdherence>;
    /// Returns a reference to the informationSource field.
    fn information_source(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> CodeableReference;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableReference>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// DeviceUsage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceUsage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUsageMutators: DomainResourceMutators {
    /// Create a new DeviceUsage with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::device_usage::DeviceUsage;
    /// use rh_hl7_fhir_r5_core::traits::device_usage::DeviceUsageMutators;
    ///
    /// let resource = DeviceUsage::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DeviceusageStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: Reference) -> Self;
    /// Sets the dateAsserted field and returns self for chaining.
    fn set_date_asserted(self, value: String) -> Self;
    /// Sets the usageStatus field and returns self for chaining.
    fn set_usage_status(self, value: CodeableConcept) -> Self;
    /// Sets the usageReason field and returns self for chaining.
    fn set_usage_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the usageReason field and returns self for chaining.
    fn add_usage_reason(self, item: CodeableConcept) -> Self;
    /// Sets the adherence field and returns self for chaining.
    fn set_adherence(self, value: DeviceUsageAdherence) -> Self;
    /// Sets the informationSource field and returns self for chaining.
    fn set_information_source(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: CodeableReference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableReference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// DeviceUsage Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceUsage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUsageExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the context field is present (Some).
    fn has_context(&self) -> bool;
    /// Returns true if the timing field is present (Some).
    fn has_timing(&self) -> bool;
    /// Returns true if the date_asserted field is present (Some).
    fn has_date_asserted(&self) -> bool;
    /// Returns true if the usage_status field is present (Some).
    fn has_usage_status(&self) -> bool;
    /// Returns true if the usage_reason field is not empty.
    fn has_usage_reason(&self) -> bool;
    /// Returns true if the adherence field is present (Some).
    fn has_adherence(&self) -> bool;
    /// Returns true if the information_source field is present (Some).
    fn has_information_source(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
