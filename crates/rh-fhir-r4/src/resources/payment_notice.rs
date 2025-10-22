use crate::bindings::fm_status::FmStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PaymentNotice
///
/// This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentNotice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentNotice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for the payment noctice
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Request reference
    pub request: Option<Reference>,
    /// Response reference
    pub response: Option<Reference>,
    /// Creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Responsible practitioner
    pub provider: Option<Reference>,
    /// Payment reference
    pub payment: Reference,
    /// Payment or clearing date
    #[serde(rename = "paymentDate")]
    pub payment_date: Option<StringType>,
    /// Extension element for the 'paymentDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paymentDate")]
    pub _payment_date: Option<Element>,
    /// Party being paid
    pub payee: Option<Reference>,
    /// Party being notified
    pub recipient: Reference,
    /// Monetary amount of the payment
    pub amount: Money,
    /// Issued or cleared Status of the payment
    ///
    /// Binding: example (The payment conveyance status codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-status
    #[serde(rename = "paymentStatus")]
    pub payment_status: Option<CodeableConcept>,
}

impl Default for PaymentNotice {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            request: Default::default(),
            response: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            provider: Default::default(),
            payment: Reference::default(),
            payment_date: Default::default(),
            _payment_date: Default::default(),
            payee: Default::default(),
            recipient: Reference::default(),
            amount: Money::default(),
            payment_status: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PaymentNotice {
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

impl crate::traits::resource::ResourceMutators for PaymentNotice {
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

impl crate::traits::resource::ResourceExistence for PaymentNotice {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PaymentNotice {
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

impl crate::traits::domain_resource::DomainResourceMutators for PaymentNotice {
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

impl crate::traits::domain_resource::DomainResourceExistence for PaymentNotice {
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

impl crate::traits::payment_notice::PaymentNoticeAccessors for PaymentNotice {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn response(&self) -> Option<Reference> {
        self.response.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn provider(&self) -> Option<Reference> {
        self.provider.clone()
    }
    fn payment(&self) -> Reference {
        self.payment.clone()
    }
    fn payment_date(&self) -> Option<StringType> {
        self.payment_date.clone()
    }
    fn payee(&self) -> Option<Reference> {
        self.payee.clone()
    }
    fn recipient(&self) -> Reference {
        self.recipient.clone()
    }
    fn amount(&self) -> Money {
        self.amount.clone()
    }
    fn payment_status(&self) -> Option<CodeableConcept> {
        self.payment_status.clone()
    }
}

impl crate::traits::payment_notice::PaymentNoticeMutators for PaymentNotice {
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
        resource.status = value;
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_response(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.response = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.provider = Some(value);
        resource
    }
    fn set_payment(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.payment = value;
        resource
    }
    fn set_payment_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.payment_date = Some(value);
        resource
    }
    fn set_payee(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.payee = Some(value);
        resource
    }
    fn set_recipient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recipient = value;
        resource
    }
    fn set_amount(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.amount = value;
        resource
    }
    fn set_payment_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.payment_status = Some(value);
        resource
    }
}

impl crate::traits::payment_notice::PaymentNoticeExistence for PaymentNotice {
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
        true
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_response(&self) -> bool {
        self.response.is_some()
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_provider(&self) -> bool {
        self.provider.is_some()
    }
    fn has_payment(&self) -> bool {
        true
    }
    fn has_payment_date(&self) -> bool {
        self.payment_date.is_some()
    }
    fn has_payee(&self) -> bool {
        self.payee.is_some()
    }
    fn has_recipient(&self) -> bool {
        true
    }
    fn has_amount(&self) -> bool {
        true
    }
    fn has_payment_status(&self) -> bool {
        self.payment_status.is_some()
    }
}
