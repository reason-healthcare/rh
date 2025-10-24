use crate::bindings::claim_use::ClaimUse;
use crate::bindings::fm_status::FmStatus;
use crate::bindings::note_type::NoteType;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ClaimResponse
///
/// This resource provides the adjudication details from the processing of a Claim resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClaimResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for a claim response
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// More granular claim type
    ///
    /// Binding: extensible (The type or discipline-style of the claim.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// More granular claim type
    ///
    /// Binding: example (A more granular claim typecode.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-subtype
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,
    /// claim | preauthorization | predetermination
    #[serde(rename = "use")]
    pub use_: ClaimUse,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// The recipient of the products and services
    pub patient: Reference,
    /// Response creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Party responsible for reimbursement
    pub insurer: Reference,
    /// Party responsible for the claim
    pub requestor: Option<Reference>,
    /// Id of resource triggering adjudication
    pub request: Option<Reference>,
    /// queued | complete | error | partial
    pub outcome: RemittanceOutcome,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Disposition Message
    pub disposition: Option<StringType>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<StringType>,
    /// Extension element for the 'preAuthRef' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preAuthRef")]
    pub _pre_auth_ref: Option<Element>,
    /// Preauthorization reference effective period
    #[serde(rename = "preAuthPeriod")]
    pub pre_auth_period: Option<Period>,
    /// Party to be paid any benefits payable
    ///
    /// Binding: example (A code for the party to be reimbursed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payeetype
    #[serde(rename = "payeeType")]
    pub payee_type: Option<CodeableConcept>,
    /// Adjudication for claim line items
    pub item: Option<Vec<ClaimResponseItem>>,
    /// Insurer added line items
    #[serde(rename = "addItem")]
    pub add_item: Option<Vec<ClaimResponseAdditem>>,
    /// Header-level adjudication
    pub adjudication: Option<Vec<StringType>>,
    /// Adjudication totals
    pub total: Option<Vec<ClaimResponseTotal>>,
    /// Payment Details
    pub payment: Option<ClaimResponsePayment>,
    /// Funds reserved status
    ///
    /// Binding: example (For whom funds are to be reserved: (Patient, Provider, None).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/fundsreserve
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,
    /// Printed form identifier
    ///
    /// Binding: example (The forms codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/forms
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,
    /// Printed reference or actual form
    pub form: Option<Attachment>,
    /// Note concerning adjudication
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<ClaimResponseProcessnote>>,
    /// Request for additional information
    #[serde(rename = "communicationRequest")]
    pub communication_request: Option<Vec<Reference>>,
    /// Patient insurance information
    pub insurance: Option<Vec<ClaimResponseInsurance>>,
    /// Processing errors
    pub error: Option<Vec<ClaimResponseError>>,
}
/// ClaimResponseAdditemDetail nested structure for the 'subDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseAdditemDetailSubdetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Billing, service, product, or drug code
    ///
    /// Binding: example (Allowable service and product codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-uscls
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,
    /// Service/Product billing modifiers
    ///
    /// Binding: example (Item type or modifiers codes, eg for Oral whether the treatment is cosmetic or associated with TMJ, or an appliance was lost or stolen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-modifiers
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Count of products or services
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Total item cost
    pub net: Option<Money>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Added items detail adjudication
    pub adjudication: Vec<StringType>,
}
/// ClaimResponse nested structure for the 'error' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseError {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Item sequence number
    #[serde(rename = "itemSequence")]
    pub item_sequence: Option<PositiveIntType>,
    /// Extension element for the 'itemSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_itemSequence")]
    pub _item_sequence: Option<Element>,
    /// Detail sequence number
    #[serde(rename = "detailSequence")]
    pub detail_sequence: Option<PositiveIntType>,
    /// Extension element for the 'detailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_detailSequence")]
    pub _detail_sequence: Option<Element>,
    /// Subdetail sequence number
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: Option<PositiveIntType>,
    /// Extension element for the 'subDetailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subDetailSequence")]
    pub _sub_detail_sequence: Option<Element>,
    /// Error code detailing processing issues
    ///
    /// Binding: example (The adjudication error codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication-error
    pub code: CodeableConcept,
}
/// ClaimResponseItem nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseItemDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Claim detail instance identifier
    #[serde(rename = "detailSequence")]
    pub detail_sequence: PositiveIntType,
    /// Extension element for the 'detailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_detailSequence")]
    pub _detail_sequence: Option<Element>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Detail level adjudication details
    pub adjudication: Vec<StringType>,
}
/// ClaimResponseItem nested structure for the 'adjudication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseItemAdjudication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of adjudication information
    ///
    /// Binding: example (The adjudication codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication
    pub category: CodeableConcept,
    /// Explanation of adjudication outcome
    ///
    /// Binding: example (The adjudication reason codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication-reason
    pub reason: Option<CodeableConcept>,
    /// Monetary amount
    pub amount: Option<Money>,
    /// Non-monetary value
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// ClaimResponseAdditem nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseAdditemDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Billing, service, product, or drug code
    ///
    /// Binding: example (Allowable service and product codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-uscls
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,
    /// Service/Product billing modifiers
    ///
    /// Binding: example (Item type or modifiers codes, eg for Oral whether the treatment is cosmetic or associated with TMJ, or an appliance was lost or stolen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-modifiers
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Count of products or services
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Total item cost
    pub net: Option<Money>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Added items detail adjudication
    pub adjudication: Vec<StringType>,
}
/// ClaimResponse nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Adjudication details
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
    /// Adjudication for claim details
    pub detail: Option<Vec<ClaimResponseItemDetail>>,
    /// Claim item instance identifier
    #[serde(rename = "itemSequence")]
    pub item_sequence: PositiveIntType,
    /// Extension element for the 'itemSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_itemSequence")]
    pub _item_sequence: Option<Element>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
}
/// ClaimResponseItemDetail nested structure for the 'subDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseItemDetailSubdetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Claim sub-detail instance identifier
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: PositiveIntType,
    /// Extension element for the 'subDetailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subDetailSequence")]
    pub _sub_detail_sequence: Option<Element>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<StringType>>,
}
/// ClaimResponse nested structure for the 'insurance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseInsurance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Insurance instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Coverage to be used for adjudication
    pub focal: BooleanType,
    /// Extension element for the 'focal' primitive field. Contains metadata and extensions.
    pub _focal: Option<Element>,
    /// Insurance information
    pub coverage: Reference,
    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<StringType>,
    /// Extension element for the 'businessArrangement' primitive field. Contains metadata and extensions.
    #[serde(rename = "_businessArrangement")]
    pub _business_arrangement: Option<Element>,
    /// Adjudication results
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Reference>,
}
/// ClaimResponse nested structure for the 'total' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseTotal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of adjudication information
    ///
    /// Binding: example (The adjudication codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication
    pub category: CodeableConcept,
    /// Financial total for the category
    pub amount: Money,
}
/// ClaimResponse nested structure for the 'payment' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponsePayment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Partial or complete payment
    ///
    /// Binding: example (The type (partial, complete) of the payment.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-paymenttype
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Payment adjustment for non-claim issues
    pub adjustment: Option<Money>,
    /// Explanation for the adjustment
    ///
    /// Binding: example (Payment Adjustment reason codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-adjustment-reason
    #[serde(rename = "adjustmentReason")]
    pub adjustment_reason: Option<CodeableConcept>,
    /// Expected date of payment
    pub date: Option<DateType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Payable amount after adjustment
    pub amount: Money,
    /// Business identifier for the payment
    pub identifier: Option<Identifier>,
}
/// ClaimResponse nested structure for the 'processNote' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseProcessnote {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Note instance identifier
    pub number: Option<PositiveIntType>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// display | print | printoper
    #[serde(rename = "type")]
    pub type_: Option<NoteType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Note explanatory text
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Language of the text
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `da`: Danish
    /// - `de`: German
    /// - `de-AT`: German (Austria)
    /// - `de-CH`: German (Switzerland)
    /// - `de-DE`: German (Germany)
    /// - `el`: Greek
    /// - `en`: English
    /// - ... and 46 more values
    pub language: Option<CodeableConcept>,
}
/// ClaimResponse nested structure for the 'addItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponseAdditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Insurer added line details
    pub detail: Option<Vec<ClaimResponseAdditemDetail>>,
    /// Item sequence number
    #[serde(rename = "itemSequence")]
    pub item_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'itemSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_itemSequence")]
    pub _item_sequence: Option<Element>,
    /// Detail sequence number
    #[serde(rename = "detailSequence")]
    pub detail_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'detailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_detailSequence")]
    pub _detail_sequence: Option<Element>,
    /// Subdetail sequence number
    #[serde(rename = "subdetailSequence")]
    pub subdetail_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'subdetailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subdetailSequence")]
    pub _subdetail_sequence: Option<Element>,
    /// Authorized providers
    pub provider: Option<Vec<Reference>>,
    /// Billing, service, product, or drug code
    ///
    /// Binding: example (Allowable service and product codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-uscls
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,
    /// Service/Product billing modifiers
    ///
    /// Binding: example (Item type or modifiers codes, eg for Oral whether the treatment is cosmetic or associated with TMJ, or an appliance was lost or stolen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-modifiers
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    ///
    /// Binding: example (Program specific reason codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-program-code
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Date or dates of service or product delivery (date)
    #[serde(rename = "servicedDate")]
    pub serviced_date: Option<DateType>,
    /// Date or dates of service or product delivery (Period)
    #[serde(rename = "servicedPeriod")]
    pub serviced_period: Option<Period>,
    /// Place of service or where product was supplied (CodeableConcept)
    #[serde(rename = "locationCodeableConcept")]
    pub location_codeable_concept: Option<CodeableConcept>,
    /// Place of service or where product was supplied (Address)
    #[serde(rename = "locationAddress")]
    pub location_address: Option<Address>,
    /// Place of service or where product was supplied (Reference)
    #[serde(rename = "locationReference")]
    pub location_reference: Option<Reference>,
    /// Count of products or services
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Total item cost
    pub net: Option<Money>,
    /// Anatomical location
    ///
    /// Binding: example (The code for the teeth, quadrant, sextant and arch.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/tooth
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Anatomical sub-location
    ///
    /// Binding: example (The code for the tooth surface and surface combinations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/surface
    #[serde(rename = "subSite")]
    pub sub_site: Option<Vec<CodeableConcept>>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Added items adjudication
    pub adjudication: Vec<StringType>,
}

impl Default for ClaimResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            sub_type: Default::default(),
            use_: Default::default(),
            _use: Default::default(),
            patient: Reference::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            insurer: Reference::default(),
            requestor: Default::default(),
            request: Default::default(),
            outcome: RemittanceOutcome::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            pre_auth_ref: Default::default(),
            _pre_auth_ref: Default::default(),
            pre_auth_period: Default::default(),
            payee_type: Default::default(),
            item: Default::default(),
            add_item: Default::default(),
            adjudication: Default::default(),
            total: Default::default(),
            payment: Default::default(),
            funds_reserve: Default::default(),
            form_code: Default::default(),
            form: Default::default(),
            process_note: Default::default(),
            communication_request: Default::default(),
            insurance: Default::default(),
            error: Default::default(),
        }
    }
}

impl Default for ClaimResponseAdditemDetailSubdetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            net: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ClaimResponseError {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_sequence: Default::default(),
            _item_sequence: Default::default(),
            detail_sequence: Default::default(),
            _detail_sequence: Default::default(),
            sub_detail_sequence: Default::default(),
            _sub_detail_sequence: Default::default(),
            code: CodeableConcept::default(),
        }
    }
}

impl Default for ClaimResponseItemDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail_sequence: Default::default(),
            _detail_sequence: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ClaimResponseItemAdjudication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            reason: Default::default(),
            amount: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for ClaimResponseAdditemDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            net: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ClaimResponseItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            adjudication: Vec::new(),
            detail: Default::default(),
            item_sequence: PositiveIntType::default(),
            _item_sequence: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
        }
    }
}

impl Default for ClaimResponseItemDetailSubdetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sub_detail_sequence: Default::default(),
            _sub_detail_sequence: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ClaimResponseInsurance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: PositiveIntType::default(),
            _sequence: Default::default(),
            focal: BooleanType::default(),
            _focal: Default::default(),
            coverage: Reference::default(),
            business_arrangement: Default::default(),
            _business_arrangement: Default::default(),
            claim_response: Default::default(),
        }
    }
}

impl Default for ClaimResponseTotal {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: CodeableConcept::default(),
            amount: Money::default(),
        }
    }
}

impl Default for ClaimResponsePayment {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            adjustment: Default::default(),
            adjustment_reason: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            amount: Money::default(),
            identifier: Default::default(),
        }
    }
}

impl Default for ClaimResponseProcessnote {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            number: Default::default(),
            _number: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            language: Default::default(),
        }
    }
}

impl Default for ClaimResponseAdditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            item_sequence: Default::default(),
            _item_sequence: Default::default(),
            detail_sequence: Default::default(),
            _detail_sequence: Default::default(),
            subdetail_sequence: Default::default(),
            _subdetail_sequence: Default::default(),
            provider: Default::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            program_code: Default::default(),
            serviced_date: Default::default(),
            serviced_period: Default::default(),
            location_codeable_concept: Default::default(),
            location_address: Default::default(),
            location_reference: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            net: Default::default(),
            body_site: Default::default(),
            sub_site: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ClaimResponse {
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

impl crate::traits::resource::ResourceMutators for ClaimResponse {
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

impl crate::traits::resource::ResourceExistence for ClaimResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ClaimResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for ClaimResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for ClaimResponse {
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

impl crate::traits::claim_response::ClaimResponseAccessors for ClaimResponse {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn sub_type(&self) -> Option<CodeableConcept> {
        self.sub_type.clone()
    }
    fn use_(&self) -> ClaimUse {
        self.use_.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn insurer(&self) -> Reference {
        self.insurer.clone()
    }
    fn requestor(&self) -> Option<Reference> {
        self.requestor.clone()
    }
    fn request(&self) -> Option<Reference> {
        self.request.clone()
    }
    fn outcome(&self) -> RemittanceOutcome {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn pre_auth_ref(&self) -> Option<StringType> {
        self.pre_auth_ref.clone()
    }
    fn pre_auth_period(&self) -> Option<Period> {
        self.pre_auth_period.clone()
    }
    fn payee_type(&self) -> Option<CodeableConcept> {
        self.payee_type.clone()
    }
    fn item(&self) -> &[ClaimResponseItem] {
        self.item.as_deref().unwrap_or(&[])
    }
    fn add_item(&self) -> &[ClaimResponseAdditem] {
        self.add_item.as_deref().unwrap_or(&[])
    }
    fn total(&self) -> &[ClaimResponseTotal] {
        self.total.as_deref().unwrap_or(&[])
    }
    fn payment(&self) -> Option<ClaimResponsePayment> {
        self.payment.clone()
    }
    fn funds_reserve(&self) -> Option<CodeableConcept> {
        self.funds_reserve.clone()
    }
    fn form_code(&self) -> Option<CodeableConcept> {
        self.form_code.clone()
    }
    fn form(&self) -> Option<Attachment> {
        self.form.clone()
    }
    fn process_note(&self) -> &[ClaimResponseProcessnote] {
        self.process_note.as_deref().unwrap_or(&[])
    }
    fn communication_request(&self) -> &[Reference] {
        self.communication_request.as_deref().unwrap_or(&[])
    }
    fn insurance(&self) -> &[ClaimResponseInsurance] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn error(&self) -> &[ClaimResponseError] {
        self.error.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::claim_response::ClaimResponseMutators for ClaimResponse {
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_sub_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.sub_type = Some(value);
        resource
    }
    fn set_use_(self, value: ClaimUse) -> Self {
        let mut resource = self.clone();
        resource.use_ = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_insurer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurer = value;
        resource
    }
    fn set_requestor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requestor = Some(value);
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn set_outcome(self, value: RemittanceOutcome) -> Self {
        let mut resource = self.clone();
        resource.outcome = value;
        resource
    }
    fn set_disposition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.disposition = Some(value);
        resource
    }
    fn set_pre_auth_ref(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.pre_auth_ref = Some(value);
        resource
    }
    fn set_pre_auth_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.pre_auth_period = Some(value);
        resource
    }
    fn set_payee_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.payee_type = Some(value);
        resource
    }
    fn set_item(self, value: Vec<ClaimResponseItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: ClaimResponseItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_add_item(self, value: Vec<ClaimResponseAdditem>) -> Self {
        let mut resource = self.clone();
        resource.add_item = Some(value);
        resource
    }
    fn add_add_item(self, item: ClaimResponseAdditem) -> Self {
        let mut resource = self.clone();
        resource.add_item.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_adjudication(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.adjudication = Some(value);
        resource
    }
    fn add_adjudication(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .adjudication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_total(self, value: Vec<ClaimResponseTotal>) -> Self {
        let mut resource = self.clone();
        resource.total = Some(value);
        resource
    }
    fn add_total(self, item: ClaimResponseTotal) -> Self {
        let mut resource = self.clone();
        resource.total.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_payment(self, value: ClaimResponsePayment) -> Self {
        let mut resource = self.clone();
        resource.payment = Some(value);
        resource
    }
    fn set_funds_reserve(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funds_reserve = Some(value);
        resource
    }
    fn set_form_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form_code = Some(value);
        resource
    }
    fn set_form(self, value: Attachment) -> Self {
        let mut resource = self.clone();
        resource.form = Some(value);
        resource
    }
    fn set_process_note(self, value: Vec<ClaimResponseProcessnote>) -> Self {
        let mut resource = self.clone();
        resource.process_note = Some(value);
        resource
    }
    fn add_process_note(self, item: ClaimResponseProcessnote) -> Self {
        let mut resource = self.clone();
        resource
            .process_note
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_communication_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.communication_request = Some(value);
        resource
    }
    fn add_communication_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .communication_request
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_insurance(self, value: Vec<ClaimResponseInsurance>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: ClaimResponseInsurance) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_error(self, value: Vec<ClaimResponseError>) -> Self {
        let mut resource = self.clone();
        resource.error = Some(value);
        resource
    }
    fn add_error(self, item: ClaimResponseError) -> Self {
        let mut resource = self.clone();
        resource.error.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::claim_response::ClaimResponseExistence for ClaimResponse {
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
    fn has_type_(&self) -> bool {
        true
    }
    fn has_sub_type(&self) -> bool {
        self.sub_type.is_some()
    }
    fn has_use_(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_insurer(&self) -> bool {
        true
    }
    fn has_requestor(&self) -> bool {
        self.requestor.is_some()
    }
    fn has_request(&self) -> bool {
        self.request.is_some()
    }
    fn has_outcome(&self) -> bool {
        true
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_pre_auth_ref(&self) -> bool {
        self.pre_auth_ref.is_some()
    }
    fn has_pre_auth_period(&self) -> bool {
        self.pre_auth_period.is_some()
    }
    fn has_payee_type(&self) -> bool {
        self.payee_type.is_some()
    }
    fn has_item(&self) -> bool {
        self.item.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_add_item(&self) -> bool {
        self.add_item.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_adjudication(&self) -> bool {
        self.adjudication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_total(&self) -> bool {
        self.total.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_payment(&self) -> bool {
        self.payment.is_some()
    }
    fn has_funds_reserve(&self) -> bool {
        self.funds_reserve.is_some()
    }
    fn has_form_code(&self) -> bool {
        self.form_code.is_some()
    }
    fn has_form(&self) -> bool {
        self.form.is_some()
    }
    fn has_process_note(&self) -> bool {
        self.process_note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_communication_request(&self) -> bool {
        self.communication_request
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_error(&self) -> bool {
        self.error.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ClaimResponse {
    fn resource_type(&self) -> &'static str {
        "ClaimResponse"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ClaimResponse")
    }
}
