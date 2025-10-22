use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::population::Population;
use crate::datatypes::reference::Reference;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductContraindication
///
/// The clinical particulars - indications, contraindications etc. of a medicinal product, including for regulatory purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductContraindication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductContraindication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductContraindication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The medication for which this is an indication
    pub subject: Option<Vec<Reference>>,
    /// The disease, symptom or procedure for the contraindication
    pub disease: Option<CodeableConcept>,
    /// The status of the disease or symptom for the contraindication
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableConcept>,
    /// A comorbidity (concurrent condition) or coinfection
    pub comorbidity: Option<Vec<CodeableConcept>>,
    /// Information about the use of the medicinal product in relation to other therapies as part of the indication
    #[serde(rename = "therapeuticIndication")]
    pub therapeutic_indication: Option<Vec<Reference>>,
    /// Information about the use of the medicinal product in relation to other therapies described as part of the indication
    #[serde(rename = "otherTherapy")]
    pub other_therapy: Option<Vec<MedicinalProductContraindicationOthertherapy>>,
    /// The population group to which this applies
    pub population: Option<Vec<Population>>,
}
/// MedicinalProductContraindication nested structure for the 'otherTherapy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductContraindicationOthertherapy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of relationship between the medicinal product indication or contraindication and another therapy
    #[serde(rename = "therapyRelationshipType")]
    pub therapy_relationship_type: CodeableConcept,
    /// Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication (CodeableConcept)
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,
    /// Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication (Reference)
    #[serde(rename = "medicationReference")]
    pub medication_reference: Reference,
}

impl Default for MedicinalProductContraindication {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            subject: Default::default(),
            disease: Default::default(),
            disease_status: Default::default(),
            comorbidity: Default::default(),
            therapeutic_indication: Default::default(),
            other_therapy: Default::default(),
            population: Default::default(),
        }
    }
}

impl Default for MedicinalProductContraindicationOthertherapy {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            therapy_relationship_type: Default::default(),
            medication_codeable_concept: Default::default(),
            medication_reference: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductContraindication {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for MedicinalProductContraindication {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for MedicinalProductContraindication {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductContraindication {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductContraindication {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductContraindication {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::medicinal_product_contraindication::MedicinalProductContraindicationAccessors
    for MedicinalProductContraindication
{
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn disease(&self) -> Option<CodeableConcept> {
        self.disease.clone()
    }
    fn disease_status(&self) -> Option<CodeableConcept> {
        self.disease_status.clone()
    }
    fn comorbidity(&self) -> &[CodeableConcept] {
        self.comorbidity.as_deref().unwrap_or(&[])
    }
    fn therapeutic_indication(&self) -> &[Reference] {
        self.therapeutic_indication.as_deref().unwrap_or(&[])
    }
    fn other_therapy(&self) -> &[MedicinalProductContraindicationOthertherapy] {
        self.other_therapy.as_deref().unwrap_or(&[])
    }
    fn population(&self) -> &[Population] {
        self.population.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medicinal_product_contraindication::MedicinalProductContraindicationMutators
    for MedicinalProductContraindication
{
    fn new() -> Self {
        Self::default()
    }
    fn set_subject(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn add_subject(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_disease(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.disease = Some(value);
        resource
    }
    fn set_disease_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.disease_status = Some(value);
        resource
    }
    fn set_comorbidity(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.comorbidity = Some(value);
        resource
    }
    fn add_comorbidity(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.comorbidity.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_therapeutic_indication(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.therapeutic_indication = Some(value);
        resource
    }
    fn add_therapeutic_indication(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .therapeutic_indication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_other_therapy(self, value: Vec<MedicinalProductContraindicationOthertherapy>) -> Self {
        let mut resource = self.clone();
        resource.other_therapy = Some(value);
        resource
    }
    fn add_other_therapy(self, item: MedicinalProductContraindicationOthertherapy) -> Self {
        let mut resource = self.clone();
        resource
            .other_therapy
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_population(self, value: Vec<Population>) -> Self {
        let mut resource = self.clone();
        resource.population = Some(value);
        resource
    }
    fn add_population(self, item: Population) -> Self {
        let mut resource = self.clone();
        resource.population.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::medicinal_product_contraindication::MedicinalProductContraindicationExistence
    for MedicinalProductContraindication
{
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_disease(&self) -> bool {
        self.disease.is_some()
    }
    fn has_disease_status(&self) -> bool {
        self.disease_status.is_some()
    }
    fn has_comorbidity(&self) -> bool {
        self.comorbidity.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_therapeutic_indication(&self) -> bool {
        self.therapeutic_indication
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_other_therapy(&self) -> bool {
        self.other_therapy.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_population(&self) -> bool {
        self.population.as_ref().is_some_and(|v| !v.is_empty())
    }
}
