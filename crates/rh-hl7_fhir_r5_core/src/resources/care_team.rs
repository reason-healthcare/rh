use crate::bindings::care_team_status::CareTeamStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CareTeam
///
/// The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CareTeam
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: CareTeam
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareTeam {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this team
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | active | suspended | inactive | entered-in-error
    pub status: Option<CareTeamStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Type of team
    ///
    /// Binding: example (Indicates the type of care team.)
    ///
    /// Available values:
    /// - `LA27975-4`: Event-focused care team
    /// - `LA27976-2`: Encounter-focused care team
    /// - `LA27977-0`: Episode of care-focused care team
    /// - `LA27978-8`: Condition-focused care team
    /// - `LA28865-6`: Longitudinal care-coordination focused care team
    /// - `LA28866-4`: Home & Community Based Services (HCBS)-focused care team
    /// - `LA27980-4`: Clinical research-focused care team
    /// - `LA28867-2`: Public health-focused care team
    pub category: Option<Vec<CodeableConcept>>,
    /// Name of the team, such as crisis assessment team
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Who care team is for
    pub subject: Option<Reference>,
    /// Time period team covers
    pub period: Option<Period>,
    /// Members of the team
    pub participant: Option<Vec<CareTeamParticipant>>,
    /// Why the care team exists
    ///
    /// Binding: example (Indicates the reason for the care team.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub reason: Option<Vec<CodeableReference>>,
    /// Organization responsible for the care team
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Vec<Reference>>,
    /// A contact detail for the care team (that applies to all members)
    pub telecom: Option<Vec<ContactPoint>>,
    /// Comments made about the CareTeam
    pub note: Option<Vec<Annotation>>,
}
/// CareTeam nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareTeamParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of involvement
    ///
    /// Binding: example (Indicates specific responsibility of an individual within the care team, such as "Primary physician", "Team coordinator", "Caregiver", etc.)
    ///
    /// Available values:
    /// - `429577009`
    /// - `116154003`
    /// - `133932002`
    pub role: Option<CodeableConcept>,
    /// Who is involved
    pub member: Option<Reference>,
    /// Organization of the practitioner
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
    /// When the member is generally available within this care team (Period)
    #[serde(rename = "coveragePeriod")]
    pub coverage_period: Option<Period>,
    /// When the member is generally available within this care team (Timing)
    #[serde(rename = "coverageTiming")]
    pub coverage_timing: Option<Timing>,
}

impl Default for CareTeam {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            category: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            subject: Default::default(),
            period: Default::default(),
            participant: Default::default(),
            reason: Default::default(),
            managing_organization: Default::default(),
            telecom: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CareTeamParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            member: Default::default(),
            on_behalf_of: Default::default(),
            coverage_period: Default::default(),
            coverage_timing: Default::default(),
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
    rh_foundation::Invariant::new("ctm-1", rh_foundation::Severity::Error, "CareTeam.participant.onBehalfOf can only be populated when CareTeam.participant.member is a Practitioner", "onBehalfOf.exists() implies (member.resolve() is Practitioner)"),
    rh_foundation::Invariant::new("ctm-2", rh_foundation::Severity::Warning, "CareTeam.participant.role or CareTeam.participant.member exists", "role.exists() or member.exists()"),
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
        vec![
            rh_foundation::ElementBinding::new(
                "CareTeam.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "CareTeam.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/care-team-status|5.0.0",
            )
            .with_description("Indicates the status of the care team."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("CareTeam.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.contained", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.extension", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.identifier", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.category", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.participant", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CareTeam.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CareTeam.participant.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.participant.member", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.participant.onBehalfOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.participant.coverage[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CareTeam.reason", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.managingOrganization", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.telecom", 0, None),
            rh_foundation::ElementCardinality::new("CareTeam.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CareTeam {
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

impl crate::traits::resource::ResourceMutators for CareTeam {
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

impl crate::traits::resource::ResourceExistence for CareTeam {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CareTeam {
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

impl crate::traits::domain_resource::DomainResourceMutators for CareTeam {
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

impl crate::traits::domain_resource::DomainResourceExistence for CareTeam {
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

impl crate::traits::care_team::CareTeamAccessors for CareTeam {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<CareTeamStatus> {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn participant(&self) -> &[CareTeamParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn managing_organization(&self) -> &[Reference] {
        self.managing_organization.as_deref().unwrap_or(&[])
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::care_team::CareTeamMutators for CareTeam {
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
    fn set_status(self, value: CareTeamStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<CareTeamParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: CareTeamParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_managing_organization(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn add_managing_organization(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .managing_organization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.telecom = Some(value);
        resource
    }
    fn add_telecom(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.telecom.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::care_team::CareTeamExistence for CareTeam {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CareTeam {
    fn resource_type(&self) -> &'static str {
        "CareTeam"
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
        Some("http://hl7.org/fhir/StructureDefinition/CareTeam")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::care_team::{CareTeamAccessors, CareTeamExistence, CareTeamMutators};
