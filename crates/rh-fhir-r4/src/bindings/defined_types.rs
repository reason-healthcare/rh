use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/defined-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefinedTypes {
    /// Address
    #[serde(rename = "Address")]
    Address,
    /// Age
    #[serde(rename = "Age")]
    Age,
    /// Annotation
    #[serde(rename = "Annotation")]
    Annotation,
    /// Attachment
    #[serde(rename = "Attachment")]
    Attachment,
    /// BackboneElement
    #[serde(rename = "BackboneElement")]
    BackboneElement,
    /// CodeableConcept
    #[serde(rename = "CodeableConcept")]
    CodeableConcept,
    /// Coding
    #[serde(rename = "Coding")]
    Coding,
    /// ContactDetail
    #[serde(rename = "ContactDetail")]
    ContactDetail,
    /// ContactPoint
    #[serde(rename = "ContactPoint")]
    ContactPoint,
    /// Contributor
    #[serde(rename = "Contributor")]
    Contributor,
    /// Count
    #[serde(rename = "Count")]
    Count,
    /// DataRequirement
    #[serde(rename = "DataRequirement")]
    DataRequirement,
    /// Distance
    #[serde(rename = "Distance")]
    Distance,
    /// Dosage
    #[serde(rename = "Dosage")]
    Dosage,
    /// Duration
    #[serde(rename = "Duration")]
    Duration,
    /// Element
    #[serde(rename = "Element")]
    Element,
    /// ElementDefinition
    #[serde(rename = "ElementDefinition")]
    ElementDefinition,
    /// Expression
    #[serde(rename = "Expression")]
    Expression,
    /// Extension
    #[serde(rename = "Extension")]
    Extension,
    /// HumanName
    #[serde(rename = "HumanName")]
    HumanName,
    /// Identifier
    #[serde(rename = "Identifier")]
    Identifier,
    /// MarketingStatus
    #[serde(rename = "MarketingStatus")]
    MarketingStatus,
    /// Meta
    #[serde(rename = "Meta")]
    Meta,
    /// Money
    #[serde(rename = "Money")]
    Money,
    /// MoneyQuantity
    #[serde(rename = "MoneyQuantity")]
    MoneyQuantity,
    /// Narrative
    #[serde(rename = "Narrative")]
    Narrative,
    /// ParameterDefinition
    #[serde(rename = "ParameterDefinition")]
    ParameterDefinition,
    /// Period
    #[serde(rename = "Period")]
    Period,
    /// Population
    #[serde(rename = "Population")]
    Population,
    /// ProdCharacteristic
    #[serde(rename = "ProdCharacteristic")]
    ProdCharacteristic,
    /// ProductShelfLife
    #[serde(rename = "ProductShelfLife")]
    ProductShelfLife,
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
    /// Range
    #[serde(rename = "Range")]
    Range,
    /// Ratio
    #[serde(rename = "Ratio")]
    Ratio,
    /// Reference
    #[serde(rename = "Reference")]
    Reference,
    /// RelatedArtifact
    #[serde(rename = "RelatedArtifact")]
    RelatedArtifact,
    /// SampledData
    #[serde(rename = "SampledData")]
    SampledData,
    /// Signature
    #[serde(rename = "Signature")]
    Signature,
    /// SimpleQuantity
    #[serde(rename = "SimpleQuantity")]
    SimpleQuantity,
    /// SubstanceAmount
    #[serde(rename = "SubstanceAmount")]
    SubstanceAmount,
    /// Timing
    #[serde(rename = "Timing")]
    Timing,
    /// TriggerDefinition
    #[serde(rename = "TriggerDefinition")]
    TriggerDefinition,
    /// UsageContext
    #[serde(rename = "UsageContext")]
    UsageContext,
    /// base64Binary
    #[serde(rename = "base64Binary")]
    Base64Binary,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// canonical
    #[serde(rename = "canonical")]
    Canonical,
    /// code
    #[serde(rename = "code")]
    Code,
    /// date
    #[serde(rename = "date")]
    Date,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// id
    #[serde(rename = "id")]
    Id,
    /// instant
    #[serde(rename = "instant")]
    Instant,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// markdown
    #[serde(rename = "markdown")]
    Markdown,
    /// oid
    #[serde(rename = "oid")]
    Oid,
    /// positiveInt
    #[serde(rename = "positiveInt")]
    PositiveInt,
    /// string
    #[serde(rename = "string")]
    String,
    /// time
    #[serde(rename = "time")]
    Time,
    /// unsignedInt
    #[serde(rename = "unsignedInt")]
    UnsignedInt,
    /// uri
    #[serde(rename = "uri")]
    Uri,
    /// url
    #[serde(rename = "url")]
    Url,
    /// uuid
    #[serde(rename = "uuid")]
    Uuid,
    /// XHTML
    #[serde(rename = "xhtml")]
    Xhtml,
    /// Account
    #[serde(rename = "Account")]
    Account,
    /// ActivityDefinition
    #[serde(rename = "ActivityDefinition")]
    ActivityDefinition,
    /// AdverseEvent
    #[serde(rename = "AdverseEvent")]
    AdverseEvent,
    /// AllergyIntolerance
    #[serde(rename = "AllergyIntolerance")]
    AllergyIntolerance,
    /// Appointment
    #[serde(rename = "Appointment")]
    Appointment,
    /// AppointmentResponse
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    /// AuditEvent
    #[serde(rename = "AuditEvent")]
    AuditEvent,
    /// Basic
    #[serde(rename = "Basic")]
    Basic,
    /// Binary
    #[serde(rename = "Binary")]
    Binary,
    /// BiologicallyDerivedProduct
    #[serde(rename = "BiologicallyDerivedProduct")]
    BiologicallyDerivedProduct,
    /// BodyStructure
    #[serde(rename = "BodyStructure")]
    BodyStructure,
    /// Bundle
    #[serde(rename = "Bundle")]
    Bundle,
    /// CapabilityStatement
    #[serde(rename = "CapabilityStatement")]
    CapabilityStatement,
    /// CarePlan
    #[serde(rename = "CarePlan")]
    CarePlan,
    /// CareTeam
    #[serde(rename = "CareTeam")]
    CareTeam,
    /// CatalogEntry
    #[serde(rename = "CatalogEntry")]
    CatalogEntry,
    /// ChargeItem
    #[serde(rename = "ChargeItem")]
    ChargeItem,
    /// ChargeItemDefinition
    #[serde(rename = "ChargeItemDefinition")]
    ChargeItemDefinition,
    /// Claim
    #[serde(rename = "Claim")]
    Claim,
    /// ClaimResponse
    #[serde(rename = "ClaimResponse")]
    ClaimResponse,
    /// ClinicalImpression
    #[serde(rename = "ClinicalImpression")]
    ClinicalImpression,
    /// CodeSystem
    #[serde(rename = "CodeSystem")]
    CodeSystem,
    /// Communication
    #[serde(rename = "Communication")]
    Communication,
    /// CommunicationRequest
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    /// CompartmentDefinition
    #[serde(rename = "CompartmentDefinition")]
    CompartmentDefinition,
    /// Composition
    #[serde(rename = "Composition")]
    Composition,
    /// ConceptMap
    #[serde(rename = "ConceptMap")]
    ConceptMap,
    /// Condition
    #[serde(rename = "Condition")]
    Condition,
    /// Consent
    #[serde(rename = "Consent")]
    Consent,
    /// Contract
    #[serde(rename = "Contract")]
    Contract,
    /// Coverage
    #[serde(rename = "Coverage")]
    Coverage,
    /// CoverageEligibilityRequest
    #[serde(rename = "CoverageEligibilityRequest")]
    CoverageEligibilityRequest,
    /// CoverageEligibilityResponse
    #[serde(rename = "CoverageEligibilityResponse")]
    CoverageEligibilityResponse,
    /// DetectedIssue
    #[serde(rename = "DetectedIssue")]
    DetectedIssue,
    /// Device
    #[serde(rename = "Device")]
    Device,
    /// DeviceDefinition
    #[serde(rename = "DeviceDefinition")]
    DeviceDefinition,
    /// DeviceMetric
    #[serde(rename = "DeviceMetric")]
    DeviceMetric,
    /// DeviceRequest
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    /// DeviceUseStatement
    #[serde(rename = "DeviceUseStatement")]
    DeviceUseStatement,
    /// DiagnosticReport
    #[serde(rename = "DiagnosticReport")]
    DiagnosticReport,
    /// DocumentManifest
    #[serde(rename = "DocumentManifest")]
    DocumentManifest,
    /// DocumentReference
    #[serde(rename = "DocumentReference")]
    DocumentReference,
    /// DomainResource
    #[serde(rename = "DomainResource")]
    DomainResource,
    /// EffectEvidenceSynthesis
    #[serde(rename = "EffectEvidenceSynthesis")]
    EffectEvidenceSynthesis,
    /// Encounter
    #[serde(rename = "Encounter")]
    Encounter,
    /// Endpoint
    #[serde(rename = "Endpoint")]
    Endpoint,
    /// EnrollmentRequest
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    /// EnrollmentResponse
    #[serde(rename = "EnrollmentResponse")]
    EnrollmentResponse,
    /// EpisodeOfCare
    #[serde(rename = "EpisodeOfCare")]
    EpisodeOfCare,
    /// EventDefinition
    #[serde(rename = "EventDefinition")]
    EventDefinition,
    /// Evidence
    #[serde(rename = "Evidence")]
    Evidence,
    /// EvidenceVariable
    #[serde(rename = "EvidenceVariable")]
    EvidenceVariable,
    /// ExampleScenario
    #[serde(rename = "ExampleScenario")]
    ExampleScenario,
    /// ExplanationOfBenefit
    #[serde(rename = "ExplanationOfBenefit")]
    ExplanationOfBenefit,
    /// FamilyMemberHistory
    #[serde(rename = "FamilyMemberHistory")]
    FamilyMemberHistory,
    /// Flag
    #[serde(rename = "Flag")]
    Flag,
    /// Goal
    #[serde(rename = "Goal")]
    Goal,
    /// GraphDefinition
    #[serde(rename = "GraphDefinition")]
    GraphDefinition,
    /// Group
    #[serde(rename = "Group")]
    Group,
    /// GuidanceResponse
    #[serde(rename = "GuidanceResponse")]
    GuidanceResponse,
    /// HealthcareService
    #[serde(rename = "HealthcareService")]
    HealthcareService,
    /// ImagingStudy
    #[serde(rename = "ImagingStudy")]
    ImagingStudy,
    /// Immunization
    #[serde(rename = "Immunization")]
    Immunization,
    /// ImmunizationEvaluation
    #[serde(rename = "ImmunizationEvaluation")]
    ImmunizationEvaluation,
    /// ImmunizationRecommendation
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    /// ImplementationGuide
    #[serde(rename = "ImplementationGuide")]
    ImplementationGuide,
    /// InsurancePlan
    #[serde(rename = "InsurancePlan")]
    InsurancePlan,
    /// Invoice
    #[serde(rename = "Invoice")]
    Invoice,
    /// Library
    #[serde(rename = "Library")]
    Library,
    /// Linkage
    #[serde(rename = "Linkage")]
    Linkage,
    /// List
    #[serde(rename = "List")]
    List,
    /// Location
    #[serde(rename = "Location")]
    Location,
    /// Measure
    #[serde(rename = "Measure")]
    Measure,
    /// MeasureReport
    #[serde(rename = "MeasureReport")]
    MeasureReport,
    /// Media
    #[serde(rename = "Media")]
    Media,
    /// Medication
    #[serde(rename = "Medication")]
    Medication,
    /// MedicationAdministration
    #[serde(rename = "MedicationAdministration")]
    MedicationAdministration,
    /// MedicationDispense
    #[serde(rename = "MedicationDispense")]
    MedicationDispense,
    /// MedicationKnowledge
    #[serde(rename = "MedicationKnowledge")]
    MedicationKnowledge,
    /// MedicationRequest
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    /// MedicationStatement
    #[serde(rename = "MedicationStatement")]
    MedicationStatement,
    /// MedicinalProduct
    #[serde(rename = "MedicinalProduct")]
    MedicinalProduct,
    /// MedicinalProductAuthorization
    #[serde(rename = "MedicinalProductAuthorization")]
    MedicinalProductAuthorization,
    /// MedicinalProductContraindication
    #[serde(rename = "MedicinalProductContraindication")]
    MedicinalProductContraindication,
    /// MedicinalProductIndication
    #[serde(rename = "MedicinalProductIndication")]
    MedicinalProductIndication,
    /// MedicinalProductIngredient
    #[serde(rename = "MedicinalProductIngredient")]
    MedicinalProductIngredient,
    /// MedicinalProductInteraction
    #[serde(rename = "MedicinalProductInteraction")]
    MedicinalProductInteraction,
    /// MedicinalProductManufactured
    #[serde(rename = "MedicinalProductManufactured")]
    MedicinalProductManufactured,
    /// MedicinalProductPackaged
    #[serde(rename = "MedicinalProductPackaged")]
    MedicinalProductPackaged,
    /// MedicinalProductPharmaceutical
    #[serde(rename = "MedicinalProductPharmaceutical")]
    MedicinalProductPharmaceutical,
    /// MedicinalProductUndesirableEffect
    #[serde(rename = "MedicinalProductUndesirableEffect")]
    MedicinalProductUndesirableEffect,
    /// MessageDefinition
    #[serde(rename = "MessageDefinition")]
    MessageDefinition,
    /// MessageHeader
    #[serde(rename = "MessageHeader")]
    MessageHeader,
    /// MolecularSequence
    #[serde(rename = "MolecularSequence")]
    MolecularSequence,
    /// NamingSystem
    #[serde(rename = "NamingSystem")]
    NamingSystem,
    /// NutritionOrder
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    /// Observation
    #[serde(rename = "Observation")]
    Observation,
    /// ObservationDefinition
    #[serde(rename = "ObservationDefinition")]
    ObservationDefinition,
    /// OperationDefinition
    #[serde(rename = "OperationDefinition")]
    OperationDefinition,
    /// OperationOutcome
    #[serde(rename = "OperationOutcome")]
    OperationOutcome,
    /// Organization
    #[serde(rename = "Organization")]
    Organization,
    /// OrganizationAffiliation
    #[serde(rename = "OrganizationAffiliation")]
    OrganizationAffiliation,
    /// Parameters
    #[serde(rename = "Parameters")]
    Parameters,
    /// Patient
    #[serde(rename = "Patient")]
    Patient,
    /// PaymentNotice
    #[serde(rename = "PaymentNotice")]
    PaymentNotice,
    /// PaymentReconciliation
    #[serde(rename = "PaymentReconciliation")]
    PaymentReconciliation,
    /// Person
    #[serde(rename = "Person")]
    Person,
    /// PlanDefinition
    #[serde(rename = "PlanDefinition")]
    PlanDefinition,
    /// Practitioner
    #[serde(rename = "Practitioner")]
    Practitioner,
    /// PractitionerRole
    #[serde(rename = "PractitionerRole")]
    PractitionerRole,
    /// Procedure
    #[serde(rename = "Procedure")]
    Procedure,
    /// Provenance
    #[serde(rename = "Provenance")]
    Provenance,
    /// Questionnaire
    #[serde(rename = "Questionnaire")]
    Questionnaire,
    /// QuestionnaireResponse
    #[serde(rename = "QuestionnaireResponse")]
    QuestionnaireResponse,
    /// RelatedPerson
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    /// RequestGroup
    #[serde(rename = "RequestGroup")]
    RequestGroup,
    /// ResearchDefinition
    #[serde(rename = "ResearchDefinition")]
    ResearchDefinition,
    /// ResearchElementDefinition
    #[serde(rename = "ResearchElementDefinition")]
    ResearchElementDefinition,
    /// ResearchStudy
    #[serde(rename = "ResearchStudy")]
    ResearchStudy,
    /// ResearchSubject
    #[serde(rename = "ResearchSubject")]
    ResearchSubject,
    /// Resource
    #[serde(rename = "Resource")]
    Resource,
    /// RiskAssessment
    #[serde(rename = "RiskAssessment")]
    RiskAssessment,
    /// RiskEvidenceSynthesis
    #[serde(rename = "RiskEvidenceSynthesis")]
    RiskEvidenceSynthesis,
    /// Schedule
    #[serde(rename = "Schedule")]
    Schedule,
    /// SearchParameter
    #[serde(rename = "SearchParameter")]
    SearchParameter,
    /// ServiceRequest
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    /// Slot
    #[serde(rename = "Slot")]
    Slot,
    /// Specimen
    #[serde(rename = "Specimen")]
    Specimen,
    /// SpecimenDefinition
    #[serde(rename = "SpecimenDefinition")]
    SpecimenDefinition,
    /// StructureDefinition
    #[serde(rename = "StructureDefinition")]
    StructureDefinition,
    /// StructureMap
    #[serde(rename = "StructureMap")]
    StructureMap,
    /// Subscription
    #[serde(rename = "Subscription")]
    Subscription,
    /// Substance
    #[serde(rename = "Substance")]
    Substance,
    /// SubstanceNucleicAcid
    #[serde(rename = "SubstanceNucleicAcid")]
    SubstanceNucleicAcid,
    /// SubstancePolymer
    #[serde(rename = "SubstancePolymer")]
    SubstancePolymer,
    /// SubstanceProtein
    #[serde(rename = "SubstanceProtein")]
    SubstanceProtein,
    /// SubstanceReferenceInformation
    #[serde(rename = "SubstanceReferenceInformation")]
    SubstanceReferenceInformation,
    /// SubstanceSourceMaterial
    #[serde(rename = "SubstanceSourceMaterial")]
    SubstanceSourceMaterial,
    /// SubstanceSpecification
    #[serde(rename = "SubstanceSpecification")]
    SubstanceSpecification,
    /// SupplyDelivery
    #[serde(rename = "SupplyDelivery")]
    SupplyDelivery,
    /// SupplyRequest
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    /// Task
    #[serde(rename = "Task")]
    Task,
    /// TerminologyCapabilities
    #[serde(rename = "TerminologyCapabilities")]
    TerminologyCapabilities,
    /// TestReport
    #[serde(rename = "TestReport")]
    TestReport,
    /// TestScript
    #[serde(rename = "TestScript")]
    TestScript,
    /// ValueSet
    #[serde(rename = "ValueSet")]
    ValueSet,
    /// VerificationResult
    #[serde(rename = "VerificationResult")]
    VerificationResult,
    /// VisionPrescription
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}
impl Default for DefinedTypes {
    fn default() -> Self {
        Self::Address
    }
}
