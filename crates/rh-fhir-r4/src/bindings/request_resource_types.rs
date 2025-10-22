use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/request-resource-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestResourceTypes {
    /// Appointment
    #[serde(rename = "Appointment")]
    Appointment,
    /// AppointmentResponse
    #[serde(rename = "AppointmentResponse")]
    AppointmentResponse,
    /// CarePlan
    #[serde(rename = "CarePlan")]
    CarePlan,
    /// Claim
    #[serde(rename = "Claim")]
    Claim,
    /// CommunicationRequest
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    /// Contract
    #[serde(rename = "Contract")]
    Contract,
    /// DeviceRequest
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    /// EnrollmentRequest
    #[serde(rename = "EnrollmentRequest")]
    EnrollmentRequest,
    /// ImmunizationRecommendation
    #[serde(rename = "ImmunizationRecommendation")]
    ImmunizationRecommendation,
    /// MedicationRequest
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    /// NutritionOrder
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    /// ServiceRequest
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    /// SupplyRequest
    #[serde(rename = "SupplyRequest")]
    SupplyRequest,
    /// Task
    #[serde(rename = "Task")]
    Task,
    /// VisionPrescription
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}
impl Default for RequestResourceTypes {
    fn default() -> Self {
        Self::Appointment
    }
}
