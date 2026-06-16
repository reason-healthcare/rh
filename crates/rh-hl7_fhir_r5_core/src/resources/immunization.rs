use crate::bindings::immunization_status::ImmunizationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Immunization
///
/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Immunization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Immunization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Immunization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Authority that the immunization event is based on
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// completed | entered-in-error | not-done
    pub status: ImmunizationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `IMMUNE`
    /// - `MEDPREC`
    /// - `OSTOCK`
    /// - `PATOBJ`
    /// - `PHILISOP`
    /// - `RELIG`
    /// - `VACEFF`
    /// - `VACSAF`
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Vaccine administered
    ///
    /// Binding: example (x)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/vaccine-code
    #[serde(rename = "vaccineCode")]
    pub vaccine_code: CodeableConcept,
    /// Product that was administered
    #[serde(rename = "administeredProduct")]
    pub administered_product: Option<CodeableReference>,
    /// Vaccine manufacturer
    pub manufacturer: Option<CodeableReference>,
    /// Vaccine lot number
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// Vaccine expiration date
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// Who was immunized
    pub patient: Reference,
    /// Encounter immunization was part of
    pub encounter: Option<Reference>,
    /// Additional information in support of the immunization
    #[serde(rename = "supportingInformation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<Reference>,
    /// Vaccine administration date (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: DateTimeType,
    /// Vaccine administration date (string)
    #[serde(rename = "occurrenceString")]
    pub occurrence_string: StringType,
    /// Indicates context the data was captured in
    #[serde(rename = "primarySource")]
    pub primary_source: Option<BooleanType>,
    /// Extension element for the 'primarySource' primitive field. Contains metadata and extensions.
    #[serde(rename = "_primarySource")]
    pub _primary_source: Option<Element>,
    /// Indicates the source of a  reported record
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `provider`: Other Provider
    /// - `record`: Written Record
    /// - `recall`: Parent/Guardian/Patient Recall
    /// - `school`: School Record
    #[serde(rename = "informationSource")]
    pub information_source: Option<CodeableReference>,
    /// Where immunization occurred
    pub location: Option<Reference>,
    /// Body site vaccine  was administered
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `LA`: Left arm
    /// - `RA`: Right arm
    pub site: Option<CodeableConcept>,
    /// How vaccine entered body
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `IDINJ`: Injection, intradermal
    /// - `IM`: Injection, intramuscular
    /// - `IVINJ`: Injection, intravenous
    /// - `PO`: Swallow, oral
    /// - `SQ`: Injection, subcutaneous
    /// - `TRNSDERM`: Transdermal
    pub route: Option<CodeableConcept>,
    /// Amount of vaccine administered
    #[serde(rename = "doseQuantity")]
    pub dose_quantity: Option<Quantity>,
    /// Who performed event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImmunizationPerformer>,
    /// Additional immunization notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Why immunization occurred
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `429060002`
    /// - `14747002`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<CodeableReference>,
    /// Dose potency
    #[serde(rename = "isSubpotent")]
    pub is_subpotent: Option<BooleanType>,
    /// Extension element for the 'isSubpotent' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isSubpotent")]
    pub _is_subpotent: Option<Element>,
    /// Reason for being subpotent
    ///
    /// Binding: example (The reason why a dose is considered to be subpotent.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-subpotent-reason
    #[serde(rename = "subpotentReason")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subpotent_reason: Vec<CodeableConcept>,
    /// Patient eligibility for a specific vaccination program
    #[serde(rename = "programEligibility")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_eligibility: Vec<ImmunizationProgrameligibility>,
    /// Funding source for the vaccine
    ///
    /// Binding: example (x)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-funding-source
    #[serde(rename = "fundingSource")]
    pub funding_source: Option<CodeableConcept>,
    /// Details of a reaction that follows immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ImmunizationReaction>,
    /// Protocol followed by the provider
    #[serde(rename = "protocolApplied")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_applied: Vec<ImmunizationProtocolapplied>,
}
/// Immunization nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What type of performance was done
    ///
    /// Binding: extensible (x)
    ///
    /// Available values:
    /// - `OP`
    /// - `AP`
    pub function: Option<CodeableConcept>,
    /// Individual or organization who was performing
    pub actor: Reference,
}
/// Immunization nested structure for the 'programEligibility' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationProgrameligibility {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The program that eligibility is declared for
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `64994-7`
    pub program: CodeableConcept,
    /// The patient's eligibility status for the program
    ///
    /// Binding: example (x)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-program-eligibility
    #[serde(rename = "programStatus")]
    pub program_status: CodeableConcept,
}
/// Immunization nested structure for the 'protocolApplied' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationProtocolapplied {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of vaccine series
    pub series: Option<StringType>,
    /// Extension element for the 'series' primitive field. Contains metadata and extensions.
    pub _series: Option<Element>,
    /// Who is responsible for publishing the recommendations
    pub authority: Option<Reference>,
    /// Vaccine preventatable disease being targeted
    ///
    /// Binding: example (x)
    ///
    /// Available values:
    /// - `397428000`
    /// - `27836007`
    /// - `76902006`
    /// - `721764008`
    /// - `14189004`
    /// - `36989005`
    /// - `36653000`
    /// - `16814004`
    /// - `23511006`
    /// - `709410003`
    /// - ... and 33 more values
    #[serde(rename = "targetDisease")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<CodeableConcept>,
    /// Dose number within series
    #[serde(rename = "doseNumber")]
    pub dose_number: StringType,
    /// Extension element for the 'doseNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doseNumber")]
    pub _dose_number: Option<Element>,
    /// Recommended number of doses for immunity
    #[serde(rename = "seriesDoses")]
    pub series_doses: Option<StringType>,
    /// Extension element for the 'seriesDoses' primitive field. Contains metadata and extensions.
    #[serde(rename = "_seriesDoses")]
    pub _series_doses: Option<Element>,
}
/// Immunization nested structure for the 'reaction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationReaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// When reaction started
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Additional information on reaction
    pub manifestation: Option<CodeableReference>,
    /// Indicates self-reported reaction
    pub reported: Option<BooleanType>,
    /// Extension element for the 'reported' primitive field. Contains metadata and extensions.
    pub _reported: Option<Element>,
}

impl Default for Immunization {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            status: ImmunizationStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            vaccine_code: CodeableConcept::default(),
            administered_product: Default::default(),
            manufacturer: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            patient: Reference::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_string: Default::default(),
            primary_source: Default::default(),
            _primary_source: Default::default(),
            information_source: Default::default(),
            location: Default::default(),
            site: Default::default(),
            route: Default::default(),
            dose_quantity: Default::default(),
            performer: Default::default(),
            note: Default::default(),
            reason: Default::default(),
            is_subpotent: Default::default(),
            _is_subpotent: Default::default(),
            subpotent_reason: Default::default(),
            program_eligibility: Default::default(),
            funding_source: Default::default(),
            reaction: Default::default(),
            protocol_applied: Default::default(),
        }
    }
}

impl Default for ImmunizationPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for ImmunizationProgrameligibility {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            program: Default::default(),
            program_status: Default::default(),
        }
    }
}

impl Default for ImmunizationProtocolapplied {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            series: Default::default(),
            _series: Default::default(),
            authority: Default::default(),
            target_disease: Default::default(),
            dose_number: Default::default(),
            _dose_number: Default::default(),
            series_doses: Default::default(),
            _series_doses: Default::default(),
        }
    }
}

impl Default for ImmunizationReaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            date: Default::default(),
            _date: Default::default(),
            manifestation: Default::default(),
            reported: Default::default(),
            _reported: Default::default(),
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
        vec![
            rh_foundation::ElementBinding::new(
                "Immunization.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Immunization.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/immunization-status|5.0.0",
            )
            .with_description("x"),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Immunization.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.contained", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.extension", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.vaccineCode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.administeredProduct", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.manufacturer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.lotNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.expirationDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.supportingInformation", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.occurrence[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.primarySource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.informationSource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.site", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.route", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.doseQuantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.performer", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Immunization.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Immunization.performer.function", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.performer.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.note", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.reason", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.isSubpotent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.subpotentReason", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.programEligibility", 0, None),
            rh_foundation::ElementCardinality::new(
                "Immunization.programEligibility.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.programEligibility.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.programEligibility.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.programEligibility.program",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.programEligibility.programStatus",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Immunization.fundingSource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.reaction", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.reaction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.reaction.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Immunization.reaction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Immunization.reaction.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Immunization.reaction.manifestation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Immunization.reaction.reported", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Immunization.protocolApplied", 0, None),
            rh_foundation::ElementCardinality::new("Immunization.protocolApplied.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.series",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.authority",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.targetDisease",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.doseNumber",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Immunization.protocolApplied.seriesDoses",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Immunization {
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

impl crate::traits::resource::ResourceMutators for Immunization {
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

impl crate::traits::resource::ResourceExistence for Immunization {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Immunization {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for Immunization {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Immunization {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::immunization::ImmunizationAccessors for Immunization {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn status(&self) -> ImmunizationStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn vaccine_code(&self) -> CodeableConcept {
        self.vaccine_code.clone()
    }
    fn administered_product(&self) -> Option<CodeableReference> {
        self.administered_product.clone()
    }
    fn manufacturer(&self) -> Option<CodeableReference> {
        self.manufacturer.clone()
    }
    fn lot_number(&self) -> Option<StringType> {
        self.lot_number.clone()
    }
    fn expiration_date(&self) -> Option<DateType> {
        self.expiration_date.clone()
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_slice()
    }
    fn primary_source(&self) -> Option<BooleanType> {
        self.primary_source
    }
    fn information_source(&self) -> Option<CodeableReference> {
        self.information_source.clone()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn site(&self) -> Option<CodeableConcept> {
        self.site.clone()
    }
    fn route(&self) -> Option<CodeableConcept> {
        self.route.clone()
    }
    fn dose_quantity(&self) -> Option<Quantity> {
        self.dose_quantity.clone()
    }
    fn performer(&self) -> &[ImmunizationPerformer] {
        self.performer.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_slice()
    }
    fn is_subpotent(&self) -> Option<BooleanType> {
        self.is_subpotent
    }
    fn subpotent_reason(&self) -> &[CodeableConcept] {
        self.subpotent_reason.as_slice()
    }
    fn program_eligibility(&self) -> &[ImmunizationProgrameligibility] {
        self.program_eligibility.as_slice()
    }
    fn funding_source(&self) -> Option<CodeableConcept> {
        self.funding_source.clone()
    }
    fn reaction(&self) -> &[ImmunizationReaction] {
        self.reaction.as_slice()
    }
    fn protocol_applied(&self) -> &[ImmunizationProtocolapplied] {
        self.protocol_applied.as_slice()
    }
}

impl crate::traits::immunization::ImmunizationMutators for Immunization {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_status(self, value: ImmunizationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_vaccine_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.vaccine_code = value;
        resource
    }
    fn set_administered_product(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.administered_product = Some(value);
        resource
    }
    fn set_manufacturer(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_lot_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.lot_number = Some(value);
        resource
    }
    fn set_expiration_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expiration_date = Some(value);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = value;
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.supporting_information.push(item);
        resource
    }
    fn set_primary_source(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.primary_source = Some(value);
        resource
    }
    fn set_information_source(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.information_source = Some(value);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.site = Some(value);
        resource
    }
    fn set_route(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.route = Some(value);
        resource
    }
    fn set_dose_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.dose_quantity = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<ImmunizationPerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = value;
        resource
    }
    fn add_performer(self, item: ImmunizationPerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.push(item);
        resource
    }
    fn set_is_subpotent(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.is_subpotent = Some(value);
        resource
    }
    fn set_subpotent_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.subpotent_reason = value;
        resource
    }
    fn add_subpotent_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.subpotent_reason.push(item);
        resource
    }
    fn set_program_eligibility(self, value: Vec<ImmunizationProgrameligibility>) -> Self {
        let mut resource = self.clone();
        resource.program_eligibility = value;
        resource
    }
    fn add_program_eligibility(self, item: ImmunizationProgrameligibility) -> Self {
        let mut resource = self.clone();
        resource.program_eligibility.push(item);
        resource
    }
    fn set_funding_source(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funding_source = Some(value);
        resource
    }
    fn set_reaction(self, value: Vec<ImmunizationReaction>) -> Self {
        let mut resource = self.clone();
        resource.reaction = value;
        resource
    }
    fn add_reaction(self, item: ImmunizationReaction) -> Self {
        let mut resource = self.clone();
        resource.reaction.push(item);
        resource
    }
    fn set_protocol_applied(self, value: Vec<ImmunizationProtocolapplied>) -> Self {
        let mut resource = self.clone();
        resource.protocol_applied = value;
        resource
    }
    fn add_protocol_applied(self, item: ImmunizationProtocolapplied) -> Self {
        let mut resource = self.clone();
        resource.protocol_applied.push(item);
        resource
    }
}

impl crate::traits::immunization::ImmunizationExistence for Immunization {
    fn has_occurrence(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_vaccine_code(&self) -> bool {
        true
    }
    fn has_administered_product(&self) -> bool {
        self.administered_product.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_lot_number(&self) -> bool {
        self.lot_number.is_some()
    }
    fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        !self.supporting_information.is_empty()
    }
    fn has_primary_source(&self) -> bool {
        self.primary_source.is_some()
    }
    fn has_information_source(&self) -> bool {
        self.information_source.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_site(&self) -> bool {
        self.site.is_some()
    }
    fn has_route(&self) -> bool {
        self.route.is_some()
    }
    fn has_dose_quantity(&self) -> bool {
        self.dose_quantity.is_some()
    }
    fn has_performer(&self) -> bool {
        !self.performer.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_reason(&self) -> bool {
        !self.reason.is_empty()
    }
    fn has_is_subpotent(&self) -> bool {
        self.is_subpotent.is_some()
    }
    fn has_subpotent_reason(&self) -> bool {
        !self.subpotent_reason.is_empty()
    }
    fn has_program_eligibility(&self) -> bool {
        !self.program_eligibility.is_empty()
    }
    fn has_funding_source(&self) -> bool {
        self.funding_source.is_some()
    }
    fn has_reaction(&self) -> bool {
        !self.reaction.is_empty()
    }
    fn has_protocol_applied(&self) -> bool {
        !self.protocol_applied.is_empty()
    }
}

impl crate::validation::ValidatableResource for Immunization {
    fn resource_type(&self) -> &'static str {
        "Immunization"
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
        Some("http://hl7.org/fhir/StructureDefinition/Immunization")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::immunization::{
    ImmunizationAccessors, ImmunizationExistence, ImmunizationMutators,
};
