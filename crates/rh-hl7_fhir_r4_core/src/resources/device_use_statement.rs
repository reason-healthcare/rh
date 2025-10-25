use crate::bindings::device_statement_status::DeviceStatementStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceUseStatement
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or another clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUseStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceUseStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUseStatement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier for this record
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// active | completed | entered-in-error +
    pub status: DeviceStatementStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Patient using device
    pub subject: Reference,
    /// Supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// How often  the device was used (Timing)
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,
    /// How often  the device was used (Period)
    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,
    /// How often  the device was used (dateTime)
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<DateTimeType>,
    /// When statement was recorded
    #[serde(rename = "recordedOn")]
    pub recorded_on: Option<DateTimeType>,
    /// Extension element for the 'recordedOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recordedOn")]
    pub _recorded_on: Option<Element>,
    /// Who made the statement
    pub source: Option<Reference>,
    /// Reference to device used
    pub device: Reference,
    /// Why device was used
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why was DeviceUseStatement performed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Target body site
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Addition details (comments, instructions)
    pub note: Option<Vec<Annotation>>,
}

impl Default for DeviceUseStatement {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            status: DeviceStatementStatus::default(),
            _status: Default::default(),
            subject: Reference::default(),
            derived_from: Default::default(),
            timing_timing: Default::default(),
            timing_period: Default::default(),
            timing_date_time: Default::default(),
            recorded_on: Default::default(),
            _recorded_on: Default::default(),
            source: Default::default(),
            device: Reference::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            body_site: Default::default(),
            note: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "DeviceUseStatement.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/device-statement-status|4.0.1",
        )
        .with_description("A coded concept indicating the current status of the Device Usage.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DeviceUseStatement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.contained", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.extension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.recordedOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.source", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.device", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUseStatement.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceUseStatement {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for DeviceUseStatement {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for DeviceUseStatement {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceUseStatement {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for DeviceUseStatement {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for DeviceUseStatement {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::device_use_statement::DeviceUseStatementAccessors for DeviceUseStatement {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DeviceStatementStatus {
        self.status.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn recorded_on(&self) -> Option<DateTimeType> {
        self.recorded_on.clone()
    }
    fn source(&self) -> Option<Reference> {
        self.source.clone()
    }
    fn device(&self) -> Reference {
        self.device.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> Option<CodeableConcept> {
        self.body_site.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_use_statement::DeviceUseStatementMutators for DeviceUseStatement {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: DeviceStatementStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_derived_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_recorded_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded_on = Some(value);
        resource
    }
    fn set_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = value;
        resource
    }
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_body_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::device_use_statement::DeviceUseStatementExistence for DeviceUseStatement {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_timing(&self) -> bool {
        self.timing_timing.is_some()
            || self.timing_period.is_some()
            || self.timing_date_time.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_recorded_on(&self) -> bool {
        self.recorded_on.is_some()
    }
    fn has_source(&self) -> bool {
        self.source.is_some()
    }
    fn has_device(&self) -> bool {
        true
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceUseStatement {
    fn resource_type(&self) -> &'static str {
        "DeviceUseStatement"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/DeviceUseStatement")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device_use_statement::{
    DeviceUseStatementAccessors, DeviceUseStatementExistence, DeviceUseStatementMutators,
};
