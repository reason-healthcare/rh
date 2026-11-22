//! Precision-aware Date/DateTime/Time comparison per FHIRPath spec.
//!
//! Per §5.1.1: when one input has a value for a sub-component the other lacks,
//! the result of the comparison is empty (None here). Equality treats trailing
//! zero fractional seconds (`.0`) as equivalent to no fractional. When both
//! inputs carry a timezone, comparison happens at the UTC instant.

use std::cmp::Ordering;

use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::ast::DateTimePrecision;
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::types::FhirPathValue;

use DateTimePrecision::{Day, Hour, Millisecond, Minute, Month, Second, Week, Year};

/// Components of a parsed temporal value, with explicit presence tracking.
#[derive(Debug, Clone, Copy)]
pub struct TemporalParts {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    /// Fractional seconds normalized to nanoseconds (None if the input had no `.fff`).
    pub fraction_ns: Option<u32>,
    fraction_digits: usize,
    /// Timezone offset in minutes from UTC (None if absent in the input).
    pub tz_offset_min: Option<i32>,
}

impl TemporalParts {
    fn empty() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            fraction_ns: None,
            fraction_digits: 0,
            tz_offset_min: None,
        }
    }
}

/// Parse a FHIRPath temporal string (Date, DateTime, or Time storage form).
///
/// Accepts the same shapes that the parser emits:
/// - Date:     `YYYY` | `YYYY-MM` | `YYYY-MM-DD`
/// - DateTime: `YYYY[-MM[-DD]]T[hh[:mm[:ss[.fff]]]][TZ]`
/// - Time:     `Thh[:mm[:ss[.fff]]]`
///
/// Returns `None` if the string is not recognisable as a temporal value.
pub fn parse_temporal(s: &str) -> Option<TemporalParts> {
    let mut parts = TemporalParts::empty();
    let mut rest = s;

    // Time-only form starts with `T`.
    if let Some(time_body) = rest.strip_prefix('T') {
        parse_time(&mut parts, time_body)?;
        parts.hour?;
        parts.local_datetime()?;
        return Some(parts);
    }

    // Date portion: YYYY (-MM (-DD)?)?
    let (year_str, after_year) = take_digits(rest, 4)?;
    parts.year = Some(year_str.parse().ok()?);
    rest = after_year;

    if let Some(after_dash) = rest.strip_prefix('-') {
        let (month_str, after_month) = take_digits(after_dash, 2)?;
        parts.month = Some(month_str.parse().ok()?);
        rest = after_month;
        if let Some(after_dash2) = rest.strip_prefix('-') {
            let (day_str, after_day) = take_digits(after_dash2, 2)?;
            parts.day = Some(day_str.parse().ok()?);
            rest = after_day;
        }
    }

    // Optional T<time>
    if let Some(after_t) = rest.strip_prefix('T') {
        parse_time(&mut parts, after_t)?;
    } else if !rest.is_empty() {
        return None;
    }

    if parts.hour.is_some() && parts.day.is_none() {
        return None;
    }
    parts.local_datetime()?;
    Some(parts)
}

fn parse_time(parts: &mut TemporalParts, mut body: &str) -> Option<()> {
    if body.is_empty() {
        return Some(()); // `@2015T` style — no time portion.
    }
    let (hh, after) = take_digits(body, 2)?;
    parts.hour = Some(hh.parse().ok()?);
    body = after;
    if let Some(after_colon) = body.strip_prefix(':') {
        let (mm, after) = take_digits(after_colon, 2)?;
        parts.minute = Some(mm.parse().ok()?);
        body = after;
        if let Some(after_colon2) = body.strip_prefix(':') {
            let (ss, after) = take_digits(after_colon2, 2)?;
            parts.second = Some(ss.parse().ok()?);
            body = after;
            if let Some(after_dot) = body.strip_prefix('.') {
                let frac_end = after_dot
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(after_dot.len());
                let frac_str = &after_dot[..frac_end];
                if frac_str.is_empty() {
                    return None;
                }
                parts.fraction_ns = Some(parse_fraction_to_ns(frac_str));
                parts.fraction_digits = frac_str.len();
                body = &after_dot[frac_end..];
            }
        }
    }
    // Optional timezone
    if !body.is_empty() {
        parts.tz_offset_min = Some(parse_tz(body)?);
    }
    Some(())
}

fn parse_tz(s: &str) -> Option<i32> {
    if s == "Z" {
        return Some(0);
    }
    let sign = match s.chars().next()? {
        '+' => 1,
        '-' => -1,
        _ => return None,
    };
    let body = &s[1..];
    if body.as_bytes().get(2) != Some(&b':') || body.len() != 5 {
        return None;
    }
    let (hours, minutes) = body.split_once(':')?;
    let (h, _) = take_digits(hours, 2)?;
    let (m, _) = take_digits(minutes, 2)?;
    let h: i32 = h.parse().ok()?;
    let m: i32 = m.parse().ok()?;
    (h <= 23 && m <= 59).then_some(sign * (h * 60 + m))
}

/// Convert a decimal fraction string (e.g. "5", "123", "123456789") to nanoseconds.
fn parse_fraction_to_ns(frac: &str) -> u32 {
    // Pad/truncate to 9 digits, parse as u32 (fits up to 999_999_999).
    let mut buf = [b'0'; 9];
    for (i, b) in frac.as_bytes().iter().take(9).enumerate() {
        buf[i] = *b;
    }
    // SAFETY: digits are ASCII.
    std::str::from_utf8(&buf).unwrap().parse().unwrap_or(0)
}

fn take_digits(s: &str, n: usize) -> Option<(&str, &str)> {
    let head = s.get(..n)?;
    if !head.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((head, &s[n..]))
}

/// Compare values component by component, normalizing explicit offsets first.
pub fn compare_parts(a: &TemporalParts, b: &TemporalParts) -> Option<Ordering> {
    compare(a, b, false)
}

fn compare(a: &TemporalParts, b: &TemporalParts, equivalent: bool) -> Option<Ordering> {
    let left;
    let right;
    let (a, b) = match (a.tz_offset_min, b.tz_offset_min) {
        (Some(a_offset), Some(b_offset)) if a_offset != b_offset => {
            left = a.utc_parts()?;
            right = b.utc_parts()?;
            (&left, &right)
        }
        _ => (a, b),
    };

    if let Some(ord) = compare_date(a, b)? {
        return Some(ord);
    }
    compare_time(a, b, equivalent)
}

/// Equality per FHIRPath spec for temporal values.
///
/// Returns `None` for non-comparable precision (one TZ vs none at the time
/// portion, or mismatched non-fractional precision). Returns `Some(true)`/
/// `Some(false)` otherwise.
pub fn equals_parts(a: &TemporalParts, b: &TemporalParts) -> Option<bool> {
    compare_parts(a, b).map(|o| o == Ordering::Equal)
}

/// Equivalence rounds explicit fractional seconds to the least precise
/// input. A seconds-only value remains distinct from a nonzero fraction.
pub fn equivalent_parts(a: &TemporalParts, b: &TemporalParts) -> bool {
    compare(a, b, true) == Some(Ordering::Equal)
}

/// Walk year→month→day. Returns:
///   `Ok(Some(ord))` — date components settled the order
///   `Ok(None)`      — date components fully matched (or both absent)
///   `Err(())`       — precision mismatch on calendar (one Date, one DateTime
///                     where day differs) — propagate as None to caller.
fn compare_date(a: &TemporalParts, b: &TemporalParts) -> Option<Option<Ordering>> {
    macro_rules! step {
        ($field:ident) => {
            match (a.$field, b.$field) {
                (Some(av), Some(bv)) => match av.cmp(&bv) {
                    Ordering::Equal => {}
                    other => return Some(Some(other)),
                },
                (None, None) => {}
                _ => return None, // precision mismatch on the date portion
            }
        };
    }
    step!(year);
    step!(month);
    step!(day);
    Some(None)
}

/// Walk hour→fraction with TZ handling. Called only after date components matched.
fn compare_time(a: &TemporalParts, b: &TemporalParts, equivalent: bool) -> Option<Ordering> {
    if a.tz_offset_min.is_some() != b.tz_offset_min.is_some() {
        return None;
    }
    // Naive component-wise walk for time portion.
    macro_rules! step {
        ($field:ident) => {
            match (a.$field, b.$field) {
                (Some(av), Some(bv)) => match av.cmp(&bv) {
                    Ordering::Equal => {}
                    other => return Some(other),
                },
                (None, None) => {}
                _ => return None,
            }
        };
    }
    step!(hour);
    step!(minute);
    if equivalent {
        if let (Some(a_sec), Some(a_frac), Some(b_sec), Some(b_frac)) =
            (a.second, a.fraction_ns, b.second, b.fraction_ns)
        {
            let seconds =
                |second, fraction| Decimal::from(second) + Decimal::new(i64::from(fraction), 9);
            let left = seconds(a_sec, a_frac);
            let right = seconds(b_sec, b_frac);
            let digits = left.normalize().scale().min(right.normalize().scale());
            return Some(left.round_dp(digits).cmp(&right.round_dp(digits)));
        }
    }
    step!(second);
    Some(a.fraction_ns.unwrap_or(0).cmp(&b.fraction_ns.unwrap_or(0)))
}

impl TemporalParts {
    pub(crate) fn fraction_text(self) -> Option<String> {
        let nanos = self.fraction_ns?;
        let digits = self.fraction_digits.min(9);
        Some(format!(
            "{:0digits$}",
            nanos / 10_u32.pow((9 - digits) as u32)
        ))
    }

    fn local_datetime(self) -> Option<NaiveDateTime> {
        let date = NaiveDate::from_ymd_opt(
            self.year.unwrap_or(2000),
            u32::from(self.month.unwrap_or(1)),
            u32::from(self.day.unwrap_or(1)),
        )?;
        let second = u32::from(self.second.unwrap_or(0));
        if second > 60 {
            return None;
        }
        let leap_nanoseconds = if second == 60 { 1_000_000_000 } else { 0 };
        let time = NaiveTime::from_hms_nano_opt(
            u32::from(self.hour.unwrap_or(0)),
            u32::from(self.minute.unwrap_or(0)),
            second.min(59),
            self.fraction_ns.unwrap_or(0) + leap_nanoseconds,
        )?;
        Some(date.and_time(time))
    }

    fn utc_datetime(self) -> Option<NaiveDateTime> {
        self.local_datetime()?
            .checked_sub_signed(Duration::minutes(i64::from(
                self.tz_offset_min.unwrap_or(0),
            )))
    }

    fn utc_parts(self) -> Option<Self> {
        let utc = self.utc_datetime()?;
        Some(Self {
            year: self.year.map(|_| utc.year()),
            month: self.month.map(|_| utc.month() as u8),
            day: self.day.map(|_| utc.day() as u8),
            hour: self.hour.map(|_| utc.hour() as u8),
            minute: self.minute.map(|_| utc.minute() as u8),
            second: self
                .second
                .map(|_| (utc.second() + utc.nanosecond() / 1_000_000_000) as u8),
            tz_offset_min: Some(0),
            ..self
        })
    }

    fn precision(self) -> DateTimePrecision {
        if self.fraction_ns.is_some() {
            Millisecond
        } else if self.second.is_some() {
            Second
        } else if self.minute.is_some() {
            Minute
        } else if self.hour.is_some() {
            Hour
        } else if self.day.is_some() {
            Day
        } else if self.month.is_some() {
            Month
        } else {
            Year
        }
    }
}

pub(super) fn shift_temporal(
    value: &FhirPathValue,
    count: Decimal,
    unit: &str,
) -> FhirPathResult<FhirPathValue> {
    let (text, kind, parts) = temporal_value(value).ok_or_else(|| FhirPathError::TypeError {
        message: format!("Invalid date/time value: {value:?}"),
    })?;
    let precision = duration_unit(unit).filter(|precision| kind.accepts(precision));
    let Some(precision) = precision else {
        return Err(FhirPathError::TypeError {
            message: format!("Cannot apply duration unit '{unit}' to this date/time value"),
        });
    };

    let (count, precision) = convert_shift_precision(count, precision, parts.precision())
        .ok_or_else(temporal_overflow)?;
    let date_time = parts.local_datetime().ok_or_else(temporal_overflow)?;
    let shifted = if kind == TemporalKind::Time {
        // Modulo a day keeps even very large durations in Time's cyclic range.
        let nanos_per_unit =
            duration_nanoseconds(Decimal::ONE, &precision).ok_or_else(temporal_overflow)?;
        let units_per_day = Decimal::from(86_400_000_000_000_i64) / nanos_per_unit;
        let nanos = count
            .checked_rem(units_per_day)
            .and_then(|count| count.checked_mul(nanos_per_unit))
            .and_then(|nanos| nanos.to_i64())
            .ok_or_else(temporal_overflow)?;
        date_time
            .date()
            .and_time(date_time.time() + Duration::nanoseconds(nanos))
    } else {
        shift_datetime(date_time, count, &precision).ok_or_else(temporal_overflow)?
    };
    if !(1..=9999).contains(&shifted.year()) {
        return Err(temporal_overflow());
    }

    let date = match parts.precision() {
        Year => shifted.format("%Y").to_string(),
        Month => shifted.format("%Y-%m").to_string(),
        _ => shifted.format("%Y-%m-%d").to_string(),
    };
    let time = if parts.hour.is_none() {
        String::new()
    } else {
        let mut time = match parts.precision() {
            Hour => shifted.format("%H").to_string(),
            Minute => shifted.format("%H:%M").to_string(),
            _ => shifted.format("%H:%M:%S").to_string(),
        };
        let nanos = shifted.nanosecond() % 1_000_000_000;
        if parts.fraction_digits != 0 || nanos != 0 {
            let digits = parts.fraction_digits.clamp(3, 9);
            time.push_str(&format!(
                ".{:0digits$}",
                nanos / 10_u32.pow((9 - digits) as u32)
            ));
        }
        time
    };
    Ok(match kind {
        TemporalKind::Date => FhirPathValue::Date(date),
        TemporalKind::DateTime => {
            FhirPathValue::DateTime(format!("{date}T{time}{}", timezone_suffix(text)))
        }
        TemporalKind::Time => FhirPathValue::Time(time),
    })
}

fn temporal_overflow() -> FhirPathError {
    FhirPathError::EvaluationError {
        message: "Date/time arithmetic result is out of range".to_string(),
    }
}

fn convert_shift_precision(
    count: Decimal,
    unit: DateTimePrecision,
    precision: DateTimePrecision,
) -> Option<(Decimal, DateTimePrecision)> {
    let count = if matches!(unit, Second | Millisecond) {
        count
    } else {
        count.trunc()
    };
    if precision_rank(&unit) <= precision_rank(&precision)
        || (unit == Millisecond && precision == Second)
    {
        return Some((count, unit));
    }
    // Calendar years and months intentionally use different conversions to days.
    let converted = match (&unit, &precision) {
        (Month, Year) => count.checked_div(Decimal::from(12))?,
        _ => {
            let from = duration_nanoseconds(count, &unit)?;
            let to = match precision {
                Year => Decimal::from(365 * 86_400_000_000_000_i64),
                Month => Decimal::from(30 * 86_400_000_000_000_i64),
                _ => duration_nanoseconds(Decimal::ONE, &precision)?,
            };
            from.checked_div(to)?
        }
    };
    Some((converted.trunc(), precision))
}

fn duration_nanoseconds(count: Decimal, unit: &DateTimePrecision) -> Option<Decimal> {
    let nanos = match unit {
        Week => 604_800_000_000_000_i64,
        Day => 86_400_000_000_000,
        Hour => 3_600_000_000_000,
        Minute => 60_000_000_000,
        Second => 1_000_000_000,
        Millisecond => 1_000_000,
        Year | Month => return None,
    };
    count.checked_mul(Decimal::from(nanos))
}

fn shift_datetime(
    date_time: NaiveDateTime,
    count: Decimal,
    unit: &DateTimePrecision,
) -> Option<NaiveDateTime> {
    match unit {
        Year | Month => {
            let months = count
                .to_i64()?
                .checked_mul(if *unit == Year { 12 } else { 1 })?;
            let magnitude = Months::new(u32::try_from(months.unsigned_abs()).ok()?);
            if months < 0 {
                date_time.checked_sub_months(magnitude)
            } else {
                date_time.checked_add_months(magnitude)
            }
        }
        _ => {
            let nanos = duration_nanoseconds(count, unit)?;
            let nanos_per_second = Decimal::from(1_000_000_000);
            let seconds = nanos.checked_div(nanos_per_second)?.trunc().to_i64()?;
            let remainder = nanos.checked_rem(nanos_per_second)?.to_i64()?;
            date_time
                .checked_add_signed(Duration::try_seconds(seconds)?)?
                .checked_add_signed(Duration::nanoseconds(remainder))
        }
    }
}

fn duration_unit(unit: &str) -> Option<DateTimePrecision> {
    match unit {
        "year" | "years" => Some(Year),
        "month" | "months" => Some(Month),
        "week" | "weeks" | "wk" => Some(Week),
        "day" | "days" | "d" => Some(Day),
        "hour" | "hours" | "h" => Some(Hour),
        "minute" | "minutes" | "min" => Some(Minute),
        "second" | "seconds" | "s" => Some(Second),
        "millisecond" | "milliseconds" | "ms" => Some(Millisecond),
        _ => None,
    }
}

fn precision_rank(unit: &DateTimePrecision) -> u8 {
    match unit {
        Year => 0,
        Month => 1,
        Week | Day => 2,
        Hour => 3,
        Minute => 4,
        Second => 5,
        Millisecond => 6,
    }
}

#[derive(PartialEq, Eq)]
enum TemporalKind {
    Date,
    DateTime,
    Time,
}

impl TemporalKind {
    fn accepts(&self, precision: &DateTimePrecision) -> bool {
        match self {
            Self::Date => precision_rank(precision) <= precision_rank(&Day),
            Self::DateTime => true,
            Self::Time => precision_rank(precision) >= precision_rank(&Hour),
        }
    }
}

fn temporal_value(value: &FhirPathValue) -> Option<(&str, TemporalKind, TemporalParts)> {
    match value {
        FhirPathValue::Date(text) => Some((text, TemporalKind::Date, parse_temporal(text)?)),
        FhirPathValue::DateTime(text) | FhirPathValue::TypedDateTime { value: text, .. } => {
            Some((text, TemporalKind::DateTime, parse_temporal(text)?))
        }
        FhirPathValue::Time(text) => {
            let mut parts = TemporalParts::empty();
            parse_time(&mut parts, text.strip_prefix('T').unwrap_or(text))?;
            parts.hour?;
            parts.local_datetime()?;
            Some((text, TemporalKind::Time, parts))
        }
        _ => None,
    }
}

pub(crate) fn timezone_suffix(text: &str) -> &str {
    let time = text.split_once('T').map_or(text, |(_, time)| time);
    if time.ends_with('Z') {
        return "Z";
    }
    time.rfind(['+', '-'])
        .map(|index| &time[index..])
        .filter(|suffix| parse_tz(suffix).is_some())
        .unwrap_or("")
}

pub(crate) fn interval(
    target: &FhirPathValue,
    value: &FhirPathValue,
    precision: &str,
    difference: bool,
) -> FhirPathResult<FhirPathValue> {
    let incompatible = || {
        FhirPathError::TypeError {
        message: "Interval functions require two Date, DateTime, or Time values of the same type and a precision supported for that type".to_string(),
    }
    };
    let (_, start_kind, start) = temporal_value(target).ok_or_else(incompatible)?;
    let (_, end_kind, end) = temporal_value(value).ok_or_else(incompatible)?;
    let precision = duration_unit(precision)
        .filter(|unit| precision == unit.to_string())
        .ok_or_else(incompatible)?;
    if start_kind != end_kind || !start_kind.accepts(&precision) {
        return Err(incompatible());
    }
    if precision_rank(&start.precision()) < precision_rank(&precision)
        || precision_rank(&end.precision()) < precision_rank(&precision)
    {
        return Ok(FhirPathValue::Empty);
    }
    let normalize =
        start_kind == TemporalKind::DateTime && precision_rank(&precision) >= precision_rank(&Hour);
    if normalize && start.tz_offset_min.is_some() != end.tz_offset_min.is_some() {
        return Ok(FhirPathValue::Empty);
    }
    let datetime = |parts: TemporalParts| {
        if normalize {
            parts.utc_datetime()
        } else {
            parts.local_datetime()
        }
        .ok_or_else(incompatible)
    };
    let start = datetime(start)?;
    let end = datetime(end)?;
    let result = if difference {
        boundary_index(end, &precision) - boundary_index(start, &precision)
    } else if matches!(precision, Year | Month) {
        if end < start {
            -whole_calendar_periods(end, start, &precision)
        } else {
            whole_calendar_periods(start, end, &precision)
        }
    } else {
        let elapsed = end - start;
        match precision {
            Week => elapsed.num_weeks(),
            Day => elapsed.num_days(),
            Hour => elapsed.num_hours(),
            Minute => elapsed.num_minutes(),
            Second => elapsed.num_seconds(),
            Millisecond => elapsed.num_milliseconds(),
            Year | Month => unreachable!(),
        }
    };
    Ok(super::arithmetic::integer_result(result))
}

fn boundary_index(value: NaiveDateTime, precision: &DateTimePrecision) -> i64 {
    let ms = value.and_utc().timestamp_millis();
    match precision {
        Year => i64::from(value.year()),
        Month => i64::from(value.year() * 12 + value.month0() as i32),
        // 1970-01-01 is Thursday, four days from the Sunday week boundary.
        Week => (ms.div_euclid(86_400_000) + 4).div_euclid(7),
        Day => ms.div_euclid(86_400_000),
        Hour => ms.div_euclid(3_600_000),
        Minute => ms.div_euclid(60_000),
        Second => ms.div_euclid(1000),
        Millisecond => ms,
    }
}

fn whole_calendar_periods(
    start: NaiveDateTime,
    end: NaiveDateTime,
    precision: &DateTimePrecision,
) -> i64 {
    let estimate = boundary_index(end, precision) - boundary_index(start, precision);
    match shift_datetime(start, Decimal::from(estimate), precision) {
        Some(anniversary) if anniversary <= end => estimate,
        _ => estimate - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_year_only_datetime() {
        let p = parse_temporal("2015T").unwrap();
        assert_eq!(p.year, Some(2015));
        assert_eq!(p.month, None);
        assert_eq!(p.hour, None);
    }

    #[test]
    fn parses_full_datetime_with_fraction_and_tz() {
        let p = parse_temporal("1973-12-25T00:00:00.123+10:00").unwrap();
        assert_eq!(p.year, Some(1973));
        assert_eq!(p.fraction_ns, Some(123_000_000));
        assert_eq!(p.tz_offset_min, Some(600));
    }

    #[test]
    fn parses_time_only() {
        let p = parse_temporal("T10:30").unwrap();
        assert_eq!(p.hour, Some(10));
        assert_eq!(p.minute, Some(30));
        assert_eq!(p.second, None);
    }

    #[test]
    fn fractional_zero_equals_no_fraction() {
        let a = parse_temporal("2012-04-15T15:30:31").unwrap();
        let b = parse_temporal("2012-04-15T15:30:31.0").unwrap();
        assert_eq!(compare_parts(&a, &b), Some(Ordering::Equal));
        assert_eq!(equals_parts(&a, &b), Some(true));
    }

    #[test]
    fn explicit_fractional_equivalence_rounds_the_second_component() {
        let time = |text| parse_temporal(text).unwrap();
        assert!(equivalent_parts(&time("T12:30:31.0"), &time("T12:30:31")));
        assert!(!equivalent_parts(&time("T12:30:31.1"), &time("T12:30:31")));
        assert!(equivalent_parts(
            &time("T12:30:31.1"),
            &time("T12:30:31.14")
        ));
        assert!(equivalent_parts(
            &time("T12:30:31.0"),
            &time("T12:30:30.999")
        ));
        assert!(!equivalent_parts(
            &time("T12:30:59.999"),
            &time("T12:31:00.0")
        ));
    }

    #[test]
    fn fractional_one_differs() {
        let a = parse_temporal("2012-04-15T15:30:31").unwrap();
        let b = parse_temporal("2012-04-15T15:30:31.1").unwrap();
        assert_eq!(compare_parts(&a, &b), Some(Ordering::Less));
        assert_eq!(equals_parts(&a, &b), Some(false));
    }

    #[test]
    fn missing_seconds_yields_none() {
        let a = parse_temporal("2018-03-01T10:30").unwrap();
        let b = parse_temporal("2018-03-01T10:30:00").unwrap();
        assert!(compare_parts(&a, &b).is_none());
    }

    #[test]
    fn time_missing_seconds_yields_none() {
        let a = parse_temporal("T10:30").unwrap();
        let b = parse_temporal("T10:30:00").unwrap();
        assert!(compare_parts(&a, &b).is_none());
    }

    #[test]
    fn tz_aware_same_instant() {
        let a = parse_temporal("2017-11-05T01:30:00.0-04:00").unwrap();
        let b = parse_temporal("2017-11-05T00:30:00.0-05:00").unwrap();
        assert_eq!(compare_parts(&a, &b), Some(Ordering::Equal));
    }

    #[test]
    fn tz_aware_different_instant() {
        let a = parse_temporal("2017-11-05T01:30:00.0-04:00").unwrap();
        let b = parse_temporal("2017-11-05T01:15:00.0-05:00").unwrap();
        // a = 05:30 UTC, b = 06:15 UTC → a < b
        assert_eq!(compare_parts(&a, &b), Some(Ordering::Less));
    }

    #[test]
    fn one_tz_one_naive_yields_none() {
        let a = parse_temporal("2012-04-15T15:00:00Z").unwrap();
        let b = parse_temporal("2012-04-15T10:00:00").unwrap();
        assert!(compare_parts(&a, &b).is_none());
    }

    #[test]
    fn equivalent_tz_offsets() {
        // +02:00 → -120 min; +03:00 → -180 min; 15:00+02 = 13:00 UTC; 16:00+03 = 13:00 UTC.
        let a = parse_temporal("2012-04-15T15:00:00+02:00").unwrap();
        let b = parse_temporal("2012-04-15T16:00:00+03:00").unwrap();
        assert_eq!(compare_parts(&a, &b), Some(Ordering::Equal));
    }
}
