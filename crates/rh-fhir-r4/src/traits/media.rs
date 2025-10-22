use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Media Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Media
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Media
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MediaAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> EventStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the modality field.
    fn modality(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the view field.
    fn view(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<InstantType>;
    /// Returns a reference to the operator field.
    fn operator(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the deviceName field.
    fn device_name(&self) -> Option<StringType>;
    /// Returns a reference to the device field.
    fn device(&self) -> Option<Reference>;
    /// Returns a reference to the height field.
    fn height(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the width field.
    fn width(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the frames field.
    fn frames(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the duration field.
    fn duration(&self) -> Option<DecimalType>;
    /// Returns a reference to the content field.
    fn content(&self) -> Attachment;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// Media Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Media
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Media
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MediaMutators: DomainResourceMutators {
    /// Create a new Media with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::media::Media;
    /// use hl7_fhir_r4_core::traits::media::MediaMutators;
    ///
    /// let resource = Media::new();
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
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EventStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the modality field and returns self for chaining.
    fn set_modality(self, value: CodeableConcept) -> Self;
    /// Sets the view field and returns self for chaining.
    fn set_view(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the operator field and returns self for chaining.
    fn set_operator(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableConcept) -> Self;
    /// Sets the deviceName field and returns self for chaining.
    fn set_device_name(self, value: String) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the height field and returns self for chaining.
    fn set_height(self, value: i32) -> Self;
    /// Sets the width field and returns self for chaining.
    fn set_width(self, value: i32) -> Self;
    /// Sets the frames field and returns self for chaining.
    fn set_frames(self, value: i32) -> Self;
    /// Sets the duration field and returns self for chaining.
    fn set_duration(self, value: f64) -> Self;
    /// Sets the content field and returns self for chaining.
    fn set_content(self, value: Attachment) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// Media Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Media
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Media
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MediaExistence: DomainResourceExistence {
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
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the modality field is present (Some).
    fn has_modality(&self) -> bool;
    /// Returns true if the view field is present (Some).
    fn has_view(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the operator field is present (Some).
    fn has_operator(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the device_name field is present (Some).
    fn has_device_name(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the height field is present (Some).
    fn has_height(&self) -> bool;
    /// Returns true if the width field is present (Some).
    fn has_width(&self) -> bool;
    /// Returns true if the frames field is present (Some).
    fn has_frames(&self) -> bool;
    /// Returns true if the duration field is present (Some).
    fn has_duration(&self) -> bool;
    /// Returns true if the content field is present (Some).
    fn has_content(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
