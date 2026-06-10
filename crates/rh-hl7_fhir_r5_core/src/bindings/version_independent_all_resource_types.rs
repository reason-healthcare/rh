use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/version-independent-all-resource-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionIndependentAllResourceTypes {
    /// BodySite
    #[serde(rename = "BodySite")]
    BodySite,
    /// CatalogEntry
    #[serde(rename = "CatalogEntry")]
    CatalogEntry,
    /// Conformance
    #[serde(rename = "Conformance")]
    Conformance,
    /// DataElement
    #[serde(rename = "DataElement")]
    DataElement,
    /// DeviceComponent
    #[serde(rename = "DeviceComponent")]
    DeviceComponent,
    /// DeviceUseRequest
    #[serde(rename = "DeviceUseRequest")]
    DeviceUseRequest,
    /// DeviceUseStatement
    #[serde(rename = "DeviceUseStatement")]
    DeviceUseStatement,
    /// DiagnosticOrder
    #[serde(rename = "DiagnosticOrder")]
    DiagnosticOrder,
    /// DocumentManifest
    #[serde(rename = "DocumentManifest")]
    DocumentManifest,
    /// EffectEvidenceSynthesis
    #[serde(rename = "EffectEvidenceSynthesis")]
    EffectEvidenceSynthesis,
    /// EligibilityRequest
    #[serde(rename = "EligibilityRequest")]
    EligibilityRequest,
    /// EligibilityResponse
    #[serde(rename = "EligibilityResponse")]
    EligibilityResponse,
    /// ExpansionProfile
    #[serde(rename = "ExpansionProfile")]
    ExpansionProfile,
    /// ImagingManifest
    #[serde(rename = "ImagingManifest")]
    ImagingManifest,
    /// ImagingObjectSelection
    #[serde(rename = "ImagingObjectSelection")]
    ImagingObjectSelection,
    /// Media
    #[serde(rename = "Media")]
    Media,
    /// MedicationOrder
    #[serde(rename = "MedicationOrder")]
    MedicationOrder,
    /// MedicationUsage
    #[serde(rename = "MedicationUsage")]
    MedicationUsage,
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
    /// Order
    #[serde(rename = "Order")]
    Order,
    /// OrderResponse
    #[serde(rename = "OrderResponse")]
    OrderResponse,
    /// ProcedureRequest
    #[serde(rename = "ProcedureRequest")]
    ProcedureRequest,
    /// ProcessRequest
    #[serde(rename = "ProcessRequest")]
    ProcessRequest,
    /// ProcessResponse
    #[serde(rename = "ProcessResponse")]
    ProcessResponse,
    /// ReferralRequest
    #[serde(rename = "ReferralRequest")]
    ReferralRequest,
    /// RequestGroup
    #[serde(rename = "RequestGroup")]
    RequestGroup,
    /// ResearchDefinition
    #[serde(rename = "ResearchDefinition")]
    ResearchDefinition,
    /// ResearchElementDefinition
    #[serde(rename = "ResearchElementDefinition")]
    ResearchElementDefinition,
    /// RiskEvidenceSynthesis
    #[serde(rename = "RiskEvidenceSynthesis")]
    RiskEvidenceSynthesis,
    /// Sequence
    #[serde(rename = "Sequence")]
    Sequence,
    /// ServiceDefinition
    #[serde(rename = "ServiceDefinition")]
    ServiceDefinition,
    /// SubstanceSpecification
    #[serde(rename = "SubstanceSpecification")]
    SubstanceSpecification,
}
impl Default for VersionIndependentAllResourceTypes {
    fn default() -> Self {
        Self::BodySite
    }
}
