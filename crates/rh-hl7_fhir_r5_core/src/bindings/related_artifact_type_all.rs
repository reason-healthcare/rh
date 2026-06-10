use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/related-artifact-type-all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelatedArtifactTypeAll {
    /// Documentation
    #[serde(rename = "documentation")]
    Documentation,
    /// Justification
    #[serde(rename = "justification")]
    Justification,
    /// Citation
    #[serde(rename = "citation")]
    Citation,
    /// Predecessor
    #[serde(rename = "predecessor")]
    Predecessor,
    /// Successor
    #[serde(rename = "successor")]
    Successor,
    /// Derived From
    #[serde(rename = "derived-from")]
    DerivedFrom,
    /// Depends On
    #[serde(rename = "depends-on")]
    DependsOn,
    /// Composed Of
    #[serde(rename = "composed-of")]
    ComposedOf,
    /// Part Of
    #[serde(rename = "part-of")]
    PartOf,
    /// Amends
    #[serde(rename = "amends")]
    Amends,
    /// Amended With
    #[serde(rename = "amended-with")]
    AmendedWith,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
    /// Appended With
    #[serde(rename = "appended-with")]
    AppendedWith,
    /// Cites
    #[serde(rename = "cites")]
    Cites,
    /// Cited By
    #[serde(rename = "cited-by")]
    CitedBy,
    /// Is Comment On
    #[serde(rename = "comments-on")]
    CommentsOn,
    /// Has Comment In
    #[serde(rename = "comment-in")]
    CommentIn,
    /// Contains
    #[serde(rename = "contains")]
    Contains,
    /// Contained In
    #[serde(rename = "contained-in")]
    ContainedIn,
    /// Corrects
    #[serde(rename = "corrects")]
    Corrects,
    /// Correction In
    #[serde(rename = "correction-in")]
    CorrectionIn,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Replaced With
    #[serde(rename = "replaced-with")]
    ReplacedWith,
    /// Retracts
    #[serde(rename = "retracts")]
    Retracts,
    /// Retracted By
    #[serde(rename = "retracted-by")]
    RetractedBy,
    /// Signs
    #[serde(rename = "signs")]
    Signs,
    /// Similar To
    #[serde(rename = "similar-to")]
    SimilarTo,
    /// Supports
    #[serde(rename = "supports")]
    Supports,
    /// Supported With
    #[serde(rename = "supported-with")]
    SupportedWith,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Transformed Into
    #[serde(rename = "transformed-into")]
    TransformedInto,
    /// Transformed With
    #[serde(rename = "transformed-with")]
    TransformedWith,
    /// Documents
    #[serde(rename = "documents")]
    Documents,
    /// Specification Of
    #[serde(rename = "specification-of")]
    SpecificationOf,
    /// Created With
    #[serde(rename = "created-with")]
    CreatedWith,
    /// Cite As
    #[serde(rename = "cite-as")]
    CiteAs,
    /// Reprint
    #[serde(rename = "reprint")]
    Reprint,
    /// Reprint Of
    #[serde(rename = "reprint-of")]
    ReprintOf,
}
impl Default for RelatedArtifactTypeAll {
    fn default() -> Self {
        Self::Documentation
    }
}
