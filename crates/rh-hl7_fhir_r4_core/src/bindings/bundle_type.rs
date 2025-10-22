use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/bundle-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BundleType {
    /// Document
    #[serde(rename = "document")]
    Document,
    /// Message
    #[serde(rename = "message")]
    Message,
    /// Transaction
    #[serde(rename = "transaction")]
    Transaction,
    /// Transaction Response
    #[serde(rename = "transaction-response")]
    TransactionResponse,
    /// Batch
    #[serde(rename = "batch")]
    Batch,
    /// Batch Response
    #[serde(rename = "batch-response")]
    BatchResponse,
    /// History List
    #[serde(rename = "history")]
    History,
    /// Search Results
    #[serde(rename = "searchset")]
    Searchset,
    /// Collection
    #[serde(rename = "collection")]
    Collection,
}
impl Default for BundleType {
    fn default() -> Self {
        Self::Document
    }
}
