use crate::bindings::immunization_status::ImmunizationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Immunization
///
/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Immunization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Immunization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Immunization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// completed | entered-in-error | not-done
    pub status: ImmunizationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason not done
    ///
    /// Binding: example (The reason why a vaccine was not administered.)
    ///
    /// Available values:
    /// - `IMMUNE`
    /// - `MEDPREC`
    /// - `OSTOCK`
    /// - `PATOBJ`
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Vaccine product administered
    ///
    /// Binding: example (The code for vaccine product administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/vaccine-code
    #[serde(rename = "vaccineCode")]
    pub vaccine_code: CodeableConcept,
    /// Who was immunized
    pub patient: Reference,
    /// Encounter immunization was part of
    pub encounter: Option<Reference>,
    /// Vaccine administration date (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: DateTimeType,
    /// Vaccine administration date (string)
    #[serde(rename = "occurrenceString")]
    pub occurrence_string: StringType,
    /// When the immunization was first captured in the subject's record
    pub recorded: Option<DateTimeType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Indicates context the data was recorded in
    #[serde(rename = "primarySource")]
    pub primary_source: Option<BooleanType>,
    /// Extension element for the 'primarySource' primitive field. Contains metadata and extensions.
    #[serde(rename = "_primarySource")]
    pub _primary_source: Option<Element>,
    /// Indicates the source of a secondarily reported record
    ///
    /// Binding: example (The source of the data for a record which is not from a primary source.)
    ///
    /// Available values:
    /// - `provider`: Other Provider
    /// - `record`: Written Record
    /// - `recall`: Parent/Guardian/Patient Recall
    /// - `school`: School Record
    #[serde(rename = "reportOrigin")]
    pub report_origin: Option<CodeableConcept>,
    /// Where immunization occurred
    pub location: Option<Reference>,
    /// Vaccine manufacturer
    pub manufacturer: Option<Reference>,
    /// Vaccine lot number
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// Vaccine expiration date
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<StringType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// Body site vaccine  was administered
    ///
    /// Binding: example (The site at which the vaccine was administered.)
    ///
    /// Available values:
    /// - `LA`: Left arm
    /// - `RA`: Right arm
    pub site: Option<CodeableConcept>,
    /// How vaccine entered body
    ///
    /// Binding: example (The route by which the vaccine was administered.)
    ///
    /// Available values:
    /// - `IDINJ`: Injection, intradermal
    /// - `IM`: Injection, intramuscular
    /// - `NASINHLC`: Inhalation, nasal
    /// - `IVINJ`: Injection, intravenous
    /// - `PO`: Swallow, oral
    /// - `SQ`: Injection, subcutaneous
    /// - `TRNSDERM`: Transdermal
    pub route: Option<CodeableConcept>,
    /// Amount of vaccine administered
    #[serde(rename = "doseQuantity")]
    pub dose_quantity: Option<Quantity>,
    /// Who performed event
    pub performer: Option<Vec<ImmunizationPerformer>>,
    /// Additional immunization notes
    pub note: Option<Vec<Annotation>>,
    /// Why immunization occurred
    ///
    /// Binding: example (The reason why a vaccine was administered.)
    ///
    /// Available values:
    /// - `429060002`
    /// - `281657000`
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why immunization occurred
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
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
    pub subpotent_reason: Option<Vec<CodeableConcept>>,
    /// Educational material presented to patient
    pub education: Option<Vec<ImmunizationEducation>>,
    /// Patient eligibility for a vaccination program
    ///
    /// Binding: example (The patient's eligibility for a vaccation program.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-program-eligibility
    #[serde(rename = "programEligibility")]
    pub program_eligibility: Option<Vec<CodeableConcept>>,
    /// Funding source for the vaccine
    ///
    /// Binding: example (The source of funding used to purchase the vaccine administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/immunization-funding-source
    #[serde(rename = "fundingSource")]
    pub funding_source: Option<CodeableConcept>,
    /// Details of a reaction that follows immunization
    pub reaction: Option<Vec<ImmunizationReaction>>,
    /// Protocol followed by the provider
    #[serde(rename = "protocolApplied")]
    pub protocol_applied: Option<Vec<ImmunizationProtocolapplied>>,
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
    /// Vaccine preventatable disease being targetted
    ///
    /// Binding: example (The vaccine preventable disease the dose is being administered for.)
    ///
    /// Available values:
    /// - `1857005`
    /// - `397430003`
    /// - `14189004`
    /// - `36989005`
    /// - `36653000`
    /// - `76902006`
    /// - `709410003`
    /// - `27836007`
    /// - `398102009`
    #[serde(rename = "targetDisease")]
    pub target_disease: Option<Vec<CodeableConcept>>,
    /// Dose number within series (positiveInt)
    #[serde(rename = "doseNumberPositiveInt")]
    pub dose_number_positive_int: PositiveIntType,
    /// Dose number within series (string)
    #[serde(rename = "doseNumberString")]
    pub dose_number_string: StringType,
    /// Recommended number of doses for immunity (positiveInt)
    #[serde(rename = "seriesDosesPositiveInt")]
    pub series_doses_positive_int: Option<PositiveIntType>,
    /// Recommended number of doses for immunity (string)
    #[serde(rename = "seriesDosesString")]
    pub series_doses_string: Option<StringType>,
}
/// Immunization nested structure for the 'education' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationEducation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Educational material document identifier
    #[serde(rename = "documentType")]
    pub document_type: Option<StringType>,
    /// Extension element for the 'documentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_documentType")]
    pub _document_type: Option<Element>,
    /// Educational material reference pointer
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Educational material publication date
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<DateTimeType>,
    /// Extension element for the 'publicationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationDate")]
    pub _publication_date: Option<Element>,
    /// Educational material presentation date
    #[serde(rename = "presentationDate")]
    pub presentation_date: Option<DateTimeType>,
    /// Extension element for the 'presentationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_presentationDate")]
    pub _presentation_date: Option<Element>,
}
/// Immunization nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunizationPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What type of performance was done
    ///
    /// Binding: extensible (The role a practitioner or organization plays in the immunization event.)
    ///
    /// Available values:
    /// - `OP`
    /// - `AP`
    pub function: Option<CodeableConcept>,
    /// Individual or organization who was performing
    pub actor: Reference,
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
    pub detail: Option<Reference>,
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
            status: ImmunizationStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            vaccine_code: CodeableConcept::default(),
            patient: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_string: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            primary_source: Default::default(),
            _primary_source: Default::default(),
            report_origin: Default::default(),
            location: Default::default(),
            manufacturer: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            site: Default::default(),
            route: Default::default(),
            dose_quantity: Default::default(),
            performer: Default::default(),
            note: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            is_subpotent: Default::default(),
            _is_subpotent: Default::default(),
            subpotent_reason: Default::default(),
            education: Default::default(),
            program_eligibility: Default::default(),
            funding_source: Default::default(),
            reaction: Default::default(),
            protocol_applied: Default::default(),
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
            dose_number_positive_int: Default::default(),
            dose_number_string: Default::default(),
            series_doses_positive_int: Default::default(),
            series_doses_string: Default::default(),
        }
    }
}

impl Default for ImmunizationEducation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            document_type: Default::default(),
            _document_type: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
            publication_date: Default::default(),
            _publication_date: Default::default(),
            presentation_date: Default::default(),
            _presentation_date: Default::default(),
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

impl Default for ImmunizationReaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            date: Default::default(),
            _date: Default::default(),
            detail: Default::default(),
            reported: Default::default(),
            _reported: Default::default(),
        }
    }
}

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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for Immunization {
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

impl crate::traits::immunization::ImmunizationAccessors for Immunization {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
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
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorded(&self) -> Option<DateTimeType> {
        self.recorded.clone()
    }
    fn primary_source(&self) -> Option<BooleanType> {
        self.primary_source
    }
    fn report_origin(&self) -> Option<CodeableConcept> {
        self.report_origin.clone()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn manufacturer(&self) -> Option<Reference> {
        self.manufacturer.clone()
    }
    fn lot_number(&self) -> Option<StringType> {
        self.lot_number.clone()
    }
    fn expiration_date(&self) -> Option<StringType> {
        self.expiration_date.clone()
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
        self.performer.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn is_subpotent(&self) -> Option<BooleanType> {
        self.is_subpotent
    }
    fn subpotent_reason(&self) -> &[CodeableConcept] {
        self.subpotent_reason.as_deref().unwrap_or(&[])
    }
    fn education(&self) -> &[ImmunizationEducation] {
        self.education.as_deref().unwrap_or(&[])
    }
    fn program_eligibility(&self) -> &[CodeableConcept] {
        self.program_eligibility.as_deref().unwrap_or(&[])
    }
    fn funding_source(&self) -> Option<CodeableConcept> {
        self.funding_source.clone()
    }
    fn reaction(&self) -> &[ImmunizationReaction] {
        self.reaction.as_deref().unwrap_or(&[])
    }
    fn protocol_applied(&self) -> &[ImmunizationProtocolapplied] {
        self.protocol_applied.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::immunization::ImmunizationMutators for Immunization {
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
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
        resource
    }
    fn set_primary_source(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.primary_source = Some(value);
        resource
    }
    fn set_report_origin(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.report_origin = Some(value);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Reference) -> Self {
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
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: ImmunizationPerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
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
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_is_subpotent(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.is_subpotent = Some(value);
        resource
    }
    fn set_subpotent_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.subpotent_reason = Some(value);
        resource
    }
    fn add_subpotent_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .subpotent_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_education(self, value: Vec<ImmunizationEducation>) -> Self {
        let mut resource = self.clone();
        resource.education = Some(value);
        resource
    }
    fn add_education(self, item: ImmunizationEducation) -> Self {
        let mut resource = self.clone();
        resource.education.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_program_eligibility(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.program_eligibility = Some(value);
        resource
    }
    fn add_program_eligibility(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .program_eligibility
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_funding_source(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.funding_source = Some(value);
        resource
    }
    fn set_reaction(self, value: Vec<ImmunizationReaction>) -> Self {
        let mut resource = self.clone();
        resource.reaction = Some(value);
        resource
    }
    fn add_reaction(self, item: ImmunizationReaction) -> Self {
        let mut resource = self.clone();
        resource.reaction.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_protocol_applied(self, value: Vec<ImmunizationProtocolapplied>) -> Self {
        let mut resource = self.clone();
        resource.protocol_applied = Some(value);
        resource
    }
    fn add_protocol_applied(self, item: ImmunizationProtocolapplied) -> Self {
        let mut resource = self.clone();
        resource
            .protocol_applied
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::immunization::ImmunizationExistence for Immunization {
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
    fn has_occurrence(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_patient(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
    }
    fn has_primary_source(&self) -> bool {
        self.primary_source.is_some()
    }
    fn has_report_origin(&self) -> bool {
        self.report_origin.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
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
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_is_subpotent(&self) -> bool {
        self.is_subpotent.is_some()
    }
    fn has_subpotent_reason(&self) -> bool {
        self.subpotent_reason
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_education(&self) -> bool {
        self.education.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_program_eligibility(&self) -> bool {
        self.program_eligibility
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_funding_source(&self) -> bool {
        self.funding_source.is_some()
    }
    fn has_reaction(&self) -> bool {
        self.reaction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_protocol_applied(&self) -> bool {
        self.protocol_applied
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
