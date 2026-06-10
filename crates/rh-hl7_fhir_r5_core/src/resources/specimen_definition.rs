use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::specimen_contained_preference::SpecimenContainedPreference;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SpecimenDefinition
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Logical canonical URL to reference this SpecimenDefinition (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business identifier
    pub identifier: Option<Identifier>,
    /// Business version of the SpecimenDefinition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this {{title}} (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this SpecimenDefinition (Human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Based on FHIR definition of another SpecimenDefinition
    #[serde(rename = "derivedFromCanonical")]
    pub derived_from_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFromCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFromCanonical")]
    pub _derived_from_canonical: Option<Element>,
    /// Based on external definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFromUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFromUri")]
    pub _derived_from_uri: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// If this SpecimenDefinition is not for real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Type of subject for specimen collection (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// Type of subject for specimen collection (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Date status first applied
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The name of the individual or organization that published the SpecimenDefinition
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the SpecimenDefinition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Content intends to support these contexts
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    ///
    /// Binding: extensible (Codes for country, country subdivision and region for indicating where a resource is intended to be used.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this SpecimenDefinition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// When SpecimenDefinition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// The date on which the asset content was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// The effective date range for the SpecimenDefinition
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Kind of material to collect
    ///
    /// Binding: example (The type of the specimen to be collected.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0487
    #[serde(rename = "typeCollected")]
    pub type_collected: Option<CodeableConcept>,
    /// Patient preparation for collection
    ///
    /// Binding: example (SCT descendants of 703763000 |Precondition value (qualifier value)|)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/prepare-patient-prior-specimen-collection
    #[serde(rename = "patientPreparation")]
    pub patient_preparation: Option<Vec<CodeableConcept>>,
    /// Time aspect for collection
    #[serde(rename = "timeAspect")]
    pub time_aspect: Option<StringType>,
    /// Extension element for the 'timeAspect' primitive field. Contains metadata and extensions.
    #[serde(rename = "_timeAspect")]
    pub _time_aspect: Option<Element>,
    /// Specimen collection procedure
    ///
    /// Binding: example (SCT actions and procedures for specimen collection)
    ///
    /// Available values:
    /// - `129316008`: Aspiration - action
    /// - `129314006`: Biopsy - action
    /// - `129300006`: Puncture - action
    /// - `129304002`: Excision - action
    /// - `129323009`: Scraping - action
    /// - `73416001`: Urine specimen collection, clean catch
    /// - `225113003`: Timed urine collection
    /// - `70777001`: Urine specimen collection, catheterized
    /// - `386089008`: Collection of coughed sputum
    /// - `278450005`: Finger-prick sampling
    pub collection: Option<Vec<CodeableConcept>>,
    /// Specimen in container intended for testing by lab
    #[serde(rename = "typeTested")]
    pub type_tested: Option<Vec<SpecimenDefinitionTypetested>>,
}
/// SpecimenDefinitionTypetested nested structure for the 'container' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedContainer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The material type used for the container
    ///
    /// Binding: example (SCT 32039001 |Glass|, 61088005 |Plastic|, 425620007 |Metal|)
    ///
    /// Available values:
    /// - `32039001`: glass
    /// - `61088005`: plastic
    /// - `425620007`: metal
    pub material: Option<CodeableConcept>,
    /// Kind of container associated with the kind of specimen
    ///
    /// Binding: example (SCT descendants of 706041008 |Device for body fluid and tissue collection/transfer/processing (physical object)|)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/specimen-container-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Color of container cap
    ///
    /// Binding: example (Color of the container cap.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/container-cap
    pub cap: Option<CodeableConcept>,
    /// The description of the kind of container
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The capacity of this kind of container
    pub capacity: Option<Quantity>,
    /// Minimum volume (Quantity)
    #[serde(rename = "minimumVolumeQuantity")]
    pub minimum_volume_quantity: Option<Quantity>,
    /// Minimum volume (string)
    #[serde(rename = "minimumVolumeString")]
    pub minimum_volume_string: Option<StringType>,
    /// Special processing applied to the container for this specimen type
    pub preparation: Option<StringType>,
    /// Extension element for the 'preparation' primitive field. Contains metadata and extensions.
    pub _preparation: Option<Element>,
}
/// SpecimenDefinition nested structure for the 'typeTested' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetested {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypetestedContainer>,
    /// Specimen handling before testing
    pub handling: Option<Vec<SpecimenDefinitionTypetestedHandling>>,
    /// Primary or secondary specimen
    #[serde(rename = "isDerived")]
    pub is_derived: Option<BooleanType>,
    /// Extension element for the 'isDerived' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDerived")]
    pub _is_derived: Option<Element>,
    /// Type of intended specimen
    ///
    /// Binding: example (The type of specimen conditioned in a container for lab testing.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0487
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// preferred | alternate
    pub preference: SpecimenContainedPreference,
    /// Extension element for the 'preference' primitive field. Contains metadata and extensions.
    pub _preference: Option<Element>,
    /// Requirements for specimen delivery and special handling
    pub requirement: Option<StringType>,
    /// Extension element for the 'requirement' primitive field. Contains metadata and extensions.
    pub _requirement: Option<Element>,
    /// The usual time for retaining this kind of specimen
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<Duration>,
    /// Specimen for single use only
    #[serde(rename = "singleUse")]
    pub single_use: Option<BooleanType>,
    /// Extension element for the 'singleUse' primitive field. Contains metadata and extensions.
    #[serde(rename = "_singleUse")]
    pub _single_use: Option<Element>,
    /// Criterion specified for specimen rejection
    ///
    /// Binding: example (Criterion for rejection of the specimen by laboratory.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/rejection-criteria
    #[serde(rename = "rejectionCriterion")]
    pub rejection_criterion: Option<Vec<CodeableConcept>>,
    /// Where the specimen will be tested
    ///
    /// Binding: example (Codes specifying where the specimen will be tested.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnostic-service-sections
    #[serde(rename = "testingDestination")]
    pub testing_destination: Option<Vec<CodeableConcept>>,
}
/// SpecimenDefinitionTypetested nested structure for the 'handling' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedHandling {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Qualifies the interval of temperature
    ///
    /// Binding: example (Set of handling instructions prior testing of the specimen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/handling-condition
    #[serde(rename = "temperatureQualifier")]
    pub temperature_qualifier: Option<CodeableConcept>,
    /// Temperature range for these handling instructions
    #[serde(rename = "temperatureRange")]
    pub temperature_range: Option<Range>,
    /// Maximum preservation time
    #[serde(rename = "maxDuration")]
    pub max_duration: Option<Duration>,
    /// Preservation instruction
    pub instruction: Option<StringType>,
    /// Extension element for the 'instruction' primitive field. Contains metadata and extensions.
    pub _instruction: Option<Element>,
}
/// SpecimenDefinitionTypetestedContainer nested structure for the 'additive' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenDefinitionTypetestedContainerAdditive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Additive associated with container (CodeableConcept)
    #[serde(rename = "additiveCodeableConcept")]
    pub additive_codeable_concept: CodeableConcept,
    /// Additive associated with container (Reference)
    #[serde(rename = "additiveReference")]
    pub additive_reference: Reference,
}

impl Default for SpecimenDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            derived_from_canonical: Default::default(),
            _derived_from_canonical: Default::default(),
            derived_from_uri: Default::default(),
            _derived_from_uri: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            effective_period: Default::default(),
            type_collected: Default::default(),
            patient_preparation: Default::default(),
            time_aspect: Default::default(),
            _time_aspect: Default::default(),
            collection: Default::default(),
            type_tested: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedContainer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            material: Default::default(),
            type_: Default::default(),
            cap: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            capacity: Default::default(),
            minimum_volume_quantity: Default::default(),
            minimum_volume_string: Default::default(),
            preparation: Default::default(),
            _preparation: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetested {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            container: Default::default(),
            handling: Default::default(),
            is_derived: Default::default(),
            _is_derived: Default::default(),
            type_: Default::default(),
            preference: Default::default(),
            _preference: Default::default(),
            requirement: Default::default(),
            _requirement: Default::default(),
            retention_time: Default::default(),
            single_use: Default::default(),
            _single_use: Default::default(),
            rejection_criterion: Default::default(),
            testing_destination: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedHandling {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            temperature_qualifier: Default::default(),
            temperature_range: Default::default(),
            max_duration: Default::default(),
            instruction: Default::default(),
            _instruction: Default::default(),
        }
    }
}

impl Default for SpecimenDefinitionTypetestedContainerAdditive {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            additive_codeable_concept: Default::default(),
            additive_reference: Default::default(),
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
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
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
                "SpecimenDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "SpecimenDefinition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("Codes identifying the status of a SpecimenDefinition resource."),
            rh_foundation::ElementBinding::new(
                "SpecimenDefinition.typeTested.preference",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/specimen-contained-preference|5.0.0",
            )
            .with_description("Degree of preference of a type of conditioned specimen."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SpecimenDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.derivedFromCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.derivedFromUri", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeCollected", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.patientPreparation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.timeAspect", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.collection", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeTested", 0, None),
            rh_foundation::ElementCardinality::new("SpecimenDefinition.typeTested.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.isDerived",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.preference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.material",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.cap",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.capacity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.minimumVolume[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.additive.additive[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.container.preparation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.retentionTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.singleUse",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.rejectionCriterion",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.temperatureQualifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.temperatureRange",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.maxDuration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.handling.instruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SpecimenDefinition.typeTested.testingDestination",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SpecimenDefinition {
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

impl crate::traits::resource::ResourceMutators for SpecimenDefinition {
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

impl crate::traits::resource::ResourceExistence for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for SpecimenDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for SpecimenDefinition {
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

impl crate::traits::specimen_definition::SpecimenDefinitionAccessors for SpecimenDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn derived_from_canonical(&self) -> &[StringType] {
        self.derived_from_canonical.as_deref().unwrap_or(&[])
    }
    fn derived_from_uri(&self) -> &[StringType] {
        self.derived_from_uri.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn effective_period(&self) -> Option<Period> {
        self.effective_period.clone()
    }
    fn type_collected(&self) -> Option<CodeableConcept> {
        self.type_collected.clone()
    }
    fn patient_preparation(&self) -> &[CodeableConcept] {
        self.patient_preparation.as_deref().unwrap_or(&[])
    }
    fn time_aspect(&self) -> Option<StringType> {
        self.time_aspect.clone()
    }
    fn collection(&self) -> &[CodeableConcept] {
        self.collection.as_deref().unwrap_or(&[])
    }
    fn type_tested(&self) -> &[SpecimenDefinitionTypetested] {
        self.type_tested.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::specimen_definition::SpecimenDefinitionMutators for SpecimenDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_derived_from_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from_canonical = Some(value);
        resource
    }
    fn add_derived_from_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_derived_from_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from_uri = Some(value);
        resource
    }
    fn add_derived_from_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_approval_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.approval_date = Some(value);
        resource
    }
    fn set_last_review_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_review_date = Some(value);
        resource
    }
    fn set_effective_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.effective_period = Some(value);
        resource
    }
    fn set_type_collected(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_collected = Some(value);
        resource
    }
    fn set_patient_preparation(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.patient_preparation = Some(value);
        resource
    }
    fn add_patient_preparation(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .patient_preparation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_time_aspect(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.time_aspect = Some(value);
        resource
    }
    fn set_collection(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.collection = Some(value);
        resource
    }
    fn add_collection(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.collection.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_type_tested(self, value: Vec<SpecimenDefinitionTypetested>) -> Self {
        let mut resource = self.clone();
        resource.type_tested = Some(value);
        resource
    }
    fn add_type_tested(self, item: SpecimenDefinitionTypetested) -> Self {
        let mut resource = self.clone();
        resource.type_tested.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::specimen_definition::SpecimenDefinitionExistence for SpecimenDefinition {
    fn has_subject(&self) -> bool {
        self.subject_codeable_concept.is_some() || self.subject_reference.is_some()
    }
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_derived_from_canonical(&self) -> bool {
        self.derived_from_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_derived_from_uri(&self) -> bool {
        self.derived_from_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_effective_period(&self) -> bool {
        self.effective_period.is_some()
    }
    fn has_type_collected(&self) -> bool {
        self.type_collected.is_some()
    }
    fn has_patient_preparation(&self) -> bool {
        self.patient_preparation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_time_aspect(&self) -> bool {
        self.time_aspect.is_some()
    }
    fn has_collection(&self) -> bool {
        self.collection.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_tested(&self) -> bool {
        self.type_tested.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SpecimenDefinition {
    fn resource_type(&self) -> &'static str {
        "SpecimenDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/SpecimenDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::specimen_definition::{
    SpecimenDefinitionAccessors, SpecimenDefinitionExistence, SpecimenDefinitionMutators,
};
