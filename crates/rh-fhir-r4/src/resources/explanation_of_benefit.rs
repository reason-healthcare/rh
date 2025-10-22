use crate::bindings::claim_use::ClaimUse;
use crate::bindings::explanationofbenefit_status::ExplanationofbenefitStatus;
use crate::bindings::note_type::NoteType;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ExplanationOfBenefit
///
/// This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ExplanationOfBenefit
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for the resource
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: ExplanationofbenefitStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Category or discipline
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
    /// Relevant time frame for the claim
    #[serde(rename = "billablePeriod")]
    pub billable_period: Option<Period>,
    /// Response creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Author of the claim
    pub enterer: Option<Reference>,
    /// Party responsible for reimbursement
    pub insurer: Reference,
    /// Party responsible for the claim
    pub provider: Reference,
    /// Desired processing urgency
    ///
    /// Binding: example (The timeliness with which processing is required: stat, normal, deferred.)
    ///
    /// ValueSet: http://terminology.hl7.org/CodeSystem/processpriority
    pub priority: Option<CodeableConcept>,
    /// For whom to reserve funds
    ///
    /// Binding: example (For whom funds are to be reserved: (Patient, Provider, None).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/fundsreserve
    #[serde(rename = "fundsReserveRequested")]
    pub funds_reserve_requested: Option<CodeableConcept>,
    /// Funds reserved status
    ///
    /// Binding: example (For whom funds are to be reserved: (Patient, Provider, None).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/fundsreserve
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,
    /// Prior or corollary claims
    pub related: Option<Vec<ExplanationOfBenefitRelated>>,
    /// Prescription authorizing services or products
    pub prescription: Option<Reference>,
    /// Original prescription if superceded by fulfiller
    #[serde(rename = "originalPrescription")]
    pub original_prescription: Option<Reference>,
    /// Recipient of benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,
    /// Treatment Referral
    pub referral: Option<Reference>,
    /// Servicing Facility
    pub facility: Option<Reference>,
    /// Claim reference
    pub claim: Option<Reference>,
    /// Claim response reference
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Reference>,
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
    pub pre_auth_ref: Option<Vec<StringType>>,
    /// Extension element for the 'preAuthRef' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preAuthRef")]
    pub _pre_auth_ref: Option<Element>,
    /// Preauthorization in-effect period
    #[serde(rename = "preAuthRefPeriod")]
    pub pre_auth_ref_period: Option<Vec<Period>>,
    /// Care Team members
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<ExplanationOfBenefitCareteam>>,
    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<ExplanationOfBenefitSupportinginfo>>,
    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ExplanationOfBenefitDiagnosis>>,
    /// Clinical procedures performed
    pub procedure: Option<Vec<ExplanationOfBenefitProcedure>>,
    /// Precedence (primary, secondary, etc.)
    pub precedence: Option<PositiveIntType>,
    /// Extension element for the 'precedence' primitive field. Contains metadata and extensions.
    pub _precedence: Option<Element>,
    /// Patient insurance information
    pub insurance: Vec<ExplanationOfBenefitInsurance>,
    /// Details of the event
    pub accident: Option<ExplanationOfBenefitAccident>,
    /// Product or service provided
    pub item: Option<Vec<ExplanationOfBenefitItem>>,
    /// Insurer added line items
    #[serde(rename = "addItem")]
    pub add_item: Option<Vec<ExplanationOfBenefitAdditem>>,
    /// Header-level adjudication
    pub adjudication: Option<Vec<StringType>>,
    /// Adjudication totals
    pub total: Option<Vec<ExplanationOfBenefitTotal>>,
    /// Payment Details
    pub payment: Option<ExplanationOfBenefitPayment>,
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
    pub process_note: Option<Vec<ExplanationOfBenefitProcessnote>>,
    /// When the benefits are applicable
    #[serde(rename = "benefitPeriod")]
    pub benefit_period: Option<Period>,
    /// Balance by Benefit Category
    #[serde(rename = "benefitBalance")]
    pub benefit_balance: Option<Vec<ExplanationOfBenefitBenefitbalance>>,
}
/// ExplanationOfBenefit nested structure for the 'benefitBalance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitBenefitbalance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Benefit Summary
    pub financial: Option<Vec<ExplanationOfBenefitBenefitbalanceFinancial>>,
    /// Benefit classification
    ///
    /// Binding: example (Benefit categories such as: oral, medical, vision, oral-basic etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: CodeableConcept,
    /// Excluded from the plan
    pub excluded: Option<BooleanType>,
    /// Extension element for the 'excluded' primitive field. Contains metadata and extensions.
    pub _excluded: Option<Element>,
    /// Short name for the benefit
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Description of the benefit or services covered
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// In or out of network
    ///
    /// Binding: example (Code to classify in or out of network services.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-network
    pub network: Option<CodeableConcept>,
    /// Individual or family
    ///
    /// Binding: example (Unit covered/serviced - individual or family.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-unit
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    ///
    /// Binding: example (Coverage unit - annual, lifetime.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-term
    pub term: Option<CodeableConcept>,
}
/// ExplanationOfBenefit nested structure for the 'processNote' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitProcessnote {
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
    pub text: Option<StringType>,
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
/// ExplanationOfBenefit nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Diagnosis instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Nature of illness or problem (CodeableConcept)
    #[serde(rename = "diagnosisCodeableConcept")]
    pub diagnosis_codeable_concept: CodeableConcept,
    /// Nature of illness or problem (Reference)
    #[serde(rename = "diagnosisReference")]
    pub diagnosis_reference: Reference,
    /// Timing or nature of the diagnosis
    ///
    /// Binding: example (The type of the diagnosis: admitting, principal, discharge.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-diagnosistype
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Present on admission
    ///
    /// Binding: example (Present on admission.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-diagnosis-on-admission
    #[serde(rename = "onAdmission")]
    pub on_admission: Option<CodeableConcept>,
    /// Package billing code
    ///
    /// Binding: example (The DRG codes associated with the diagnosis.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-diagnosisrelatedgroup
    #[serde(rename = "packageCode")]
    pub package_code: Option<CodeableConcept>,
}
/// ExplanationOfBenefit nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Additional items
    pub detail: Option<Vec<ExplanationOfBenefitItemDetail>>,
    /// Adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
    /// Item instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Applicable care team members
    #[serde(rename = "careTeamSequence")]
    pub care_team_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'careTeamSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_careTeamSequence")]
    pub _care_team_sequence: Option<Element>,
    /// Applicable diagnoses
    #[serde(rename = "diagnosisSequence")]
    pub diagnosis_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'diagnosisSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_diagnosisSequence")]
    pub _diagnosis_sequence: Option<Element>,
    /// Applicable procedures
    #[serde(rename = "procedureSequence")]
    pub procedure_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'procedureSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_procedureSequence")]
    pub _procedure_sequence: Option<Element>,
    /// Applicable exception and supporting information
    #[serde(rename = "informationSequence")]
    pub information_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'informationSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_informationSequence")]
    pub _information_sequence: Option<Element>,
    /// Revenue or cost center code
    ///
    /// Binding: example (Codes for the revenue or cost centers supplying the service and/or products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-revenue-center
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    ///
    /// Binding: example (Benefit categories such as: oral, medical, vision, oral-basic etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    ///
    /// Binding: example (Allowable service and product codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-uscls
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,
    /// Product or service billing modifiers
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
    pub serviced_date: Option<StringType>,
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
    /// Unique device identifier
    pub udi: Option<Vec<Reference>>,
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
    /// Encounters related to this billed item
    pub encounter: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
}
/// ExplanationOfBenefit nested structure for the 'accident' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitAccident {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// When the incident occurred
    pub date: Option<StringType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The nature of the accident
    ///
    /// Binding: extensible (Type of accident: work place, auto, etc.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActIncidentCode
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Where the event occurred (Address)
    #[serde(rename = "locationAddress")]
    pub location_address: Option<Address>,
    /// Where the event occurred (Reference)
    #[serde(rename = "locationReference")]
    pub location_reference: Option<Reference>,
}
/// ExplanationOfBenefit nested structure for the 'addItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitAdditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Insurer added line items
    pub detail: Option<Vec<ExplanationOfBenefitAdditemDetail>>,
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
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'subDetailSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subDetailSequence")]
    pub _sub_detail_sequence: Option<Element>,
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
    pub serviced_date: Option<StringType>,
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
    pub adjudication: Option<Vec<StringType>>,
}
/// ExplanationOfBenefitItemDetail nested structure for the 'subDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitItemDetailSubdetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Product or service provided
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Revenue or cost center code
    ///
    /// Binding: example (Codes for the revenue or cost centers supplying the service and/or products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-revenue-center
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    ///
    /// Binding: example (Benefit categories such as: oral, medical, vision, oral-basic etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: Option<CodeableConcept>,
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
    /// Unique device identifier
    pub udi: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<StringType>>,
}
/// ExplanationOfBenefit nested structure for the 'total' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitTotal {
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
/// ExplanationOfBenefitAdditemDetail nested structure for the 'subDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitAdditemDetailSubdetail {
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
    /// Added items adjudication
    pub adjudication: Option<Vec<StringType>>,
}
/// ExplanationOfBenefit nested structure for the 'supportingInfo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitSupportinginfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Classification of the supplied information
    ///
    /// Binding: example (The valuset used for additional information category codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-informationcategory
    pub category: CodeableConcept,
    /// Type of information
    ///
    /// Binding: example (The valuset used for additional information codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-exception
    pub code: Option<CodeableConcept>,
    /// When it occurred (date)
    #[serde(rename = "timingDate")]
    pub timing_date: Option<StringType>,
    /// When it occurred (Period)
    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,
    /// Data to be provided (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Data to be provided (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Data to be provided (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Data to be provided (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// Data to be provided (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// Explanation for the information
    ///
    /// Binding: example (Reason codes for the missing teeth.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/missing-tooth-reason
    pub reason: Option<Coding>,
}
/// ExplanationOfBenefit nested structure for the 'careTeam' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitCareteam {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Order of care team
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Practitioner or organization
    pub provider: Reference,
    /// Indicator of the lead practitioner
    pub responsible: Option<BooleanType>,
    /// Extension element for the 'responsible' primitive field. Contains metadata and extensions.
    pub _responsible: Option<Element>,
    /// Function within the team
    ///
    /// Binding: example (The role codes for the care team members.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-careteamrole
    pub role: Option<CodeableConcept>,
    /// Practitioner credential or specialization
    ///
    /// Binding: example (Provider professional qualifications.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/provider-qualification
    pub qualification: Option<CodeableConcept>,
}
/// ExplanationOfBenefit nested structure for the 'payee' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitPayee {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category of recipient
    ///
    /// Binding: example (A code for the party to be reimbursed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payeetype
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Recipient reference
    pub party: Option<Reference>,
}
/// ExplanationOfBenefitAdditem nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitAdditemDetail {
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
    /// Added items adjudication
    pub adjudication: Option<Vec<StringType>>,
}
/// ExplanationOfBenefitItem nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitItemDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Product or service provided
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Revenue or cost center code
    ///
    /// Binding: example (Codes for the revenue or cost centers supplying the service and/or products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-revenue-center
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    ///
    /// Binding: example (Benefit categories such as: oral, medical, vision, oral-basic etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: Option<CodeableConcept>,
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
    /// Unique device identifier
    pub udi: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'noteNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noteNumber")]
    pub _note_number: Option<Element>,
    /// Detail level adjudication details
    pub adjudication: Option<Vec<StringType>>,
}
/// ExplanationOfBenefitBenefitbalance nested structure for the 'financial' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitBenefitbalanceFinancial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Benefit classification
    ///
    /// Binding: example (Deductable, visits, co-pay, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Benefits allowed (unsignedInt)
    #[serde(rename = "allowedUnsignedInt")]
    pub allowed_unsigned_int: Option<UnsignedIntType>,
    /// Benefits allowed (string)
    #[serde(rename = "allowedString")]
    pub allowed_string: Option<StringType>,
    /// Benefits allowed (Money)
    #[serde(rename = "allowedMoney")]
    pub allowed_money: Option<Money>,
    /// Benefits used (unsignedInt)
    #[serde(rename = "usedUnsignedInt")]
    pub used_unsigned_int: Option<UnsignedIntType>,
    /// Benefits used (Money)
    #[serde(rename = "usedMoney")]
    pub used_money: Option<Money>,
}
/// ExplanationOfBenefit nested structure for the 'insurance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitInsurance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coverage to be used for adjudication
    pub focal: BooleanType,
    /// Extension element for the 'focal' primitive field. Contains metadata and extensions.
    pub _focal: Option<Element>,
    /// Insurance information
    pub coverage: Reference,
    /// Prior authorization reference number
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<Vec<StringType>>,
    /// Extension element for the 'preAuthRef' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preAuthRef")]
    pub _pre_auth_ref: Option<Element>,
}
/// ExplanationOfBenefitItem nested structure for the 'adjudication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitItemAdjudication {
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
    /// Binding: example (Adjudication reason codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication-reason
    pub reason: Option<CodeableConcept>,
    /// Monetary amount
    pub amount: Option<Money>,
    /// Non-monitary value
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// ExplanationOfBenefit nested structure for the 'payment' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitPayment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Partial or complete payment
    ///
    /// Binding: example (The type (partial, complete) of the payment.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-paymenttype
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Payment adjustment for non-claim issues
    pub adjustment: Option<Money>,
    /// Explanation for the variance
    ///
    /// Binding: example (Payment Adjustment reason codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payment-adjustment-reason
    #[serde(rename = "adjustmentReason")]
    pub adjustment_reason: Option<CodeableConcept>,
    /// Expected date of payment
    pub date: Option<StringType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Payable amount after adjustment
    pub amount: Option<Money>,
    /// Business identifier for the payment
    pub identifier: Option<Identifier>,
}
/// ExplanationOfBenefit nested structure for the 'related' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitRelated {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to the related claim
    pub claim: Option<Reference>,
    /// How the reference claim is related
    ///
    /// Binding: example (Relationship of this claim to a related Claim.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/related-claim-relationship
    pub relationship: Option<CodeableConcept>,
    /// File or case reference
    pub reference: Option<Identifier>,
}
/// ExplanationOfBenefit nested structure for the 'procedure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationOfBenefitProcedure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Procedure instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Category of Procedure
    ///
    /// Binding: example (Example procedure type codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-procedure-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// When the procedure was performed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Specific clinical procedure (CodeableConcept)
    #[serde(rename = "procedureCodeableConcept")]
    pub procedure_codeable_concept: CodeableConcept,
    /// Specific clinical procedure (Reference)
    #[serde(rename = "procedureReference")]
    pub procedure_reference: Reference,
    /// Unique device identifier
    pub udi: Option<Vec<Reference>>,
}

impl Default for ExplanationOfBenefit {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: ExplanationofbenefitStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            sub_type: Default::default(),
            use_: Default::default(),
            _use: Default::default(),
            patient: Reference::default(),
            billable_period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            enterer: Default::default(),
            insurer: Reference::default(),
            provider: Reference::default(),
            priority: Default::default(),
            funds_reserve_requested: Default::default(),
            funds_reserve: Default::default(),
            related: Default::default(),
            prescription: Default::default(),
            original_prescription: Default::default(),
            payee: Default::default(),
            referral: Default::default(),
            facility: Default::default(),
            claim: Default::default(),
            claim_response: Default::default(),
            outcome: RemittanceOutcome::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            pre_auth_ref: Default::default(),
            _pre_auth_ref: Default::default(),
            pre_auth_ref_period: Default::default(),
            care_team: Default::default(),
            supporting_info: Default::default(),
            diagnosis: Default::default(),
            procedure: Default::default(),
            precedence: Default::default(),
            _precedence: Default::default(),
            insurance: Vec::new(),
            accident: Default::default(),
            item: Default::default(),
            add_item: Default::default(),
            adjudication: Default::default(),
            total: Default::default(),
            payment: Default::default(),
            form_code: Default::default(),
            form: Default::default(),
            process_note: Default::default(),
            benefit_period: Default::default(),
            benefit_balance: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitBenefitbalance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            financial: Default::default(),
            category: Default::default(),
            excluded: Default::default(),
            _excluded: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            network: Default::default(),
            unit: Default::default(),
            term: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitProcessnote {
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

impl Default for ExplanationOfBenefitDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: PositiveIntType::default(),
            _sequence: Default::default(),
            diagnosis_codeable_concept: Default::default(),
            diagnosis_reference: Default::default(),
            type_: Default::default(),
            on_admission: Default::default(),
            package_code: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            adjudication: Default::default(),
            sequence: PositiveIntType::default(),
            _sequence: Default::default(),
            care_team_sequence: Default::default(),
            _care_team_sequence: Default::default(),
            diagnosis_sequence: Default::default(),
            _diagnosis_sequence: Default::default(),
            procedure_sequence: Default::default(),
            _procedure_sequence: Default::default(),
            information_sequence: Default::default(),
            _information_sequence: Default::default(),
            revenue: Default::default(),
            category: Default::default(),
            product_or_service: CodeableConcept::default(),
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
            udi: Default::default(),
            body_site: Default::default(),
            sub_site: Default::default(),
            encounter: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitAccident {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            date: Default::default(),
            _date: Default::default(),
            type_: Default::default(),
            location_address: Default::default(),
            location_reference: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitAdditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            item_sequence: Default::default(),
            _item_sequence: Default::default(),
            detail_sequence: Default::default(),
            _detail_sequence: Default::default(),
            sub_detail_sequence: Default::default(),
            _sub_detail_sequence: Default::default(),
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

impl Default for ExplanationOfBenefitItemDetailSubdetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            revenue: Default::default(),
            category: Default::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            program_code: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            net: Default::default(),
            udi: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitTotal {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: CodeableConcept::default(),
            amount: Money::default(),
        }
    }
}

impl Default for ExplanationOfBenefitAdditemDetailSubdetail {
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

impl Default for ExplanationOfBenefitSupportinginfo {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            category: Default::default(),
            code: Default::default(),
            timing_date: Default::default(),
            timing_period: Default::default(),
            value_boolean: Default::default(),
            value_string: Default::default(),
            value_quantity: Default::default(),
            value_attachment: Default::default(),
            value_reference: Default::default(),
            reason: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitCareteam {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            provider: Default::default(),
            responsible: Default::default(),
            _responsible: Default::default(),
            role: Default::default(),
            qualification: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitPayee {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            party: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitAdditemDetail {
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

impl Default for ExplanationOfBenefitItemDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            revenue: Default::default(),
            category: Default::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            program_code: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            net: Default::default(),
            udi: Default::default(),
            note_number: Default::default(),
            _note_number: Default::default(),
            adjudication: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitBenefitbalanceFinancial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            allowed_unsigned_int: Default::default(),
            allowed_string: Default::default(),
            allowed_money: Default::default(),
            used_unsigned_int: Default::default(),
            used_money: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitInsurance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            focal: BooleanType::default(),
            _focal: Default::default(),
            coverage: Reference::default(),
            pre_auth_ref: Default::default(),
            _pre_auth_ref: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitItemAdjudication {
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

impl Default for ExplanationOfBenefitPayment {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            adjustment: Default::default(),
            adjustment_reason: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            amount: Default::default(),
            identifier: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitRelated {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            claim: Default::default(),
            relationship: Default::default(),
            reference: Default::default(),
        }
    }
}

impl Default for ExplanationOfBenefitProcedure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: PositiveIntType::default(),
            _sequence: Default::default(),
            type_: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            procedure_codeable_concept: Default::default(),
            procedure_reference: Default::default(),
            udi: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ExplanationOfBenefit {
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

impl crate::traits::resource::ResourceMutators for ExplanationOfBenefit {
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

impl crate::traits::resource::ResourceExistence for ExplanationOfBenefit {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ExplanationOfBenefit {
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

impl crate::traits::domain_resource::DomainResourceMutators for ExplanationOfBenefit {
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

impl crate::traits::domain_resource::DomainResourceExistence for ExplanationOfBenefit {
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

impl crate::traits::explanation_of_benefit::ExplanationOfBenefitAccessors for ExplanationOfBenefit {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> ExplanationofbenefitStatus {
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
    fn billable_period(&self) -> Option<Period> {
        self.billable_period.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn insurer(&self) -> Reference {
        self.insurer.clone()
    }
    fn provider(&self) -> Reference {
        self.provider.clone()
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn funds_reserve_requested(&self) -> Option<CodeableConcept> {
        self.funds_reserve_requested.clone()
    }
    fn funds_reserve(&self) -> Option<CodeableConcept> {
        self.funds_reserve.clone()
    }
    fn related(&self) -> &[ExplanationOfBenefitRelated] {
        self.related.as_deref().unwrap_or(&[])
    }
    fn prescription(&self) -> Option<Reference> {
        self.prescription.clone()
    }
    fn original_prescription(&self) -> Option<Reference> {
        self.original_prescription.clone()
    }
    fn payee(&self) -> Option<ExplanationOfBenefitPayee> {
        self.payee.clone()
    }
    fn referral(&self) -> Option<Reference> {
        self.referral.clone()
    }
    fn facility(&self) -> Option<Reference> {
        self.facility.clone()
    }
    fn claim(&self) -> Option<Reference> {
        self.claim.clone()
    }
    fn claim_response(&self) -> Option<Reference> {
        self.claim_response.clone()
    }
    fn outcome(&self) -> RemittanceOutcome {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn pre_auth_ref(&self) -> &[StringType] {
        self.pre_auth_ref.as_deref().unwrap_or(&[])
    }
    fn pre_auth_ref_period(&self) -> &[Period] {
        self.pre_auth_ref_period.as_deref().unwrap_or(&[])
    }
    fn care_team(&self) -> &[ExplanationOfBenefitCareteam] {
        self.care_team.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[ExplanationOfBenefitSupportinginfo] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn diagnosis(&self) -> &[ExplanationOfBenefitDiagnosis] {
        self.diagnosis.as_deref().unwrap_or(&[])
    }
    fn procedure(&self) -> &[ExplanationOfBenefitProcedure] {
        self.procedure.as_deref().unwrap_or(&[])
    }
    fn precedence(&self) -> Option<PositiveIntType> {
        self.precedence
    }
    fn insurance(&self) -> &[ExplanationOfBenefitInsurance] {
        &self.insurance
    }
    fn accident(&self) -> Option<ExplanationOfBenefitAccident> {
        self.accident.clone()
    }
    fn item(&self) -> &[ExplanationOfBenefitItem] {
        self.item.as_deref().unwrap_or(&[])
    }
    fn add_item(&self) -> &[ExplanationOfBenefitAdditem] {
        self.add_item.as_deref().unwrap_or(&[])
    }
    fn total(&self) -> &[ExplanationOfBenefitTotal] {
        self.total.as_deref().unwrap_or(&[])
    }
    fn payment(&self) -> Option<ExplanationOfBenefitPayment> {
        self.payment.clone()
    }
    fn form_code(&self) -> Option<CodeableConcept> {
        self.form_code.clone()
    }
    fn form(&self) -> Option<Attachment> {
        self.form.clone()
    }
    fn process_note(&self) -> &[ExplanationOfBenefitProcessnote] {
        self.process_note.as_deref().unwrap_or(&[])
    }
    fn benefit_period(&self) -> Option<Period> {
        self.benefit_period.clone()
    }
    fn benefit_balance(&self) -> &[ExplanationOfBenefitBenefitbalance] {
        self.benefit_balance.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::explanation_of_benefit::ExplanationOfBenefitMutators for ExplanationOfBenefit {
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
    fn set_status(self, value: ExplanationofbenefitStatus) -> Self {
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
    fn set_billable_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.billable_period = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_enterer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.enterer = Some(value);
        resource
    }
    fn set_insurer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurer = value;
        resource
    }
    fn set_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.provider = value;
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_funds_reserve_requested(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funds_reserve_requested = Some(value);
        resource
    }
    fn set_funds_reserve(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funds_reserve = Some(value);
        resource
    }
    fn set_related(self, value: Vec<ExplanationOfBenefitRelated>) -> Self {
        let mut resource = self.clone();
        resource.related = Some(value);
        resource
    }
    fn add_related(self, item: ExplanationOfBenefitRelated) -> Self {
        let mut resource = self.clone();
        resource.related.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_prescription(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.prescription = Some(value);
        resource
    }
    fn set_original_prescription(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.original_prescription = Some(value);
        resource
    }
    fn set_payee(self, value: ExplanationOfBenefitPayee) -> Self {
        let mut resource = self.clone();
        resource.payee = Some(value);
        resource
    }
    fn set_referral(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.referral = Some(value);
        resource
    }
    fn set_facility(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.facility = Some(value);
        resource
    }
    fn set_claim(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.claim = Some(value);
        resource
    }
    fn set_claim_response(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.claim_response = Some(value);
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
    fn set_pre_auth_ref(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.pre_auth_ref = Some(value);
        resource
    }
    fn add_pre_auth_ref(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .pre_auth_ref
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_pre_auth_ref_period(self, value: Vec<Period>) -> Self {
        let mut resource = self.clone();
        resource.pre_auth_ref_period = Some(value);
        resource
    }
    fn add_pre_auth_ref_period(self, item: Period) -> Self {
        let mut resource = self.clone();
        resource
            .pre_auth_ref_period
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_care_team(self, value: Vec<ExplanationOfBenefitCareteam>) -> Self {
        let mut resource = self.clone();
        resource.care_team = Some(value);
        resource
    }
    fn add_care_team(self, item: ExplanationOfBenefitCareteam) -> Self {
        let mut resource = self.clone();
        resource.care_team.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<ExplanationOfBenefitSupportinginfo>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: ExplanationOfBenefitSupportinginfo) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_diagnosis(self, value: Vec<ExplanationOfBenefitDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = Some(value);
        resource
    }
    fn add_diagnosis(self, item: ExplanationOfBenefitDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_procedure(self, value: Vec<ExplanationOfBenefitProcedure>) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
    fn add_procedure(self, item: ExplanationOfBenefitProcedure) -> Self {
        let mut resource = self.clone();
        resource.procedure.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_precedence(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.precedence = Some(value);
        resource
    }
    fn set_insurance(self, value: Vec<ExplanationOfBenefitInsurance>) -> Self {
        let mut resource = self.clone();
        resource.insurance = value;
        resource
    }
    fn add_insurance(self, item: ExplanationOfBenefitInsurance) -> Self {
        let mut resource = self.clone();
        resource.insurance.push(item);
        resource
    }
    fn set_accident(self, value: ExplanationOfBenefitAccident) -> Self {
        let mut resource = self.clone();
        resource.accident = Some(value);
        resource
    }
    fn set_item(self, value: Vec<ExplanationOfBenefitItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: ExplanationOfBenefitItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_add_item(self, value: Vec<ExplanationOfBenefitAdditem>) -> Self {
        let mut resource = self.clone();
        resource.add_item = Some(value);
        resource
    }
    fn add_add_item(self, item: ExplanationOfBenefitAdditem) -> Self {
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
    fn set_total(self, value: Vec<ExplanationOfBenefitTotal>) -> Self {
        let mut resource = self.clone();
        resource.total = Some(value);
        resource
    }
    fn add_total(self, item: ExplanationOfBenefitTotal) -> Self {
        let mut resource = self.clone();
        resource.total.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_payment(self, value: ExplanationOfBenefitPayment) -> Self {
        let mut resource = self.clone();
        resource.payment = Some(value);
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
    fn set_process_note(self, value: Vec<ExplanationOfBenefitProcessnote>) -> Self {
        let mut resource = self.clone();
        resource.process_note = Some(value);
        resource
    }
    fn add_process_note(self, item: ExplanationOfBenefitProcessnote) -> Self {
        let mut resource = self.clone();
        resource
            .process_note
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_benefit_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.benefit_period = Some(value);
        resource
    }
    fn set_benefit_balance(self, value: Vec<ExplanationOfBenefitBenefitbalance>) -> Self {
        let mut resource = self.clone();
        resource.benefit_balance = Some(value);
        resource
    }
    fn add_benefit_balance(self, item: ExplanationOfBenefitBenefitbalance) -> Self {
        let mut resource = self.clone();
        resource
            .benefit_balance
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::explanation_of_benefit::ExplanationOfBenefitExistence for ExplanationOfBenefit {
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
    fn has_billable_period(&self) -> bool {
        self.billable_period.is_some()
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_enterer(&self) -> bool {
        self.enterer.is_some()
    }
    fn has_insurer(&self) -> bool {
        true
    }
    fn has_provider(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_funds_reserve_requested(&self) -> bool {
        self.funds_reserve_requested.is_some()
    }
    fn has_funds_reserve(&self) -> bool {
        self.funds_reserve.is_some()
    }
    fn has_related(&self) -> bool {
        self.related.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_prescription(&self) -> bool {
        self.prescription.is_some()
    }
    fn has_original_prescription(&self) -> bool {
        self.original_prescription.is_some()
    }
    fn has_payee(&self) -> bool {
        self.payee.is_some()
    }
    fn has_referral(&self) -> bool {
        self.referral.is_some()
    }
    fn has_facility(&self) -> bool {
        self.facility.is_some()
    }
    fn has_claim(&self) -> bool {
        self.claim.is_some()
    }
    fn has_claim_response(&self) -> bool {
        self.claim_response.is_some()
    }
    fn has_outcome(&self) -> bool {
        true
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_pre_auth_ref(&self) -> bool {
        self.pre_auth_ref.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_pre_auth_ref_period(&self) -> bool {
        self.pre_auth_ref_period
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_care_team(&self) -> bool {
        self.care_team.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_diagnosis(&self) -> bool {
        self.diagnosis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_procedure(&self) -> bool {
        self.procedure.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_precedence(&self) -> bool {
        self.precedence.is_some()
    }
    fn has_insurance(&self) -> bool {
        !self.insurance.is_empty()
    }
    fn has_accident(&self) -> bool {
        self.accident.is_some()
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
    fn has_form_code(&self) -> bool {
        self.form_code.is_some()
    }
    fn has_form(&self) -> bool {
        self.form.is_some()
    }
    fn has_process_note(&self) -> bool {
        self.process_note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_benefit_period(&self) -> bool {
        self.benefit_period.is_some()
    }
    fn has_benefit_balance(&self) -> bool {
        self.benefit_balance.as_ref().is_some_and(|v| !v.is_empty())
    }
}
