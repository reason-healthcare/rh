use crate::bindings::narrative_status::NarrativeStatus;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Narrative
///
/// Base StructureDefinition for Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Narrative
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Narrative
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// generated | extensions | additional | empty
    pub status: NarrativeStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Limited xhtml content
    pub div: StringType,
    /// Extension element for the 'div' primitive field. Contains metadata and extensions.
    pub _div: Option<Element>,
}
/// Narrative Link
///
/// A human language representation of the concept (resource/element), as a url that is a reference to a portion of the narrative of a resource ([DomainResource.text](narrative.html)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/narrativeLink
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Narrative {
    fn default() -> Self {
        Self {
            base: Element::default(),
            status: NarrativeStatus::default(),
            _status: Default::default(),
            div: StringType::default(),
            _div: Default::default(),
        }
    }
}

impl Default for NarrativeLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("txt-1", rh_foundation::Severity::Error, "The narrative SHALL contain only the basic html formatting elements and attributes described in chapters 7-11 (except section 4 of chapter 9) and 15 of the HTML 4.0 standard, <a> elements (either name or href), images and internally contained style attributes", "htmlChecks()").with_xpath("not(descendant-or-self::*[not(local-name(.)=('a', 'abbr', 'acronym', 'b', 'big', 'blockquote', 'br', 'caption', 'cite', 'code', 'col', 'colgroup', 'dd', 'dfn', 'div', 'dl', 'dt', 'em', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'hr', 'i', 'img', 'li', 'ol', 'p', 'pre', 'q', 'samp', 'small', 'span', 'strong', 'sub', 'sup', 'table', 'tbody', 'td', 'tfoot', 'th', 'thead', 'tr', 'tt', 'ul', 'var'))]) and not(descendant-or-self::*/@*[not(name(.)=('abbr', 'accesskey', 'align', 'alt', 'axis', 'bgcolor', 'border', 'cellhalign', 'cellpadding', 'cellspacing', 'cellvalign', 'char', 'charoff', 'charset', 'cite', 'class', 'colspan', 'compact', 'coords', 'dir', 'frame', 'headers', 'height', 'href', 'hreflang', 'hspace', 'id', 'lang', 'longdesc', 'name', 'nowrap', 'rel', 'rev', 'rowspan', 'rules', 'scope', 'shape', 'span', 'src', 'start', 'style', 'summary', 'tabindex', 'title', 'type', 'valign', 'value', 'vspace', 'width'))])"),
    rh_foundation::Invariant::new("txt-2", rh_foundation::Severity::Error, "The narrative SHALL have some non-whitespace content", "htmlChecks()").with_xpath("descendant::text()[normalize-space(.)!=''] or descendant::h:img[@src]"),
]
    });

impl crate::validation::ValidatableResource for Narrative {
    fn resource_type(&self) -> &'static str {
        "Narrative"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Narrative")
    }
}
