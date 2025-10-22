use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::population::Population;
use crate::datatypes::reference::Reference;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductUndesirableEffect
///
/// Describe the undesirable effects of the medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductUndesirableEffect
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductUndesirableEffect
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductUndesirableEffect {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The medication for which this is an indication
    pub subject: Option<Vec<Reference>>,
    /// The symptom, condition or undesirable effect
    #[serde(rename = "symptomConditionEffect")]
    pub symptom_condition_effect: Option<CodeableConcept>,
    /// Classification of the effect
    pub classification: Option<CodeableConcept>,
    /// The frequency of occurrence of the effect
    #[serde(rename = "frequencyOfOccurrence")]
    pub frequency_of_occurrence: Option<CodeableConcept>,
    /// The population group to which this applies
    pub population: Option<Vec<Population>>,
}

impl Default for MedicinalProductUndesirableEffect {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            subject: Default::default(),
            symptom_condition_effect: Default::default(),
            classification: Default::default(),
            frequency_of_occurrence: Default::default(),
            population: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductUndesirableEffect {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductUndesirableEffect {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductUndesirableEffect {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductUndesirableEffect {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductUndesirableEffect {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductUndesirableEffect {
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

impl crate::traits::medicinal_product_undesirable_effect::MedicinalProductUndesirableEffectAccessors
    for MedicinalProductUndesirableEffect
{
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn symptom_condition_effect(&self) -> Option<CodeableConcept> {
        self.symptom_condition_effect.clone()
    }
    fn classification(&self) -> Option<CodeableConcept> {
        self.classification.clone()
    }
    fn frequency_of_occurrence(&self) -> Option<CodeableConcept> {
        self.frequency_of_occurrence.clone()
    }
    fn population(&self) -> &[Population] {
        self.population.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medicinal_product_undesirable_effect::MedicinalProductUndesirableEffectMutators
    for MedicinalProductUndesirableEffect
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
    fn set_symptom_condition_effect(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.symptom_condition_effect = Some(value);
        resource
    }
    fn set_classification(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn set_frequency_of_occurrence(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.frequency_of_occurrence = Some(value);
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

impl crate::traits::medicinal_product_undesirable_effect::MedicinalProductUndesirableEffectExistence
    for MedicinalProductUndesirableEffect
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
    fn has_symptom_condition_effect(&self) -> bool {
        self.symptom_condition_effect.is_some()
    }
    fn has_classification(&self) -> bool {
        self.classification.is_some()
    }
    fn has_frequency_of_occurrence(&self) -> bool {
        self.frequency_of_occurrence.is_some()
    }
    fn has_population(&self) -> bool {
        self.population.as_ref().is_some_and(|v| !v.is_empty())
    }
}
