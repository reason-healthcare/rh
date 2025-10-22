use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/note-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoteType {
    /// Display
    #[serde(rename = "display")]
    Display,
    /// Print (Form)
    #[serde(rename = "print")]
    Print,
    /// Print (Operator)
    #[serde(rename = "printoper")]
    Printoper,
}
impl Default for NoteType {
    fn default() -> Self {
        Self::Display
    }
}
