use serde::{Deserialize, Serialize};

#[doc = "Patient"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    #[doc = "Logical id of this artifact"]
    pub id: Option<String>,
    #[doc = "A name associated with the individual"]
    pub name: Option<Vec<HumanName>>,
    #[doc = "male | female | other | unknown"]
    pub gender: Option<String>,
    #[serde(rename = "birthDate")]
    #[doc = "The date of birth for the individual"]
    pub birth_date: Option<String>,
    #[doc = "Whether this patient record is in active use"]
    pub active: Option<bool>,
}
