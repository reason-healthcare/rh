use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/product-category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductCategory {
    /// Organ
    #[serde(rename = "organ")]
    Organ,
    /// Tissue
    #[serde(rename = "tissue")]
    Tissue,
    /// Fluid
    #[serde(rename = "fluid")]
    Fluid,
    /// Cells
    #[serde(rename = "cells")]
    Cells,
    /// BiologicalAgent
    #[serde(rename = "biologicalAgent")]
    BiologicalAgent,
}
impl Default for ProductCategory {
    fn default() -> Self {
        Self::Organ
    }
}
