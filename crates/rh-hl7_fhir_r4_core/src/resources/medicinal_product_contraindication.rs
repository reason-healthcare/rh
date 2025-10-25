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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.meta",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.text",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.contained",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.subject",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.disease",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.diseaseStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.comorbidity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.therapeuticIndication",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy.therapyRelationshipType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.otherTherapy.medication[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicinalProductContraindication.population",
                0,
                None,
            ),
        ]
    });

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

impl crate::validation::ValidatableResource for MedicinalProductContraindication {
    fn resource_type(&self) -> &'static str {
        "MedicinalProductContraindication"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/MedicinalProductContraindication")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medicinal_product_contraindication::{
    MedicinalProductContraindicationAccessors, MedicinalProductContraindicationExistence,
    MedicinalProductContraindicationMutators,
};
