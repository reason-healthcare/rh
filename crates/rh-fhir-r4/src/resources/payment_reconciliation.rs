use crate::bindings::fm_status::FmStatus;
use crate::bindings::note_type::NoteType;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PaymentReconciliation
///
/// This resource provides the details including amount of a payment and allocates the payment items being paid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for a payment reconciliation
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Period covered
    pub period: Option<Period>,
    /// Creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Party generating payment
    #[serde(rename = "paymentIssuer")]
    pub payment_issuer: Option<Reference>,
    /// Reference to requesting resource
    pub request: Option<Reference>,
    /// Responsible practitioner
    pub requestor: Option<Reference>,
    /// queued | complete | error | partial
    pub outcome: Option<RemittanceOutcome>,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Disposition message
    pub disposition: Option<StringType>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
    /// When payment issued
    #[serde(rename = "paymentDate")]
    pub payment_date: StringType,
    /// Extension element for the 'paymentDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paymentDate")]
    pub _payment_date: Option<Element>,
    /// Total amount of Payment
    #[serde(rename = "paymentAmount")]
    pub payment_amount: Money,
    /// Business identifier for the payment
    #[serde(rename = "paymentIdentifier")]
    pub payment_identifier: Option<Identifier>,
    /// Settlement particulars
    pub detail: Option<Vec<PaymentReconciliationDetail>>,
    /// Printed form identifier
    ///
    /// Binding: example (The forms codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/forms
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,
    /// Note concerning processing
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<PaymentReconciliationProcessnote>>,
}
/// PaymentReconciliation nested structure for the 'processNote' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliationProcessnote {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// display | print | printoper
    #[serde(rename = "type")]
    pub type_: Option<NoteType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Note explanatory text
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}
/// PaymentReconciliation nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReconciliationDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Business identifier of the payment detail
    pub identifier: Option<Identifier>,
    /// Business identifier of the prior payment detail
    pub predecessor: Option<Identifier>,
    /// Category of payment
    ///
    /// Binding: example (The reason for the amount: payment, adjustment, advance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Request giving rise to the payment
    pub request: Option<Reference>,
    /// Submitter of the request
    pub submitter: Option<Reference>,
    /// Response committing to a payment
    pub response: Option<Reference>,
    /// Date of commitment to pay
    pub date: Option<StringType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Contact for the response
    pub responsible: Option<Reference>,
    /// Recipient of the payment
    pub payee: Option<Reference>,
    /// Amount allocated to this payable
    pub amount: Option<Money>,
}

impl Default for PaymentReconciliation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            payment_issuer: Default::default(),
            request: Default::default(),
            requestor: Default::default(),
            outcome: Default::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            payment_date: StringType::default(),
            _payment_date: Default::default(),
            payment_amount: Money::default(),
            payment_identifier: Default::default(),
            detail: Default::default(),
            form_code: Default::default(),
            process_note: Default::default(),
        }
    }
}

impl Default for PaymentReconciliationProcessnote {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}

impl Default for PaymentReconciliationDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            predecessor: Default::default(),
            type_: Default::default(),
            request: Default::default(),
            submitter: Default::default(),
            response: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            responsible: Default::default(),
            payee: Default::default(),
            amount: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PaymentReconciliation {
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

impl crate::traits::resource::ResourceMutators for PaymentReconciliation {
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

impl crate::traits::resource::ResourceExistence for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceMutators for PaymentReconciliation {
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

impl crate::traits::domain_resource::DomainResourceExistence for PaymentReconciliation {
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

impl crate::traits::payment_reconciliation::PaymentReconciliationAccessors
    for PaymentReconciliation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn payment_issuer(&self) -> Option<Reference> {
        self.payment_issuer.clone()
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn requestor(&self) -> Option<Reference> {
        self.requestor.clone()
    }
    fn outcome(&self) -> Option<RemittanceOutcome> {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn payment_date(&self) -> StringType {
        self.payment_date.clone()
    }
    fn payment_amount(&self) -> Money {
        self.payment_amount.clone()
    }
    fn payment_identifier(&self) -> Option<Identifier> {
        self.payment_identifier.clone()
    }
    fn detail(&self) -> &[PaymentReconciliationDetail] {
        self.detail.as_deref().unwrap_or(&[])
    }
    fn form_code(&self) -> Option<CodeableConcept> {
        self.form_code.clone()
    }
    fn process_note(&self) -> &[PaymentReconciliationProcessnote] {
        self.process_note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::payment_reconciliation::PaymentReconciliationMutators
    for PaymentReconciliation
{
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
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_payment_issuer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.payment_issuer = Some(value);
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_requestor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requestor = Some(value);
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
    fn set_payment_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.payment_date = value;
        resource
    }
    fn set_payment_amount(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.payment_amount = value;
        resource
    }
    fn set_payment_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.payment_identifier = Some(value);
        resource
    }
    fn set_detail(self, value: Vec<PaymentReconciliationDetail>) -> Self {
        let mut resource = self.clone();
        resource.detail = Some(value);
        resource
    }
    fn add_detail(self, item: PaymentReconciliationDetail) -> Self {
        let mut resource = self.clone();
        resource.detail.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_form_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form_code = Some(value);
        resource
    }
    fn set_process_note(self, value: Vec<PaymentReconciliationProcessnote>) -> Self {
        let mut resource = self.clone();
        resource.process_note = Some(value);
        resource
    }
    fn add_process_note(self, item: PaymentReconciliationProcessnote) -> Self {
        let mut resource = self.clone();
        resource
            .process_note
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::payment_reconciliation::PaymentReconciliationExistence
    for PaymentReconciliation
{
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
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_payment_issuer(&self) -> bool {
        self.payment_issuer.is_some()
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_requestor(&self) -> bool {
        self.requestor.is_some()
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_payment_date(&self) -> bool {
        true
    }
    fn has_payment_amount(&self) -> bool {
        true
    }
    fn has_payment_identifier(&self) -> bool {
        self.payment_identifier.is_some()
    }
    fn has_detail(&self) -> bool {
        self.detail.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_form_code(&self) -> bool {
        self.form_code.is_some()
    }
    fn has_process_note(&self) -> bool {
        self.process_note.as_ref().is_some_and(|v| !v.is_empty())
    }
}
