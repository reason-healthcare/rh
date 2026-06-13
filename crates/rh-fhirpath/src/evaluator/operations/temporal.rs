//! Precision-aware Date/DateTime/Time comparison per FHIRPath spec.
//!
//! Per §5.1.1: when one input has a value for a sub-component the other lacks,
//! the result of the comparison is empty (None here). Equality treats trailing
//! zero fractional seconds (`.0`) as equivalent to no fractional. When both
//! inputs carry a timezone, comparison happens at the UTC instant.

use std::cmp::Ordering;

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
    if body.len() != 5 || !body.is_char_boundary(2) || &body[2..3] != ":" {
        return None;
    }
    let h: i32 = body[..2].parse().ok()?;
    let m: i32 = body[3..].parse().ok()?;
    Some(sign * (h * 60 + m))
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
    if s.len() < n {
        return None;
    }
    let head = &s[..n];
    if !head.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((head, &s[n..]))
}

/// Compare two temporal values per FHIRPath spec.
///
/// Returns `None` when precisions differ at a non-fractional component (per
/// spec, this yields the empty collection). For fractional seconds, a missing
/// fraction is treated as zero — so `T10:30:00` and `T10:30:00.0` compare equal.
///
/// When the time portion is reached and both sides have a timezone, the time
/// portion is compared at the UTC instant. When only one side has a timezone
/// at the time portion, returns `None` (precision mismatch). TZ does not matter
/// when the calendar date alone settles the ordering.
pub fn compare_parts(a: &TemporalParts, b: &TemporalParts) -> Option<Ordering> {
    // Calendar date — TZ is irrelevant here. If the date components settle
    // the order, return immediately without inspecting time/TZ.
    if let Some(ord) = compare_date(a, b)? {
        return Some(ord);
    }
    // At this point the date components either matched fully or were both
    // absent (time-only on both sides). Move on to time + TZ.
    compare_time(a, b)
}

/// Equality per FHIRPath spec for temporal values.
///
/// Returns `None` for non-comparable precision (one TZ vs none at the time
/// portion, or mismatched non-fractional precision). Returns `Some(true)`/
/// `Some(false)` otherwise.
pub fn equals_parts(a: &TemporalParts, b: &TemporalParts) -> Option<bool> {
    compare_parts(a, b).map(|o| o == Ordering::Equal)
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
fn compare_time(a: &TemporalParts, b: &TemporalParts) -> Option<Ordering> {
    // If both sides reach at least an hour AND both carry a timezone, do
    // instant comparison for the time portion (handles +02:00 vs +03:00 etc).
    let both_have_hour = a.hour.is_some() && b.hour.is_some();
    if both_have_hour {
        match (a.tz_offset_min, b.tz_offset_min) {
            (Some(_), Some(_)) => return compare_time_as_instants(a, b),
            (None, None) => {}
            // testEquality23 case: one TZ, one naive — precision mismatch.
            _ => return None,
        }
    } else if a.tz_offset_min.is_some() != b.tz_offset_min.is_some() {
        // One naked date, one date+TZ — caller couldn't have settled this on
        // dates alone (or we wouldn't be here); treat as precision mismatch.
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
    step!(second);
    if a.second.is_none() && b.second.is_none() {
        return Some(Ordering::Equal);
    }
    let af = a.fraction_ns.unwrap_or(0);
    let bf = b.fraction_ns.unwrap_or(0);
    Some(af.cmp(&bf))
}

/// Compare the time portion as UTC instants. Both sides must have hour and TZ.
/// Missing minute/second/fraction are treated as 0. The date portion is
/// included in the instant so that e.g. 2012-04-15T23:00+02:00 and
/// 2012-04-16T01:00+04:00 (same UTC instant) compare equal.
fn compare_time_as_instants(a: &TemporalParts, b: &TemporalParts) -> Option<Ordering> {
    let ai = to_instant_seconds(a)?;
    let bi = to_instant_seconds(b)?;
    let af = a.fraction_ns.unwrap_or(0);
    let bf = b.fraction_ns.unwrap_or(0);
    Some(ai.cmp(&bi).then_with(|| af.cmp(&bf)))
}

/// Compute seconds since UNIX epoch in UTC. Requires year through hour and TZ.
/// Missing minute/second default to 0.
fn to_instant_seconds(p: &TemporalParts) -> Option<i64> {
    let year = p.year?;
    let month = p.month? as u32;
    let day = p.day? as u32;
    let hour = p.hour? as u32;
    let minute = p.minute.unwrap_or(0) as u32;
    let second = p.second.unwrap_or(0) as u32;
    let tz = p.tz_offset_min?;
    let date = chrono::NaiveDate::from_ymd_opt(year, month, day)?;
    let time = chrono::NaiveTime::from_hms_opt(hour, minute, second)?;
    let local_secs = date.and_time(time).and_utc().timestamp();
    Some(local_secs - (tz as i64) * 60)
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
