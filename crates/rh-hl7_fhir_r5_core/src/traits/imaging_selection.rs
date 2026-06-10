use crate::bindings::imagingselection_status::ImagingselectionStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::imaging_selection::ImagingSelectionInstance;
use crate::resources::imaging_selection::ImagingSelectionPerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ImagingSelection Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ImagingSelection
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingSelectionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ImagingselectionStatus;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<InstantType>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[ImagingSelectionPerformer];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the studyUid field.
    fn study_uid(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
    /// Returns a reference to the seriesUid field.
    fn series_uid(&self) -> Option<StringType>;
    /// Returns a reference to the seriesNumber field.
    fn series_number(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the frameOfReferenceUid field.
    fn frame_of_reference_uid(&self) -> Option<StringType>;
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableReference>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[Reference];
    /// Returns a reference to the instance field.
    fn instance(&self) -> &[ImagingSelectionInstance];
}
/// ImagingSelection Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ImagingSelection
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingSelectionMutators: DomainResourceMutators {
    /// Create a new ImagingSelection with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::imaging_selection::ImagingSelection;
    /// use rh_hl7_fhir_r5_core::traits::imaging_selection::ImagingSelectionMutators;
    ///
    /// let resource = ImagingSelection::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ImagingselectionStatus) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<ImagingSelectionPerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: ImagingSelectionPerformer) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the studyUid field and returns self for chaining.
    fn set_study_uid(self, value: String) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
    /// Sets the seriesUid field and returns self for chaining.
    fn set_series_uid(self, value: String) -> Self;
    /// Sets the seriesNumber field and returns self for chaining.
    fn set_series_number(self, value: i32) -> Self;
    /// Sets the frameOfReferenceUid field and returns self for chaining.
    fn set_frame_of_reference_uid(self, value: String) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableReference) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: Reference) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: Vec<ImagingSelectionInstance>) -> Self;
    /// Adds an item to the instance field and returns self for chaining.
    fn add_instance(self, item: ImagingSelectionInstance) -> Self;
}
/// ImagingSelection Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ImagingSelection
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingSelectionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the study_uid field is present (Some).
    fn has_study_uid(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
    /// Returns true if the series_uid field is present (Some).
    fn has_series_uid(&self) -> bool;
    /// Returns true if the series_number field is present (Some).
    fn has_series_number(&self) -> bool;
    /// Returns true if the frame_of_reference_uid field is present (Some).
    fn has_frame_of_reference_uid(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the instance field is not empty.
    fn has_instance(&self) -> bool;
}
