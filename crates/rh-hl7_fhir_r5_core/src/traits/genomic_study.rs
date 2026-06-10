use crate::bindings::genomicstudy_status::GenomicstudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::genomic_study::GenomicStudyAnalysis;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// GenomicStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of analyses performed to analyze and generate genomic data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GenomicStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GenomicStudyAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> GenomicstudyStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the startDate field.
    fn start_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the referrer field.
    fn referrer(&self) -> Option<Reference>;
    /// Returns a reference to the interpreter field.
    fn interpreter(&self) -> &[Reference];
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> Option<StringType>;
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the analysis field.
    fn analysis(&self) -> &[GenomicStudyAnalysis];
}
/// GenomicStudy Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of analyses performed to analyze and generate genomic data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GenomicStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GenomicStudyMutators: DomainResourceMutators {
    /// Create a new GenomicStudy with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::genomic_study::GenomicStudy;
    /// use rh_hl7_fhir_r5_core::traits::genomic_study::GenomicStudyMutators;
    ///
    /// let resource = GenomicStudy::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: GenomicstudyStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the startDate field and returns self for chaining.
    fn set_start_date(self, value: String) -> Self;
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
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the analysis field and returns self for chaining.
    fn set_analysis(self, value: Vec<GenomicStudyAnalysis>) -> Self;
    /// Adds an item to the analysis field and returns self for chaining.
    fn add_analysis(self, item: GenomicStudyAnalysis) -> Self;
}
/// GenomicStudy Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A set of analyses performed to analyze and generate genomic data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GenomicStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GenomicStudyExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the start_date field is present (Some).
    fn has_start_date(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the referrer field is present (Some).
    fn has_referrer(&self) -> bool;
    /// Returns true if the interpreter field is not empty.
    fn has_interpreter(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the instantiates_canonical field is present (Some).
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is present (Some).
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the analysis field is not empty.
    fn has_analysis(&self) -> bool;
}
