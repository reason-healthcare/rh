use crate::bindings::inventoryreport_counttype::InventoryreportCounttype;
use crate::bindings::inventoryreport_status::InventoryreportStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::inventory_report::InventoryReportInventorylisting;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// InventoryReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A report of inventory or stock items.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryReportAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> InventoryreportStatus;
    /// Returns a reference to the countType field.
    fn count_type(&self) -> InventoryreportCounttype;
    /// Returns a reference to the operationType field.
    fn operation_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the operationTypeReason field.
    fn operation_type_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the reportedDateTime field.
    fn reported_date_time(&self) -> DateTimeType;
    /// Returns a reference to the reporter field.
    fn reporter(&self) -> Option<Reference>;
    /// Returns a reference to the reportingPeriod field.
    fn reporting_period(&self) -> Option<Period>;
    /// Returns a reference to the inventoryListing field.
    fn inventory_listing(&self) -> &[InventoryReportInventorylisting];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// InventoryReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A report of inventory or stock items.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryReportMutators: DomainResourceMutators {
    /// Create a new InventoryReport with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::inventory_report::InventoryReport;
    /// use rh_hl7_fhir_r5_core::traits::inventory_report::InventoryReportMutators;
    ///
    /// let resource = InventoryReport::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: InventoryreportStatus) -> Self;
    /// Sets the countType field and returns self for chaining.
    fn set_count_type(self, value: InventoryreportCounttype) -> Self;
    /// Sets the operationType field and returns self for chaining.
    fn set_operation_type(self, value: CodeableConcept) -> Self;
    /// Sets the operationTypeReason field and returns self for chaining.
    fn set_operation_type_reason(self, value: CodeableConcept) -> Self;
    /// Sets the reportedDateTime field and returns self for chaining.
    fn set_reported_date_time(self, value: String) -> Self;
    /// Sets the reporter field and returns self for chaining.
    fn set_reporter(self, value: Reference) -> Self;
    /// Sets the reportingPeriod field and returns self for chaining.
    fn set_reporting_period(self, value: Period) -> Self;
    /// Sets the inventoryListing field and returns self for chaining.
    fn set_inventory_listing(self, value: Vec<InventoryReportInventorylisting>) -> Self;
    /// Adds an item to the inventoryListing field and returns self for chaining.
    fn add_inventory_listing(self, item: InventoryReportInventorylisting) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// InventoryReport Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A report of inventory or stock items.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryReportExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the count_type field is present (Some).
    fn has_count_type(&self) -> bool;
    /// Returns true if the operation_type field is present (Some).
    fn has_operation_type(&self) -> bool;
    /// Returns true if the operation_type_reason field is present (Some).
    fn has_operation_type_reason(&self) -> bool;
    /// Returns true if the reported_date_time field is present (Some).
    fn has_reported_date_time(&self) -> bool;
    /// Returns true if the reporter field is present (Some).
    fn has_reporter(&self) -> bool;
    /// Returns true if the reporting_period field is present (Some).
    fn has_reporting_period(&self) -> bool;
    /// Returns true if the inventory_listing field is not empty.
    fn has_inventory_listing(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
