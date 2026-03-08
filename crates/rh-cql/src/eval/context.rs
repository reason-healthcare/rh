//! # Evaluation Context and Provider Traits
//!
//! This module defines the runtime environment for CQL evaluation:
//!
//! - [`DataProvider`] — trait for retrieving clinical resources
//! - [`TerminologyProvider`] — trait for terminology membership checks
//! - [`EvalContext`] — bundles the deterministic clock, timezone, parameters,
//!   current context value, and provider references
//! - [`InMemoryDataProvider`] — simple in-memory implementation for tests
//! - [`InMemoryTerminologyProvider`] — simple in-memory implementation for tests

use std::collections::HashMap;
use std::sync::Arc;

use super::value::{CqlCode, CqlDate, CqlDateTime, CqlTime, Value};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// A retrieve operation failed.
    RetrieveError(String),
    /// A terminology operation failed.
    TerminologyError(String),
    /// An expression was not found in the library.
    ExpressionNotFound(String),
    /// A general evaluation error.
    General(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::RetrieveError(msg) => write!(f, "Retrieve error: {msg}"),
            EvalError::TerminologyError(msg) => write!(f, "Terminology error: {msg}"),
            EvalError::ExpressionNotFound(name) => {
                write!(f, "Expression not found: '{name}'")
            }
            EvalError::General(msg) => write!(f, "Eval error: {msg}"),
        }
    }
}

impl std::error::Error for EvalError {}

// ---------------------------------------------------------------------------
// DataProvider trait
// ---------------------------------------------------------------------------

/// Provides clinical resource data for `Retrieve` operations.
///
/// Implementors supply lists of `Value` objects that represent resources of a
/// given data type.  The engine passes optional code-filter and date-range
/// criteria; providers may apply the filtering themselves or return unfiltered
/// results (the engine will post-filter).
pub trait DataProvider: Send + Sync {
    /// Retrieve resources of `data_type`.
    ///
    /// # Parameters
    ///
    /// - `data_type`: The model-qualified data type name (e.g. `"Patient"`,
    ///   `"{http://hl7.org/fhir}Observation"`).
    /// - `code_path`: Optional property path for code filtering (e.g. `"code"`).
    /// - `codes`: Optional set of codes to filter by.
    /// - `date_path`: Optional property path for date-range filtering.
    /// - `date_range`: Optional `Value::Interval` bounding the date range.
    ///
    /// Returns a `Vec<Value>` — typically `Value::Tuple` records representing
    /// individual resources.
    fn retrieve(
        &self,
        context: Option<&str>,
        data_type: &str,
        code_path: Option<&str>,
        codes: Option<&Value>,
        date_path: Option<&str>,
        date_range: Option<&Value>,
    ) -> Result<Vec<Value>, EvalError>;
}

// ---------------------------------------------------------------------------
// TerminologyProvider trait
// ---------------------------------------------------------------------------

/// Provides terminology services for valueset membership and code lookup.
pub trait TerminologyProvider: Send + Sync {
    /// Check whether `code` is a member of the valueset identified by
    /// `valueset_url`.
    ///
    /// Returns `Ok(true)` / `Ok(false)` for a definitive answer, or
    /// `Err(EvalError::TerminologyError(...))` when the valueset cannot be
    /// resolved.
    fn in_valueset(&self, code: &CqlCode, valueset_url: &str) -> Result<bool, EvalError>;

    /// Expand the valueset at `valueset_url` to its member codes.
    fn expand_valueset(&self, valueset_url: &str) -> Result<Vec<CqlCode>, EvalError>;

    /// Look up a property of a code.
    ///
    /// Returns `Ok(None)` when the code is found but the property is absent.
    fn lookup(&self, code: &CqlCode, property: &str) -> Result<Option<Value>, EvalError>;
}

// ---------------------------------------------------------------------------
// Clock abstraction
// ---------------------------------------------------------------------------

/// An injectable clock for deterministic date/time evaluation.
///
/// By default, implementations return a fixed timestamp supplied at
/// construction time; production code may supply a real-time clock.
pub trait Clock: Send + Sync {
    /// Returns the current date-time (equivalent to CQL `Now()`).
    fn now(&self) -> CqlDateTime;

    /// Returns today's date (equivalent to CQL `Today()`).
    fn today(&self) -> CqlDate;

    /// Returns the current time of day (equivalent to CQL `TimeOfDay()`).
    fn time_of_day(&self) -> CqlTime;
}

/// A fixed-time clock that always returns the same timestamp.
///
/// Use this in tests or any context where determinism is required.
#[derive(Debug, Clone)]
pub struct FixedClock {
    now: CqlDateTime,
}

impl FixedClock {
    /// Create a new `FixedClock` pinned to the given date-time.
    pub fn new(now: CqlDateTime) -> Self {
        FixedClock { now }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> CqlDateTime {
        self.now.clone()
    }

    fn today(&self) -> CqlDate {
        CqlDate {
            year: self.now.year,
            month: self.now.month,
            day: self.now.day,
        }
    }

    fn time_of_day(&self) -> CqlTime {
        CqlTime {
            hour: self.now.hour.unwrap_or(0),
            minute: self.now.minute,
            second: self.now.second,
            millisecond: self.now.millisecond,
        }
    }
}

// ---------------------------------------------------------------------------
// EvalContext
// ---------------------------------------------------------------------------

/// The runtime evaluation environment for a single CQL evaluation pass.
///
/// `EvalContext` is constructed once per evaluation and passed (by reference)
/// through the entire expression tree.  It is intentionally read-only during
/// evaluation so that sub-expression evaluations are pure functions of their
/// arguments and the context.
///
/// # Example
///
/// ```rust
/// use rh_cql::eval::context::{EvalContext, EvalContextBuilder, FixedClock};
/// use rh_cql::eval::value::{CqlDateTime, Value};
///
/// let clock = FixedClock::new(CqlDateTime {
///     year: 2024,
///     month: Some(1),
///     day: Some(15),
///     hour: Some(10),
///     minute: Some(30),
///     second: Some(0),
///     millisecond: None,
///     offset_seconds: Some(-18000), // UTC-5
/// });
///
/// let ctx = EvalContextBuilder::new(clock).build();
/// assert_eq!(ctx.now().year, 2024);
/// ```
pub struct EvalContext {
    /// The clock used for `Now()`, `Today()`, and `TimeOfDay()`.
    clock: Arc<dyn Clock>,

    /// Timezone offset in seconds east of UTC used as the default when
    /// date/time values do not carry their own offset.
    pub timezone_offset_seconds: i32,

    /// Named parameter values supplied by the caller.
    pub parameters: HashMap<String, Value>,

    /// The current context value (e.g. the Patient resource in Patient
    /// context).
    pub context_value: Option<Value>,

    /// Provider for retrieve operations.
    data_provider: Option<Arc<dyn DataProvider>>,

    /// Provider for terminology operations.
    terminology_provider: Option<Arc<dyn TerminologyProvider>>,
}

impl EvalContext {
    /// Returns the current date-time.
    pub fn now(&self) -> CqlDateTime {
        self.clock.now()
    }

    /// Returns today's date.
    pub fn today(&self) -> CqlDate {
        self.clock.today()
    }

    /// Returns the current time of day.
    pub fn time_of_day(&self) -> CqlTime {
        self.clock.time_of_day()
    }

    /// Look up a named parameter value.
    pub fn parameter(&self, name: &str) -> Option<&Value> {
        self.parameters.get(name)
    }

    /// Retrieve resources via the `DataProvider`, if one is present.
    ///
    /// Returns `Ok(vec![])` when no provider is configured.
    pub fn retrieve(
        &self,
        context: Option<&str>,
        data_type: &str,
        code_path: Option<&str>,
        codes: Option<&Value>,
        date_path: Option<&str>,
        date_range: Option<&Value>,
    ) -> Result<Vec<Value>, EvalError> {
        match &self.data_provider {
            Some(p) => p.retrieve(context, data_type, code_path, codes, date_path, date_range),
            None => Ok(vec![]),
        }
    }

    /// Check valueset membership via the `TerminologyProvider`, if present.
    ///
    /// Returns `Ok(false)` when no provider is configured.
    pub fn in_valueset(&self, code: &CqlCode, valueset_url: &str) -> Result<bool, EvalError> {
        match &self.terminology_provider {
            Some(p) => p.in_valueset(code, valueset_url),
            None => Ok(false),
        }
    }

    /// Expand a valueset via the `TerminologyProvider`, if present.
    ///
    /// Returns `Ok(vec![])` when no provider is configured.
    pub fn expand_valueset(&self, valueset_url: &str) -> Result<Vec<CqlCode>, EvalError> {
        match &self.terminology_provider {
            Some(p) => p.expand_valueset(valueset_url),
            None => Ok(vec![]),
        }
    }
}

// ---------------------------------------------------------------------------
// EvalContextBuilder
// ---------------------------------------------------------------------------

/// Builder for [`EvalContext`].
///
/// The only required field is the clock; all others are optional.
pub struct EvalContextBuilder {
    clock: Arc<dyn Clock>,
    timezone_offset_seconds: i32,
    parameters: HashMap<String, Value>,
    context_value: Option<Value>,
    data_provider: Option<Arc<dyn DataProvider>>,
    terminology_provider: Option<Arc<dyn TerminologyProvider>>,
}

impl EvalContextBuilder {
    /// Create a builder with the given clock.
    pub fn new(clock: impl Clock + 'static) -> Self {
        EvalContextBuilder {
            clock: Arc::new(clock),
            timezone_offset_seconds: 0,
            parameters: HashMap::new(),
            context_value: None,
            data_provider: None,
            terminology_provider: None,
        }
    }

    /// Set the default timezone offset in seconds east of UTC.
    pub fn timezone_offset_seconds(mut self, offset: i32) -> Self {
        self.timezone_offset_seconds = offset;
        self
    }

    /// Add a named parameter value.
    pub fn parameter(mut self, name: impl Into<String>, value: Value) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }

    /// Set all parameter values at once (overwrites any previously set).
    pub fn parameters(mut self, params: HashMap<String, Value>) -> Self {
        self.parameters = params;
        self
    }

    /// Set the current context value (e.g. the Patient resource).
    pub fn context_value(mut self, value: Value) -> Self {
        self.context_value = Some(value);
        self
    }

    /// Attach a `DataProvider`.
    pub fn data_provider(mut self, provider: impl DataProvider + 'static) -> Self {
        self.data_provider = Some(Arc::new(provider));
        self
    }

    /// Attach a `TerminologyProvider`.
    pub fn terminology_provider(mut self, provider: impl TerminologyProvider + 'static) -> Self {
        self.terminology_provider = Some(Arc::new(provider));
        self
    }

    /// Consume the builder and produce an [`EvalContext`].
    pub fn build(self) -> EvalContext {
        EvalContext {
            clock: self.clock,
            timezone_offset_seconds: self.timezone_offset_seconds,
            parameters: self.parameters,
            context_value: self.context_value,
            data_provider: self.data_provider,
            terminology_provider: self.terminology_provider,
        }
    }
}

// ---------------------------------------------------------------------------
// InMemoryDataProvider
// ---------------------------------------------------------------------------

/// A simple in-memory `DataProvider` for use in tests.
///
/// Resources are stored by data type name.  The provider returns all
/// resources of the requested type without applying code or date-range
/// filtering (the evaluation engine is expected to post-filter).
///
/// # Example
///
/// ```rust
/// use std::collections::BTreeMap;
/// use rh_cql::eval::context::{InMemoryDataProvider, DataProvider};
/// use rh_cql::eval::value::Value;
///
/// let mut provider = InMemoryDataProvider::new();
/// let mut obs = BTreeMap::new();
/// obs.insert("status".to_string(), Value::String("final".to_string()));
/// provider.add_resource("Observation", Value::Tuple(obs));
///
/// let results = provider.retrieve(None, "Observation", None, None, None, None).unwrap();
/// assert_eq!(results.len(), 1);
/// ```
#[derive(Debug, Default, Clone)]
pub struct InMemoryDataProvider {
    /// Map from data type name to list of resource values.
    resources: HashMap<String, Vec<Value>>,
}

impl InMemoryDataProvider {
    /// Create an empty provider.
    pub fn new() -> Self {
        InMemoryDataProvider {
            resources: HashMap::new(),
        }
    }

    /// Register a resource under `data_type`.
    pub fn add_resource(&mut self, data_type: impl Into<String>, resource: Value) {
        self.resources
            .entry(data_type.into())
            .or_default()
            .push(resource);
    }

    /// Return all resources for `data_type` (used internally and in tests).
    pub fn get_resources(&self, data_type: &str) -> &[Value] {
        self.resources
            .get(data_type)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

impl DataProvider for InMemoryDataProvider {
    /// Returns all resources of `data_type` without applying any filters.
    ///
    /// `context`, `code_path`, `codes`, `date_path`, and `date_range` are
    /// accepted but intentionally ignored — the caller (evaluation engine) is
    /// responsible for post-filtering based on code and date constraints.
    fn retrieve(
        &self,
        _context: Option<&str>,
        data_type: &str,
        _code_path: Option<&str>,
        _codes: Option<&Value>,
        _date_path: Option<&str>,
        _date_range: Option<&Value>,
    ) -> Result<Vec<Value>, EvalError> {
        Ok(self.resources.get(data_type).cloned().unwrap_or_default())
    }
}

// ---------------------------------------------------------------------------
// InMemoryTerminologyProvider
// ---------------------------------------------------------------------------

/// A simple in-memory `TerminologyProvider` for use in tests.
///
/// Valuesets are registered as sets of `(code, system)` pairs.  The provider
/// answers membership questions deterministically and supports `expand`.
///
/// # Example
///
/// ```rust
/// use rh_cql::eval::context::{InMemoryTerminologyProvider, TerminologyProvider};
/// use rh_cql::eval::value::CqlCode;
///
/// let mut provider = InMemoryTerminologyProvider::new();
/// provider.add_code(
///     "http://example.org/vs/bp",
///     CqlCode {
///         code: "8480-6".to_string(),
///         system: "http://loinc.org".to_string(),
///         display: Some("Systolic BP".to_string()),
///         version: None,
///     },
/// );
///
/// let code = CqlCode {
///     code: "8480-6".to_string(),
///     system: "http://loinc.org".to_string(),
///     display: None,
///     version: None,
/// };
/// assert_eq!(
///     provider.in_valueset(&code, "http://example.org/vs/bp").unwrap(),
///     true
/// );
/// ```
#[derive(Debug, Default, Clone)]
pub struct InMemoryTerminologyProvider {
    /// Map from valueset URL to list of member codes.
    valuesets: HashMap<String, Vec<CqlCode>>,
}

impl InMemoryTerminologyProvider {
    /// Create an empty provider.
    pub fn new() -> Self {
        InMemoryTerminologyProvider {
            valuesets: HashMap::new(),
        }
    }

    /// Add a code to the valueset at `valueset_url`.
    pub fn add_code(&mut self, valueset_url: impl Into<String>, code: CqlCode) {
        self.valuesets
            .entry(valueset_url.into())
            .or_default()
            .push(code);
    }

    /// Register an entire valueset at once (overwrites any existing entries).
    pub fn register_valueset(&mut self, valueset_url: impl Into<String>, codes: Vec<CqlCode>) {
        self.valuesets.insert(valueset_url.into(), codes);
    }
}

impl TerminologyProvider for InMemoryTerminologyProvider {
    fn in_valueset(&self, code: &CqlCode, valueset_url: &str) -> Result<bool, EvalError> {
        match self.valuesets.get(valueset_url) {
            None => Err(EvalError::TerminologyError(format!(
                "Valueset not found: '{valueset_url}'"
            ))),
            Some(codes) => {
                let found = codes
                    .iter()
                    .any(|c| c.code == code.code && c.system == code.system);
                Ok(found)
            }
        }
    }

    fn expand_valueset(&self, valueset_url: &str) -> Result<Vec<CqlCode>, EvalError> {
        match self.valuesets.get(valueset_url) {
            None => Err(EvalError::TerminologyError(format!(
                "Valueset not found: '{valueset_url}'"
            ))),
            Some(codes) => Ok(codes.clone()),
        }
    }

    fn lookup(&self, _code: &CqlCode, _property: &str) -> Result<Option<Value>, EvalError> {
        // In-memory provider does not store property metadata; return absent.
        Ok(None)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn fixed_now() -> CqlDateTime {
        CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(15),
            hour: Some(10),
            minute: Some(30),
            second: Some(0),
            millisecond: None,
            offset_seconds: Some(-18000),
        }
    }

    // -----------------------------------------------------------------------
    // Clock
    // -----------------------------------------------------------------------

    #[test]
    fn fixed_clock_now_returns_pinned_datetime() {
        let clock = FixedClock::new(fixed_now());
        let now = clock.now();
        assert_eq!(now.year, 2024);
        assert_eq!(now.month, Some(1));
        assert_eq!(now.day, Some(15));
        assert_eq!(now.offset_seconds, Some(-18000));
    }

    #[test]
    fn fixed_clock_today_extracts_date() {
        let clock = FixedClock::new(fixed_now());
        let today = clock.today();
        assert_eq!(today.year, 2024);
        assert_eq!(today.month, Some(1));
        assert_eq!(today.day, Some(15));
    }

    #[test]
    fn fixed_clock_time_of_day_extracts_time() {
        let clock = FixedClock::new(fixed_now());
        let t = clock.time_of_day();
        assert_eq!(t.hour, 10);
        assert_eq!(t.minute, Some(30));
        assert_eq!(t.second, Some(0));
    }

    // -----------------------------------------------------------------------
    // EvalContextBuilder
    // -----------------------------------------------------------------------

    #[test]
    fn eval_context_default_no_providers() {
        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now())).build();
        assert_eq!(ctx.today().year, 2024);
        assert_eq!(ctx.parameter("x"), None);
        // No provider → retrieve returns empty list
        let rows = ctx
            .retrieve(None, "Patient", None, None, None, None)
            .unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn eval_context_with_parameters() {
        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now()))
            .parameter("MeasurementYear", Value::Integer(2024))
            .build();
        assert_eq!(
            ctx.parameter("MeasurementYear"),
            Some(&Value::Integer(2024))
        );
        assert_eq!(ctx.parameter("Missing"), None);
    }

    #[test]
    fn eval_context_with_context_value() {
        let mut patient = BTreeMap::new();
        patient.insert("id".to_string(), Value::String("pt-1".to_string()));
        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now()))
            .context_value(Value::Tuple(patient))
            .build();
        match &ctx.context_value {
            Some(Value::Tuple(fields)) => {
                assert_eq!(fields.get("id"), Some(&Value::String("pt-1".to_string())));
            }
            _ => panic!("Expected Tuple context value"),
        }
    }

    #[test]
    fn eval_context_timezone_offset() {
        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now()))
            .timezone_offset_seconds(-18000)
            .build();
        assert_eq!(ctx.timezone_offset_seconds, -18000);
    }

    // -----------------------------------------------------------------------
    // InMemoryDataProvider
    // -----------------------------------------------------------------------

    #[test]
    fn in_memory_data_provider_returns_registered_resources() {
        let mut provider = InMemoryDataProvider::new();
        let mut obs = BTreeMap::new();
        obs.insert("status".to_string(), Value::String("final".to_string()));
        provider.add_resource("Observation", Value::Tuple(obs));

        let results = provider
            .retrieve(None, "Observation", None, None, None, None)
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn in_memory_data_provider_empty_for_unknown_type() {
        let provider = InMemoryDataProvider::new();
        let results = provider
            .retrieve(None, "Condition", None, None, None, None)
            .unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn in_memory_data_provider_multiple_resources() {
        let mut provider = InMemoryDataProvider::new();
        for i in 0..3 {
            let mut obs = BTreeMap::new();
            obs.insert("id".to_string(), Value::Integer(i));
            provider.add_resource("Observation", Value::Tuple(obs));
        }
        assert_eq!(
            provider
                .retrieve(None, "Observation", None, None, None, None)
                .unwrap()
                .len(),
            3
        );
    }

    #[test]
    fn in_memory_data_provider_passes_through_filter_args() {
        // InMemoryDataProvider accepts but ignores code/date-range filters,
        // returning all resources for the type.  Post-filtering on code and
        // date constraints is delegated to the evaluation engine.
        let mut provider = InMemoryDataProvider::new();
        let mut obs = BTreeMap::new();
        obs.insert("status".to_string(), Value::String("final".to_string()));
        provider.add_resource("Observation", Value::Tuple(obs));

        let code_filter = Value::String("8480-6".to_string());
        let results = provider
            .retrieve(
                Some("Patient"),
                "Observation",
                Some("code"),
                Some(&code_filter),
                Some("effective"),
                None,
            )
            .unwrap();
        // All resources returned regardless of filter arguments
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn eval_context_with_data_provider_returns_resources() {
        let mut data = InMemoryDataProvider::new();
        let mut obs = BTreeMap::new();
        obs.insert("status".to_string(), Value::String("final".to_string()));
        data.add_resource("Observation", Value::Tuple(obs));

        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now()))
            .data_provider(data)
            .build();

        let rows = ctx
            .retrieve(None, "Observation", None, None, None, None)
            .unwrap();
        assert_eq!(rows.len(), 1);
    }

    // -----------------------------------------------------------------------
    // InMemoryTerminologyProvider
    // -----------------------------------------------------------------------

    #[test]
    fn in_memory_terminology_provider_membership_positive() {
        let mut provider = InMemoryTerminologyProvider::new();
        provider.add_code(
            "http://example.org/vs/bp",
            CqlCode {
                code: "8480-6".to_string(),
                system: "http://loinc.org".to_string(),
                display: Some("Systolic BP".to_string()),
                version: None,
            },
        );
        let code = CqlCode {
            code: "8480-6".to_string(),
            system: "http://loinc.org".to_string(),
            display: None,
            version: None,
        };
        assert!(provider
            .in_valueset(&code, "http://example.org/vs/bp")
            .unwrap());
    }

    #[test]
    fn in_memory_terminology_provider_membership_negative() {
        let mut provider = InMemoryTerminologyProvider::new();
        provider.add_code(
            "http://example.org/vs/bp",
            CqlCode {
                code: "8480-6".to_string(),
                system: "http://loinc.org".to_string(),
                display: None,
                version: None,
            },
        );
        let other_code = CqlCode {
            code: "9999-9".to_string(),
            system: "http://loinc.org".to_string(),
            display: None,
            version: None,
        };
        assert!(!provider
            .in_valueset(&other_code, "http://example.org/vs/bp")
            .unwrap());
    }

    #[test]
    fn in_memory_terminology_provider_unknown_valueset_errors() {
        let provider = InMemoryTerminologyProvider::new();
        let code = CqlCode {
            code: "x".to_string(),
            system: "http://example.org".to_string(),
            display: None,
            version: None,
        };
        let result = provider.in_valueset(&code, "http://example.org/vs/unknown");
        assert!(matches!(result, Err(EvalError::TerminologyError(_))));
    }

    #[test]
    fn in_memory_terminology_provider_expand() {
        let mut provider = InMemoryTerminologyProvider::new();
        let url = "http://example.org/vs/codes";
        for i in 0..3_u8 {
            provider.add_code(
                url,
                CqlCode {
                    code: format!("CODE-{i}"),
                    system: "http://example.org".to_string(),
                    display: None,
                    version: None,
                },
            );
        }
        let expanded = provider.expand_valueset(url).unwrap();
        assert_eq!(expanded.len(), 3);
    }

    #[test]
    fn eval_context_with_terminology_provider_membership() {
        let mut term = InMemoryTerminologyProvider::new();
        term.add_code(
            "http://example.org/vs/active",
            CqlCode {
                code: "active".to_string(),
                system: "http://hl7.org/fhir/condition-clinical".to_string(),
                display: None,
                version: None,
            },
        );
        let ctx = EvalContextBuilder::new(FixedClock::new(fixed_now()))
            .terminology_provider(term)
            .build();
        let code = CqlCode {
            code: "active".to_string(),
            system: "http://hl7.org/fhir/condition-clinical".to_string(),
            display: None,
            version: None,
        };
        assert!(ctx
            .in_valueset(&code, "http://example.org/vs/active")
            .unwrap());
    }
}
