//! Tests for parser gaps surfaced by the CMS122 FHIR measure validation work.
//!
//! These tests document and guard the specific CQL constructs that the
//! published CMS122 (Diabetes: HbA1c Poor Control > 9%) FHIR 4.0.1 measure
//! requires but that were previously unsupported by the rh-cql parser.

use rh_cql::parser::ast::{BinaryOperator, DateTimePrecision, Expression, UnaryOperator};
use rh_cql::parser::CqlParser;

/// Helper: assert that a CQL snippet parses successfully.
fn assert_parses(cql: &str) {
    let parser = CqlParser::new();
    let result = parser.parse(cql);
    assert!(
        result.is_ok(),
        "Expected successful parse, but got error: {:?}",
        result.err()
    );
}

// ===========================================================================
// Gap 1: `union [Type: "ValueSet"]` chained directly on a Retrieve
// ===========================================================================
//
// Root cause: `parse_single_source_query` consumed `union` as a potential
// alias after parsing the first `[Encounter: ...]` retrieve.  The fix added
// `"union"`, `"except"`, and `"intersect"` to the alias-rejection keyword
// list so the retrieve falls through to the union-expression precedence
// level.
//
// This pattern appears in `AdultOutpatientEncountersFHIR4.cql` from the
// published CMS122 measure.

#[test]
fn test_union_of_two_retrieves() {
    let cql = r#"
library TestUnionRetrieve version '0.1.0'
using FHIR version '4.0.1'
valueset "Office Visit": 'http://example.org/vs/office-visit'
valueset "Annual Wellness Visit": 'http://example.org/vs/awv'
context Patient
define "Union Two Retrieves":
  [Encounter: "Office Visit"] union [Encounter: "Annual Wellness Visit"]
"#;
    assert_parses(cql);
}

#[test]
fn test_union_of_retrieves_in_parenthesized_query_source() {
    // This is the exact pattern from AdultOutpatientEncountersFHIR4.cql:
    //   ( [Encounter: "VS1"] union [Encounter: "VS2"] ) alias where ...
    let cql = r#"
library TestUnionQuery version '0.1.0'
using FHIR version '4.0.1'
valueset "Office Visit": 'http://example.org/vs/office-visit'
valueset "Annual Wellness Visit": 'http://example.org/vs/awv'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "Qualifying Encounters":
  (
      [Encounter: "Office Visit"]
        union [Encounter: "Annual Wellness Visit"]
  ) ValidEncounter
    where ValidEncounter.period during "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_union_of_five_retrieves_with_where() {
    // Full pattern matching the published measure's "Qualifying Encounters".
    let cql = r#"
library TestUnionFull version '0.1.0'
using FHIR version '4.0.1'
valueset "Office Visit": 'http://example.org/vs/office-visit'
valueset "Annual Wellness Visit": 'http://example.org/vs/awv'
valueset "Preventive Care Services - Established Office Visit, 18 and Up": 'http://example.org/vs/preventive-established'
valueset "Preventive Care Services-Initial Office Visit, 18 and Up": 'http://example.org/vs/preventive-initial'
valueset "Home Healthcare Services": 'http://example.org/vs/home-health'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "Qualifying Encounters":
  (
      [Encounter: "Office Visit"]
        union [Encounter: "Annual Wellness Visit"]
        union [Encounter: "Preventive Care Services - Established Office Visit, 18 and Up"]
        union [Encounter: "Preventive Care Services-Initial Office Visit, 18 and Up"]
        union [Encounter: "Home Healthcare Services"]
  ) ValidEncounter
    where ValidEncounter.period during "Measurement Period"
      and ValidEncounter.status = 'finished'
"#;
    assert_parses(cql);
}

// ===========================================================================
// Gap 2: Timing phrase `ends before start of X` (compound + right boundary)
// ===========================================================================
//
// Root cause: The compound timing path (`starts/ends + during/before/after +
// precision`) did not parse an optional right boundary (`start of` / `end of`)
// before the operand.  The fix added `opt(tuple((boundary, opt(keyword("of")))))`
// to the compound path and switched to `parse_invocation_expression` when a
// right boundary is present.
//
// The same issue existed in `parse_relative_timing` (for offset-bearing
// phrases like `ends 1 hour or less on or before start of X`).

#[test]
fn test_ends_before_start_of() {
    let cql = r#"
library TestTimingB1 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "B1": [Encounter] E where E.period ends before start of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_ends_before_end_of() {
    let cql = r#"
library TestTimingB2 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "B2": [Encounter] E where E.period ends before end of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_starts_after_start_of() {
    let cql = r#"
library TestTimingB3 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "B3": [Encounter] E where E.period starts after start of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_starts_after_end_of() {
    let cql = r#"
library TestTimingB4 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "B4": [Encounter] E where E.period starts after end of "Measurement Period"
"#;
    assert_parses(cql);
}

// ===========================================================================
// Gap 3: Timing phrase `ends <offset> or less on or before start of X`
// ===========================================================================
//
// Root cause: `parse_timing_direction` only handled `on or before` / `on or
// after` when NO offset was present.  When an offset WAS parsed (e.g. `1 hour
// or less`), it went straight to `before` / `after` with an optional `or on`
// suffix, producing `before or on` but not `on or before`.  The fix made
// `parse_timing_direction` try `on or before` / `on or after` first,
// regardless of whether an offset was parsed.
//
// This pattern appears in `MATGlobalCommonFunctionsFHIR4.cql` (ED Visit
// function).

#[test]
fn test_offset_or_less_on_or_before() {
    let cql = r#"
library TestTimingD1 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "D1": [Encounter] E where E.period ends 1 hour or less on or before start of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_offset_or_less_on_or_before_end_of() {
    let cql = r#"
library TestTimingD2 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "D2": [Encounter] E where E.period ends 1 hour or less on or before end of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_offset_or_less_on_or_before_no_boundary() {
    let cql = r#"
library TestTimingC3 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "C3": [Encounter] E where E.period ends 1 hour or less on or before "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_on_or_before_start_of() {
    let cql = r#"
library TestTimingE1 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "E1": [Encounter] E where E.period ends on or before start of "Measurement Period"
"#;
    assert_parses(cql);
}

// ===========================================================================
// Regression: ensure existing timing phrases still work
// ===========================================================================

#[test]
fn test_ends_during_interval() {
    let cql = r#"
library TestTimingA3 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "A3": [Encounter] E where E.period ends during "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_ends_during_day_of_interval() {
    let cql = r#"
library TestTimingA4 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "A4": [Encounter] E where E.period ends during day of "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_ends_before_interval() {
    let cql = r#"
library TestTimingA1 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "A1": [Encounter] E where E.period ends before "Measurement Period"
"#;
    assert_parses(cql);
}

#[test]
fn test_offset_or_less_before() {
    let cql = r#"
library TestTimingC2 version '0.1.0'
using FHIR version '4.0.1'
parameter "Measurement Period" Interval<DateTime>
context Patient
define "C2": [Encounter] E where E.period ends 1 hour or less before "Measurement Period"
"#;
    assert_parses(cql);
}

// ===========================================================================
// Gap 4: `duration in <precision> of <interval>`
// ===========================================================================
//
// Root cause: the duration/difference parser accepted only the `between X and
// Y` spelling. The `of <interval>` spelling is required by CMS122's long-term
// care duration logic and is equivalent to:
// `difference in <precision> between start of I and end of I`.

#[test]
fn test_duration_in_days_of_interval() {
    let parser = CqlParser::new();
    let expr = parser
        .parse_expression(r#"duration in days of "Measurement Period""#)
        .expect("`duration in days of` should parse");

    match expr {
        Expression::BinaryExpression(bin) => {
            assert_eq!(
                bin.operator,
                BinaryOperator::DifferenceBetween(DateTimePrecision::Day)
            );
            assert!(matches!(
                bin.left.as_ref(),
                Expression::UnaryExpression(left) if left.operator == UnaryOperator::Start
            ));
            assert!(matches!(
                bin.right.as_ref(),
                Expression::UnaryExpression(right) if right.operator == UnaryOperator::End
            ));
        }
        other => panic!("expected DifferenceBetween, got {other:?}"),
    }
}

// ===========================================================================
// Gap 5: `start of X is null`
// ===========================================================================
//
// Root cause: literal null tests were parsed by the invocation/type-expression
// path. The unary-expression path matched `start of X` first and returned
// before the suffix was consumed. The parser now checks for literal test
// operators after the unary alternative completes.

#[test]
fn test_start_of_parameter_is_null() {
    let parser = CqlParser::new();
    let expr = parser
        .parse_expression(r#"start of "Measurement Period" is null"#)
        .expect("`start of X is null` should parse");

    match expr {
        Expression::UnaryExpression(test) => {
            assert_eq!(test.operator, UnaryOperator::IsNull);
            assert!(matches!(
                test.operand.as_ref(),
                Expression::UnaryExpression(operand) if operand.operator == UnaryOperator::Start
            ));
        }
        other => panic!("expected IsNull, got {other:?}"),
    }
}
