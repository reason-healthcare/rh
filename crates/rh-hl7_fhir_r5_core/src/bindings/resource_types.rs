use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/resource-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceTypes {
    #[serde(rename = "Account")]
    Account,
    #[serde(rename = "ActivityDefinition")]
    ActivityDefinition,
    #[serde(rename = "ActorDefinition")]
    ActorDefinition,
    #[serde(rename = "AdministrableProductDefinition")]
    AdministrableProductDefinition,
    #[serde(rename = "AdverseEvent")]
    AdverseEvent,
    #[serde(rename = "AllergyIntolerance")]
    AllergyIntolerance,
    #[serde(rename = "Appointment")]
    Appointment,
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    #[serde(rename = "ArtifactAssessment")]
    ArtifactAssessment,
    #[serde(rename = "AuditEvent")]
    AuditEvent,
    #[serde(rename = "Basic")]
    Basic,
    #[serde(rename = "Binary")]
    Binary,
    #[serde(rename = "BiologicallyDerivedProduct")]
    BiologicallyDerivedProduct,
    #[serde(rename = "BiologicallyDerivedProductDispense")]
    BiologicallyDerivedProductDispense,
    #[serde(rename = "BodyStructure")]
    BodyStructure,
    #[serde(rename = "Bundle")]
    Bundle,
    #[serde(rename = "CapabilityStatement")]
    CapabilityStatement,
    #[serde(rename = "CarePlan")]
    CarePlan,
    #[serde(rename = "CareTeam")]
    CareTeam,
    #[serde(rename = "ChargeItem")]
    ChargeItem,
    #[serde(rename = "ChargeItemDefinition")]
    ChargeItemDefinition,
    #[serde(rename = "Citation")]
    Citation,
    #[serde(rename = "Claim")]
    Claim,
    #[serde(rename = "ClaimResponse")]
    ClaimResponse,
    #[serde(rename = "ClinicalImpression")]
    ClinicalImpression,
    #[serde(rename = "ClinicalUseDefinition")]
    ClinicalUseDefinition,
    #[serde(rename = "CodeSystem")]
    CodeSystem,
    #[serde(rename = "Communication")]
    Communication,
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    #[serde(rename = "CompartmentDefinition")]
    CompartmentDefinition,
    #[serde(rename = "Composition")]
    Composition,
    #[serde(rename = "ConceptMap")]
    ConceptMap,
    #[serde(rename = "Condition")]
    Condition,
    #[serde(rename = "ConditionDefinition")]
    ConditionDefinition,
    #[serde(rename = "Consent")]
    Consent,
    #[serde(rename = "Contract")]
    Contract,
    #[serde(rename = "Coverage")]
    Coverage,
    #[serde(rename = "CoverageEligibilityRequest")]
    CoverageEligibilityRequest,
    #[serde(rename = "CoverageEligibilityResponse")]
    CoverageEligibilityResponse,
    #[serde(rename = "DetectedIssue")]
    DetectedIssue,
    #[serde(rename = "Device")]
    Device,
    #[serde(rename = "DeviceAssociation")]
    DeviceAssociation,
    #[serde(rename = "DeviceDefinition")]
    DeviceDefinition,
    #[serde(rename = "DeviceDispense")]
    DeviceDispense,
    #[serde(rename = "DeviceMetric")]
    DeviceMetric,
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    #[serde(rename = "DeviceUsage")]
    DeviceUsage,
    #[serde(rename = "DiagnosticReport")]
    DiagnosticReport,
    #[serde(rename = "DocumentReference")]
    DocumentReference,
    #[serde(rename = "Encounter")]
    Encounter,
    #[serde(rename = "EncounterHistory")]
    EncounterHistory,
    #[serde(rename = "Endpoint")]
    Endpoint,
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    #[serde(rename = "EnrollmentResponse")]
    EnrollmentResponse,
    #[serde(rename = "EpisodeOfCare")]
    EpisodeOfCare,
    #[serde(rename = "EventDefinition")]
    EventDefinition,
    #[serde(rename = "Evidence")]
    Evidence,
    #[serde(rename = "EvidenceReport")]
    EvidenceReport,
    #[serde(rename = "EvidenceVariable")]
    EvidenceVariable,
    #[serde(rename = "ExampleScenario")]
    ExampleScenario,
    #[serde(rename = "ExplanationOfBenefit")]
    ExplanationOfBenefit,
    #[serde(rename = "FamilyMemberHistory")]
    FamilyMemberHistory,
    #[serde(rename = "Flag")]
    Flag,
    #[serde(rename = "FormularyItem")]
    FormularyItem,
    #[serde(rename = "GenomicStudy")]
    GenomicStudy,
    #[serde(rename = "Goal")]
    Goal,
    #[serde(rename = "GraphDefinition")]
    GraphDefinition,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "GuidanceResponse")]
    GuidanceResponse,
    #[serde(rename = "HealthcareService")]
    HealthcareService,
    #[serde(rename = "ImagingSelection")]
    ImagingSelection,
    #[serde(rename = "ImagingStudy")]
    ImagingStudy,
    #[serde(rename = "Immunization")]
    Immunization,
    #[serde(rename = "ImmunizationEvaluation")]
    ImmunizationEvaluation,
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    #[serde(rename = "ImplementationGuide")]
    ImplementationGuide,
    #[serde(rename = "Ingredient")]
    Ingredient,
    #[serde(rename = "InsurancePlan")]
    InsurancePlan,
    #[serde(rename = "InventoryItem")]
    InventoryItem,
    #[serde(rename = "InventoryReport")]
    InventoryReport,
    #[serde(rename = "Invoice")]
    Invoice,
    #[serde(rename = "Library")]
    Library,
    #[serde(rename = "Linkage")]
    Linkage,
    #[serde(rename = "List")]
    List,
    #[serde(rename = "Location")]
    Location,
    #[serde(rename = "ManufacturedItemDefinition")]
    ManufacturedItemDefinition,
    #[serde(rename = "Measure")]
    Measure,
    #[serde(rename = "MeasureReport")]
    MeasureReport,
    #[serde(rename = "Medication")]
    Medication,
    #[serde(rename = "MedicationAdministration")]
    MedicationAdministration,
    #[serde(rename = "MedicationDispense")]
    MedicationDispense,
    #[serde(rename = "MedicationKnowledge")]
    MedicationKnowledge,
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    #[serde(rename = "MedicationStatement")]
    MedicationStatement,
    #[serde(rename = "MedicinalProductDefinition")]
    MedicinalProductDefinition,
    #[serde(rename = "MessageDefinition")]
    MessageDefinition,
    #[serde(rename = "MessageHeader")]
    MessageHeader,
    #[serde(rename = "MolecularSequence")]
    MolecularSequence,
    #[serde(rename = "NamingSystem")]
    NamingSystem,
    #[serde(rename = "NutritionIntake")]
    NutritionIntake,
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    #[serde(rename = "NutritionProduct")]
    NutritionProduct,
    #[serde(rename = "Observation")]
    Observation,
    #[serde(rename = "ObservationDefinition")]
    ObservationDefinition,
    #[serde(rename = "OperationDefinition")]
    OperationDefinition,
    #[serde(rename = "OperationOutcome")]
    OperationOutcome,
    #[serde(rename = "Organization")]
    Organization,
    #[serde(rename = "OrganizationAffiliation")]
    OrganizationAffiliation,
    #[serde(rename = "PackagedProductDefinition")]
    PackagedProductDefinition,
    #[serde(rename = "Parameters")]
    Parameters,
    #[serde(rename = "Patient")]
    Patient,
    #[serde(rename = "PaymentNotice")]
    PaymentNotice,
    #[serde(rename = "PaymentReconciliation")]
    PaymentReconciliation,
    #[serde(rename = "Permission")]
    Permission,
    #[serde(rename = "Person")]
    Person,
    #[serde(rename = "PlanDefinition")]
    PlanDefinition,
    #[serde(rename = "Practitioner")]
    Practitioner,
    #[serde(rename = "PractitionerRole")]
    PractitionerRole,
    #[serde(rename = "Procedure")]
    Procedure,
    #[serde(rename = "Provenance")]
    Provenance,
    #[serde(rename = "Questionnaire")]
    Questionnaire,
    #[serde(rename = "QuestionnaireResponse")]
    QuestionnaireResponse,
    #[serde(rename = "RegulatedAuthorization")]
    RegulatedAuthorization,
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    #[serde(rename = "RequestOrchestration")]
    RequestOrchestration,
    #[serde(rename = "Requirements")]
    Requirements,
    #[serde(rename = "ResearchStudy")]
    ResearchStudy,
    #[serde(rename = "ResearchSubject")]
    ResearchSubject,
    #[serde(rename = "RiskAssessment")]
    RiskAssessment,
    #[serde(rename = "Schedule")]
    Schedule,
    #[serde(rename = "SearchParameter")]
    SearchParameter,
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    #[serde(rename = "Slot")]
    Slot,
    #[serde(rename = "Specimen")]
    Specimen,
    #[serde(rename = "SpecimenDefinition")]
    SpecimenDefinition,
    #[serde(rename = "StructureDefinition")]
    StructureDefinition,
    #[serde(rename = "StructureMap")]
    StructureMap,
    #[serde(rename = "Subscription")]
    Subscription,
    #[serde(rename = "SubscriptionStatus")]
    SubscriptionStatus,
    #[serde(rename = "SubscriptionTopic")]
    SubscriptionTopic,
    #[serde(rename = "Substance")]
    Substance,
    #[serde(rename = "SubstanceDefinition")]
    SubstanceDefinition,
    #[serde(rename = "SubstanceNucleicAcid")]
    SubstanceNucleicAcid,
    #[serde(rename = "SubstancePolymer")]
    SubstancePolymer,
    #[serde(rename = "SubstanceProtein")]
    SubstanceProtein,
    #[serde(rename = "SubstanceReferenceInformation")]
    SubstanceReferenceInformation,
    #[serde(rename = "SubstanceSourceMaterial")]
    SubstanceSourceMaterial,
    #[serde(rename = "SupplyDelivery")]
    SupplyDelivery,
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    #[serde(rename = "Task")]
    Task,
    #[serde(rename = "TerminologyCapabilities")]
    TerminologyCapabilities,
    #[serde(rename = "TestPlan")]
    TestPlan,
    #[serde(rename = "TestReport")]
    TestReport,
    #[serde(rename = "TestScript")]
    TestScript,
    #[serde(rename = "Transport")]
    Transport,
    #[serde(rename = "ValueSet")]
    ValueSet,
    #[serde(rename = "VerificationResult")]
    VerificationResult,
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}
impl Default for ResourceTypes {
    fn default() -> Self {
        Self::Account
    }
}
