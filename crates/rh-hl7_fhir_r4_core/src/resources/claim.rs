use crate::bindings::claim_use::ClaimUse;
use crate::bindings::fm_status::FmStatus;
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
/// Claim
///
/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Claim
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Claim
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for claim
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
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
    /// Resource creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Author of the claim
    pub enterer: Option<Reference>,
    /// Target
    pub insurer: Option<Reference>,
    /// Party responsible for the claim
    pub provider: Reference,
    /// Desired processing ugency
    ///
    /// Binding: example (The timeliness with which processing is required: stat, normal, deferred.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/process-priority
    pub priority: CodeableConcept,
    /// For whom to reserve funds
    ///
    /// Binding: example (For whom funds are to be reserved: (Patient, Provider, None).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/fundsreserve
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,
    /// Prior or corollary claims
    pub related: Option<Vec<ClaimRelated>>,
    /// Prescription authorizing services and products
    pub prescription: Option<Reference>,
    /// Original prescription if superseded by fulfiller
    #[serde(rename = "originalPrescription")]
    pub original_prescription: Option<Reference>,
    /// Recipient of benefits payable
    pub payee: Option<ClaimPayee>,
    /// Treatment referral
    pub referral: Option<Reference>,
    /// Servicing facility
    pub facility: Option<Reference>,
    /// Members of the care team
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<ClaimCareteam>>,
    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<ClaimSupportinginfo>>,
    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ClaimDiagnosis>>,
    /// Clinical procedures performed
    pub procedure: Option<Vec<ClaimProcedure>>,
    /// Patient insurance information
    pub insurance: Vec<ClaimInsurance>,
    /// Details of the event
    pub accident: Option<ClaimAccident>,
    /// Product or service provided
    pub item: Option<Vec<ClaimItem>>,
    /// Total claim cost
    pub total: Option<Money>,
}
/// Claim nested structure for the 'insurance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimInsurance {
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
    /// Pre-assigned Claim number
    pub identifier: Option<Identifier>,
    /// Insurance information
    pub coverage: Reference,
    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<StringType>,
    /// Extension element for the 'businessArrangement' primitive field. Contains metadata and extensions.
    #[serde(rename = "_businessArrangement")]
    pub _business_arrangement: Option<Element>,
    /// Prior authorization reference number
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<Vec<StringType>>,
    /// Extension element for the 'preAuthRef' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preAuthRef")]
    pub _pre_auth_ref: Option<Element>,
    /// Adjudication results
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Reference>,
}
/// Claim nested structure for the 'payee' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimPayee {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category of recipient
    ///
    /// Binding: example (A code for the party to be reimbursed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/payeetype
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Recipient reference
    pub party: Option<Reference>,
}
/// Claim nested structure for the 'careTeam' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimCareteam {
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
/// Claim nested structure for the 'supportingInfo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimSupportinginfo {
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
    pub timing_date: Option<DateType>,
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
    pub reason: Option<CodeableConcept>,
}
/// ClaimItemDetail nested structure for the 'subDetail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimItemDetailSubdetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Item instance identifier
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
    /// Binding: example (Benefit categories such as: oral-basic, major, glasses.)
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
}
/// ClaimItem nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimItemDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Item instance identifier
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
    /// Binding: example (Benefit categories such as: oral-basic, major, glasses.)
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
}
/// Claim nested structure for the 'related' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRelated {
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
/// Claim nested structure for the 'accident' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimAccident {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// When the incident occurred
    pub date: DateType,
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
/// Claim nested structure for the 'procedure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimProcedure {
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
/// Claim nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimDiagnosis {
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
/// Claim nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Product or service provided
    pub detail: Option<Vec<ClaimItemDetail>>,
    /// Item instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Applicable careTeam members
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
    /// Binding: example (Benefit categories such as: oral-basic, major, glasses.)
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
}

impl Default for Claim {
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
            billable_period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            enterer: Default::default(),
            insurer: Default::default(),
            provider: Reference::default(),
            priority: CodeableConcept::default(),
            funds_reserve: Default::default(),
            related: Default::default(),
            prescription: Default::default(),
            original_prescription: Default::default(),
            payee: Default::default(),
            referral: Default::default(),
            facility: Default::default(),
            care_team: Default::default(),
            supporting_info: Default::default(),
            diagnosis: Default::default(),
            procedure: Default::default(),
            insurance: Vec::new(),
            accident: Default::default(),
            item: Default::default(),
            total: Default::default(),
        }
    }
}

impl Default for ClaimInsurance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: PositiveIntType::default(),
            _sequence: Default::default(),
            focal: BooleanType::default(),
            _focal: Default::default(),
            identifier: Default::default(),
            coverage: Reference::default(),
            business_arrangement: Default::default(),
            _business_arrangement: Default::default(),
            pre_auth_ref: Default::default(),
            _pre_auth_ref: Default::default(),
            claim_response: Default::default(),
        }
    }
}

impl Default for ClaimPayee {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            party: Default::default(),
        }
    }
}

impl Default for ClaimCareteam {
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

impl Default for ClaimSupportinginfo {
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

impl Default for ClaimItemDetailSubdetail {
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
        }
    }
}

impl Default for ClaimItemDetail {
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
        }
    }
}

impl Default for ClaimRelated {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            claim: Default::default(),
            relationship: Default::default(),
            reference: Default::default(),
        }
    }
}

impl Default for ClaimAccident {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            date: DateType::default(),
            _date: Default::default(),
            type_: Default::default(),
            location_address: Default::default(),
            location_reference: Default::default(),
        }
    }
}

impl Default for ClaimProcedure {
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

impl Default for ClaimDiagnosis {
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

impl Default for ClaimItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
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
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Claim {
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

impl crate::traits::resource::ResourceMutators for Claim {
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

impl crate::traits::resource::ResourceExistence for Claim {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Claim {
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

impl crate::traits::domain_resource::DomainResourceMutators for Claim {
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

impl crate::traits::domain_resource::DomainResourceExistence for Claim {
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

impl crate::traits::claim::ClaimAccessors for Claim {
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
    fn billable_period(&self) -> Option<Period> {
        self.billable_period.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn insurer(&self) -> Option<Reference> {
        self.insurer.clone()
    }
    fn provider(&self) -> Reference {
        self.provider.clone()
    }
    fn priority(&self) -> CodeableConcept {
        self.priority.clone()
    }
    fn funds_reserve(&self) -> Option<CodeableConcept> {
        self.funds_reserve.clone()
    }
    fn related(&self) -> &[ClaimRelated] {
        self.related.as_deref().unwrap_or(&[])
    }
    fn prescription(&self) -> Option<Reference> {
        self.prescription.clone()
    }
    fn original_prescription(&self) -> Option<Reference> {
        self.original_prescription.clone()
    }
    fn payee(&self) -> Option<ClaimPayee> {
        self.payee.clone()
    }
    fn referral(&self) -> Option<Reference> {
        self.referral.clone()
    }
    fn facility(&self) -> Option<Reference> {
        self.facility.clone()
    }
    fn care_team(&self) -> &[ClaimCareteam] {
        self.care_team.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[ClaimSupportinginfo] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn diagnosis(&self) -> &[ClaimDiagnosis] {
        self.diagnosis.as_deref().unwrap_or(&[])
    }
    fn procedure(&self) -> &[ClaimProcedure] {
        self.procedure.as_deref().unwrap_or(&[])
    }
    fn insurance(&self) -> &[ClaimInsurance] {
        &self.insurance
    }
    fn accident(&self) -> Option<ClaimAccident> {
        self.accident.clone()
    }
    fn item(&self) -> &[ClaimItem] {
        self.item.as_deref().unwrap_or(&[])
    }
    fn total(&self) -> Option<Money> {
        self.total.clone()
    }
}

impl crate::traits::claim::ClaimMutators for Claim {
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
        resource.insurer = Some(value);
        resource
    }
    fn set_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.provider = value;
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = value;
        resource
    }
    fn set_funds_reserve(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funds_reserve = Some(value);
        resource
    }
    fn set_related(self, value: Vec<ClaimRelated>) -> Self {
        let mut resource = self.clone();
        resource.related = Some(value);
        resource
    }
    fn add_related(self, item: ClaimRelated) -> Self {
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
    fn set_payee(self, value: ClaimPayee) -> Self {
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
    fn set_care_team(self, value: Vec<ClaimCareteam>) -> Self {
        let mut resource = self.clone();
        resource.care_team = Some(value);
        resource
    }
    fn add_care_team(self, item: ClaimCareteam) -> Self {
        let mut resource = self.clone();
        resource.care_team.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<ClaimSupportinginfo>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: ClaimSupportinginfo) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_diagnosis(self, value: Vec<ClaimDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = Some(value);
        resource
    }
    fn add_diagnosis(self, item: ClaimDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_procedure(self, value: Vec<ClaimProcedure>) -> Self {
        let mut resource = self.clone();
        resource.procedure = Some(value);
        resource
    }
    fn add_procedure(self, item: ClaimProcedure) -> Self {
        let mut resource = self.clone();
        resource.procedure.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_insurance(self, value: Vec<ClaimInsurance>) -> Self {
        let mut resource = self.clone();
        resource.insurance = value;
        resource
    }
    fn add_insurance(self, item: ClaimInsurance) -> Self {
        let mut resource = self.clone();
        resource.insurance.push(item);
        resource
    }
    fn set_accident(self, value: ClaimAccident) -> Self {
        let mut resource = self.clone();
        resource.accident = Some(value);
        resource
    }
    fn set_item(self, value: Vec<ClaimItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: ClaimItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_total(self, value: Money) -> Self {
        let mut resource = self.clone();
        resource.total = Some(value);
        resource
    }
}

impl crate::traits::claim::ClaimExistence for Claim {
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
        self.insurer.is_some()
    }
    fn has_provider(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        true
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
    fn has_insurance(&self) -> bool {
        !self.insurance.is_empty()
    }
    fn has_accident(&self) -> bool {
        self.accident.is_some()
    }
    fn has_item(&self) -> bool {
        self.item.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_total(&self) -> bool {
        self.total.is_some()
    }
}
