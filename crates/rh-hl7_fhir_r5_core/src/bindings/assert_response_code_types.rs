use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/assert-response-code-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertResponseCodeTypes {
    /// Continue
    #[serde(rename = "continue")]
    ContinueValue,
    /// Switching Protocols
    #[serde(rename = "switchingProtocols")]
    SwitchingProtocols,
    /// OK
    #[serde(rename = "okay")]
    Okay,
    /// Created
    #[serde(rename = "created")]
    Created,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Non-Authoritative Information
    #[serde(rename = "nonAuthoritativeInformation")]
    NonAuthoritativeInformation,
    /// No Content
    #[serde(rename = "noContent")]
    NoContent,
    /// Reset Content
    #[serde(rename = "resetContent")]
    ResetContent,
    /// Partial Content
    #[serde(rename = "partialContent")]
    PartialContent,
    /// Multiple Choices
    #[serde(rename = "multipleChoices")]
    MultipleChoices,
    /// Moved Permanently
    #[serde(rename = "movedPermanently")]
    MovedPermanently,
    /// Found
    #[serde(rename = "found")]
    Found,
    /// See Other
    #[serde(rename = "seeOther")]
    SeeOther,
    /// Not Modified
    #[serde(rename = "notModified")]
    NotModified,
    /// Use Proxy
    #[serde(rename = "useProxy")]
    UseProxy,
    /// Temporary Redirect
    #[serde(rename = "temporaryRedirect")]
    TemporaryRedirect,
    /// Permanent Redirect
    #[serde(rename = "permanentRedirect")]
    PermanentRedirect,
    /// Bad Request
    #[serde(rename = "badRequest")]
    BadRequest,
    /// Unauthorized
    #[serde(rename = "unauthorized")]
    Unauthorized,
    /// Payment Required
    #[serde(rename = "paymentRequired")]
    PaymentRequired,
    /// Forbidden
    #[serde(rename = "forbidden")]
    Forbidden,
    /// Not Found
    #[serde(rename = "notFound")]
    NotFound,
    /// Method Not Allowed
    #[serde(rename = "methodNotAllowed")]
    MethodNotAllowed,
    /// Not Acceptable
    #[serde(rename = "notAcceptable")]
    NotAcceptable,
    /// Proxy Authentication Required
    #[serde(rename = "proxyAuthenticationRequired")]
    ProxyAuthenticationRequired,
    /// Request Timeout
    #[serde(rename = "requestTimeout")]
    RequestTimeout,
    /// Conflict
    #[serde(rename = "conflict")]
    Conflict,
    /// Gone
    #[serde(rename = "gone")]
    Gone,
    /// Length Required
    #[serde(rename = "lengthRequired")]
    LengthRequired,
    /// Precondition Failed
    #[serde(rename = "preconditionFailed")]
    PreconditionFailed,
    /// Content Too Large
    #[serde(rename = "contentTooLarge")]
    ContentTooLarge,
    /// URI Too Long
    #[serde(rename = "uriTooLong")]
    UriTooLong,
    /// Unsupported Media Type
    #[serde(rename = "unsupportedMediaType")]
    UnsupportedMediaType,
    /// Range Not Satisfiable
    #[serde(rename = "rangeNotSatisfiable")]
    RangeNotSatisfiable,
    /// Expectation Failed
    #[serde(rename = "expectationFailed")]
    ExpectationFailed,
    /// Misdirected Request
    #[serde(rename = "misdirectedRequest")]
    MisdirectedRequest,
    /// Unprocessable Content
    #[serde(rename = "unprocessableContent")]
    UnprocessableContent,
    /// Upgrade Required
    #[serde(rename = "upgradeRequired")]
    UpgradeRequired,
    /// Internal Server Error
    #[serde(rename = "internalServerError")]
    InternalServerError,
    /// Not Implemented
    #[serde(rename = "notImplemented")]
    NotImplemented,
    /// Bad Gateway
    #[serde(rename = "badGateway")]
    BadGateway,
    /// Service Unavailable
    #[serde(rename = "serviceUnavailable")]
    ServiceUnavailable,
    /// Gateway Timeout
    #[serde(rename = "gatewayTimeout")]
    GatewayTimeout,
    /// HTTP Version Not Supported
    #[serde(rename = "httpVersionNotSupported")]
    HttpVersionNotSupported,
}
impl Default for AssertResponseCodeTypes {
    fn default() -> Self {
        Self::ContinueValue
    }
}
