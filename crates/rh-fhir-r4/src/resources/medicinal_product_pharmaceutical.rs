use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductPharmaceutical
///
/// A pharmaceutical product described in terms of its composition and dose form.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPharmaceutical
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPharmaceutical
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPharmaceutical {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// An identifier for the pharmaceutical medicinal product
    pub identifier: Option<Vec<Identifier>>,
    /// The administrable dose form, after necessary reconstitution
    #[serde(rename = "administrableDoseForm")]
    pub administrable_dose_form: CodeableConcept,
    /// Todo
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// Ingredient
    pub ingredient: Option<Vec<Reference>>,
    /// Accompanying device
    pub device: Option<Vec<Reference>>,
    /// Characteristics e.g. a products onset of action
    pub characteristics: Option<Vec<MedicinalProductPharmaceuticalCharacteristics>>,
    /// The path by which the pharmaceutical product is taken into or makes contact with the body
    #[serde(rename = "routeOfAdministration")]
    pub route_of_administration: Vec<MedicinalProductPharmaceuticalRouteofadministration>,
}
/// MedicinalProductPharmaceuticalRouteofadministrationTargetspecies nested structure for the 'withdrawalPeriod' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPharmaceuticalRouteofadministrationTargetspeciesWithdrawalperiod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coded expression for the type of tissue for which the withdrawal period applues, e.g. meat, milk
    pub tissue: CodeableConcept,
    /// A value for the time
    pub value: Quantity,
    /// Extra information about the withdrawal period
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<StringType>,
    /// Extension element for the 'supportingInformation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_supportingInformation")]
    pub _supporting_information: Option<Element>,
}
/// MedicinalProductPharmaceutical nested structure for the 'characteristics' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A coded characteristic
    pub code: CodeableConcept,
    /// The status of characteristic e.g. assigned or pending
    pub status: Option<CodeableConcept>,
}
/// MedicinalProductPharmaceuticalRouteofadministration nested structure for the 'targetSpecies' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPharmaceuticalRouteofadministrationTargetspecies {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coded expression for the species
    pub code: CodeableConcept,
}
/// MedicinalProductPharmaceutical nested structure for the 'routeOfAdministration' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPharmaceuticalRouteofadministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A species for which this route applies
    #[serde(rename = "targetSpecies")]
    pub target_species:
        Option<Vec<MedicinalProductPharmaceuticalRouteofadministrationTargetspecies>>,
    /// Coded expression for the route
    pub code: CodeableConcept,
    /// The first dose (dose quantity) administered in humans can be specified, for a product under investigation, using a numerical value and its unit of measurement
    #[serde(rename = "firstDose")]
    pub first_dose: Option<Quantity>,
    /// The maximum single dose that can be administered as per the protocol of a clinical trial can be specified using a numerical value and its unit of measurement
    #[serde(rename = "maxSingleDose")]
    pub max_single_dose: Option<Quantity>,
    /// The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxDosePerDay")]
    pub max_dose_per_day: Option<Quantity>,
    /// The maximum dose per treatment period that can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxDosePerTreatmentPeriod")]
    pub max_dose_per_treatment_period: Option<Ratio>,
    /// The maximum treatment period during which an Investigational Medicinal Product can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxTreatmentPeriod")]
    pub max_treatment_period: Option<Duration>,
}

impl Default for MedicinalProductPharmaceutical {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            administrable_dose_form: CodeableConcept::default(),
            unit_of_presentation: Default::default(),
            ingredient: Default::default(),
            device: Default::default(),
            characteristics: Default::default(),
            route_of_administration: Vec::new(),
        }
    }
}

impl Default for MedicinalProductPharmaceuticalRouteofadministrationTargetspeciesWithdrawalperiod {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            tissue: Default::default(),
            value: Default::default(),
            supporting_information: Default::default(),
            _supporting_information: Default::default(),
        }
    }
}

impl Default for MedicinalProductPharmaceuticalCharacteristics {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            status: Default::default(),
        }
    }
}

impl Default for MedicinalProductPharmaceuticalRouteofadministrationTargetspecies {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
        }
    }
}

impl Default for MedicinalProductPharmaceuticalRouteofadministration {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target_species: Default::default(),
            code: Default::default(),
            first_dose: Default::default(),
            max_single_dose: Default::default(),
            max_dose_per_day: Default::default(),
            max_dose_per_treatment_period: Default::default(),
            max_treatment_period: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductPharmaceutical {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductPharmaceutical {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductPharmaceutical {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductPharmaceutical {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductPharmaceutical {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductPharmaceutical {
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

impl crate::traits::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalAccessors
    for MedicinalProductPharmaceutical
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn administrable_dose_form(&self) -> CodeableConcept {
        self.administrable_dose_form.clone()
    }
    fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        self.unit_of_presentation.clone()
    }
    fn ingredient(&self) -> &[Reference] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn device(&self) -> &[Reference] {
        self.device.as_deref().unwrap_or(&[])
    }
    fn characteristics(&self) -> &[MedicinalProductPharmaceuticalCharacteristics] {
        self.characteristics.as_deref().unwrap_or(&[])
    }
    fn route_of_administration(&self) -> &[MedicinalProductPharmaceuticalRouteofadministration] {
        &self.route_of_administration
    }
}

impl crate::traits::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalMutators
    for MedicinalProductPharmaceutical
{
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_administrable_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.administrable_dose_form = value;
        resource
    }
    fn set_unit_of_presentation(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.unit_of_presentation = Some(value);
        resource
    }
    fn set_ingredient(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_device(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn add_device(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.device.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_characteristics(
        self,
        value: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    ) -> Self {
        let mut resource = self.clone();
        resource.characteristics = Some(value);
        resource
    }
    fn add_characteristics(self, item: MedicinalProductPharmaceuticalCharacteristics) -> Self {
        let mut resource = self.clone();
        resource
            .characteristics
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_route_of_administration(
        self,
        value: Vec<MedicinalProductPharmaceuticalRouteofadministration>,
    ) -> Self {
        let mut resource = self.clone();
        resource.route_of_administration = value;
        resource
    }
    fn add_route_of_administration(
        self,
        item: MedicinalProductPharmaceuticalRouteofadministration,
    ) -> Self {
        let mut resource = self.clone();
        resource.route_of_administration.push(item);
        resource
    }
}

impl crate::traits::medicinal_product_pharmaceutical::MedicinalProductPharmaceuticalExistence
    for MedicinalProductPharmaceutical
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_administrable_dose_form(&self) -> bool {
        true
    }
    fn has_unit_of_presentation(&self) -> bool {
        self.unit_of_presentation.is_some()
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_device(&self) -> bool {
        self.device.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characteristics(&self) -> bool {
        self.characteristics.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_route_of_administration(&self) -> bool {
        !self.route_of_administration.is_empty()
    }
}
