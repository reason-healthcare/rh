use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Secondary Finding
///
/// Secondary findings are genetic test results that provide information about variants in a gene unrelated to the primary purpose for the testing, most often discovered when [Whole Exome Sequencing (WES)](https://en.wikipedia.org/wiki/Exome_sequencing) or [Whole Genome Sequencing (WGS)](https://en.wikipedia.org/wiki/Whole_genome_sequencing) is performed. This extension should be used to denote when a genetic finding is being shared as a secondary finding, and ideally refer to a corresponding guideline or policy statement.  For more detail, please see: https://ghr.nlm.nih.gov/primer/testing/secondaryfindings.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-secondaryFinding
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSecondaryFinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationSecondaryFinding {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), 'value')])"),
        ]
    });

impl crate::validation::ValidatableResource for ObservationSecondaryFinding {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/observation-secondaryFinding")
    }
}
