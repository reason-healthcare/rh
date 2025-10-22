use crate::bindings::measure_report_status::MeasureReportStatus;
use crate::bindings::measure_report_type::MeasureReportType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::measure_report::MeasureReportGroup;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MeasureReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MeasureReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureReportAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> MeasureReportStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> MeasureReportType;
    /// Returns a reference to the measure field.
    fn measure(&self) -> StringType;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the reporter field.
    fn reporter(&self) -> Option<Reference>;
    /// Returns a reference to the period field.
    fn period(&self) -> Period;
    /// Returns a reference to the improvementNotation field.
    fn improvement_notation(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the group field.
    fn group(&self) -> &[MeasureReportGroup];
    /// Returns a reference to the evaluatedResource field.
    fn evaluated_resource(&self) -> &[Reference];
}
/// MeasureReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MeasureReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureReportMutators: DomainResourceMutators {
    /// Create a new MeasureReport with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::measure_report::MeasureReport;
    /// use hl7_fhir_r4_core::traits::measure_report::MeasureReportMutators;
    ///
    /// let resource = MeasureReport::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MeasureReportStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: MeasureReportType) -> Self;
    /// Sets the measure field and returns self for chaining.
    fn set_measure(self, value: String) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the reporter field and returns self for chaining.
    fn set_reporter(self, value: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the improvementNotation field and returns self for chaining.
    fn set_improvement_notation(self, value: CodeableConcept) -> Self;
    /// Sets the group field and returns self for chaining.
    fn set_group(self, value: Vec<MeasureReportGroup>) -> Self;
    /// Adds an item to the group field and returns self for chaining.
    fn add_group(self, item: MeasureReportGroup) -> Self;
    /// Sets the evaluatedResource field and returns self for chaining.
    fn set_evaluated_resource(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the evaluatedResource field and returns self for chaining.
    fn add_evaluated_resource(self, item: Reference) -> Self;
}
/// MeasureReport Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MeasureReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MeasureReportExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the measure field is present (Some).
    fn has_measure(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the reporter field is present (Some).
    fn has_reporter(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the improvement_notation field is present (Some).
    fn has_improvement_notation(&self) -> bool;
    /// Returns true if the group field is not empty.
    fn has_group(&self) -> bool;
    /// Returns true if the evaluated_resource field is not empty.
    fn has_evaluated_resource(&self) -> bool;
}
