use crate::bindings::invoice_price_component_type::InvoicePriceComponentType;
use crate::bindings::invoice_status::InvoiceStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Invoice
///
/// Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Invoice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Invoice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for item
    pub identifier: Option<Vec<Identifier>>,
    /// draft | issued | balanced | cancelled | entered-in-error
    pub status: InvoiceStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for cancellation of this Invoice
    #[serde(rename = "cancelledReason")]
    pub cancelled_reason: Option<StringType>,
    /// Extension element for the 'cancelledReason' primitive field. Contains metadata and extensions.
    #[serde(rename = "_cancelledReason")]
    pub _cancelled_reason: Option<Element>,
    /// Type of Invoice
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Recipient(s) of goods and services
    pub subject: Option<Reference>,
    /// Recipient of this invoice
    pub recipient: Option<Reference>,
    /// Invoice date / posting date
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Participant in creation of this Invoice
    pub participant: Option<Vec<InvoiceParticipant>>,
    /// Issuing Organization of Invoice
    pub issuer: Option<Reference>,
    /// Account that is being balanced
    pub account: Option<Reference>,
    /// Line items of this Invoice
    #[serde(rename = "lineItem")]
    pub line_item: Option<Vec<InvoiceLineitem>>,
    /// Components of Invoice total
    #[serde(rename = "totalPriceComponent")]
    pub total_price_component: Option<Vec<StringType>>,
    /// Net total of this Invoice
    #[serde(rename = "totalNet")]
    pub total_net: Option<Money>,
    /// Gross total of this Invoice
    #[serde(rename = "totalGross")]
    pub total_gross: Option<Money>,
    /// Payment details
    #[serde(rename = "paymentTerms")]
    pub payment_terms: Option<StringType>,
    /// Extension element for the 'paymentTerms' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paymentTerms")]
    pub _payment_terms: Option<Element>,
    /// Comments made about the invoice
    pub note: Option<Vec<Annotation>>,
}
/// Invoice nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of involvement in creation of this Invoice
    pub role: Option<CodeableConcept>,
    /// Individual who was involved
    pub actor: Reference,
}
/// InvoiceLineitem nested structure for the 'priceComponent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineitemPricecomponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// base | surcharge | deduction | discount | tax | informational
    #[serde(rename = "type")]
    pub type_: InvoicePriceComponentType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Code identifying the specific component
    pub code: Option<CodeableConcept>,
    /// Factor used for calculating this component
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Monetary amount associated with this component
    pub amount: Option<Money>,
}
/// Invoice nested structure for the 'lineItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineitem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Components of total line item price
    #[serde(rename = "priceComponent")]
    pub price_component: Option<Vec<InvoiceLineitemPricecomponent>>,
    /// Sequence number of line item
    pub sequence: Option<PositiveIntType>,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Reference to ChargeItem containing details of this line item or an inline billing code (Reference)
    #[serde(rename = "chargeItemReference")]
    pub charge_item_reference: Reference,
    /// Reference to ChargeItem containing details of this line item or an inline billing code (CodeableConcept)
    #[serde(rename = "chargeItemCodeableConcept")]
    pub charge_item_codeable_concept: CodeableConcept,
}

impl Default for Invoice {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: InvoiceStatus::default(),
            _status: Default::default(),
            cancelled_reason: Default::default(),
            _cancelled_reason: Default::default(),
            type_: Default::default(),
            subject: Default::default(),
            recipient: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            participant: Default::default(),
            issuer: Default::default(),
            account: Default::default(),
            line_item: Default::default(),
            total_price_component: Default::default(),
            total_net: Default::default(),
            total_gross: Default::default(),
            payment_terms: Default::default(),
            _payment_terms: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for InvoiceParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for InvoiceLineitemPricecomponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            code: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for InvoiceLineitem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            price_component: Default::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            charge_item_reference: Default::default(),
            charge_item_codeable_concept: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Invoice {
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

impl crate::traits::resource::ResourceMutators for Invoice {
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

impl crate::traits::resource::ResourceExistence for Invoice {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Invoice {
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

impl crate::traits::domain_resource::DomainResourceMutators for Invoice {
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

impl crate::traits::domain_resource::DomainResourceExistence for Invoice {
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

impl crate::traits::invoice::InvoiceAccessors for Invoice {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> InvoiceStatus {
        self.status.clone()
    }
    fn cancelled_reason(&self) -> Option<StringType> {
        self.cancelled_reason.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn recipient(&self) -> Option<Reference> {
        self.recipient.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn participant(&self) -> &[InvoiceParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn issuer(&self) -> Option<Reference> {
        self.issuer.clone()
    }
    fn account(&self) -> Option<Reference> {
        self.account.clone()
    }
    fn line_item(&self) -> &[InvoiceLineitem] {
        self.line_item.as_deref().unwrap_or(&[])
    }
    fn total_net(&self) -> Option<Money> {
        self.total_net.clone()
    }
    fn total_gross(&self) -> Option<Money> {
        self.total_gross.clone()
    }
    fn payment_terms(&self) -> Option<StringType> {
        self.payment_terms.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::invoice::InvoiceMutators for Invoice {
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
    fn set_status(self, value: InvoiceStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_cancelled_reason(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.cancelled_reason = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_recipient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recipient = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<InvoiceParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: InvoiceParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_issuer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.issuer = Some(value);
        resource
    }
    fn set_account(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.account = Some(value);
        resource
    }
    fn set_line_item(self, value: Vec<InvoiceLineitem>) -> Self {
        let mut resource = self.clone();
        resource.line_item = Some(value);
        resource
    }
    fn add_line_item(self, item: InvoiceLineitem) -> Self {
        let mut resource = self.clone();
        resource.line_item.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_total_price_component(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.total_price_component = Some(value);
        resource
    }
    fn add_total_price_component(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .total_price_component
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_total_net(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.total_net = Some(value);
        resource
    }
    fn set_total_gross(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.total_gross = Some(value);
        resource
    }
    fn set_payment_terms(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.payment_terms = Some(value);
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

impl crate::traits::invoice::InvoiceExistence for Invoice {
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
    fn has_cancelled_reason(&self) -> bool {
        self.cancelled_reason.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_recipient(&self) -> bool {
        self.recipient.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_issuer(&self) -> bool {
        self.issuer.is_some()
    }
    fn has_account(&self) -> bool {
        self.account.is_some()
    }
    fn has_line_item(&self) -> bool {
        self.line_item.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_total_price_component(&self) -> bool {
        self.total_price_component
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_total_net(&self) -> bool {
        self.total_net.is_some()
    }
    fn has_total_gross(&self) -> bool {
        self.total_gross.is_some()
    }
    fn has_payment_terms(&self) -> bool {
        self.payment_terms.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}
