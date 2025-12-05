//! Source Location Tracking
//!
//! This module provides span types for tracking source locations during parsing,
//! enabling accurate error reporting with line and column numbers.

use nom::{
    error::{ErrorKind, ParseError},
    Compare, CompareResult, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use serde::{Deserialize, Serialize};
use std::ops::{Range, RangeFrom, RangeTo};

/// A span of source code with location tracking
///
/// Wraps a string slice and tracks position information for error reporting.
#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    /// The source text
    input: &'a str,
    /// Byte offset from the start of the original input
    offset: usize,
    /// Line number (1-indexed)
    line: usize,
    /// Column number (1-indexed, in characters not bytes)
    column: usize,
}

impl<'a> Span<'a> {
    /// Create a new span from source text
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the current line number (1-indexed)
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get the current column number (1-indexed)
    pub fn column(&self) -> usize {
        self.column
    }

    /// Get the byte offset from the start
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the remaining input as a string slice
    pub fn fragment(&self) -> &'a str {
        self.input
    }

    /// Check if the span is empty
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Get the length in bytes
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Create a source location from this span
    pub fn location(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.offset,
        }
    }
}

/// A source location (line, column, offset)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset from start of source
    pub offset: usize,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

/// A range in source code (start to end location)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRange {
    /// Start location
    pub start: SourceLocation,
    /// End location
    pub end: SourceLocation,
}

impl SourceRange {
    /// Create a new source range
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Create a range from two spans
    pub fn from_spans(start: &Span<'_>, end: &Span<'_>) -> Self {
        Self {
            start: start.location(),
            end: end.location(),
        }
    }
}

// ============================================================================
// nom trait implementations for Span
// ============================================================================

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.input.len()
    }
}

impl<'a> InputTake for Span<'a> {
    fn take(&self, count: usize) -> Self {
        let taken = &self.input[..count];
        let (new_line, new_column) = Self::update_position(self.line, self.column, taken);
        Self {
            input: taken,
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
        .with_updated_position(new_line, new_column)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.input.split_at(count);

        // Calculate new position after prefix
        let (new_line, new_column) = Self::update_position(self.line, self.column, prefix);

        let taken = Self {
            input: prefix,
            offset: self.offset,
            line: self.line,
            column: self.column,
        };

        let remaining = Self {
            input: suffix,
            offset: self.offset + count,
            line: new_line,
            column: new_column,
        };

        (remaining, taken)
    }
}

impl<'a> Span<'a> {
    /// Update line and column after consuming text
    fn update_position(mut line: usize, mut column: usize, consumed: &str) -> (usize, usize) {
        for ch in consumed.chars() {
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    /// Create a new span with updated position
    fn with_updated_position(mut self, line: usize, column: usize) -> Self {
        self.line = line;
        self.column = column;
        self
    }
}

impl<'a> InputTakeAtPosition for Span<'a> {
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.find(predicate) {
            Some(i) => Ok(self.take_split(i)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.find(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(*self, e))),
            Some(i) => Ok(self.take_split(i)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.find(predicate) {
            Some(i) => Ok(self.take_split(i)),
            None => Ok(self.take_split(self.input.len())),
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.find(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(*self, e))),
            Some(i) => Ok(self.take_split(i)),
            None => {
                if self.input.is_empty() {
                    Err(nom::Err::Error(E::from_error_kind(*self, e)))
                } else {
                    Ok(self.take_split(self.input.len()))
                }
            }
        }
    }
}

impl<'a> InputIter for Span<'a> {
    type Item = char;
    type Iter = std::str::CharIndices<'a>;
    type IterElem = std::str::Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.char_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.find(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        let mut cnt = 0;
        for (index, _) in self.input.char_indices() {
            if cnt == count {
                return Ok(index);
            }
            cnt += 1;
        }
        if cnt == count {
            return Ok(self.input.len());
        }
        Err(nom::Needed::Unknown)
    }
}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        let first = self.input.as_ptr() as usize;
        let second = second.input.as_ptr() as usize;
        second.saturating_sub(first)
    }
}

impl<'a> Slice<Range<usize>> for Span<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        let sliced = &self.input[range.clone()];
        let prefix = &self.input[..range.start];
        let (new_line, new_column) = Self::update_position(self.line, self.column, prefix);

        Self {
            input: sliced,
            offset: self.offset + range.start,
            line: new_line,
            column: new_column,
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for Span<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let sliced = &self.input[range.start..];
        let prefix = &self.input[..range.start];
        let (new_line, new_column) = Self::update_position(self.line, self.column, prefix);

        Self {
            input: sliced,
            offset: self.offset + range.start,
            line: new_line,
            column: new_column,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Span<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Self {
            input: &self.input[..range.end],
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }
}

impl<'a, 'b> Compare<&'b str> for Span<'a> {
    fn compare(&self, t: &'b str) -> CompareResult {
        self.input.compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a, 'b> FindSubstring<&'b str> for Span<'a> {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.input.find(substr)
    }
}

impl<'a> PartialEq for Span<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input
    }
}

impl<'a> Eq for Span<'a> {}

impl<'a> std::fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation() {
        let span = Span::new("hello world");
        assert_eq!(span.line(), 1);
        assert_eq!(span.column(), 1);
        assert_eq!(span.offset(), 0);
        assert_eq!(span.fragment(), "hello world");
    }

    #[test]
    fn test_span_location() {
        let span = Span::new("test");
        let loc = span.location();
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
        assert_eq!(loc.offset, 0);
    }

    #[test]
    fn test_update_position_newlines() {
        let (line, column) = Span::update_position(1, 1, "hello\nworld");
        assert_eq!(line, 2);
        assert_eq!(column, 6); // "world" has 5 chars + 1
    }

    #[test]
    fn test_update_position_multiple_newlines() {
        let (line, column) = Span::update_position(1, 1, "a\nb\nc");
        assert_eq!(line, 3);
        assert_eq!(column, 2);
    }

    #[test]
    fn test_take_split() {
        let span = Span::new("hello world");
        let (remaining, taken) = span.take_split(5);
        assert_eq!(taken.fragment(), "hello");
        assert_eq!(remaining.fragment(), " world");
        assert_eq!(remaining.offset(), 5);
        assert_eq!(remaining.column(), 6);
    }

    #[test]
    fn test_take_split_with_newline() {
        let span = Span::new("line1\nline2");
        let (remaining, taken) = span.take_split(6); // "line1\n"
        assert_eq!(taken.fragment(), "line1\n");
        assert_eq!(remaining.fragment(), "line2");
        assert_eq!(remaining.line(), 2);
        assert_eq!(remaining.column(), 1);
    }

    #[test]
    fn test_source_range() {
        let start = SourceLocation {
            line: 1,
            column: 5,
            offset: 4,
        };
        let end = SourceLocation {
            line: 1,
            column: 10,
            offset: 9,
        };
        let range = SourceRange::new(start, end);
        assert_eq!(range.start.column, 5);
        assert_eq!(range.end.column, 10);
    }
}
