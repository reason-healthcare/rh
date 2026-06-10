use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::related_artifact_type_all::RelatedArtifactTypeAll;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Citation
///
/// The Citation Resource enables reference to any knowledge artifact for purposes of identification and attribution. The Citation Resource supports existing reference structures and developing publication practices such as versioning, expressing complex contributorship roles, and referencing computable resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Citation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Citation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this citation record, represented as a globally unique URI
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Identifier for the citation record itself
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the citation record
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this citation record (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this citation record (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The publisher of the citation record, not the publisher of the article or artifact being cited
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher of the citation record
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the citation
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the citation record content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for citation record (if applicable)
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this citation is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions for the citation record, not for the cited artifact
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s) for the ciation record, not for the cited artifact
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// When the citation record was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the citation record was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the citation record is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Who authored the citation record
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the citation record
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the citation record
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the citation record
    pub endorser: Option<Vec<ContactDetail>>,
    /// A human-readable display of key concepts to represent the citation
    pub summary: Option<Vec<CitationSummary>>,
    /// The assignment to an organizing scheme
    pub classification: Option<Vec<CitationClassification>>,
    /// Used for general notes and annotations not coded elsewhere
    pub note: Option<Vec<Annotation>>,
    /// The status of the citation record
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-status-type
    #[serde(rename = "currentState")]
    pub current_state: Option<Vec<CodeableConcept>>,
    /// An effective date or period for a status of the citation record
    #[serde(rename = "statusDate")]
    pub status_date: Option<Vec<CitationStatusdate>>,
    /// Artifact related to the citation record
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// The article or artifact being described
    #[serde(rename = "citedArtifact")]
    pub cited_artifact: Option<CitationCitedartifact>,
}
/// CitationCitedartifact nested structure for the 'title' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactTitle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of title
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/title-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Used to express the specific language
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bg`: Bulgarian
    /// - `bg-BG`: Bulgarian (Bulgaria)
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `cs-CZ`: Czech (Czechia)
    /// - `bs`: Bosnian
    /// - `bs-BA`: Bosnian (Bosnia and Herzegovina)
    /// - `da`: Danish
    /// - `da-DK`: Danish (Denmark)
    /// - ... and 72 more values
    pub language: Option<StringType>,
    /// The title of the article or artifact
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}
/// CitationCitedartifact nested structure for the 'abstract' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactAbstract {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of abstract
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-artifact-abstract-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Used to express the specific language
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bg`: Bulgarian
    /// - `bg-BG`: Bulgarian (Bulgaria)
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `cs-CZ`: Czech (Czechia)
    /// - `bs`: Bosnian
    /// - `bs-BA`: Bosnian (Bosnia and Herzegovina)
    /// - `da`: Danish
    /// - `da-DK`: Danish (Denmark)
    /// - ... and 72 more values
    pub language: Option<StringType>,
    /// Abstract content
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Copyright notice for the abstract
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
}
/// CitationCitedartifact nested structure for the 'contributorship' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactContributorship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Indicates if the list includes all authors and/or contributors
    pub complete: Option<BooleanType>,
    /// Extension element for the 'complete' primitive field. Contains metadata and extensions.
    pub _complete: Option<Element>,
}
/// Citation nested structure for the 'summary' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationSummary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Format for display of the citation summary
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-summary-style
    pub style: Option<CodeableConcept>,
    /// The human-readable display of the citation summary
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}
/// CitationCitedartifact nested structure for the 'webLocation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactWeblocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code the reason for different URLs, e.g. abstract and full-text
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/artifact-url-classifier
    pub classifier: Option<Vec<CodeableConcept>>,
    /// The specific URL
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
}
/// Citation nested structure for the 'citedArtifact' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The title details of the article or artifact
    pub title: Option<Vec<CitationCitedartifactTitle>>,
    /// An effective date or period for a status of the cited artifact
    #[serde(rename = "statusDate")]
    pub status_date: Option<Vec<CitationCitedartifactStatusdate>>,
    /// Summary of the article or artifact
    #[serde(rename = "abstract")]
    pub abstract_: Option<Vec<CitationCitedartifactAbstract>>,
    /// The artifact related to the cited artifact
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<CitationCitedartifactRelatesto>>,
    /// The component of the article or artifact
    pub part: Option<CitationCitedartifactPart>,
    /// If multiple, used to represent alternative forms of the article that are not separate citations
    #[serde(rename = "publicationForm")]
    pub publication_form: Option<Vec<CitationCitedartifactPublicationform>>,
    /// The defined version of the cited artifact
    pub version: Option<CitationCitedartifactVersion>,
    /// The assignment to an organizing scheme
    pub classification: Option<Vec<CitationCitedartifactClassification>>,
    /// Attribution of authors and other contributors
    pub contributorship: Option<CitationCitedartifactContributorship>,
    /// Used for any URL for the article or artifact cited
    #[serde(rename = "webLocation")]
    pub web_location: Option<Vec<CitationCitedartifactWeblocation>>,
    /// Unique identifier. May include DOI, PMID, PMCID, etc
    pub identifier: Option<Vec<Identifier>>,
    /// Identifier not unique to the cited artifact. May include trial registry identifiers
    #[serde(rename = "relatedIdentifier")]
    pub related_identifier: Option<Vec<Identifier>>,
    /// When the cited artifact was accessed
    #[serde(rename = "dateAccessed")]
    pub date_accessed: Option<DateTimeType>,
    /// Extension element for the 'dateAccessed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateAccessed")]
    pub _date_accessed: Option<Element>,
    /// The status of the cited artifact
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-artifact-status-type
    #[serde(rename = "currentState")]
    pub current_state: Option<Vec<CodeableConcept>>,
    /// Any additional information or content for the article or artifact
    pub note: Option<Vec<Annotation>>,
}
/// CitationCitedartifact nested structure for the 'classification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactClassification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of classifier (e.g. publication type, keyword)
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-artifact-classification-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The specific classification value
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-artifact-classifier
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Complex or externally created classification
    #[serde(rename = "artifactAssessment")]
    pub artifact_assessment: Option<Vec<Reference>>,
}
/// CitationCitedartifact nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The version number or other version identifier
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Citation for the main version of the cited artifact
    #[serde(rename = "baseCitation")]
    pub base_citation: Option<Reference>,
}
/// CitationCitedartifact nested structure for the 'part' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactPart {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of component
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-artifact-part-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The specification of the component
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// The citation for the full article or artifact
    #[serde(rename = "baseCitation")]
    pub base_citation: Option<Reference>,
}
/// Citation nested structure for the 'statusDate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationStatusdate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Classification of the status
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-status-type
    pub activity: CodeableConcept,
    /// Either occurred or expected
    pub actual: Option<BooleanType>,
    /// Extension element for the 'actual' primitive field. Contains metadata and extensions.
    pub _actual: Option<Element>,
    /// When the status started and/or ended
    pub period: Period,
}
/// CitationCitedartifact nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as | reprint | reprint-of
    #[serde(rename = "type")]
    pub type_: RelatedArtifactTypeAll,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Additional classifiers
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-artifact-classifier
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Short label
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Brief description of the related artifact
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Bibliographic citation for the artifact
    pub citation: Option<StringType>,
    /// Extension element for the 'citation' primitive field. Contains metadata and extensions.
    pub _citation: Option<Element>,
    /// What document is being referenced
    pub document: Option<Attachment>,
    /// What artifact is being referenced
    pub resource: Option<StringType>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// What artifact, if not a conformance resource
    #[serde(rename = "resourceReference")]
    pub resource_reference: Option<Reference>,
}
/// Citation nested structure for the 'classification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationClassification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of classifier (e.g. publication type, keyword)
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-classification-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The specific classification value
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-artifact-classifier
    pub classifier: Option<Vec<CodeableConcept>>,
}
/// CitationCitedartifact nested structure for the 'publicationForm' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactPublicationform {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Internet or Print
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-medium
    #[serde(rename = "citedMedium")]
    pub cited_medium: Option<CodeableConcept>,
    /// Volume number of journal or other collection in which the article is published
    pub volume: Option<StringType>,
    /// Extension element for the 'volume' primitive field. Contains metadata and extensions.
    pub _volume: Option<Element>,
    /// Issue, part or supplement of journal or other collection in which the article is published
    pub issue: Option<StringType>,
    /// Extension element for the 'issue' primitive field. Contains metadata and extensions.
    pub _issue: Option<Element>,
    /// The date the article was added to the database, or the date the article was released
    #[serde(rename = "articleDate")]
    pub article_date: Option<DateTimeType>,
    /// Extension element for the 'articleDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_articleDate")]
    pub _article_date: Option<Element>,
    /// Text representation of the date on which the issue of the cited artifact was published
    #[serde(rename = "publicationDateText")]
    pub publication_date_text: Option<StringType>,
    /// Extension element for the 'publicationDateText' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationDateText")]
    pub _publication_date_text: Option<Element>,
    /// Season in which the cited artifact was published
    #[serde(rename = "publicationDateSeason")]
    pub publication_date_season: Option<StringType>,
    /// Extension element for the 'publicationDateSeason' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationDateSeason")]
    pub _publication_date_season: Option<Element>,
    /// The date the article was last revised or updated in the database
    #[serde(rename = "lastRevisionDate")]
    pub last_revision_date: Option<DateTimeType>,
    /// Extension element for the 'lastRevisionDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastRevisionDate")]
    pub _last_revision_date: Option<Element>,
    /// Language(s) in which this form of the article is published
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bg`: Bulgarian
    /// - `bg-BG`: Bulgarian (Bulgaria)
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `cs-CZ`: Czech (Czechia)
    /// - `bs`: Bosnian
    /// - `bs-BA`: Bosnian (Bosnia and Herzegovina)
    /// - `da`: Danish
    /// - `da-DK`: Danish (Denmark)
    /// - ... and 72 more values
    pub language: Option<Vec<StringType>>,
    /// Entry number or identifier for inclusion in a database
    #[serde(rename = "accessionNumber")]
    pub accession_number: Option<StringType>,
    /// Extension element for the 'accessionNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_accessionNumber")]
    pub _accession_number: Option<Element>,
    /// Used for full display of pagination
    #[serde(rename = "pageString")]
    pub page_string: Option<StringType>,
    /// Extension element for the 'pageString' primitive field. Contains metadata and extensions.
    #[serde(rename = "_pageString")]
    pub _page_string: Option<Element>,
    /// Used for isolated representation of first page
    #[serde(rename = "firstPage")]
    pub first_page: Option<StringType>,
    /// Extension element for the 'firstPage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_firstPage")]
    pub _first_page: Option<Element>,
    /// Used for isolated representation of last page
    #[serde(rename = "lastPage")]
    pub last_page: Option<StringType>,
    /// Extension element for the 'lastPage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastPage")]
    pub _last_page: Option<Element>,
    /// Number of pages or screens
    #[serde(rename = "pageCount")]
    pub page_count: Option<StringType>,
    /// Extension element for the 'pageCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_pageCount")]
    pub _page_count: Option<Element>,
    /// Copyright notice for the full article or artifact
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
}
/// CitationCitedartifact nested structure for the 'statusDate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCitedartifactStatusdate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Classification of the status
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/cited-artifact-status-type
    pub activity: CodeableConcept,
    /// Either occurred or expected
    pub actual: Option<BooleanType>,
    /// Extension element for the 'actual' primitive field. Contains metadata and extensions.
    pub _actual: Option<Element>,
    /// When the status started and/or ended
    pub period: Period,
}

impl Default for Citation {
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
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
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
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            summary: Default::default(),
            classification: Default::default(),
            note: Default::default(),
            current_state: Default::default(),
            status_date: Default::default(),
            related_artifact: Default::default(),
            cited_artifact: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactTitle {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            language: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactAbstract {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            language: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactContributorship {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            complete: Default::default(),
            _complete: Default::default(),
        }
    }
}

impl Default for CitationSummary {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            style: Default::default(),
            text: StringType::default(),
            _text: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactWeblocation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            classifier: Default::default(),
            url: Default::default(),
            _url: Default::default(),
        }
    }
}

impl Default for CitationCitedartifact {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            status_date: Default::default(),
            abstract_: Default::default(),
            relates_to: Default::default(),
            part: Default::default(),
            publication_form: Default::default(),
            version: Default::default(),
            classification: Default::default(),
            contributorship: Default::default(),
            web_location: Default::default(),
            identifier: Default::default(),
            related_identifier: Default::default(),
            date_accessed: Default::default(),
            _date_accessed: Default::default(),
            current_state: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactClassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            classifier: Default::default(),
            artifact_assessment: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value: Default::default(),
            _value: Default::default(),
            base_citation: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactPart {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            base_citation: Default::default(),
        }
    }
}

impl Default for CitationStatusdate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            activity: Default::default(),
            actual: Default::default(),
            _actual: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            classifier: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            citation: Default::default(),
            _citation: Default::default(),
            document: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            resource_reference: Default::default(),
        }
    }
}

impl Default for CitationClassification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            classifier: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactPublicationform {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            cited_medium: Default::default(),
            volume: Default::default(),
            _volume: Default::default(),
            issue: Default::default(),
            _issue: Default::default(),
            article_date: Default::default(),
            _article_date: Default::default(),
            publication_date_text: Default::default(),
            _publication_date_text: Default::default(),
            publication_date_season: Default::default(),
            _publication_date_season: Default::default(),
            last_revision_date: Default::default(),
            _last_revision_date: Default::default(),
            language: Default::default(),
            accession_number: Default::default(),
            _accession_number: Default::default(),
            page_string: Default::default(),
            _page_string: Default::default(),
            first_page: Default::default(),
            _first_page: Default::default(),
            last_page: Default::default(),
            _last_page: Default::default(),
            page_count: Default::default(),
            _page_count: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
        }
    }
}

impl Default for CitationCitedartifactStatusdate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            activity: Default::default(),
            actual: Default::default(),
            _actual: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
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
                "Citation.citedArtifact.relatesTo.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/related-artifact-type-all|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Citation.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Citation.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementCardinality::new("Citation.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.meta", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.implicitRules", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.text", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.contained", 0, None),
    rh_foundation::ElementCardinality::new("Citation.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.url", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.identifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.version", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.versionAlgorithm[x]", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.name", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.title", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.status", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.experimental", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.date", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.publisher", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.contact", 0, None),
    rh_foundation::ElementCardinality::new("Citation.description", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.useContext", 0, None),
    rh_foundation::ElementCardinality::new("Citation.jurisdiction", 0, None),
    rh_foundation::ElementCardinality::new("Citation.purpose", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.copyright", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.copyrightLabel", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.approvalDate", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.lastReviewDate", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.effectivePeriod", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.author", 0, None),
    rh_foundation::ElementCardinality::new("Citation.editor", 0, None),
    rh_foundation::ElementCardinality::new("Citation.reviewer", 0, None),
    rh_foundation::ElementCardinality::new("Citation.endorser", 0, None),
    rh_foundation::ElementCardinality::new("Citation.summary", 0, None),
    rh_foundation::ElementCardinality::new("Citation.summary.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.summary.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.summary.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.summary.style", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.summary.text", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.classification", 0, None),
    rh_foundation::ElementCardinality::new("Citation.classification.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.classification.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.classification.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.classification.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.classification.classifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.note", 0, None),
    rh_foundation::ElementCardinality::new("Citation.currentState", 0, None),
    rh_foundation::ElementCardinality::new("Citation.statusDate", 0, None),
    rh_foundation::ElementCardinality::new("Citation.statusDate.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.statusDate.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.statusDate.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.statusDate.activity", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.statusDate.actual", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.statusDate.period", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.relatedArtifact", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.identifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatedIdentifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.dateAccessed", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version.value", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.version.baseCitation", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.currentState", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.activity", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.actual", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.statusDate.period", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.type", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.title.text", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.language", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.text", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.abstract.copyright", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.value", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.part.baseCitation", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.classifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.label", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.display", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.citation", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.document", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.resource", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.relatesTo.resourceReference", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.identifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.title", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.publisher", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publishedIn.publisherLocation", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.citedMedium", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.volume", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.issue", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.articleDate", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publicationDateText", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.publicationDateSeason", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.lastRevisionDate", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.language", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.accessionNumber", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.pageString", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.firstPage", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.lastPage", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.pageCount", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.publicationForm.copyright", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation.classifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.webLocation.url", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.classifier", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.classification.artifactAssessment", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.complete", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributor", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.forenameInitials", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.affiliation", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionType", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.role", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance.type", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.contributionInstance.time", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.correspondingContact", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.entry.rankingOrder", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.id", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.extension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.modifierExtension", 0, None),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.type", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.style", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.source", 0, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.contributorship.summary.value", 1, Some(1)),
    rh_foundation::ElementCardinality::new("Citation.citedArtifact.note", 0, None),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Citation {
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

impl crate::traits::resource::ResourceMutators for Citation {
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

impl crate::traits::resource::ResourceExistence for Citation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Citation {
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

impl crate::traits::domain_resource::DomainResourceMutators for Citation {
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

impl crate::traits::domain_resource::DomainResourceExistence for Citation {
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

impl crate::traits::citation::CitationAccessors for Citation {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
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
    fn author(&self) -> &[ContactDetail] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_deref().unwrap_or(&[])
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_deref().unwrap_or(&[])
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_deref().unwrap_or(&[])
    }
    fn summary(&self) -> &[CitationSummary] {
        self.summary.as_deref().unwrap_or(&[])
    }
    fn classification(&self) -> &[CitationClassification] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn current_state(&self) -> &[CodeableConcept] {
        self.current_state.as_deref().unwrap_or(&[])
    }
    fn status_date(&self) -> &[CitationStatusdate] {
        self.status_date.as_deref().unwrap_or(&[])
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_deref().unwrap_or(&[])
    }
    fn cited_artifact(&self) -> Option<CitationCitedartifact> {
        self.cited_artifact.clone()
    }
}

impl crate::traits::citation::CitationMutators for Citation {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
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
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = Some(value);
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = Some(value);
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = Some(value);
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_summary(self, value: Vec<CitationSummary>) -> Self {
        let mut resource = self.clone();
        resource.summary = Some(value);
        resource
    }
    fn add_summary(self, item: CitationSummary) -> Self {
        let mut resource = self.clone();
        resource.summary.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_classification(self, value: Vec<CitationClassification>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: CitationClassification) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_current_state(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.current_state = Some(value);
        resource
    }
    fn add_current_state(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .current_state
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status_date(self, value: Vec<CitationStatusdate>) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn add_status_date(self, item: CitationStatusdate) -> Self {
        let mut resource = self.clone();
        resource.status_date.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = Some(value);
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource
            .related_artifact
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_cited_artifact(self, value: CitationCitedartifact) -> Self {
        let mut resource = self.clone();
        resource.cited_artifact = Some(value);
        resource
    }
}

impl crate::traits::citation::CitationExistence for Citation {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_editor(&self) -> bool {
        self.editor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reviewer(&self) -> bool {
        self.reviewer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endorser(&self) -> bool {
        self.endorser.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_summary(&self) -> bool {
        self.summary.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_current_state(&self) -> bool {
        self.current_state.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status_date(&self) -> bool {
        self.status_date.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_artifact(&self) -> bool {
        self.related_artifact
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_cited_artifact(&self) -> bool {
        self.cited_artifact.is_some()
    }
}

impl crate::validation::ValidatableResource for Citation {
    fn resource_type(&self) -> &'static str {
        "Citation"
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
        Some("http://hl7.org/fhir/StructureDefinition/Citation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::citation::{CitationAccessors, CitationExistence, CitationMutators};
