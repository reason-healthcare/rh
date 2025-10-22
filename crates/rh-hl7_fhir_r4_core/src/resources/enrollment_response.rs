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
