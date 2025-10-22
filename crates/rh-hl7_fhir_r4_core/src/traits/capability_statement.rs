use crate::bindings::capability_statement_kind::CapabilityStatementKind;
use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::capability_statement::CapabilityStatementDocument;
use crate::resources::capability_statement::CapabilityStatementImplementation;
use crate::resources::capability_statement::CapabilityStatementMessaging;
use crate::resources::capability_statement::CapabilityStatementRest;
use crate::resources::capability_statement::CapabilityStatementSoftware;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CapabilityStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CapabilityStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CapabilityStatementAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the instantiates field.
    fn instantiates(&self) -> &[StringType];
    /// Returns a reference to the imports field.
    fn imports(&self) -> &[StringType];
    /// Returns a reference to the software field.
    fn software(&self) -> Option<CapabilityStatementSoftware>;
    /// Returns a reference to the implementation field.
    fn implementation(&self) -> Option<CapabilityStatementImplementation>;
    /// Returns a reference to the fhirVersion field.
    fn fhir_version(&self) -> FHIRVersion;
    /// Returns a reference to the format field.
    fn format(&self) -> &[Mimetypes];
    /// Returns a reference to the patchFormat field.
    fn patch_format(&self) -> &[Mimetypes];
    /// Returns a reference to the implementationGuide field.
    fn implementation_guide(&self) -> &[StringType];
    /// Returns a reference to the rest field.
    fn rest(&self) -> &[CapabilityStatementRest];
    /// Returns a reference to the messaging field.
    fn messaging(&self) -> &[CapabilityStatementMessaging];
    /// Returns a reference to the document field.
    fn document(&self) -> &[CapabilityStatementDocument];
}
/// CapabilityStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CapabilityStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CapabilityStatementMutators: DomainResourceMutators {
    /// Create a new CapabilityStatement with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::capability_statement::CapabilityStatement;
    /// use hl7_fhir_r4_core::traits::capability_statement::CapabilityStatementMutators;
    ///
    /// let resource = CapabilityStatement::new();
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
    /// Sets the instantiates field and returns self for chaining.
    fn set_instantiates(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiates field and returns self for chaining.
    fn add_instantiates(self, item: String) -> Self;
    /// Sets the imports field and returns self for chaining.
    fn set_imports(self, value: Vec<String>) -> Self;
    /// Adds an item to the imports field and returns self for chaining.
    fn add_imports(self, item: String) -> Self;
    /// Sets the software field and returns self for chaining.
    fn set_software(self, value: CapabilityStatementSoftware) -> Self;
    /// Sets the implementation field and returns self for chaining.
    fn set_implementation(self, value: CapabilityStatementImplementation) -> Self;
    /// Sets the fhirVersion field and returns self for chaining.
    fn set_fhir_version(self, value: FHIRVersion) -> Self;
    /// Sets the format field and returns self for chaining.
    fn set_format(self, value: Vec<Mimetypes>) -> Self;
    /// Adds an item to the format field and returns self for chaining.
    fn add_format(self, item: Mimetypes) -> Self;
    /// Sets the patchFormat field and returns self for chaining.
    fn set_patch_format(self, value: Vec<Mimetypes>) -> Self;
    /// Adds an item to the patchFormat field and returns self for chaining.
    fn add_patch_format(self, item: Mimetypes) -> Self;
    /// Sets the implementationGuide field and returns self for chaining.
    fn set_implementation_guide(self, value: Vec<String>) -> Self;
    /// Adds an item to the implementationGuide field and returns self for chaining.
    fn add_implementation_guide(self, item: String) -> Self;
    /// Sets the rest field and returns self for chaining.
    fn set_rest(self, value: Vec<CapabilityStatementRest>) -> Self;
    /// Adds an item to the rest field and returns self for chaining.
    fn add_rest(self, item: CapabilityStatementRest) -> Self;
    /// Sets the messaging field and returns self for chaining.
    fn set_messaging(self, value: Vec<CapabilityStatementMessaging>) -> Self;
    /// Adds an item to the messaging field and returns self for chaining.
    fn add_messaging(self, item: CapabilityStatementMessaging) -> Self;
    /// Sets the document field and returns self for chaining.
    fn set_document(self, value: Vec<CapabilityStatementDocument>) -> Self;
    /// Adds an item to the document field and returns self for chaining.
    fn add_document(self, item: CapabilityStatementDocument) -> Self;
}
/// CapabilityStatement Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CapabilityStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CapabilityStatementExistence: DomainResourceExistence {
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
    /// Returns true if the instantiates field is not empty.
    fn has_instantiates(&self) -> bool;
    /// Returns true if the imports field is not empty.
    fn has_imports(&self) -> bool;
    /// Returns true if the software field is present (Some).
    fn has_software(&self) -> bool;
    /// Returns true if the implementation field is present (Some).
    fn has_implementation(&self) -> bool;
    /// Returns true if the fhir_version field is present (Some).
    fn has_fhir_version(&self) -> bool;
    /// Returns true if the format field is not empty.
    fn has_format(&self) -> bool;
    /// Returns true if the patch_format field is not empty.
    fn has_patch_format(&self) -> bool;
    /// Returns true if the implementation_guide field is not empty.
    fn has_implementation_guide(&self) -> bool;
    /// Returns true if the rest field is not empty.
    fn has_rest(&self) -> bool;
    /// Returns true if the messaging field is not empty.
    fn has_messaging(&self) -> bool;
    /// Returns true if the document field is not empty.
    fn has_document(&self) -> bool;
}
