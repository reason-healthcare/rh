use crate::bindings::capability_statement_kind::CapabilityStatementKind;
use crate::bindings::code_search_support::CodeSearchSupport;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesClosure;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesCodesystem;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesExpansion;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesImplementation;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesSoftware;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesTranslation;
use crate::resources::terminology_capabilities::TerminologyCapabilitiesValidatecode;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// TerminologyCapabilities Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TerminologyCapabilities
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TerminologyCapabilitiesAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the kind field.
    fn kind(&self) -> CapabilityStatementKind;
    /// Returns a reference to the software field.
    fn software(&self) -> Option<TerminologyCapabilitiesSoftware>;
    /// Returns a reference to the implementation field.
    fn implementation(&self) -> Option<TerminologyCapabilitiesImplementation>;
    /// Returns a reference to the lockedDate field.
    fn locked_date(&self) -> Option<BooleanType>;
    /// Returns a reference to the codeSystem field.
    fn code_system(&self) -> &[TerminologyCapabilitiesCodesystem];
    /// Returns a reference to the expansion field.
    fn expansion(&self) -> Option<TerminologyCapabilitiesExpansion>;
    /// Returns a reference to the codeSearch field.
    fn code_search(&self) -> Option<CodeSearchSupport>;
    /// Returns a reference to the validateCode field.
    fn validate_code(&self) -> Option<TerminologyCapabilitiesValidatecode>;
    /// Returns a reference to the translation field.
    fn translation(&self) -> Option<TerminologyCapabilitiesTranslation>;
    /// Returns a reference to the closure field.
    fn closure(&self) -> Option<TerminologyCapabilitiesClosure>;
}
/// TerminologyCapabilities Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TerminologyCapabilities
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TerminologyCapabilitiesMutators: DomainResourceMutators {
    /// Create a new TerminologyCapabilities with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::terminology_capabilities::TerminologyCapabilities;
    /// use hl7_fhir_r4_core::traits::terminology_capabilities::TerminologyCapabilitiesMutators;
    ///
    /// let resource = TerminologyCapabilities::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: CapabilityStatementKind) -> Self;
    /// Sets the software field and returns self for chaining.
    fn set_software(self, value: TerminologyCapabilitiesSoftware) -> Self;
    /// Sets the implementation field and returns self for chaining.
    fn set_implementation(self, value: TerminologyCapabilitiesImplementation) -> Self;
    /// Sets the lockedDate field and returns self for chaining.
    fn set_locked_date(self, value: bool) -> Self;
    /// Sets the codeSystem field and returns self for chaining.
    fn set_code_system(self, value: Vec<TerminologyCapabilitiesCodesystem>) -> Self;
    /// Adds an item to the codeSystem field and returns self for chaining.
    fn add_code_system(self, item: TerminologyCapabilitiesCodesystem) -> Self;
    /// Sets the expansion field and returns self for chaining.
    fn set_expansion(self, value: TerminologyCapabilitiesExpansion) -> Self;
    /// Sets the codeSearch field and returns self for chaining.
    fn set_code_search(self, value: CodeSearchSupport) -> Self;
    /// Sets the validateCode field and returns self for chaining.
    fn set_validate_code(self, value: TerminologyCapabilitiesValidatecode) -> Self;
    /// Sets the translation field and returns self for chaining.
    fn set_translation(self, value: TerminologyCapabilitiesTranslation) -> Self;
    /// Sets the closure field and returns self for chaining.
    fn set_closure(self, value: TerminologyCapabilitiesClosure) -> Self;
}
/// TerminologyCapabilities Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TerminologyCapabilities
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TerminologyCapabilitiesExistence: DomainResourceExistence {
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
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
    /// Returns true if the software field is present (Some).
    fn has_software(&self) -> bool;
    /// Returns true if the implementation field is present (Some).
    fn has_implementation(&self) -> bool;
    /// Returns true if the locked_date field is present (Some).
    fn has_locked_date(&self) -> bool;
    /// Returns true if the code_system field is not empty.
    fn has_code_system(&self) -> bool;
    /// Returns true if the expansion field is present (Some).
    fn has_expansion(&self) -> bool;
    /// Returns true if the code_search field is present (Some).
    fn has_code_search(&self) -> bool;
    /// Returns true if the validate_code field is present (Some).
    fn has_validate_code(&self) -> bool;
    /// Returns true if the translation field is present (Some).
    fn has_translation(&self) -> bool;
    /// Returns true if the closure field is present (Some).
    fn has_closure(&self) -> bool;
}
