use crate::bindings::diagnostic_report_status::DiagnosticReportStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::diagnostic_report::DiagnosticReportMedia;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DiagnosticReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The findings and interpretation of diagnostic  tests performed on patients, groups of patients, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting and provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DiagnosticReportAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> DiagnosticReportStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<InstantType>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[Reference];
    /// Returns a reference to the resultsInterpreter field.
    fn results_interpreter(&self) -> &[Reference];
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> &[Reference];
    /// Returns a reference to the result field.
    fn result(&self) -> &[Reference];
    /// Returns a reference to the imagingStudy field.
    fn imaging_study(&self) -> &[Reference];
    /// Returns a reference to the media field.
    fn media(&self) -> &[DiagnosticReportMedia];
    /// Returns a reference to the conclusion field.
    fn conclusion(&self) -> Option<StringType>;
    /// Returns a reference to the conclusionCode field.
    fn conclusion_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the presentedForm field.
    fn presented_form(&self) -> &[Attachment];
}
/// DiagnosticReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The findings and interpretation of diagnostic  tests performed on patients, groups of patients, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting and provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DiagnosticReportMutators: DomainResourceMutators {
    /// Create a new DiagnosticReport with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::diagnostic_report::DiagnosticReport;
    /// use hl7_fhir_r4_core::traits::diagnostic_report::DiagnosticReportMutators;
    ///
    /// let resource = DiagnosticReport::new();
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
    fn set_status(self, value: DiagnosticReportStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: Reference) -> Self;
    /// Sets the resultsInterpreter field and returns self for chaining.
    fn set_results_interpreter(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the resultsInterpreter field and returns self for chaining.
    fn add_results_interpreter(self, item: Reference) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the specimen field and returns self for chaining.
    fn add_specimen(self, item: Reference) -> Self;
    /// Sets the result field and returns self for chaining.
    fn set_result(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the result field and returns self for chaining.
    fn add_result(self, item: Reference) -> Self;
    /// Sets the imagingStudy field and returns self for chaining.
    fn set_imaging_study(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the imagingStudy field and returns self for chaining.
    fn add_imaging_study(self, item: Reference) -> Self;
    /// Sets the media field and returns self for chaining.
    fn set_media(self, value: Vec<DiagnosticReportMedia>) -> Self;
    /// Adds an item to the media field and returns self for chaining.
    fn add_media(self, item: DiagnosticReportMedia) -> Self;
    /// Sets the conclusion field and returns self for chaining.
    fn set_conclusion(self, value: String) -> Self;
    /// Sets the conclusionCode field and returns self for chaining.
    fn set_conclusion_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the conclusionCode field and returns self for chaining.
    fn add_conclusion_code(self, item: CodeableConcept) -> Self;
    /// Sets the presentedForm field and returns self for chaining.
    fn set_presented_form(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the presentedForm field and returns self for chaining.
    fn add_presented_form(self, item: Attachment) -> Self;
}
/// DiagnosticReport Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The findings and interpretation of diagnostic  tests performed on patients, groups of patients, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting and provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DiagnosticReportExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the effective field is present (Some).
    fn has_effective(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the results_interpreter field is not empty.
    fn has_results_interpreter(&self) -> bool;
    /// Returns true if the specimen field is not empty.
    fn has_specimen(&self) -> bool;
    /// Returns true if the result field is not empty.
    fn has_result(&self) -> bool;
    /// Returns true if the imaging_study field is not empty.
    fn has_imaging_study(&self) -> bool;
    /// Returns true if the media field is not empty.
    fn has_media(&self) -> bool;
    /// Returns true if the conclusion field is present (Some).
    fn has_conclusion(&self) -> bool;
    /// Returns true if the conclusion_code field is not empty.
    fn has_conclusion_code(&self) -> bool;
    /// Returns true if the presented_form field is not empty.
    fn has_presented_form(&self) -> bool;
}
