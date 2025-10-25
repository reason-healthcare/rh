use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EnrollmentResponse
///
/// This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: Option<FmStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Claim reference
    pub request: Option<Reference>,
    /// queued | complete | error | partial
    pub outcome: Option<RemittanceOutcome>,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Disposition Message
    pub disposition: Option<StringType>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
    /// Creation date
    pub created: Option<DateTimeType>,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Insurer
    pub organization: Option<Reference>,
    /// Responsible practitioner
    #[serde(rename = "requestProvider")]
    pub request_provider: Option<Reference>,
}

impl Default for EnrollmentResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            request: Default::default(),
            outcome: Default::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            created: Default::default(),
            _created: Default::default(),
            organization: Default::default(),
            request_provider: Default::default(),
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
        vec![
            rh_foundation::ElementBinding::new(
                "EnrollmentResponse.outcome",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/remittance-outcome|4.0.1",
            )
            .with_description("The outcome of the processing."),
            rh_foundation::ElementBinding::new(
                "EnrollmentResponse.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/fm-status|4.0.1",
            )
            .with_description("A code specifying the state of the resource instance."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EnrollmentResponse.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.contained", 0, None),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.extension", 0, None),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.request", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.disposition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.created", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EnrollmentResponse.organization", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EnrollmentResponse.requestProvider",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EnrollmentResponse {
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

impl crate::traits::resource::ResourceMutators for EnrollmentResponse {
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

impl crate::traits::resource::ResourceExistence for EnrollmentResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EnrollmentResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for EnrollmentResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for EnrollmentResponse {
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

impl crate::traits::enrollment_response::EnrollmentResponseAccessors for EnrollmentResponse {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<FmStatus> {
        self.status.clone()
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn outcome(&self) -> Option<RemittanceOutcome> {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn created(&self) -> Option<DateTimeType> {
        self.created.clone()
    }
    fn organization(&self) -> Option<Reference> {
        self.organization.clone()
    }
    fn request_provider(&self) -> Option<Reference> {
        self.request_provider.clone()
    }
}

impl crate::traits::enrollment_response::EnrollmentResponseMutators for EnrollmentResponse {
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
    fn set_status(self, value: FmStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_outcome(self, value: RemittanceOutcome) -> Self {
        let mut resource = self.clone();
        resource.outcome = Some(value);
        resource
    }
    fn set_disposition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.disposition = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = Some(value);
        resource
    }
    fn set_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.organization = Some(value);
        resource
    }
    fn set_request_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request_provider = Some(value);
        resource
    }
}

impl crate::traits::enrollment_response::EnrollmentResponseExistence for EnrollmentResponse {
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_created(&self) -> bool {
        self.created.is_some()
    }
    fn has_organization(&self) -> bool {
        self.organization.is_some()
    }
    fn has_request_provider(&self) -> bool {
        self.request_provider.is_some()
    }
}

impl crate::validation::ValidatableResource for EnrollmentResponse {
    fn resource_type(&self) -> &'static str {
        "EnrollmentResponse"
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
        Some("http://hl7.org/fhir/StructureDefinition/EnrollmentResponse")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::enrollment_response::{
    EnrollmentResponseAccessors, EnrollmentResponseExistence, EnrollmentResponseMutators,
};
