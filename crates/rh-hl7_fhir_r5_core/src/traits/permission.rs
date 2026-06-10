use crate::bindings::permission_rule_combining::PermissionRuleCombining;
use crate::bindings::permission_status::PermissionStatus;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::permission::PermissionJustification;
use crate::resources::permission::PermissionRule;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Permission Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Permission resource holds access rules for a given data and context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Permission
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Permission
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PermissionAccessors: DomainResourceAccessors {
    /// Returns a reference to the status field.
    fn status(&self) -> PermissionStatus;
    /// Returns a reference to the asserter field.
    fn asserter(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> &[DateTimeType];
    /// Returns a reference to the validity field.
    fn validity(&self) -> Option<Period>;
    /// Returns a reference to the justification field.
    fn justification(&self) -> Option<PermissionJustification>;
    /// Returns a reference to the combining field.
    fn combining(&self) -> PermissionRuleCombining;
    /// Returns a reference to the rule field.
    fn rule(&self) -> &[PermissionRule];
}
/// Permission Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Permission resource holds access rules for a given data and context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Permission
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Permission
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PermissionMutators: DomainResourceMutators {
    /// Create a new Permission with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::permission::Permission;
    /// use rh_hl7_fhir_r5_core::traits::permission::PermissionMutators;
    ///
    /// let resource = Permission::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PermissionStatus) -> Self;
    /// Sets the asserter field and returns self for chaining.
    fn set_asserter(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: Vec<String>) -> Self;
    /// Adds an item to the date field and returns self for chaining.
    fn add_date(self, item: String) -> Self;
    /// Sets the validity field and returns self for chaining.
    fn set_validity(self, value: Period) -> Self;
    /// Sets the justification field and returns self for chaining.
    fn set_justification(self, value: PermissionJustification) -> Self;
    /// Sets the combining field and returns self for chaining.
    fn set_combining(self, value: PermissionRuleCombining) -> Self;
    /// Sets the rule field and returns self for chaining.
    fn set_rule(self, value: Vec<PermissionRule>) -> Self;
    /// Adds an item to the rule field and returns self for chaining.
    fn add_rule(self, item: PermissionRule) -> Self;
}
/// Permission Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Permission resource holds access rules for a given data and context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Permission
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Permission
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PermissionExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the asserter field is present (Some).
    fn has_asserter(&self) -> bool;
    /// Returns true if the date field is not empty.
    fn has_date(&self) -> bool;
    /// Returns true if the validity field is present (Some).
    fn has_validity(&self) -> bool;
    /// Returns true if the justification field is present (Some).
    fn has_justification(&self) -> bool;
    /// Returns true if the combining field is present (Some).
    fn has_combining(&self) -> bool;
    /// Returns true if the rule field is not empty.
    fn has_rule(&self) -> bool;
}
