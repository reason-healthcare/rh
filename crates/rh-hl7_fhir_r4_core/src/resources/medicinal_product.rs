use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProduct
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProduct {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this product. Could be an MPID
    pub identifier: Option<Vec<Identifier>>,
    /// Regulatory type, e.g. Investigational or Authorized
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// If this medicine applies to human or veterinary uses
    pub domain: Option<Coding>,
    /// The dose form for a single part product, or combined form of a multiple part product
    #[serde(rename = "combinedPharmaceuticalDoseForm")]
    pub combined_pharmaceutical_dose_form: Option<CodeableConcept>,
    /// The legal status of supply of the medicinal product as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    #[serde(rename = "additionalMonitoringIndicator")]
    pub additional_monitoring_indicator: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    #[serde(rename = "specialMeasures")]
    pub special_measures: Option<Vec<StringType>>,
    /// Extension element for the 'specialMeasures' primitive field. Contains metadata and extensions.
    #[serde(rename = "_specialMeasures")]
    pub _special_measures: Option<Element>,
    /// If authorised for use in children
    #[serde(rename = "paediatricUseIndicator")]
    pub paediatric_use_indicator: Option<CodeableConcept>,
    /// Allows the product to be classified by various systems
    #[serde(rename = "productClassification")]
    pub product_classification: Option<Vec<CodeableConcept>>,
    /// Marketing status of the medicinal product, in contrast to marketing authorizaton
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// Pharmaceutical aspects of product
    #[serde(rename = "pharmaceuticalProduct")]
    pub pharmaceutical_product: Option<Vec<Reference>>,
    /// Package representation for the product
    #[serde(rename = "packagedMedicinalProduct")]
    pub packaged_medicinal_product: Option<Vec<Reference>>,
    /// Supporting documentation, typically for regulatory submission
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Reference>>,
    /// A master file for to the medicinal product (e.g. Pharmacovigilance System Master File)
    #[serde(rename = "masterFile")]
    pub master_file: Option<Vec<Reference>>,
    /// A product specific contact, person (in a role), or an organization
    pub contact: Option<Vec<Reference>>,
    /// Clinical trials or studies that this product is involved in
    #[serde(rename = "clinicalTrial")]
    pub clinical_trial: Option<Vec<Reference>>,
    /// The product's name, including full name and possibly coded parts
    pub name: Vec<MedicinalProductName>,
    /// Reference to another product, e.g. for linking authorised to investigational product
    #[serde(rename = "crossReference")]
    pub cross_reference: Option<Vec<Identifier>>,
    /// An operation applied to the product, for manufacturing or adminsitrative purpose
    #[serde(rename = "manufacturingBusinessOperation")]
    pub manufacturing_business_operation:
        Option<Vec<MedicinalProductManufacturingbusinessoperation>>,
    /// Indicates if the medicinal product has an orphan designation for the treatment of a rare disease
    #[serde(rename = "specialDesignation")]
    pub special_designation: Option<Vec<MedicinalProductSpecialdesignation>>,
}
/// MedicinalProduct nested structure for the 'specialDesignation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductSpecialdesignation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier for the designation, or procedure number
    pub identifier: Option<Vec<Identifier>>,
    /// The type of special designation, e.g. orphan drug, minor use
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The intended use of the product, e.g. prevention, treatment
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<CodeableConcept>,
    /// Condition for which the medicinal use applies (CodeableConcept)
    #[serde(rename = "indicationCodeableConcept")]
    pub indication_codeable_concept: Option<CodeableConcept>,
    /// Condition for which the medicinal use applies (Reference)
    #[serde(rename = "indicationReference")]
    pub indication_reference: Option<Reference>,
    /// For example granted, pending, expired or withdrawn
    pub status: Option<CodeableConcept>,
    /// Date when the designation was granted
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Animal species for which this applies
    pub species: Option<CodeableConcept>,
}
/// MedicinalProduct nested structure for the 'manufacturingBusinessOperation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductManufacturingbusinessoperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of manufacturing operation
    #[serde(rename = "operationType")]
    pub operation_type: Option<CodeableConcept>,
    /// Regulatory authorization reference number
    #[serde(rename = "authorisationReferenceNumber")]
    pub authorisation_reference_number: Option<Identifier>,
    /// Regulatory authorization date
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<DateTimeType>,
    /// Extension element for the 'effectiveDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_effectiveDate")]
    pub _effective_date: Option<Element>,
    /// To indicate if this proces is commercially confidential
    #[serde(rename = "confidentialityIndicator")]
    pub confidentiality_indicator: Option<CodeableConcept>,
    /// The manufacturer or establishment associated with the process
    pub manufacturer: Option<Vec<Reference>>,
    /// A regulator which oversees the operation
    pub regulator: Option<Reference>,
}
/// MedicinalProduct nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Country where the name applies
    #[serde(rename = "countryLanguage")]
    pub country_language: Option<Vec<MedicinalProductNameCountrylanguage>>,
    /// Coding words or phrases of the name
    #[serde(rename = "namePart")]
    pub name_part: Option<Vec<MedicinalProductNameNamepart>>,
    /// The full product name
    #[serde(rename = "productName")]
    pub product_name: StringType,
    /// Extension element for the 'productName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productName")]
    pub _product_name: Option<Element>,
}
/// MedicinalProductName nested structure for the 'countryLanguage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductNameCountrylanguage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Country code for where this name applies
    pub country: CodeableConcept,
    /// Jurisdiction code for where this name applies
    pub jurisdiction: Option<CodeableConcept>,
    /// Language code for this name
    pub language: CodeableConcept,
}
/// MedicinalProductName nested structure for the 'namePart' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductNameNamepart {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A fragment of a product name
    pub part: StringType,
    /// Extension element for the 'part' primitive field. Contains metadata and extensions.
    pub _part: Option<Element>,
    /// Idenifying type for this part of the name (e.g. strength part)
    #[serde(rename = "type")]
    pub type_: Coding,
}

impl Default for MedicinalProduct {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            domain: Default::default(),
            combined_pharmaceutical_dose_form: Default::default(),
            legal_status_of_supply: Default::default(),
            additional_monitoring_indicator: Default::default(),
            special_measures: Default::default(),
            _special_measures: Default::default(),
            paediatric_use_indicator: Default::default(),
            product_classification: Default::default(),
            marketing_status: Default::default(),
            pharmaceutical_product: Default::default(),
            packaged_medicinal_product: Default::default(),
            attached_document: Default::default(),
            master_file: Default::default(),
            contact: Default::default(),
            clinical_trial: Default::default(),
            name: Vec::new(),
            cross_reference: Default::default(),
            manufacturing_business_operation: Default::default(),
            special_designation: Default::default(),
        }
    }
}

impl Default for MedicinalProductSpecialdesignation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            type_: Default::default(),
            intended_use: Default::default(),
            indication_codeable_concept: Default::default(),
            indication_reference: Default::default(),
            status: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            species: Default::default(),
        }
    }
}

impl Default for MedicinalProductManufacturingbusinessoperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            operation_type: Default::default(),
            authorisation_reference_number: Default::default(),
            effective_date: Default::default(),
            _effective_date: Default::default(),
            confidentiality_indicator: Default::default(),
            manufacturer: Default::default(),
            regulator: Default::default(),
        }
    }
}

impl Default for MedicinalProductName {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            country_language: Default::default(),
            name_part: Default::default(),
            product_name: StringType::default(),
            _product_name: Default::default(),
        }
    }
}

impl Default for MedicinalProductNameCountrylanguage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            country: Default::default(),
            jurisdiction: Default::default(),
            language: Default::default(),
        }
    }
}

impl Default for MedicinalProductNameNamepart {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            part: Default::default(),
            _part: Default::default(),
            type_: Default::default(),
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProduct {
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

impl crate::traits::resource::ResourceMutators for MedicinalProduct {
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

impl crate::traits::resource::ResourceExistence for MedicinalProduct {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProduct {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProduct {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProduct {
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

impl crate::traits::medicinal_product::MedicinalProductAccessors for MedicinalProduct {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn domain(&self) -> Option<Coding> {
        self.domain.clone()
    }
    fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept> {
        self.combined_pharmaceutical_dose_form.clone()
    }
    fn legal_status_of_supply(&self) -> Option<CodeableConcept> {
        self.legal_status_of_supply.clone()
    }
    fn additional_monitoring_indicator(&self) -> Option<CodeableConcept> {
        self.additional_monitoring_indicator.clone()
    }
    fn special_measures(&self) -> &[StringType] {
        self.special_measures.as_deref().unwrap_or(&[])
    }
    fn paediatric_use_indicator(&self) -> Option<CodeableConcept> {
        self.paediatric_use_indicator.clone()
    }
    fn product_classification(&self) -> &[CodeableConcept] {
        self.product_classification.as_deref().unwrap_or(&[])
    }
    fn marketing_status(&self) -> &[MarketingStatus] {
        self.marketing_status.as_deref().unwrap_or(&[])
    }
    fn pharmaceutical_product(&self) -> &[Reference] {
        self.pharmaceutical_product.as_deref().unwrap_or(&[])
    }
    fn packaged_medicinal_product(&self) -> &[Reference] {
        self.packaged_medicinal_product.as_deref().unwrap_or(&[])
    }
    fn attached_document(&self) -> &[Reference] {
        self.attached_document.as_deref().unwrap_or(&[])
    }
    fn master_file(&self) -> &[Reference] {
        self.master_file.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[Reference] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn clinical_trial(&self) -> &[Reference] {
        self.clinical_trial.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[MedicinalProductName] {
        &self.name
    }
    fn cross_reference(&self) -> &[Identifier] {
        self.cross_reference.as_deref().unwrap_or(&[])
    }
    fn manufacturing_business_operation(
        &self,
    ) -> &[MedicinalProductManufacturingbusinessoperation] {
        self.manufacturing_business_operation
            .as_deref()
            .unwrap_or(&[])
    }
    fn special_designation(&self) -> &[MedicinalProductSpecialdesignation] {
        self.special_designation.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medicinal_product::MedicinalProductMutators for MedicinalProduct {
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_domain(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.domain = Some(value);
        resource
    }
    fn set_combined_pharmaceutical_dose_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.combined_pharmaceutical_dose_form = Some(value);
        resource
    }
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.legal_status_of_supply = Some(value);
        resource
    }
    fn set_additional_monitoring_indicator(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.additional_monitoring_indicator = Some(value);
        resource
    }
    fn set_special_measures(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.special_measures = Some(value);
        resource
    }
    fn add_special_measures(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .special_measures
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_paediatric_use_indicator(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.paediatric_use_indicator = Some(value);
        resource
    }
    fn set_product_classification(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.product_classification = Some(value);
        resource
    }
    fn add_product_classification(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .product_classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self {
        let mut resource = self.clone();
        resource.marketing_status = Some(value);
        resource
    }
    fn add_marketing_status(self, item: MarketingStatus) -> Self {
        let mut resource = self.clone();
        resource
            .marketing_status
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_pharmaceutical_product(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.pharmaceutical_product = Some(value);
        resource
    }
    fn add_pharmaceutical_product(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .pharmaceutical_product
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_packaged_medicinal_product(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.packaged_medicinal_product = Some(value);
        resource
    }
    fn add_packaged_medicinal_product(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .packaged_medicinal_product
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_attached_document(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.attached_document = Some(value);
        resource
    }
    fn add_attached_document(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .attached_document
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_master_file(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.master_file = Some(value);
        resource
    }
    fn add_master_file(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.master_file.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_contact(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_clinical_trial(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.clinical_trial = Some(value);
        resource
    }
    fn add_clinical_trial(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .clinical_trial
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_name(self, value: Vec<MedicinalProductName>) -> Self {
        let mut resource = self.clone();
        resource.name = value;
        resource
    }
    fn add_name(self, item: MedicinalProductName) -> Self {
        let mut resource = self.clone();
        resource.name.push(item);
        resource
    }
    fn set_cross_reference(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.cross_reference = Some(value);
        resource
    }
    fn add_cross_reference(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource
            .cross_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_manufacturing_business_operation(
        self,
        value: Vec<MedicinalProductManufacturingbusinessoperation>,
    ) -> Self {
        let mut resource = self.clone();
        resource.manufacturing_business_operation = Some(value);
        resource
    }
    fn add_manufacturing_business_operation(
        self,
        item: MedicinalProductManufacturingbusinessoperation,
    ) -> Self {
        let mut resource = self.clone();
        resource
            .manufacturing_business_operation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_special_designation(self, value: Vec<MedicinalProductSpecialdesignation>) -> Self {
        let mut resource = self.clone();
        resource.special_designation = Some(value);
        resource
    }
    fn add_special_designation(self, item: MedicinalProductSpecialdesignation) -> Self {
        let mut resource = self.clone();
        resource
            .special_designation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medicinal_product::MedicinalProductExistence for MedicinalProduct {
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
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_domain(&self) -> bool {
        self.domain.is_some()
    }
    fn has_combined_pharmaceutical_dose_form(&self) -> bool {
        self.combined_pharmaceutical_dose_form.is_some()
    }
    fn has_legal_status_of_supply(&self) -> bool {
        self.legal_status_of_supply.is_some()
    }
    fn has_additional_monitoring_indicator(&self) -> bool {
        self.additional_monitoring_indicator.is_some()
    }
    fn has_special_measures(&self) -> bool {
        self.special_measures
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_paediatric_use_indicator(&self) -> bool {
        self.paediatric_use_indicator.is_some()
    }
    fn has_product_classification(&self) -> bool {
        self.product_classification
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_marketing_status(&self) -> bool {
        self.marketing_status
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_pharmaceutical_product(&self) -> bool {
        self.pharmaceutical_product
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_packaged_medicinal_product(&self) -> bool {
        self.packaged_medicinal_product
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_attached_document(&self) -> bool {
        self.attached_document
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_master_file(&self) -> bool {
        self.master_file.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_clinical_trial(&self) -> bool {
        self.clinical_trial.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        !self.name.is_empty()
    }
    fn has_cross_reference(&self) -> bool {
        self.cross_reference.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_manufacturing_business_operation(&self) -> bool {
        self.manufacturing_business_operation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_special_designation(&self) -> bool {
        self.special_designation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicinalProduct {
    fn resource_type(&self) -> &'static str {
        "MedicinalProduct"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/MedicinalProduct")
    }
}
