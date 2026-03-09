//! CQL runtime value model.
//!
//! [`Value`] represents every possible CQL runtime value, including the special
//! `Null` sentinel.
//!
//! # Date/Time types
//!
//! CQL dates and times are _partial_: a date may be specified at year, month,
//! or day precision; a time at hour, minute, second, or millisecond precision.
//! The "coarser" fields are always present while the "finer" fields are
//! `Option`.
//!
//! # Equality vs. Equivalence
//!
//! - **CQL equality** (`cql_equal`): `null` propagates.  If either operand is
//!   `Null`, the result is `Value::Null`.
//! - **CQL equivalence** (`cql_equivalent`): null-safe.  `Null` is equivalent
//!   to `Null`; two values of different types are not equivalent.

use std::collections::BTreeMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Scalar date/time types
// ---------------------------------------------------------------------------

/// A partial CQL date: year is always present; month and day are optional.
///
/// Valid precisions (per §3 of the CQL specification):
/// - year only → `month = None, day = None`
/// - year-month → `day = None`
/// - year-month-day → fully specified
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlDate {
    pub year: i32,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

/// A partial CQL date-time with an optional timezone offset.
///
/// Finer components are present only when coarser components are also present
/// (e.g. `minute` requires `hour` to be `Some`).  `offset_seconds` is the
/// signed offset east of UTC in seconds (e.g. `-18000` for UTC−5).
#[derive(Debug, Clone, PartialEq)]
pub struct CqlDateTime {
    pub year: i32,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub millisecond: Option<u32>,
    /// Timezone offset in seconds east of UTC, or `None` when unspecified.
    pub offset_seconds: Option<i32>,
}

/// A partial CQL time of day.
///
/// `hour` is always present; finer components are optional.
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTime {
    pub hour: u8,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub millisecond: Option<u32>,
}

// ---------------------------------------------------------------------------
// Quantity and Code types
// ---------------------------------------------------------------------------

/// A CQL quantity: a numeric value paired with a UCUM unit string.
///
/// The numeric value is stored as `f64`; future phases may upgrade this to an
/// arbitrary-precision decimal if needed.
#[derive(Debug, Clone, PartialEq)]
pub struct CqlQuantity {
    pub value: f64,
    pub unit: String,
}

/// A single clinical code from a given code system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlCode {
    pub code: String,
    pub system: String,
    pub display: Option<String>,
    pub version: Option<String>,
}

/// A clinical concept: a set of clinically equivalent codes with an optional
/// display label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlConcept {
    pub codes: Vec<CqlCode>,
    pub display: Option<String>,
}

// ---------------------------------------------------------------------------
// Value enum
// ---------------------------------------------------------------------------

/// All possible CQL runtime values.
///
/// `Value::Null` represents the CQL `null` sentinel.  All other variants carry
/// the corresponding Rust representation of the CQL type.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// CQL `null`.
    Null,

    /// `Boolean` — true or false.
    Boolean(bool),

    /// `Integer` — 32-bit signed integer in CQL spec, stored as `i64` for
    /// safety and forward compatibility.
    Integer(i64),

    /// `Long` — 64-bit signed integer in the CQL spec (2020+).
    Long(i128),

    /// `Decimal` — arbitrary-precision in the CQL spec; stored as `f64` for
    /// now.  Precision is sufficient for clinical use and matches the ELM
    /// JSON representation used elsewhere in this crate.
    Decimal(f64),

    /// `String`.
    String(String),

    /// `Date` — partial calendar date.
    Date(CqlDate),

    /// `DateTime` — partial date-time with optional timezone.
    DateTime(CqlDateTime),

    /// `Time` — partial time of day.
    Time(CqlTime),

    /// `Quantity` — number with UCUM unit.
    Quantity(CqlQuantity),

    /// `Ratio` — fraction of two quantities.
    Ratio {
        numerator: CqlQuantity,
        denominator: CqlQuantity,
    },

    /// `Code` — a single coded value.
    Code(CqlCode),

    /// `Concept` — a set of equivalent codes.
    Concept(CqlConcept),

    /// `List` — ordered, possibly heterogeneous collection.
    List(Vec<Value>),

    /// `Tuple` — named fields; field names are sorted for determinism.
    Tuple(BTreeMap<std::string::String, Value>),

    /// `Interval` — lower/upper bounds with open/closed endpoints.
    ///
    /// When `low` or `high` is `None` the bound is conceptually −∞ or +∞.
    Interval {
        low: Option<Box<Value>>,
        high: Option<Box<Value>>,
        low_closed: bool,
        high_closed: bool,
    },
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for CqlDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.year)?;
        if let Some(m) = self.month {
            write!(f, "-{:02}", m)?;
            if let Some(d) = self.day {
                write!(f, "-{:02}", d)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for CqlDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.year)?;
        if let Some(m) = self.month {
            write!(f, "-{:02}", m)?;
            if let Some(d) = self.day {
                write!(f, "-{:02}", d)?;
                if let Some(h) = self.hour {
                    write!(f, "T{:02}", h)?;
                    if let Some(min) = self.minute {
                        write!(f, ":{:02}", min)?;
                        if let Some(s) = self.second {
                            write!(f, ":{:02}", s)?;
                            if let Some(ms) = self.millisecond {
                                write!(f, ".{:03}", ms)?;
                            }
                        }
                    }
                }
            }
        }
        if let Some(off) = self.offset_seconds {
            if off == 0 {
                write!(f, "Z")?;
            } else {
                let sign = if off >= 0 { '+' } else { '-' };
                let abs = off.unsigned_abs();
                write!(f, "{}{:02}:{:02}", sign, abs / 3600, (abs % 3600) / 60)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for CqlTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.hour)?;
        if let Some(m) = self.minute {
            write!(f, ":{:02}", m)?;
            if let Some(s) = self.second {
                write!(f, ":{:02}", s)?;
                if let Some(ms) = self.millisecond {
                    write!(f, ".{:03}", ms)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for CqlQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} '{}'", self.value, self.unit)
    }
}

impl fmt::Display for CqlCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code '{}' from {}", self.code, self.system)?;
        if let Some(d) = &self.display {
            write!(f, " display '{d}'")?;
        }
        Ok(())
    }
}

impl fmt::Display for CqlConcept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Concept {{")?;
        for (i, code) in self.codes.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{code}")?;
        }
        write!(f, "}}")?;
        if let Some(d) = &self.display {
            write!(f, " display '{d}'")?;
        }
        Ok(())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Long(n) => write!(f, "{n}"),
            Value::Decimal(d) => write!(f, "{d}"),
            Value::String(s) => write!(f, "'{s}'"),
            Value::Date(d) => write!(f, "@{d}"),
            Value::DateTime(dt) => write!(f, "@{dt}"),
            Value::Time(t) => write!(f, "@T{t}"),
            Value::Quantity(q) => write!(f, "{q}"),
            Value::Ratio {
                numerator,
                denominator,
            } => {
                write!(f, "{numerator} : {denominator}")
            }
            Value::Code(c) => write!(f, "{c}"),
            Value::Concept(c) => write!(f, "{c}"),
            Value::List(items) => {
                write!(f, "{{")?;
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "}}")
            }
            Value::Tuple(fields) => {
                write!(f, "Tuple{{")?;
                for (i, (key, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{key}: {val}")?;
                }
                write!(f, "}}")
            }
            Value::Interval {
                low,
                high,
                low_closed,
                high_closed,
            } => {
                let open_l = if *low_closed { '[' } else { '(' };
                let open_r = if *high_closed { ']' } else { ')' };
                let low_str = low
                    .as_deref()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "null".to_string());
                let high_str = high
                    .as_deref()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "null".to_string());
                write!(f, "Interval{open_l}{low_str}, {high_str}{open_r}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CQL equality and equivalence
// ---------------------------------------------------------------------------

/// CQL equality — returns `Value::Null` when either operand is `Null`,
/// otherwise returns `Value::Boolean(true/false)`.
///
/// Per the CQL specification, equality is null-propagating: if either argument
/// is `null` the result is `null`.
///
/// ```
/// use rh_cql::eval::value::{cql_equal, Value};
///
/// assert_eq!(cql_equal(&Value::Integer(3), &Value::Integer(3)), Value::Boolean(true));
/// assert_eq!(cql_equal(&Value::Integer(3), &Value::Integer(4)), Value::Boolean(false));
/// assert_eq!(cql_equal(&Value::Null, &Value::Integer(3)), Value::Null);
/// assert_eq!(cql_equal(&Value::Null, &Value::Null), Value::Null);
/// ```
pub fn cql_equal(a: &Value, b: &Value) -> Value {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Value::Null;
    }
    // Integer/Decimal cross-type comparison: implicit promotion.
    match (a, b) {
        (Value::Decimal(x), Value::Integer(y)) => return Value::Boolean(*x == *y as f64),
        (Value::Integer(x), Value::Decimal(y)) => return Value::Boolean(*x as f64 == *y),
        // Time equality: different precision → Null (CQL §5.6).
        (Value::Time(at), Value::Time(bt)) => {
            if at.millisecond.is_some() != bt.millisecond.is_some()
                || at.second.is_some() != bt.second.is_some()
                || at.minute.is_some() != bt.minute.is_some()
            {
                return Value::Null;
            }
            return Value::Boolean(at == bt);
        }
        // DateTime equality: different precision → Null.
        (Value::DateTime(adt), Value::DateTime(bdt)) => {
            if adt.millisecond.is_some() != bdt.millisecond.is_some()
                || adt.second.is_some() != bdt.second.is_some()
                || adt.minute.is_some() != bdt.minute.is_some()
                || adt.hour.is_some() != bdt.hour.is_some()
                || adt.day.is_some() != bdt.day.is_some()
                || adt.month.is_some() != bdt.month.is_some()
            {
                return Value::Null;
            }
            return Value::Boolean(adt == bdt);
        }
        // Date equality: different precision → Null.
        (Value::Date(ad), Value::Date(bd)) => {
            if ad.day.is_some() != bd.day.is_some() || ad.month.is_some() != bd.month.is_some() {
                return Value::Null;
            }
            return Value::Boolean(ad == bd);
        }
        // List equality: element-by-element with null propagation.
        (Value::List(av), Value::List(bv)) => {
            if av.len() != bv.len() {
                return Value::Boolean(false);
            }
            let mut result = Value::Boolean(true);
            for (ai, bi) in av.iter().zip(bv.iter()) {
                match cql_equal(ai, bi) {
                    Value::Null => {
                        // A null comparison propagates null (unless we later see false).
                        result = Value::Null;
                    }
                    Value::Boolean(false) => return Value::Boolean(false),
                    _ => {}
                }
            }
            return result;
        }
        _ => {}
    }
    Value::Boolean(a == b)
}

/// CQL equivalence — null-safe comparison.
///
/// Unlike CQL equality, equivalence treats `null` as equivalent to `null`
/// (returns `true`).  For non-null values the comparison mirrors structural
/// equality.
///
/// Per the CQL specification, equivalence is used in `~` comparisons and for
/// membership in concepts.
///
/// ```
/// use rh_cql::eval::value::{cql_equivalent, Value};
///
/// assert!(cql_equivalent(&Value::Null, &Value::Null));
/// assert!(!cql_equivalent(&Value::Null, &Value::Integer(1)));
/// assert!(cql_equivalent(&Value::Integer(5), &Value::Integer(5)));
/// assert!(!cql_equivalent(&Value::Integer(5), &Value::Integer(6)));
/// ```
pub fn cql_equivalent(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Null, _) | (_, Value::Null) => false,
        // Integer/Decimal cross-type: 1.0 ~ 1 → true
        (Value::Decimal(x), Value::Integer(y)) => *x == *y as f64,
        (Value::Integer(x), Value::Decimal(y)) => *x as f64 == *y,
        // Decimal: treat NaN as not equivalent; use total-order bitwise compare
        // to avoid surprises from IEEE 754 NaN behavior.
        (Value::Decimal(x), Value::Decimal(y)) => x.to_bits() == y.to_bits(),
        _ => a == b,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_display_primitives() {
        assert_eq!(Value::Null.to_string(), "null");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Integer(42).to_string(), "42");
        assert_eq!(
            Value::Long(9_999_999_999_999_999_999).to_string(),
            "9999999999999999999"
        );
        assert_eq!(Value::Decimal(1.25).to_string(), "1.25");
        assert_eq!(Value::String("hello".to_string()).to_string(), "'hello'");
    }

    #[test]
    fn value_display_date() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: Some(3),
            day: Some(15),
        });
        assert_eq!(d.to_string(), "@2024-03-15");

        let d_year = Value::Date(CqlDate {
            year: 2024,
            month: None,
            day: None,
        });
        assert_eq!(d_year.to_string(), "@2024");
    }

    #[test]
    fn value_display_datetime() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(15),
            hour: Some(10),
            minute: Some(30),
            second: Some(0),
            millisecond: None,
            offset_seconds: Some(-18000),
        });
        assert_eq!(dt.to_string(), "@2024-01-15T10:30:00-05:00");
    }

    #[test]
    fn value_display_time() {
        let t = Value::Time(CqlTime {
            hour: 14,
            minute: Some(30),
            second: Some(45),
            millisecond: None,
        });
        assert_eq!(t.to_string(), "@T14:30:45");
    }

    #[test]
    fn value_display_list() {
        let list = Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        assert_eq!(list.to_string(), "{1, 2, 3}");
    }

    #[test]
    fn value_display_interval() {
        let interval = Value::Interval {
            low: Some(Box::new(Value::Integer(1))),
            high: Some(Box::new(Value::Integer(10))),
            low_closed: true,
            high_closed: true,
        };
        assert_eq!(interval.to_string(), "Interval[1, 10]");
    }

    #[test]
    fn cql_equal_primitives() {
        assert_eq!(
            cql_equal(&Value::Integer(3), &Value::Integer(3)),
            Value::Boolean(true)
        );
        assert_eq!(
            cql_equal(&Value::Integer(3), &Value::Integer(4)),
            Value::Boolean(false)
        );
        assert_eq!(
            cql_equal(&Value::Boolean(true), &Value::Boolean(true)),
            Value::Boolean(true)
        );
        assert_eq!(
            cql_equal(&Value::Boolean(true), &Value::Boolean(false)),
            Value::Boolean(false)
        );
    }

    #[test]
    fn cql_equal_null_propagation() {
        // Null propagates: any operand null → result null
        assert_eq!(cql_equal(&Value::Null, &Value::Integer(3)), Value::Null);
        assert_eq!(cql_equal(&Value::Integer(3), &Value::Null), Value::Null);
        assert_eq!(cql_equal(&Value::Null, &Value::Null), Value::Null);
    }

    #[test]
    fn cql_equivalent_null_safe() {
        assert!(cql_equivalent(&Value::Null, &Value::Null));
        assert!(!cql_equivalent(&Value::Null, &Value::Integer(1)));
        assert!(!cql_equivalent(&Value::Integer(1), &Value::Null));
        assert!(cql_equivalent(&Value::Integer(5), &Value::Integer(5)));
        assert!(!cql_equivalent(&Value::Integer(5), &Value::Integer(6)));
    }

    #[test]
    fn value_list_contains_three_integers() {
        let list = Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        if let Value::List(items) = &list {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Integer(1));
        } else {
            panic!("expected List");
        }
    }
}
