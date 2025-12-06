//! Error reporting infrastructure for CQL-to-ELM translation.
//!
//! This module provides comprehensive error reporting with:
//! - Source location tracking (line/column ranges)
//! - Severity levels (Error, Warning, Info)
//! - Error types/codes for categorization
//! - Conversion to ELM annotation format
//!
//! # Example
//!
//! ```
//! use rh_cql::reporting::{CqlCompilerException, ExceptionType, Severity, SourceLocator};
//!
//! // Create an error with source location
//! let locator = SourceLocator::new(1, 0, 1, 10);
//! let error = CqlCompilerException::new("Undefined symbol 'foo'")
//!     .with_locator(locator)
//!     .with_severity(Severity::Error)
//!     .with_error_type(ExceptionType::Semantic);
//!
//! assert!(error.is_error());
//! assert_eq!(error.start_line(), Some(1));
//! ```

use crate::elm::CqlToElmError;
use crate::options::ErrorSeverity;
use serde::{Deserialize, Serialize};

/// Severity level for compiler messages.
///
/// Determines how the message should be treated during compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Informational message - does not indicate a problem.
    Info,
    /// Warning - potential issue that should be reviewed.
    Warning,
    /// Error - compilation cannot proceed successfully.
    #[default]
    Error,
}

impl Severity {
    /// Convert to string representation for ELM output.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        }
    }

    /// Check if this severity meets or exceeds a minimum level.
    ///
    /// Returns true if this severity should be reported given the minimum level.
    pub fn meets_threshold(&self, min_severity: ErrorSeverity) -> bool {
        match min_severity {
            ErrorSeverity::Info => true,
            ErrorSeverity::Warning => matches!(self, Severity::Warning | Severity::Error),
            ErrorSeverity::Error => matches!(self, Severity::Error),
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Type/category of compiler exception.
///
/// Provides categorization for error messages to help identify
/// the phase or nature of the problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExceptionType {
    /// Syntax error during parsing.
    Syntax,
    /// Semantic error during analysis (type errors, unresolved symbols).
    #[default]
    Semantic,
    /// Error including external libraries.
    Include,
    /// Internal compiler error (bug).
    Internal,
}

impl ExceptionType {
    /// Convert to string representation for ELM output.
    pub fn as_str(&self) -> &'static str {
        match self {
            ExceptionType::Syntax => "syntax",
            ExceptionType::Semantic => "semantic",
            ExceptionType::Include => "include",
            ExceptionType::Internal => "internal",
        }
    }
}

impl std::fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Source location information for tracking where issues occur.
///
/// Tracks both start and end positions to highlight the entire
/// problematic region in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocator {
    /// Starting line (1-based).
    pub start_line: i32,
    /// Starting character position within the line (0-based).
    pub start_char: i32,
    /// Ending line (1-based).
    pub end_line: i32,
    /// Ending character position within the line (0-based).
    pub end_char: i32,
}

impl SourceLocator {
    /// Create a new source locator.
    ///
    /// # Arguments
    ///
    /// * `start_line` - Starting line number (1-based)
    /// * `start_char` - Starting character position (0-based)
    /// * `end_line` - Ending line number (1-based)
    /// * `end_char` - Ending character position (0-based)
    pub fn new(start_line: i32, start_char: i32, end_line: i32, end_char: i32) -> Self {
        Self {
            start_line,
            start_char,
            end_line,
            end_char,
        }
    }

    /// Create a point locator (start equals end).
    pub fn point(line: i32, char_pos: i32) -> Self {
        Self::new(line, char_pos, line, char_pos)
    }

    /// Create a locator from usize values (converts to i32).
    pub fn from_usize(start_line: usize, start_char: usize, end_line: usize, end_char: usize) -> Self {
        Self::new(
            start_line as i32,
            start_char as i32,
            end_line as i32,
            end_char as i32,
        )
    }

    /// Format as a human-readable location string.
    pub fn to_location_string(&self) -> String {
        if self.start_line == self.end_line {
            if self.start_char == self.end_char {
                format!("[{}:{}]", self.start_line, self.start_char)
            } else {
                format!("[{}:{}-{}]", self.start_line, self.start_char, self.end_char)
            }
        } else {
            format!(
                "[{}:{}-{}:{}]",
                self.start_line, self.start_char, self.end_line, self.end_char
            )
        }
    }
}

impl Default for SourceLocator {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl std::fmt::Display for SourceLocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_location_string())
    }
}

/// A compiler exception with full diagnostic information.
///
/// This is the primary type for reporting issues during CQL-to-ELM
/// translation. It captures:
/// - The error message
/// - Source location (if available)
/// - Severity level
/// - Error type/category
/// - Optional cause (for error chaining)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlCompilerException {
    /// The error message.
    message: String,
    /// Source location where the error occurred.
    locator: Option<SourceLocator>,
    /// Severity of this exception.
    severity: Severity,
    /// Type/category of this exception.
    error_type: ExceptionType,
    /// Optional identifier for the target library (for include errors).
    target_library: Option<String>,
    /// Optional cause message (for error chaining).
    cause: Option<String>,
}

impl CqlCompilerException {
    /// Create a new exception with the given message.
    ///
    /// Defaults to Error severity and Semantic type.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            locator: None,
            severity: Severity::Error,
            error_type: ExceptionType::Semantic,
            target_library: None,
            cause: None,
        }
    }

    /// Create a syntax error.
    pub fn syntax(message: impl Into<String>) -> Self {
        Self::new(message).with_error_type(ExceptionType::Syntax)
    }

    /// Create a semantic error.
    pub fn semantic(message: impl Into<String>) -> Self {
        Self::new(message).with_error_type(ExceptionType::Semantic)
    }

    /// Create an include error.
    pub fn include(message: impl Into<String>) -> Self {
        Self::new(message).with_error_type(ExceptionType::Include)
    }

    /// Create an internal compiler error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(message).with_error_type(ExceptionType::Internal)
    }

    /// Create a warning.
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).with_severity(Severity::Warning)
    }

    /// Create an info message.
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).with_severity(Severity::Info)
    }

    /// Set the source locator.
    pub fn with_locator(mut self, locator: SourceLocator) -> Self {
        self.locator = Some(locator);
        self
    }

    /// Set the source locator from optional components.
    pub fn with_location(
        mut self,
        start_line: Option<i32>,
        start_char: Option<i32>,
        end_line: Option<i32>,
        end_char: Option<i32>,
    ) -> Self {
        if let (Some(sl), Some(sc), Some(el), Some(ec)) = (start_line, start_char, end_line, end_char) {
            self.locator = Some(SourceLocator::new(sl, sc, el, ec));
        }
        self
    }

    /// Set the severity level.
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Set the error type.
    pub fn with_error_type(mut self, error_type: ExceptionType) -> Self {
        self.error_type = error_type;
        self
    }

    /// Set the target library (for include errors).
    pub fn with_target_library(mut self, library: impl Into<String>) -> Self {
        self.target_library = Some(library.into());
        self
    }

    /// Set the cause message.
    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = Some(cause.into());
        self
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the source locator.
    pub fn locator(&self) -> Option<&SourceLocator> {
        self.locator.as_ref()
    }

    /// Get the severity.
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// Get the error type.
    pub fn error_type(&self) -> ExceptionType {
        self.error_type
    }

    /// Get the target library.
    pub fn target_library(&self) -> Option<&str> {
        self.target_library.as_deref()
    }

    /// Get the cause.
    pub fn cause(&self) -> Option<&str> {
        self.cause.as_deref()
    }

    /// Check if this is an error.
    pub fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
    }

    /// Check if this is a warning.
    pub fn is_warning(&self) -> bool {
        matches!(self.severity, Severity::Warning)
    }

    /// Check if this is info.
    pub fn is_info(&self) -> bool {
        matches!(self.severity, Severity::Info)
    }

    /// Get the start line if available.
    pub fn start_line(&self) -> Option<i32> {
        self.locator.map(|l| l.start_line)
    }

    /// Get the start character if available.
    pub fn start_char(&self) -> Option<i32> {
        self.locator.map(|l| l.start_char)
    }

    /// Get the end line if available.
    pub fn end_line(&self) -> Option<i32> {
        self.locator.map(|l| l.end_line)
    }

    /// Get the end character if available.
    pub fn end_char(&self) -> Option<i32> {
        self.locator.map(|l| l.end_char)
    }

    /// Check if this exception meets the minimum severity threshold.
    pub fn meets_threshold(&self, min_severity: ErrorSeverity) -> bool {
        self.severity.meets_threshold(min_severity)
    }

    /// Convert to CqlToElmError for ELM annotation output.
    pub fn to_elm_error(&self) -> CqlToElmError {
        CqlToElmError {
            message: self.full_message(),
            error_type: Some(self.error_type.as_str().to_string()),
            error_severity: Some(self.severity.as_str().to_string()),
            start_line: self.locator.map(|l| l.start_line),
            start_char: self.locator.map(|l| l.start_char),
            end_line: self.locator.map(|l| l.end_line),
            end_char: self.locator.map(|l| l.end_char),
        }
    }

    /// Get the full message including cause if present.
    pub fn full_message(&self) -> String {
        match &self.cause {
            Some(cause) => format!("{}: {}", self.message, cause),
            None => self.message.clone(),
        }
    }

    /// Format as a detailed diagnostic string.
    pub fn to_diagnostic_string(&self) -> String {
        let mut result = format!("{}: {}", self.severity, self.message);
        
        if let Some(loc) = &self.locator {
            result.push_str(&format!(" at {loc}"));
        }
        
        if let Some(cause) = &self.cause {
            result.push_str(&format!("\n  Caused by: {cause}"));
        }
        
        result
    }
}

impl std::fmt::Display for CqlCompilerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_diagnostic_string())
    }
}

impl std::error::Error for CqlCompilerException {}

/// Collector for compiler exceptions during translation.
///
/// Aggregates errors, warnings, and info messages during compilation,
/// with filtering based on minimum severity level.
///
/// # Example
///
/// ```
/// use rh_cql::reporting::{ExceptionCollector, CqlCompilerException, Severity};
/// use rh_cql::options::ErrorSeverity;
///
/// let mut collector = ExceptionCollector::new(ErrorSeverity::Warning);
///
/// // Add various exceptions
/// collector.add(CqlCompilerException::new("This is an error"));
/// collector.add(CqlCompilerException::warning("This is a warning"));
/// collector.add(CqlCompilerException::info("This is info")); // filtered out
///
/// assert_eq!(collector.error_count(), 1);
/// assert_eq!(collector.warning_count(), 1);
/// assert_eq!(collector.len(), 2); // info was filtered
/// ```
#[derive(Debug, Clone, Default)]
pub struct ExceptionCollector {
    /// Collected exceptions.
    exceptions: Vec<CqlCompilerException>,
    /// Minimum severity level for collection.
    min_severity: ErrorSeverity,
}

impl ExceptionCollector {
    /// Create a new collector with the given minimum severity.
    pub fn new(min_severity: ErrorSeverity) -> Self {
        Self {
            exceptions: Vec::new(),
            min_severity,
        }
    }

    /// Create a collector that collects all messages (Info and above).
    pub fn all() -> Self {
        Self::new(ErrorSeverity::Info)
    }

    /// Create a collector for warnings and errors only.
    pub fn warnings_and_errors() -> Self {
        Self::new(ErrorSeverity::Warning)
    }

    /// Create a collector for errors only.
    pub fn errors_only() -> Self {
        Self::new(ErrorSeverity::Error)
    }

    /// Add an exception if it meets the minimum severity threshold.
    pub fn add(&mut self, exception: CqlCompilerException) {
        if exception.meets_threshold(self.min_severity) {
            self.exceptions.push(exception);
        }
    }

    /// Add an error.
    pub fn add_error(&mut self, message: impl Into<String>) {
        self.add(CqlCompilerException::new(message));
    }

    /// Add a warning.
    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.add(CqlCompilerException::warning(message));
    }

    /// Add an info message.
    pub fn add_info(&mut self, message: impl Into<String>) {
        self.add(CqlCompilerException::info(message));
    }

    /// Add a syntax error with location.
    pub fn add_syntax_error(&mut self, message: impl Into<String>, locator: SourceLocator) {
        self.add(CqlCompilerException::syntax(message).with_locator(locator));
    }

    /// Add a semantic error with optional location.
    pub fn add_semantic_error(&mut self, message: impl Into<String>, locator: Option<SourceLocator>) {
        let mut exc = CqlCompilerException::semantic(message);
        if let Some(loc) = locator {
            exc = exc.with_locator(loc);
        }
        self.add(exc);
    }

    /// Add an include error.
    pub fn add_include_error(&mut self, message: impl Into<String>, library: impl Into<String>) {
        self.add(
            CqlCompilerException::include(message)
                .with_target_library(library)
        );
    }

    /// Get all collected exceptions.
    pub fn exceptions(&self) -> &[CqlCompilerException] {
        &self.exceptions
    }

    /// Get only errors.
    pub fn errors(&self) -> impl Iterator<Item = &CqlCompilerException> {
        self.exceptions.iter().filter(|e| e.is_error())
    }

    /// Get only warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &CqlCompilerException> {
        self.exceptions.iter().filter(|e| e.is_warning())
    }

    /// Get number of collected exceptions.
    pub fn len(&self) -> usize {
        self.exceptions.len()
    }

    /// Check if no exceptions were collected.
    pub fn is_empty(&self) -> bool {
        self.exceptions.is_empty()
    }

    /// Get the number of errors.
    pub fn error_count(&self) -> usize {
        self.exceptions.iter().filter(|e| e.is_error()).count()
    }

    /// Get the number of warnings.
    pub fn warning_count(&self) -> usize {
        self.exceptions.iter().filter(|e| e.is_warning()).count()
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.exceptions.iter().any(|e| e.is_error())
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        self.exceptions.iter().any(|e| e.is_warning())
    }

    /// Take all exceptions, clearing the collector.
    pub fn take(&mut self) -> Vec<CqlCompilerException> {
        std::mem::take(&mut self.exceptions)
    }

    /// Convert all exceptions to ELM errors.
    pub fn to_elm_errors(&self) -> Vec<CqlToElmError> {
        self.exceptions.iter().map(|e| e.to_elm_error()).collect()
    }

    /// Clear all collected exceptions.
    pub fn clear(&mut self) {
        self.exceptions.clear();
    }

    /// Merge another collector into this one.
    pub fn merge(&mut self, other: &ExceptionCollector) {
        for exc in &other.exceptions {
            self.add(exc.clone());
        }
    }

    /// Get a summary string.
    pub fn summary(&self) -> String {
        let errors = self.error_count();
        let warnings = self.warning_count();
        
        match (errors, warnings) {
            (0, 0) => "No issues".to_string(),
            (e, 0) => format!("{e} error{}", if e == 1 { "" } else { "s" }),
            (0, w) => format!("{w} warning{}", if w == 1 { "" } else { "s" }),
            (e, w) => format!(
                "{e} error{}, {w} warning{}",
                if e == 1 { "" } else { "s" },
                if w == 1 { "" } else { "s" }
            ),
        }
    }

    /// Print all diagnostics to stderr.
    pub fn print_diagnostics(&self) {
        for exc in &self.exceptions {
            eprintln!("{}", exc.to_diagnostic_string());
        }
    }
}

impl IntoIterator for ExceptionCollector {
    type Item = CqlCompilerException;
    type IntoIter = std::vec::IntoIter<CqlCompilerException>;

    fn into_iter(self) -> Self::IntoIter {
        self.exceptions.into_iter()
    }
}

impl<'a> IntoIterator for &'a ExceptionCollector {
    type Item = &'a CqlCompilerException;
    type IntoIter = std::slice::Iter<'a, CqlCompilerException>;

    fn into_iter(self) -> Self::IntoIter {
        self.exceptions.iter()
    }
}

impl From<CqlCompilerException> for CqlToElmError {
    fn from(exc: CqlCompilerException) -> Self {
        exc.to_elm_error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_threshold() {
        assert!(Severity::Error.meets_threshold(ErrorSeverity::Error));
        assert!(Severity::Error.meets_threshold(ErrorSeverity::Warning));
        assert!(Severity::Error.meets_threshold(ErrorSeverity::Info));

        assert!(!Severity::Warning.meets_threshold(ErrorSeverity::Error));
        assert!(Severity::Warning.meets_threshold(ErrorSeverity::Warning));
        assert!(Severity::Warning.meets_threshold(ErrorSeverity::Info));

        assert!(!Severity::Info.meets_threshold(ErrorSeverity::Error));
        assert!(!Severity::Info.meets_threshold(ErrorSeverity::Warning));
        assert!(Severity::Info.meets_threshold(ErrorSeverity::Info));
    }

    #[test]
    fn test_source_locator() {
        let loc = SourceLocator::new(1, 0, 1, 10);
        assert_eq!(loc.to_location_string(), "[1:0-10]");

        let point = SourceLocator::point(5, 3);
        assert_eq!(point.to_location_string(), "[5:3]");

        let multiline = SourceLocator::new(1, 0, 3, 5);
        assert_eq!(multiline.to_location_string(), "[1:0-3:5]");
    }

    #[test]
    fn test_exception_creation() {
        let exc = CqlCompilerException::new("Test error")
            .with_locator(SourceLocator::new(1, 0, 1, 5))
            .with_severity(Severity::Error)
            .with_error_type(ExceptionType::Semantic);

        assert_eq!(exc.message(), "Test error");
        assert!(exc.is_error());
        assert_eq!(exc.start_line(), Some(1));
        assert_eq!(exc.error_type(), ExceptionType::Semantic);
    }

    #[test]
    fn test_exception_convenience_constructors() {
        let syntax = CqlCompilerException::syntax("syntax error");
        assert_eq!(syntax.error_type(), ExceptionType::Syntax);
        assert!(syntax.is_error());

        let warning = CqlCompilerException::warning("a warning");
        assert!(warning.is_warning());
        assert_eq!(warning.error_type(), ExceptionType::Semantic);

        let info = CqlCompilerException::info("info message");
        assert!(info.is_info());
    }

    #[test]
    fn test_to_elm_error() {
        let exc = CqlCompilerException::new("Test error")
            .with_locator(SourceLocator::new(1, 0, 1, 10))
            .with_error_type(ExceptionType::Syntax);

        let elm_error = exc.to_elm_error();
        assert_eq!(elm_error.message, "Test error");
        assert_eq!(elm_error.error_type, Some("syntax".to_string()));
        assert_eq!(elm_error.error_severity, Some("error".to_string()));
        assert_eq!(elm_error.start_line, Some(1));
        assert_eq!(elm_error.start_char, Some(0));
    }

    #[test]
    fn test_exception_with_cause() {
        let exc = CqlCompilerException::new("Failed to resolve")
            .with_cause("Library 'FHIRHelpers' not found");

        assert_eq!(exc.full_message(), "Failed to resolve: Library 'FHIRHelpers' not found");
    }

    #[test]
    fn test_collector_basic() {
        let mut collector = ExceptionCollector::all();

        collector.add_error("error 1");
        collector.add_warning("warning 1");
        collector.add_info("info 1");

        assert_eq!(collector.len(), 3);
        assert_eq!(collector.error_count(), 1);
        assert_eq!(collector.warning_count(), 1);
        assert!(collector.has_errors());
        assert!(collector.has_warnings());
    }

    #[test]
    fn test_collector_filtering() {
        let mut collector = ExceptionCollector::warnings_and_errors();

        collector.add_error("error 1");
        collector.add_warning("warning 1");
        collector.add_info("info 1"); // Should be filtered

        assert_eq!(collector.len(), 2);
        assert_eq!(collector.error_count(), 1);
        assert_eq!(collector.warning_count(), 1);
    }

    #[test]
    fn test_collector_errors_only() {
        let mut collector = ExceptionCollector::errors_only();

        collector.add_error("error 1");
        collector.add_warning("warning 1"); // Should be filtered
        collector.add_info("info 1"); // Should be filtered

        assert_eq!(collector.len(), 1);
        assert_eq!(collector.error_count(), 1);
        assert!(!collector.has_warnings());
    }

    #[test]
    fn test_collector_to_elm_errors() {
        let mut collector = ExceptionCollector::all();

        collector.add(
            CqlCompilerException::new("error")
                .with_locator(SourceLocator::new(1, 0, 1, 5))
        );

        let elm_errors = collector.to_elm_errors();
        assert_eq!(elm_errors.len(), 1);
        assert_eq!(elm_errors[0].message, "error");
        assert_eq!(elm_errors[0].start_line, Some(1));
    }

    #[test]
    fn test_collector_summary() {
        let mut collector = ExceptionCollector::all();
        assert_eq!(collector.summary(), "No issues");

        collector.add_error("e1");
        assert_eq!(collector.summary(), "1 error");

        collector.add_error("e2");
        assert_eq!(collector.summary(), "2 errors");

        collector.add_warning("w1");
        assert_eq!(collector.summary(), "2 errors, 1 warning");

        collector.add_warning("w2");
        assert_eq!(collector.summary(), "2 errors, 2 warnings");
    }

    #[test]
    fn test_collector_merge() {
        let mut collector1 = ExceptionCollector::all();
        collector1.add_error("error 1");

        let mut collector2 = ExceptionCollector::all();
        collector2.add_warning("warning 1");

        collector1.merge(&collector2);
        assert_eq!(collector1.len(), 2);
        assert_eq!(collector1.error_count(), 1);
        assert_eq!(collector1.warning_count(), 1);
    }

    #[test]
    fn test_collector_take() {
        let mut collector = ExceptionCollector::all();
        collector.add_error("error 1");

        let exceptions = collector.take();
        assert_eq!(exceptions.len(), 1);
        assert!(collector.is_empty());
    }

    #[test]
    fn test_diagnostic_string() {
        let exc = CqlCompilerException::new("Undefined symbol 'foo'")
            .with_locator(SourceLocator::new(10, 5, 10, 8))
            .with_error_type(ExceptionType::Semantic);

        let diag = exc.to_diagnostic_string();
        assert!(diag.contains("error:"));
        assert!(diag.contains("Undefined symbol 'foo'"));
        assert!(diag.contains("[10:5-8]"));
    }

    #[test]
    fn test_source_locator_from_usize() {
        let loc = SourceLocator::from_usize(10, 5, 10, 15);
        assert_eq!(loc.start_line, 10);
        assert_eq!(loc.start_char, 5);
        assert_eq!(loc.end_line, 10);
        assert_eq!(loc.end_char, 15);
    }

    #[test]
    fn test_iterator_impl() {
        let mut collector = ExceptionCollector::all();
        collector.add_error("e1");
        collector.add_warning("w1");

        // Test reference iterator
        let count = (&collector).into_iter().count();
        assert_eq!(count, 2);

        // Test owned iterator
        let items: Vec<_> = collector.into_iter().collect();
        assert_eq!(items.len(), 2);
    }
}
