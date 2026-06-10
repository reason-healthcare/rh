use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// RegulatedAuthorization
///
/// Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RegulatedAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatedAuthorization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the authorization, typically assigned by the authorizing body
    pub identifier: Option<Vec<Identifier>>,
    /// The product type, treatment, facility or activity that is being authorized
    pub subject: Option<Vec<Reference>>,
    /// Overall type of this authorization, for example drug marketing approval, orphan drug designation
    ///
    /// Binding: example (Overall type of this authorization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/regulated-authorization-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// General textual supporting information
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The territory in which the authorization has been granted
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub region: Option<Vec<CodeableConcept>>,
    /// The status that is authorised e.g. approved. Intermediate states can be tracked with cases and applications
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// The date at which the current status was assigned
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// The time period in which the regulatory approval etc. is in effect, e.g. a Marketing Authorization includes the date of authorization and/or expiration date
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
    /// Condition for which the use of the regulated product applies
    pub indication: Option<Vec<CodeableReference>>,
    /// The intended use of the product, e.g. prevention, treatment
    ///
    /// Binding: preferred (The overall intended use of a product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-intended-use
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<CodeableConcept>,
    /// The legal/regulatory framework or reasons under which this authorization is granted
    ///
    /// Binding: example (A legal or regulatory framework against which an authorization is granted, or other reasons for it.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/regulated-authorization-basis
    pub basis: Option<Vec<CodeableConcept>>,
    /// The organization that has been granted this authorization, by the regulator
    pub holder: Option<Reference>,
    /// The regulatory authority or authorizing body granting the authorization
    pub regulator: Option<Reference>,
    /// Additional information or supporting documentation about the authorization
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Reference>>,
    /// The case or regulatory procedure for granting or amending a regulated authorization. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)
    pub case: Option<RegulatedAuthorizationCase>,
}
/// RegulatedAuthorization nested structure for the 'case' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatedAuthorizationCase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier by which this case can be referenced
    pub identifier: Option<Identifier>,
    /// The defining type of case
    ///
    /// Binding: example (The type of a case involved in an application.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/regulated-authorization-case-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The status associated with the case
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// Relevant date for this case (Period)
    #[serde(rename = "datePeriod")]
    pub date_period: Option<Period>,
    /// Relevant date for this case (dateTime)
    #[serde(rename = "dateDateTime")]
    pub date_date_time: Option<DateTimeType>,
    /// Applications submitted to obtain a regulated authorization. Steps within the longer running case or procedure
    pub application: Option<Vec<StringType>>,
}

impl Default for RegulatedAuthorization {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            subject: Default::default(),
            type_: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            region: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            validity_period: Default::default(),
            indication: Default::default(),
            intended_use: Default::default(),
            basis: Default::default(),
            holder: Default::default(),
            regulator: Default::default(),
            attached_document: Default::default(),
            case: Default::default(),
        }
    }
}

impl Default for RegulatedAuthorizationCase {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            type_: Default::default(),
            status: Default::default(),
            date_period: Default::default(),
            date_date_time: Default::default(),
            application: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "RegulatedAuthorization.language",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
        )
        .with_description("IETF language tag for a human language")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.contained", 0, None),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.identifier", 0, None),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.subject", 0, None),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.region", 0, None),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.statusDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.validityPeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.indication", 0, None),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.intendedUse",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.basis", 0, None),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.holder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.regulator", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.attachedDocument",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.case", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.case.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.identifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RegulatedAuthorization.case.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.status",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.date[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RegulatedAuthorization.case.application",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RegulatedAuthorization {
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

impl crate::traits::resource::ResourceMutators for RegulatedAuthorization {
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

impl crate::traits::resource::ResourceExistence for RegulatedAuthorization {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RegulatedAuthorization {
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

impl crate::traits::domain_resource::DomainResourceMutators for RegulatedAuthorization {
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

impl crate::traits::domain_resource::DomainResourceExistence for RegulatedAuthorization {
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

impl crate::traits::regulated_authorization::RegulatedAuthorizationAccessors
    for RegulatedAuthorization
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn region(&self) -> &[CodeableConcept] {
        self.region.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn status_date(&self) -> Option<DateTimeType> {
        self.status_date.clone()
    }
    fn validity_period(&self) -> Option<Period> {
        self.validity_period.clone()
    }
    fn indication(&self) -> &[CodeableReference] {
        self.indication.as_deref().unwrap_or(&[])
    }
    fn intended_use(&self) -> Option<CodeableConcept> {
        self.intended_use.clone()
    }
    fn basis(&self) -> &[CodeableConcept] {
        self.basis.as_deref().unwrap_or(&[])
    }
    fn holder(&self) -> Option<Reference> {
        self.holder.clone()
    }
    fn regulator(&self) -> Option<Reference> {
        self.regulator.clone()
    }
    fn attached_document(&self) -> &[Reference] {
        self.attached_document.as_deref().unwrap_or(&[])
    }
    fn case(&self) -> Option<RegulatedAuthorizationCase> {
        self.case.clone()
    }
}

impl crate::traits::regulated_authorization::RegulatedAuthorizationMutators
    for RegulatedAuthorization
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_region(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.region = Some(value);
        resource
    }
    fn add_region(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.region.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_validity_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.validity_period = Some(value);
        resource
    }
    fn set_indication(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.indication = Some(value);
        resource
    }
    fn add_indication(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.indication.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_intended_use(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.intended_use = Some(value);
        resource
    }
    fn set_basis(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.basis = Some(value);
        resource
    }
    fn add_basis(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.basis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_holder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.holder = Some(value);
        resource
    }
    fn set_regulator(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.regulator = Some(value);
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
    fn set_case(self, value: RegulatedAuthorizationCase) -> Self {
        let mut resource = self.clone();
        resource.case = Some(value);
        resource
    }
}

impl crate::traits::regulated_authorization::RegulatedAuthorizationExistence
    for RegulatedAuthorization
{
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_region(&self) -> bool {
        self.region.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_validity_period(&self) -> bool {
        self.validity_period.is_some()
    }
    fn has_indication(&self) -> bool {
        self.indication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_intended_use(&self) -> bool {
        self.intended_use.is_some()
    }
    fn has_basis(&self) -> bool {
        self.basis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_holder(&self) -> bool {
        self.holder.is_some()
    }
    fn has_regulator(&self) -> bool {
        self.regulator.is_some()
    }
    fn has_attached_document(&self) -> bool {
        self.attached_document
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_case(&self) -> bool {
        self.case.is_some()
    }
}

impl crate::validation::ValidatableResource for RegulatedAuthorization {
    fn resource_type(&self) -> &'static str {
        "RegulatedAuthorization"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::regulated_authorization::{
    RegulatedAuthorizationAccessors, RegulatedAuthorizationExistence,
    RegulatedAuthorizationMutators,
};
