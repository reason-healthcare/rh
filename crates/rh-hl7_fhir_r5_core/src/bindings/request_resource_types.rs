use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/request-resource-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestResourceTypes {
    #[serde(rename = "Appointment")]
    Appointment,
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    #[serde(rename = "CarePlan")]
    CarePlan,
    #[serde(rename = "Claim")]
    Claim,
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    #[serde(rename = "CoverageEligibilityRequest")]
    CoverageEligibilityRequest,
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    #[serde(rename = "RequestOrchestration")]
    RequestOrchestration,
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    #[serde(rename = "Task")]
    Task,
    #[serde(rename = "Transport")]
    Transport,
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}
impl Default for RequestResourceTypes {
    fn default() -> Self {
        Self::Appointment
    }
}
