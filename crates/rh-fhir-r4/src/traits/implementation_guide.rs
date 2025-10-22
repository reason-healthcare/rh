use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::spdx_license::SpdxLicense;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::implementation_guide::ImplementationGuideDefinition;
use crate::resources::implementation_guide::ImplementationGuideDependson;
use crate::resources::implementation_guide::ImplementationGuideGlobal;
use crate::resources::implementation_guide::ImplementationGuideManifest;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ImplementationGuide Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImplementationGuide
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImplementationGuideAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
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
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the packageId field.
    fn package_id(&self) -> StringType;
    /// Returns a reference to the license field.
    fn license(&self) -> Option<SpdxLicense>;
    /// Returns a reference to the fhirVersion field.
    fn fhir_version(&self) -> &[FHIRVersion];
    /// Returns a reference to the dependsOn field.
    fn depends_on(&self) -> &[ImplementationGuideDependson];
    /// Returns a reference to the global field.
    fn global(&self) -> &[ImplementationGuideGlobal];
    /// Returns a reference to the definition field.
    fn definition(&self) -> Option<ImplementationGuideDefinition>;
    /// Returns a reference to the manifest field.
    fn manifest(&self) -> Option<ImplementationGuideManifest>;
}
/// ImplementationGuide Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImplementationGuide
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImplementationGuideMutators: DomainResourceMutators {
    /// Create a new ImplementationGuide with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::implementation_guide::ImplementationGuide;
    /// use hl7_fhir_r4_core::traits::implementation_guide::ImplementationGuideMutators;
    ///
    /// let resource = ImplementationGuide::new();
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
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the packageId field and returns self for chaining.
    fn set_package_id(self, value: String) -> Self;
    /// Sets the license field and returns self for chaining.
    fn set_license(self, value: SpdxLicense) -> Self;
    /// Sets the fhirVersion field and returns self for chaining.
    fn set_fhir_version(self, value: Vec<FHIRVersion>) -> Self;
    /// Adds an item to the fhirVersion field and returns self for chaining.
    fn add_fhir_version(self, item: FHIRVersion) -> Self;
    /// Sets the dependsOn field and returns self for chaining.
    fn set_depends_on(self, value: Vec<ImplementationGuideDependson>) -> Self;
    /// Adds an item to the dependsOn field and returns self for chaining.
    fn add_depends_on(self, item: ImplementationGuideDependson) -> Self;
    /// Sets the global field and returns self for chaining.
    fn set_global(self, value: Vec<ImplementationGuideGlobal>) -> Self;
    /// Adds an item to the global field and returns self for chaining.
    fn add_global(self, item: ImplementationGuideGlobal) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: ImplementationGuideDefinition) -> Self;
    /// Sets the manifest field and returns self for chaining.
    fn set_manifest(self, value: ImplementationGuideManifest) -> Self;
}
/// ImplementationGuide Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImplementationGuide
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImplementationGuideExistence: DomainResourceExistence {
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
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the package_id field is present (Some).
    fn has_package_id(&self) -> bool;
    /// Returns true if the license field is present (Some).
    fn has_license(&self) -> bool;
    /// Returns true if the fhir_version field is not empty.
    fn has_fhir_version(&self) -> bool;
    /// Returns true if the depends_on field is not empty.
    fn has_depends_on(&self) -> bool;
    /// Returns true if the global field is not empty.
    fn has_global(&self) -> bool;
    /// Returns true if the definition field is present (Some).
    fn has_definition(&self) -> bool;
    /// Returns true if the manifest field is present (Some).
    fn has_manifest(&self) -> bool;
}
