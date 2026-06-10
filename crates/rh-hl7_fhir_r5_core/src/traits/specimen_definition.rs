use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::specimen_definition::SpecimenDefinitionTypetested;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SpecimenDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFromCanonical field.
    fn derived_from_canonical(&self) -> &[StringType];
    /// Returns a reference to the derivedFromUri field.
    fn derived_from_uri(&self) -> &[StringType];
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
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the copyrightLabel field.
    fn copyright_label(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the typeCollected field.
    fn type_collected(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the patientPreparation field.
    fn patient_preparation(&self) -> &[CodeableConcept];
    /// Returns a reference to the timeAspect field.
    fn time_aspect(&self) -> Option<StringType>;
    /// Returns a reference to the collection field.
    fn collection(&self) -> &[CodeableConcept];
    /// Returns a reference to the typeTested field.
    fn type_tested(&self) -> &[SpecimenDefinitionTypetested];
}
/// SpecimenDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionMutators: DomainResourceMutators {
    /// Create a new SpecimenDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::specimen_definition::SpecimenDefinition;
    /// use rh_hl7_fhir_r5_core::traits::specimen_definition::SpecimenDefinitionMutators;
    ///
    /// let resource = SpecimenDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the derivedFromCanonical field and returns self for chaining.
    fn set_derived_from_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFromCanonical field and returns self for chaining.
    fn add_derived_from_canonical(self, item: String) -> Self;
    /// Sets the derivedFromUri field and returns self for chaining.
    fn set_derived_from_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFromUri field and returns self for chaining.
    fn add_derived_from_uri(self, item: String) -> Self;
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
    /// Sets the copyrightLabel field and returns self for chaining.
    fn set_copyright_label(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the typeCollected field and returns self for chaining.
    fn set_type_collected(self, value: CodeableConcept) -> Self;
    /// Sets the patientPreparation field and returns self for chaining.
    fn set_patient_preparation(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the patientPreparation field and returns self for chaining.
    fn add_patient_preparation(self, item: CodeableConcept) -> Self;
    /// Sets the timeAspect field and returns self for chaining.
    fn set_time_aspect(self, value: String) -> Self;
    /// Sets the collection field and returns self for chaining.
    fn set_collection(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the collection field and returns self for chaining.
    fn add_collection(self, item: CodeableConcept) -> Self;
    /// Sets the typeTested field and returns self for chaining.
    fn set_type_tested(self, value: Vec<SpecimenDefinitionTypetested>) -> Self;
    /// Adds an item to the typeTested field and returns self for chaining.
    fn add_type_tested(self, item: SpecimenDefinitionTypetested) -> Self;
}
/// SpecimenDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionExistence: DomainResourceExistence {
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the derived_from_canonical field is not empty.
    fn has_derived_from_canonical(&self) -> bool;
    /// Returns true if the derived_from_uri field is not empty.
    fn has_derived_from_uri(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
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
    /// Returns true if the copyright_label field is present (Some).
    fn has_copyright_label(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the type_collected field is present (Some).
    fn has_type_collected(&self) -> bool;
    /// Returns true if the patient_preparation field is not empty.
    fn has_patient_preparation(&self) -> bool;
    /// Returns true if the time_aspect field is present (Some).
    fn has_time_aspect(&self) -> bool;
    /// Returns true if the collection field is not empty.
    fn has_collection(&self) -> bool;
    /// Returns true if the type_tested field is not empty.
    fn has_type_tested(&self) -> bool;
}
