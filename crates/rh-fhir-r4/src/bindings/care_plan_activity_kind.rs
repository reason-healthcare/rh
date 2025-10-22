use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/care-plan-activity-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarePlanActivityKind {
    #[serde(rename = "Appointment")]
    Appointment,
    #[serde(rename = "CommunicationRequest")]
    CommunicationRequest,
    #[serde(rename = "DeviceRequest")]
    DeviceRequest,
    #[serde(rename = "MedicationRequest")]
    MedicationRequest,
    #[serde(rename = "NutritionOrder")]
    NutritionOrder,
    #[serde(rename = "Task")]
    Task,
    #[serde(rename = "ServiceRequest")]
    ServiceRequest,
    #[serde(rename = "VisionPrescription")]
    VisionPrescription,
}
impl Default for CarePlanActivityKind {
    fn default() -> Self {
        Self::Appointment
    }
}
