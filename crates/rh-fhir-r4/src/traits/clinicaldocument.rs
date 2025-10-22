use crate::traits::resource::ResourceExistence;
/// Clinical Document Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The Clinical Document profile constrains Composition to specify a clinical document (matching CDA).   The base Composition is a general resource for compositions or documents about any kind of subject that might be encountered in healthcare including such things as guidelines, medicines, etc. A clinical document is focused on documents related to the provision of care process, where the subject is a patient, a group of patients, or a closely related concept. A clinical document has additional requirements around confidentiality that do not apply in the same way to other kinds of documents.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/clinicaldocument
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait ClinicaldocumentAccessors {}
/// Clinical Document Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The Clinical Document profile constrains Composition to specify a clinical document (matching CDA).   The base Composition is a general resource for compositions or documents about any kind of subject that might be encountered in healthcare including such things as guidelines, medicines, etc. A clinical document is focused on documents related to the provision of care process, where the subject is a patient, a group of patients, or a closely related concept. A clinical document has additional requirements around confidentiality that do not apply in the same way to other kinds of documents.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/clinicaldocument
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait ClinicaldocumentMutators {
    /// Create a new Clinicaldocument with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::clinicaldocument::Clinicaldocument;
    /// use hl7_fhir_r4_core::traits::clinicaldocument::ClinicaldocumentMutators;
    ///
    /// let resource = Clinicaldocument::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// clinicaldocument Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The Clinical Document profile constrains Composition to specify a clinical document (matching CDA).
///
/// The base Composition is a general resource for compositions or documents about any kind of subject that might be encountered in healthcare including such things as guidelines, medicines, etc. A clinical document is focused on documents related to the provision of care process, where the subject is a patient, a group of patients, or a closely related concept. A clinical document has additional requirements around confidentiality that do not apply in the same way to other kinds of documents.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/clinicaldocument
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Composition
pub trait ClinicaldocumentExistence: ResourceExistence {}
