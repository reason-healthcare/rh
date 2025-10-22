use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/repository-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepositoryType {
    /// Click and see
    #[serde(rename = "directlink")]
    Directlink,
    /// The URL is the RESTful or other kind of API that can access to the result.
    #[serde(rename = "openapi")]
    Openapi,
    /// Result cannot be access unless an account is logged in
    #[serde(rename = "login")]
    Login,
    /// Result need to be fetched with API and need LOGIN( or cookies are required when visiting the link of resource)
    #[serde(rename = "oauth")]
    Oauth,
    /// Some other complicated or particular way to get resource from URL.
    #[serde(rename = "other")]
    Other,
}
impl Default for RepositoryType {
    fn default() -> Self {
        Self::Directlink
    }
}
