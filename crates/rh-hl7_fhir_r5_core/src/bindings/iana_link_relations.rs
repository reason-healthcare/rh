use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/iana-link-relations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IanaLinkRelations {
    /// Refers to a resource that is the subject of the link's context.
    #[serde(rename = "about")]
    About,
    /// Asserts that the link target provides an access control description for the link context.
    #[serde(rename = "acl")]
    Acl,
    /// Refers to a substitute for this context
    #[serde(rename = "alternate")]
    Alternate,
    /// Used to reference alternative content that uses the AMP profile of the HTML format.
    #[serde(rename = "amphtml")]
    Amphtml,
    /// Refers to an appendix.
    #[serde(rename = "appendix")]
    Appendix,
    /// Refers to an icon for the context. Synonym for icon.
    #[serde(rename = "apple-touch-icon")]
    AppleTouchIcon,
    /// Refers to a launch screen for the context.
    #[serde(rename = "apple-touch-startup-image")]
    AppleTouchStartupImage,
    /// Refers to a collection of records, documents, or other
    ///      materials of historical interest.
    #[serde(rename = "archives")]
    Archives,
    /// Refers to the context's author.
    #[serde(rename = "author")]
    Author,
    /// Identifies the entity that blocks access to a resource
    ///      following receipt of a legal demand.
    #[serde(rename = "blocked-by")]
    BlockedBy,
    /// Gives a permanent link to use for bookmarking purposes.
    #[serde(rename = "bookmark")]
    Bookmark,
    /// Designates the preferred version of a resource (the IRI and its contents).
    #[serde(rename = "canonical")]
    Canonical,
    /// Refers to a chapter in a collection of resources.
    #[serde(rename = "chapter")]
    Chapter,
    /// Indicates that the link target is preferred over the link context for the purpose of permanent citation.
    #[serde(rename = "cite-as")]
    CiteAs,
    /// The target IRI points to a resource which represents the collection resource for the context IRI.
    #[serde(rename = "collection")]
    Collection,
    /// Refers to a table of contents.
    #[serde(rename = "contents")]
    Contents,
    /// The document linked to was later converted to the
    ///      document that contains this link relation.  For example, an RFC can
    ///      have a link to the Internet-Draft that became the RFC; in that case,
    ///      the link relation would be "convertedFrom".
    #[serde(rename = "convertedFrom")]
    ConvertedFrom,
    /// Refers to a copyright statement that applies to the
    ///    link's context.
    #[serde(rename = "copyright")]
    Copyright,
    /// The target IRI points to a resource where a submission form can be obtained.
    #[serde(rename = "create-form")]
    CreateForm,
    /// Refers to a resource containing the most recent
    ///      item(s) in a collection of resources.
    #[serde(rename = "current")]
    Current,
    /// Refers to a resource providing information about the
    ///      link's context.
    #[serde(rename = "describedby")]
    Describedby,
    /// The relationship A 'describes' B asserts that
    ///      resource A provides a description of resource B. There are no
    ///      constraints on the format or representation of either A or B,
    ///      neither are there any further constraints on either resource.
    #[serde(rename = "describes")]
    Describes,
    /// Refers to a list of patent disclosures made with respect to
    ///      material for which 'disclosure' relation is specified.
    #[serde(rename = "disclosure")]
    Disclosure,
    /// Used to indicate an origin that will be used to fetch required
    ///      resources for the link context, and that the user agent ought to resolve
    ///      as early as possible.
    #[serde(rename = "dns-prefetch")]
    DnsPrefetch,
    /// Refers to a resource whose available representations
    ///      are byte-for-byte identical with the corresponding representations of
    ///      the context IRI.
    #[serde(rename = "duplicate")]
    Duplicate,
    /// Refers to a resource that can be used to edit the
    ///      link's context.
    #[serde(rename = "edit")]
    Edit,
    /// The target IRI points to a resource where a submission form for
    ///      editing associated resource can be obtained.
    #[serde(rename = "edit-form")]
    EditForm,
    /// Refers to a resource that can be used to edit media
    ///      associated with the link's context.
    #[serde(rename = "edit-media")]
    EditMedia,
    /// Identifies a related resource that is potentially
    ///      large and might require special handling.
    #[serde(rename = "enclosure")]
    Enclosure,
    /// Refers to a resource that is not part of the same site as the current context.
    #[serde(rename = "external")]
    External,
    /// An IRI that refers to the furthest preceding resource
    ///    in a series of resources.
    #[serde(rename = "first")]
    First,
    /// Refers to a glossary of terms.
    #[serde(rename = "glossary")]
    Glossary,
    /// Refers to context-sensitive help.
    #[serde(rename = "help")]
    Help,
    /// Refers to a resource hosted by the server indicated by
    ///      the link context.
    #[serde(rename = "hosts")]
    Hosts,
    /// Refers to a hub that enables registration for
    ///    notification of updates to the context.
    #[serde(rename = "hub")]
    Hub,
    /// Refers to an icon representing the link's context.
    #[serde(rename = "icon")]
    Icon,
    /// Refers to an index.
    #[serde(rename = "index")]
    Index,
    /// refers to a resource associated with a time interval that ends before the beginning of the time interval associated with the context resource
    #[serde(rename = "intervalAfter")]
    IntervalAfter,
    /// refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource
    #[serde(rename = "intervalBefore")]
    IntervalBefore,
    /// refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource
    #[serde(rename = "intervalContains")]
    IntervalContains,
    /// refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource, or ends before the beginning of the time interval associated with the context resource
    #[serde(rename = "intervalDisjoint")]
    IntervalDisjoint,
    /// refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource
    #[serde(rename = "intervalDuring")]
    IntervalDuring,
    /// refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource
    #[serde(rename = "intervalEquals")]
    IntervalEquals,
    /// refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource
    #[serde(rename = "intervalFinishedBy")]
    IntervalFinishedBy,
    /// refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource
    #[serde(rename = "intervalFinishes")]
    IntervalFinishes,
    /// refers to a resource associated with a time interval that begins before or is coincident with the beginning of the time interval associated with the context resource, and ends after or is coincident with the end of the time interval associated with the context resource
    #[serde(rename = "intervalIn")]
    IntervalIn,
    /// refers to a resource associated with a time interval whose beginning coincides with the end of the time interval associated with the context resource
    #[serde(rename = "intervalMeets")]
    IntervalMeets,
    /// refers to a resource associated with a time interval whose end coincides with the beginning of the time interval associated with the context resource
    #[serde(rename = "intervalMetBy")]
    IntervalMetBy,
    /// refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the beginning of the time interval associated with the context resource
    #[serde(rename = "intervalOverlappedBy")]
    IntervalOverlappedBy,
    /// refers to a resource associated with a time interval that begins before the end of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource
    #[serde(rename = "intervalOverlaps")]
    IntervalOverlaps,
    /// refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource
    #[serde(rename = "intervalStartedBy")]
    IntervalStartedBy,
    /// refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource
    #[serde(rename = "intervalStarts")]
    IntervalStarts,
    /// The target IRI points to a resource that is a member of the collection represented by the context IRI.
    #[serde(rename = "item")]
    Item,
    /// An IRI that refers to the furthest following resource
    ///      in a series of resources.
    #[serde(rename = "last")]
    Last,
    /// Points to a resource containing the latest (e.g.,
    ///      current) version of the context.
    #[serde(rename = "latest-version")]
    LatestVersion,
    /// Refers to a license associated with this context.
    #[serde(rename = "license")]
    License,
    /// The link target of a link with the "linkset" relation
    ///      type provides a set of links, including links in which the link
    ///      context of the link participates.
    ///
    #[serde(rename = "linkset")]
    Linkset,
    /// Refers to further information about the link's context,
    ///      expressed as a LRDD ("Link-based Resource Descriptor Document")
    ///      resource.  See  for information about
    ///      processing this relation type in host-meta documents. When used
    ///      elsewhere, it refers to additional links and other metadata.
    ///      Multiple instances indicate additional LRDD resources. LRDD
    ///      resources MUST have an "application/xrd+xml" representation, and
    ///      MAY have others.
    #[serde(rename = "lrdd")]
    Lrdd,
    /// Links to a manifest file for the context.
    #[serde(rename = "manifest")]
    Manifest,
    /// Refers to a mask that can be applied to the icon for the context.
    #[serde(rename = "mask-icon")]
    MaskIcon,
    /// Refers to a feed of personalised media recommendations relevant to the link context.
    #[serde(rename = "media-feed")]
    MediaFeed,
    /// The Target IRI points to a Memento, a fixed resource that will not change state anymore.
    #[serde(rename = "memento")]
    Memento,
    /// Links to the context's Micropub endpoint.
    #[serde(rename = "micropub")]
    Micropub,
    /// Refers to a module that the user agent is to preemptively fetch and store for use in the current context.
    #[serde(rename = "modulepreload")]
    Modulepreload,
    /// Refers to a resource that can be used to monitor changes in an HTTP resource.
    ///
    #[serde(rename = "monitor")]
    Monitor,
    /// Refers to a resource that can be used to monitor changes in a specified group of HTTP resources.
    ///
    #[serde(rename = "monitor-group")]
    MonitorGroup,
    /// Indicates that the link's context is a part of a series, and
    ///      that the next in the series is the link target.
    ///
    #[serde(rename = "next")]
    Next,
    /// Refers to the immediately following archive resource.
    #[serde(rename = "next-archive")]
    NextArchive,
    /// Indicates that the context’s original author or publisher does not endorse the link target.
    #[serde(rename = "nofollow")]
    Nofollow,
    /// Indicates that any newly created top-level browsing context which results from following the link will not be an auxiliary browsing context.
    #[serde(rename = "noopener")]
    Noopener,
    /// Indicates that no referrer information is to be leaked when following the link.
    #[serde(rename = "noreferrer")]
    Noreferrer,
    /// Indicates that any newly created top-level browsing context which results from following the link will be an auxiliary browsing context.
    #[serde(rename = "opener")]
    Opener,
    /// Refers to an OpenID Authentication server on which the context relies for an assertion that the end user controls an Identifier.
    #[serde(rename = "openid2.local_id")]
    Openid2LocalId,
    /// Refers to a resource which accepts OpenID Authentication protocol messages for the context.
    #[serde(rename = "openid2.provider")]
    Openid2Provider,
    /// The Target IRI points to an Original Resource.
    #[serde(rename = "original")]
    Original,
    /// Refers to a P3P privacy policy for the context.
    #[serde(rename = "P3Pv1")]
    P3Pv1,
    /// Indicates a resource where payment is accepted.
    #[serde(rename = "payment")]
    Payment,
    /// Gives the address of the pingback resource for the link context.
    #[serde(rename = "pingback")]
    Pingback,
    /// Used to indicate an origin that will be used to fetch required
    ///      resources for the link context. Initiating an early connection, which
    ///      includes the DNS lookup, TCP handshake, and optional TLS negotiation,
    ///      allows the user agent to mask the high latency costs of establishing a
    ///      connection.
    #[serde(rename = "preconnect")]
    Preconnect,
    /// Points to a resource containing the predecessor
    ///      version in the version history.
    ///
    #[serde(rename = "predecessor-version")]
    PredecessorVersion,
    /// The prefetch link relation type is used to identify a resource
    ///      that might be required by the next navigation from the link context, and
    ///      that the user agent ought to fetch, such that the user agent can deliver a
    ///      faster response once the resource is requested in the future.
    #[serde(rename = "prefetch")]
    Prefetch,
    /// Refers to a resource that should be loaded early in the
    ///      processing of the link's context, without blocking rendering.
    #[serde(rename = "preload")]
    Preload,
    /// Used to identify a resource that might be required by the next
    ///      navigation from the link context, and that the user agent ought to fetch
    ///      and execute, such that the user agent can deliver a faster response once
    ///      the resource is requested in the future.
    #[serde(rename = "prerender")]
    Prerender,
    /// Indicates that the link's context is a part of a series, and
    ///      that the previous in the series is the link target.
    ///
    #[serde(rename = "prev")]
    Prev,
    /// Refers to a resource that provides a preview of the link's context.
    #[serde(rename = "preview")]
    Preview,
    /// Refers to the previous resource in an ordered series
    ///      of resources.  Synonym for "prev".
    #[serde(rename = "previous")]
    Previous,
    /// Refers to the immediately preceding archive resource.
    #[serde(rename = "prev-archive")]
    PrevArchive,
    /// Refers to a privacy policy associated with the link's context.
    #[serde(rename = "privacy-policy")]
    PrivacyPolicy,
    /// Identifying that a resource representation conforms
    ///to a certain profile, without affecting the non-profile semantics
    ///of the resource representation.
    #[serde(rename = "profile")]
    Profile,
    /// Links to a publication manifest. A manifest represents
    ///      structured information about a publication, such as informative metadata,
    ///      a list of resources, and a default reading order.
    #[serde(rename = "publication")]
    Publication,
    /// Identifies a related resource.
    #[serde(rename = "related")]
    Related,
    /// Identifies the root of RESTCONF API as configured on this HTTP server.
    ///      The "restconf" relation defines the root of the API defined in RFC8040.
    ///      Subsequent revisions of RESTCONF will use alternate relation values to support
    ///      protocol versioning.
    #[serde(rename = "restconf")]
    Restconf,
    /// Identifies a resource that is a reply to the context
    ///      of the link.
    ///
    #[serde(rename = "replies")]
    Replies,
    /// The resource identified by the link target provides an input value to an
    ///    instance of a rule, where the resource which represents the rule instance is
    ///    identified by the link context.
    ///
    #[serde(rename = "ruleinput")]
    Ruleinput,
    /// Refers to a resource that can be used to search through
    ///      the link's context and related resources.
    #[serde(rename = "search")]
    Search,
    /// Refers to a section in a collection of resources.
    #[serde(rename = "section")]
    Section,
    /// Conveys an identifier for the link's context.
    ///
    #[serde(rename = "self")]
    SelfValue,
    /// Indicates a URI that can be used to retrieve a
    ///      service document.
    #[serde(rename = "service")]
    Service,
    /// Identifies service description for the context that
    ///      is primarily intended for consumption by machines.
    #[serde(rename = "service-desc")]
    ServiceDesc,
    /// Identifies service documentation for the context that
    ///      is primarily intended for human consumption.
    #[serde(rename = "service-doc")]
    ServiceDoc,
    /// Identifies general metadata for the context that is
    ///      primarily intended for consumption by machines.
    #[serde(rename = "service-meta")]
    ServiceMeta,
    /// Refers to a resource that is within a context that is
    ///		sponsored (such as advertising or another compensation agreement).
    #[serde(rename = "sponsored")]
    Sponsored,
    /// Refers to the first resource in a collection of
    ///      resources.
    #[serde(rename = "start")]
    Start,
    /// Identifies a resource that represents the context's
    ///      status.
    #[serde(rename = "status")]
    Status,
    /// Refers to a stylesheet.
    #[serde(rename = "stylesheet")]
    Stylesheet,
    /// Refers to a resource serving as a subsection in a
    ///      collection of resources.
    #[serde(rename = "subsection")]
    Subsection,
    /// Points to a resource containing the successor version
    ///      in the version history.
    ///
    #[serde(rename = "successor-version")]
    SuccessorVersion,
    /// Identifies a resource that provides information about
    ///      the context's retirement policy.
    ///
    #[serde(rename = "sunset")]
    Sunset,
    /// Gives a tag (identified by the given address) that applies to
    ///      the current document.
    ///
    #[serde(rename = "tag")]
    Tag,
    /// Refers to the terms of service associated with the link's context.
    #[serde(rename = "terms-of-service")]
    TermsOfService,
    /// The Target IRI points to a TimeGate for an Original Resource.
    #[serde(rename = "timegate")]
    Timegate,
    /// The Target IRI points to a TimeMap for an Original Resource.
    #[serde(rename = "timemap")]
    Timemap,
    /// Refers to a resource identifying the abstract semantic type of which the link's context is considered to be an instance.
    #[serde(rename = "type")]
    TypeValue,
    /// Refers to a resource that is within a context that is User Generated Content.
    ///
    #[serde(rename = "ugc")]
    Ugc,
    /// Refers to a parent document in a hierarchy of
    ///      documents.
    ///
    #[serde(rename = "up")]
    Up,
    /// Points to a resource containing the version history
    ///      for the context.
    ///
    #[serde(rename = "version-history")]
    VersionHistory,
    /// Identifies a resource that is the source of the
    ///      information in the link's context.
    ///
    #[serde(rename = "via")]
    Via,
    /// Identifies a target URI that supports the Webmention protocol.
    ///    This allows clients that mention a resource in some form of publishing process
    ///    to contact that endpoint and inform it that this resource has been mentioned.
    #[serde(rename = "webmention")]
    Webmention,
    /// Points to a working copy for this resource.
    #[serde(rename = "working-copy")]
    WorkingCopy,
    /// Points to the versioned resource from which this
    ///      working copy was obtained.
    ///
    #[serde(rename = "working-copy-of")]
    WorkingCopyOf,
}
impl Default for IanaLinkRelations {
    fn default() -> Self {
        Self::About
    }
}
