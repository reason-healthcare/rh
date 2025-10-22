use crate::datatypes::extension::Extension;
use crate::resources::observation::Observation;
use serde::{Deserialize, Serialize};
/// Observation-genetics
///
/// Describes how the observation resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObservationGenetics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Observation,
}
/// DNARegionName
///
/// A human readable name for the region of interest. Typically Exon #, Intron # or other. NOTE: This is not standardized and is mainly for convenience and display purposes.  LOINC Code: ([47999-8](http://loinc.org/47999-8)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsDNARegionName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsDNARegionName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Ancestry
///
/// Ancestry information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAncestry
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAncestry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Allele
///
/// Allele information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAllele
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAllele {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// AminoAcidChange
///
/// AminoAcidChange information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAminoAcidChange
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAminoAcidChange {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Interpretation
///
/// Clinical Interpretations for variant. It's a reference to an Observation resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsInterpretation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsInterpretation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ObservationGenetics {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for ObservationGenetics {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ObservationGenetics {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::observation_genetics::ObservationGeneticsMutators for ObservationGenetics {
    fn new() -> Self {
        Self::default()
    }
}
