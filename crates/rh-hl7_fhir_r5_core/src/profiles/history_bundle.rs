use crate::resources::bundle::Bundle;
use serde::{Deserialize, Serialize};
/// History bundle
///
/// This profile holds all the requirements and constraints related to a FHIR history bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/history-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryBundle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Bundle,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("bdl-1", rh_foundation::Severity::Error, "total only when a search or history", "total.empty() or (type = 'searchset') or (type = 'history')"),
    rh_foundation::Invariant::new("bdl-10", rh_foundation::Severity::Error, "A document must have a date", "type = 'document' implies (timestamp.hasValue())"),
    rh_foundation::Invariant::new("bdl-11", rh_foundation::Severity::Error, "A document must have a Composition as the first resource", "type = 'document' implies entry.first().resource.is(Composition)"),
    rh_foundation::Invariant::new("bdl-12", rh_foundation::Severity::Error, "A message must have a MessageHeader as the first resource", "type = 'message' implies entry.first().resource.is(MessageHeader)"),
    rh_foundation::Invariant::new("bdl-13", rh_foundation::Severity::Error, "A subscription-notification must have a SubscriptionStatus as the first resource", "type = 'subscription-notification' implies entry.first().resource.is(SubscriptionStatus)"),
    rh_foundation::Invariant::new("bdl-14", rh_foundation::Severity::Error, "entry.request.method PATCH not allowed for history", "type = 'history' implies entry.request.method != 'PATCH'"),
    rh_foundation::Invariant::new("bdl-15", rh_foundation::Severity::Error, "Bundle resources where type is not transaction, transaction-response, batch, or batch-response or when the request is a POST SHALL have Bundle.entry.fullUrl populated", "type='transaction' or type='transaction-response' or type='batch' or type='batch-response' or entry.all(fullUrl.exists() or request.method='POST')"),
    rh_foundation::Invariant::new("bdl-16", rh_foundation::Severity::Error, "Issue.severity for all issues within the OperationOutcome must be either 'information' or 'warning'.", "issues.exists() implies (issues.issue.severity = 'information' or issues.issue.severity = 'warning')"),
    rh_foundation::Invariant::new("bdl-17", rh_foundation::Severity::Error, "Use and meaning of issues for documents has not been validated because the content will not be rendered in the document.", "type = 'document' implies issues.empty()"),
    rh_foundation::Invariant::new("bdl-18", rh_foundation::Severity::Error, "Self link is required for searchsets.", "type = 'searchset' implies link.where(relation = 'self' and url.exists()).exists()"),
    rh_foundation::Invariant::new("bdl-2", rh_foundation::Severity::Error, "entry.search only when a search", "(type = 'searchset') or entry.search.empty()"),
    rh_foundation::Invariant::new("bdl-3a", rh_foundation::Severity::Error, "For collections of type document, message, searchset or collection, all entries must contain resources, and not have request or response elements", "type in ('document' | 'message' | 'searchset' | 'collection') implies entry.all(resource.exists() and request.empty() and response.empty())"),
    rh_foundation::Invariant::new("bdl-3b", rh_foundation::Severity::Error, "For collections of type history, all entries must contain request or response elements, and resources if the method is POST, PUT or PATCH", "type = 'history' implies entry.all(request.exists() and response.exists() and ((request.method in ('POST' | 'PATCH' | 'PUT')) = resource.exists()))"),
    rh_foundation::Invariant::new("bdl-3c", rh_foundation::Severity::Error, "For collections of type transaction or batch, all entries must contain request elements, and resources if the method is POST, PUT or PATCH", "type in ('transaction' | 'batch') implies entry.all(request.method.exists() and ((request.method in ('POST' | 'PATCH' | 'PUT')) = resource.exists()))"),
    rh_foundation::Invariant::new("bdl-3d", rh_foundation::Severity::Error, "For collections of type transaction-response or batch-response, all entries must contain response elements", "type in ('transaction-response' | 'batch-response') implies entry.all(response.exists())"),
    rh_foundation::Invariant::new("bdl-5", rh_foundation::Severity::Error, "must be a resource unless there's a request or response", "resource.exists() or request.exists() or response.exists()"),
    rh_foundation::Invariant::new("bdl-7", rh_foundation::Severity::Error, "FullUrl must be unique in a bundle, or else entries with the same fullUrl must have different meta.versionId (except in history bundles)", "(type = 'history') or entry.where(fullUrl.exists()).select(fullUrl&iif(resource.meta.versionId.exists(), resource.meta.versionId, '')).isDistinct()"),
    rh_foundation::Invariant::new("bdl-8", rh_foundation::Severity::Error, "fullUrl cannot be a version specific reference", "fullUrl.exists() implies fullUrl.contains('/_history/').not()"),
    rh_foundation::Invariant::new("bdl-9", rh_foundation::Severity::Error, "A document must have an identifier with a system and a value", "type = 'document' implies (identifier.system.exists() and identifier.value.exists())"),
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
    rh_foundation::ElementBinding::new("Bundle.entry.request.method", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/http-verb|5.0.0").with_description("HTTP verbs (in the HTTP command line). See [HTTP rfc](https://tools.ietf.org/html/rfc7231) for details."),
    rh_foundation::ElementBinding::new("Bundle.entry.search.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-entry-mode|5.0.0").with_description("Why an entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process."),
    rh_foundation::ElementBinding::new("Bundle.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Bundle.link.relation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/iana-link-relations|5.0.0"),
    rh_foundation::ElementBinding::new("Bundle.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/bundle-type|5.0.0").with_description("Indicates the purpose of a bundle - how it is intended to be used."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Bundle", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.timestamp", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.total", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.link.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.link.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.link.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.link.relation", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.link.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Bundle.entry.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.extension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.link", 0, None),
            rh_foundation::ElementCardinality::new("Bundle.entry.fullUrl", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.search.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.search.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.method", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.request.ifModifiedSince",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifMatch", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.request.ifNoneExist", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.etag", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Bundle.entry.response.lastModified",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Bundle.entry.response.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.signature", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Bundle.issues", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for HistoryBundle {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for HistoryBundle {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for HistoryBundle {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::history_bundle::HistoryBundleMutators for HistoryBundle {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for HistoryBundle {
    fn resource_type(&self) -> &'static str {
        "Bundle"
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
        Some("http://hl7.org/fhir/StructureDefinition/history-bundle")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::history_bundle::{
    HistoryBundleAccessors, HistoryBundleExistence, HistoryBundleMutators,
};
