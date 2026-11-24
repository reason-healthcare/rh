use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::Decimal;
use serde_json::json;

use FhirPathValue::{Boolean, Date, DateTime, Empty, Integer, Long, Number, Quantity, Time};

fn evaluate(expression: &str) -> rh_fhirpath::FhirPathResult<FhirPathValue> {
    let expression = FhirPathParser::new().parse(expression)?;
    FhirPathEvaluator::new().evaluate(&expression, &EvaluationContext::new(json!({})))
}

fn decimal(value: &str) -> Decimal {
    Decimal::from_str_exact(value).unwrap()
}

fn quantity(value: &str, unit: &str) -> FhirPathValue {
    Quantity {
        value: decimal(value),
        unit: Some(unit.to_owned()),
    }
}

fn expect(cases: &[(&str, FhirPathValue)]) {
    for (expression, expected) in cases {
        let result = evaluate(expression).unwrap_or_else(|error| panic!("{expression}: {error}"));
        assert_eq!(&result, expected, "{expression}");
    }
}

#[test]
fn checked_numeric_operations_and_conversions() {
    expect(&[
        ("2147483647 + 1", Empty),
        ("(-2147483648) div -1", Empty),
        ("(-2147483648).abs()", Empty),
        ("9223372036854775807L + 1", Empty),
        ("(-9223372036854775808L).abs()", Empty),
        ("-(-9223372036854775808L)", Empty),
        ("1 + 5L", Long(6)),
        ("5L + 4.5", Number(decimal("9.5"))),
        ("5L div 2", Long(2)),
        ("5L / 2", Number(decimal("2.5"))),
        ("5.5 div 0.7", Number(decimal("7"))),
        ("5.5 mod 0.7", Number(decimal("0.6"))),
        ("1.001 - 0.1", Number(decimal("0.901"))),
        ("'79228162514264337593543950335'.toDecimal() + 1", Empty),
        ("'2147483648'.toInteger()", Empty),
        ("2147483648L.convertsToInteger()", Boolean(false)),
        ("42L.toInteger()", Integer(42)),
        ("'42L'.toDecimal()", Number(decimal("42"))),
        ("1.0.toInteger()", Empty),
        ("2.round()", Number(decimal("2"))),
        ("2.power(3)", Number(decimal("8"))),
        ("2.power(-1)", Number(decimal("0.5"))),
        ("10.power(1000)", Empty),
        ("1.5.round(2000000000)", Empty),
        ("1L.sqrt()", Number(Decimal::ONE)),
        ("2147483647.1.ceiling()", Empty),
        ("42L.floor()", Integer(42)),
        ("2147483648L.floor()", Empty),
        ("2147483648L.ceiling()", Empty),
        ("(-2147483649L).truncate()", Empty),
        (
            "0.0000000000000000000000000001.highBoundary(28)",
            Number(decimal("0.0000000000000000000000000002")),
        ),
        (
            "0.0000000000000000000000000001.lowBoundary(28)",
            Number(decimal("0.0000000000000000000000000000")),
        ),
    ]);
}

#[test]
fn quantity_arithmetic_and_comparison() {
    expect(&[
        ("1 'ug' = 1.00000001 'ug'", Boolean(false)),
        ("1 'ug' = 0.001 'mg'", Boolean(true)),
        ("21 'mm' ~ 2 'cm'", Boolean(true)),
        ("1 'mg' / 0", Empty),
        ("60 / 1 's'", quantity("60", "/s")),
        ("120 'km' / 2 'h'", quantity("60", "km/h")),
        ("12 'cm2' / 3 'cm'", quantity("4", "cm")),
        ("10 'm/s' * 10 's'", quantity("100", "m")),
        ("12 'cm' * 3 'cm'", quantity("36", "cm2")),
        ("3 'm' + 3 'cm'", quantity("303", "cm")),
        ("2 + 2 'cm'", Empty),
        ("2 + 2 '1'", quantity("4", "1")),
        ("1 year = 12 months", Boolean(true)),
        ("1 year = 360 days", Boolean(false)),
        ("1 year > 2 months", Boolean(true)),
        ("1 year <= 2 days", Boolean(false)),
        ("1 year = 1 'a'", Empty),
        ("1 year.comparable(1 'a')", Boolean(false)),
        ("1 year ~ 12 'mo'", Boolean(true)),
        ("1 year.toQuantity('a') = 1 'a'", Boolean(true)),
        ("1 'a'.toQuantity('d')", quantity("365.25", "d")),
        ("60 's' + 2 minutes", quantity("180", "second")),
        ("1 year + 12 months", Empty),
        ("12 day * 45 'm'", Empty),
        ("23 'Cel' = 73.4 '[degF]'", Boolean(true)),
        ("37 'Cel' + 1 'Cel'", Empty),
        ("1 'm/s'.comparable(1 'cm/min')", Boolean(true)),
        ("1 'not-a-supported-unit'.comparable(1 's')", Boolean(false)),
    ]);
}

#[test]
fn laboratory_quantity_units() {
    expect(&[
        ("5 'mmol/L' = 5 'mmol/L'", Boolean(true)),
        ("5 'mmol/L' = 5000 'umol/L'", Boolean(true)),
        ("5 'mmol/L' > 4 'mmol/L'", Boolean(true)),
        ("5 'mmol/L' = 5 'U/L'", Empty),
        ("1 'U/mL' = 1000 'U/L'", Boolean(true)),
        ("7 'U/L' + 5 'U/L'", quantity("12", "U/L")),
        ("7 'U/L' - 5 'U/L'", quantity("2", "U/L")),
        ("7 'U/L' * 5", quantity("35", "U/L")),
        ("5 * 7 'U/L'", quantity("35", "U/L")),
        ("10 'U/L' / 5", quantity("2", "U/L")),
        ("5 '%' ~ 5.0 '%'", Boolean(true)),
        ("5 '%'.toQuantity('1')", quantity("0.05", "1")),
        ("1 'm²'.toQuantity('m2')", Empty),
        ("1 'not-a-supported-unit' + 2 'not-a-supported-unit'", Empty),
    ]);
}

#[test]
fn temporal_precision_timezone_and_interval_semantics() {
    expect(&[
        (
            "@2024-01-01T23:00:00Z = @2024-01-02T01:00:00+02:00",
            Boolean(true),
        ),
        ("@2024-01-01T23:00Z = @2024-01-02T01:00:00+02:00", Empty),
        ("@2014T.timeOf()", Empty),
        ("@T12:30:31.0 ~ @T12:30:31", Boolean(true)),
        ("@T12:30:31.1 ~ @T12:30:31", Boolean(false)),
        ("@T12:30:31.1 ~ @T12:30:31.14", Boolean(true)),
        ("@2026-02 + 4 weeks", Date("2026-02".to_owned())),
        ("@2026-02 + 5 weeks", Date("2026-03".to_owned())),
        ("@2016 + 365 days", Date("2017".to_owned())),
        (
            "@2024-12-31T23:30 + 40 minutes",
            DateTime("2025-01-01T00:10".to_owned()),
        ),
        (
            "@2024-02-29T13:30Z + 1 year",
            DateTime("2025-02-28T13:30Z".to_owned()),
        ),
        (
            "@1973-12-25T00:00:00.000+10:00 + 0.1 's'",
            DateTime("1973-12-25T00:00:00.100+10:00".to_owned()),
        ),
        ("@T23:30 + 1 hour", Time("00:30".to_owned())),
        (
            "@2024-01-01.toDateTime()",
            DateTime("2024-01-01".to_owned()),
        ),
        ("'150124'.toDate('ddMMyy')", Date("2024-01-15".to_owned())),
        ("'12-27'.toDate('MM-yy')", Date("2027-12".to_owned())),
        ("'2023-02-29'.toDate()", Empty),
        (
            "'January 15 2024 11:30:15.123 P -0500'.toDateTime('MMMM dd yyyy hh:mm:ss.SSS a Z')",
            DateTime("2024-01-15T23:30:15.123-05:00".to_owned()),
        ),
        (
            "@2014-01-01T08.highBoundary(17)",
            DateTime("2014-01-01T08:59:59.999".to_owned()),
        ),
        (
            "@2014-01-01T08.lowBoundary(17)",
            DateTime("2014-01-01T08:00:00.000".to_owned()),
        ),
        (
            "@2014-01-01T08.lowBoundary(8)",
            DateTime("2014-01-01".to_owned()),
        ),
        ("@T10:30.highBoundary(9)", Time("10:30:59.999".to_owned())),
        (
            "@2024-01-01T10:30+08:45.timezoneOffsetOf()",
            Number(decimal("8.75")),
        ),
        ("@2025-01-02.duration(@2025-01-07, 'week')", Integer(0)),
        ("@2025-01-02.difference(@2025-01-07, 'week')", Integer(1)),
        ("@2024-12-01.duration(@2025-09-01, 'year')", Integer(0)),
        ("@2024-12-01.difference(@2025-09-01, 'year')", Integer(1)),
        ("@2025-01-07.difference(@2025-01-02, 'week')", Integer(-1)),
        ("@2025-01.duration(@2025-02-01, 'day')", Empty),
        (
            "@2024-01-01T23:00:00Z.duration(@2024-01-02T03:00:00+02:00, 'hour')",
            Integer(2),
        ),
        ("@T02:10.difference(@T01:50, 'hour')", Integer(-1)),
        (
            "@2025-01-01T00:00:00.000.duration(@2025-01-25T20:31:23.647, 'millisecond')",
            Integer(2_147_483_647),
        ),
        (
            "@2025-01-01T00:00:00.000.duration(@2025-01-25T20:31:23.648, 'millisecond')",
            Empty,
        ),
        (
            "@2025-01-25T20:31:23.648.difference(@2025-01-01T00:00:00.000, 'millisecond')",
            Integer(-2_147_483_648),
        ),
        (
            "@2025-01-25T20:31:23.649.difference(@2025-01-01T00:00:00.000, 'millisecond')",
            Empty,
        ),
    ]);
    assert!(evaluate("@T01:00 + 1 day").is_err());
    assert!(evaluate("@2024-01-01T00:00:00 + 1 'a'").is_err());
}

#[test]
fn aggregate_functions_share_typed_operations() {
    expect(&[
        ("{}.sum()", Empty),
        ("(1 | 2 | 3).sum()", Integer(6)),
        ("(2147483647 | 1).sum()", Empty),
        ("(1L | 2L | 3L).sum()", Long(6)),
        ("(1 | 2).avg()", Number(decimal("1.5"))),
        (
            "'70000000000000000000000000000'.toDecimal().combine('60000000000000000000000000000'.toDecimal()).avg()",
            Number(decimal("65000000000000000000000000000")),
        ),
        ("(1 'm' | 20 'cm').avg()", quantity("60", "cm")),
        ("(1 'm' | 2 's').sum()", Empty),
        (
            "(@2020-02-01 | @2020-01-03).min()",
            Date("2020-01-03".to_owned()),
        ),
        ("(@2020-02 | @2020).max()", Empty),
        ("(1 'm' | 2 's').max()", Empty),
        ("('A' | 'a').max()", FhirPathValue::String("a".to_owned())),
    ]);
    assert!(evaluate("(1 | 2L).sum()").is_err());
    assert!(evaluate("(1 | '2').min()").is_err());
}
