use crate::bindings::imagingstudy_status::ImagingstudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::imaging_study::ImagingStudySeries;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ImagingStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImagingStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingStudyAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ImagingstudyStatus;
    /// Returns a reference to the modality field.
    fn modality(&self) -> &[Coding];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the started field.
    fn started(&self) -> Option<DateTimeType>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the referrer field.
    fn referrer(&self) -> Option<Reference>;
    /// Returns a reference to the interpreter field.
    fn interpreter(&self) -> &[Reference];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
    /// Returns a reference to the numberOfSeries field.
    fn number_of_series(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the numberOfInstances field.
    fn number_of_instances(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the procedureReference field.
    fn procedure_reference(&self) -> Option<Reference>;
    /// Returns a reference to the procedureCode field.
    fn procedure_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the series field.
    fn series(&self) -> &[ImagingStudySeries];
}
/// ImagingStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImagingStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingStudyMutators: DomainResourceMutators {
    /// Create a new ImagingStudy with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::imaging_study::ImagingStudy;
    /// use hl7_fhir_r4_core::traits::imaging_study::ImagingStudyMutators;
    ///
    /// let resource = ImagingStudy::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ImagingstudyStatus) -> Self;
    /// Sets the modality field and returns self for chaining.
    fn set_modality(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the modality field and returns self for chaining.
    fn add_modality(self, item: Coding) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the started field and returns self for chaining.
    fn set_started(self, value: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the referrer field and returns self for chaining.
    fn set_referrer(self, value: Reference) -> Self;
    /// Sets the interpreter field and returns self for chaining.
    fn set_interpreter(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the interpreter field and returns self for chaining.
    fn add_interpreter(self, item: Reference) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
    /// Sets the numberOfSeries field and returns self for chaining.
    fn set_number_of_series(self, value: i32) -> Self;
    /// Sets the numberOfInstances field and returns self for chaining.
    fn set_number_of_instances(self, value: i32) -> Self;
    /// Sets the procedureReference field and returns self for chaining.
    fn set_procedure_reference(self, value: Reference) -> Self;
    /// Sets the procedureCode field and returns self for chaining.
    fn set_procedure_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the procedureCode field and returns self for chaining.
    fn add_procedure_code(self, item: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the series field and returns self for chaining.
    fn set_series(self, value: Vec<ImagingStudySeries>) -> Self;
    /// Adds an item to the series field and returns self for chaining.
    fn add_series(self, item: ImagingStudySeries) -> Self;
}
/// ImagingStudy Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImagingStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImagingStudyExistence: DomainResourceExistence {
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
    /// Returns true if the modality field is not empty.
    fn has_modality(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the started field is present (Some).
    fn has_started(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the referrer field is present (Some).
    fn has_referrer(&self) -> bool;
    /// Returns true if the interpreter field is not empty.
    fn has_interpreter(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
    /// Returns true if the number_of_series field is present (Some).
    fn has_number_of_series(&self) -> bool;
    /// Returns true if the number_of_instances field is present (Some).
    fn has_number_of_instances(&self) -> bool;
    /// Returns true if the procedure_reference field is present (Some).
    fn has_procedure_reference(&self) -> bool;
    /// Returns true if the procedure_code field is not empty.
    fn has_procedure_code(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the series field is not empty.
    fn has_series(&self) -> bool;
}
