//! Date and time functions for FHIRPath expressions
//!
//! This module implements FHIRPath date/time functions like now(), today(), and tim                let _time = NaiveTime::parse_from_str(time_part, "%H:%M:%S")?;OfDay().
//! These functions return current system date/time values and don't require a target value.
//! Also includes component extraction functions for Date, DateTime, and Time values.

use crate::error::*;
use crate::evaluator::operations::temporal::parse_temporal;
use crate::evaluator::types::FhirPathValue;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;

/// Helper function to parse datetime strings with timezone handling
fn parse_datetime_str(dt_str: &str) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
    let has_tz = dt_str.ends_with('Z')
        || dt_str
            .find('T')
            .is_some_and(|t| dt_str[t + 1..].contains('+') || dt_str[t + 1..].contains('-'));
    if has_tz {
        DateTime::parse_from_rfc3339(dt_str)
    } else {
        DateTime::parse_from_rfc3339(&format!("{dt_str}Z"))
    }
}

fn component(
    value: &FhirPathValue,
    extract: impl FnOnce(crate::evaluator::operations::temporal::TemporalParts) -> Option<i64>,
) -> FhirPathResult<FhirPathValue> {
    let text = match value {
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) | FhirPathValue::Time(s) => s,
        FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
            if items.len() == 1 =>
        {
            return component(&items[0], extract);
        }
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
            if items.is_empty() =>
        {
            return Ok(FhirPathValue::Empty);
        }
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
            return Err(FhirPathError::TypeError {
                message: format!(
                    "Date/time component functions require a single item, found {}",
                    items.len()
                ),
            });
        }
        _ => return Ok(FhirPathValue::Empty),
    };
    let normalized;
    let temporal_text = if matches!(value, FhirPathValue::Time(_)) && !text.starts_with('T') {
        normalized = format!("T{text}");
        normalized.as_str()
    } else {
        text
    };
    let Some(parts) = parse_temporal(temporal_text) else {
        return Ok(FhirPathValue::Empty);
    };
    Ok(extract(parts)
        .map(FhirPathValue::Integer)
        .unwrap_or(FhirPathValue::Empty))
}

/// Date/time function evaluator
pub struct DateTimeEvaluator;

impl DateTimeEvaluator {
    /// now() - Returns the current date and time (with timezone)
    /// Returns a DateTime value in ISO 8601 format
    pub fn now() -> FhirPathResult<FhirPathValue> {
        let now = Local::now();
        let datetime_string = now.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        Ok(FhirPathValue::DateTime(datetime_string))
    }

    /// today() - Returns the current date (without time)  
    /// Returns a Date value in ISO 8601 format (YYYY-MM-DD)
    pub fn today() -> FhirPathResult<FhirPathValue> {
        let today = Local::now().date_naive();
        let date_string = today.format("%Y-%m-%d").to_string();
        Ok(FhirPathValue::Date(date_string))
    }

    /// timeOfDay() - Returns the current time of day (without date)
    /// Returns a Time value in ISO 8601 format (HH:MM:SS.sss)
    pub fn time_of_day() -> FhirPathResult<FhirPathValue> {
        let now = Local::now().time();
        let time_string = now.format("%H:%M:%S").to_string();
        Ok(FhirPathValue::Time(time_string))
    }

    /// yearOf(date) - Extract year component from Date or DateTime
    /// Returns the year as an Integer value
    pub fn year_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.year.map(i64::from))
    }

    /// monthOf(date) - Extract month component from Date or DateTime  
    /// Returns the month as an Integer value (1-12)
    pub fn month_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.month.map(i64::from))
    }

    /// dayOf(date) - Extract day component from Date or DateTime
    /// Returns the day as an Integer value (1-31)
    pub fn day_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.day.map(i64::from))
    }

    /// hourOf(time) - Extract hour component from Time or DateTime
    /// Returns the hour as an Integer value (0-23)
    pub fn hour_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.hour.map(i64::from))
    }

    /// minuteOf(time) - Extract minute component from Time or DateTime
    /// Returns the minute as an Integer value (0-59)
    pub fn minute_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.minute.map(i64::from))
    }

    /// secondOf(time) - Extract second component from Time or DateTime
    /// Returns the second as an Integer value (0-59)
    pub fn second_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| parts.second.map(i64::from))
    }

    /// millisecondOf(time) - Extract millisecond component from Time or DateTime
    /// Returns the millisecond as an Integer value (0-999)
    pub fn millisecond_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        component(value, |parts| {
            parts.fraction_ns.map(|ns| i64::from(ns / 1_000_000))
        })
    }

    /// timezoneOffsetOf(datetime) - Extract timezone offset from DateTime
    /// Returns the timezone offset as a Number in hours (e.g., -5.0 for EST, 0.0 for UTC)
    pub fn timezone_offset_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() == 1 =>
            {
                Self::timezone_offset_of(&items[0])
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty() =>
            {
                Ok(FhirPathValue::Empty)
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Err(FhirPathError::TypeError {
                    message: format!(
                        "timezoneOffsetOf() requires a single item, found {}",
                        items.len()
                    ),
                })
            }
            FhirPathValue::DateTime(dt_str) => {
                let Some(parts) = parse_temporal(dt_str) else {
                    return Ok(FhirPathValue::Empty);
                };
                if parts.tz_offset_min.is_none() {
                    return Ok(FhirPathValue::Empty);
                }
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                let offset_seconds = datetime.offset().local_minus_utc() as f64;
                let offset_hours =
                    Decimal::from_f64_retain(offset_seconds / 3600.0).unwrap_or(Decimal::ZERO);
                Ok(FhirPathValue::Number(offset_hours))
            }
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// dateOf(datetime) - Extract date component from DateTime
    /// Returns the date part as a Date value
    pub fn date_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() == 1 =>
            {
                Self::date_of(&items[0])
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty() =>
            {
                Ok(FhirPathValue::Empty)
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Err(FhirPathError::TypeError {
                    message: format!("dateOf() requires a single item, found {}", items.len()),
                })
            }
            FhirPathValue::Date(date_str) => Ok(FhirPathValue::Date(date_str.clone())),
            FhirPathValue::DateTime(dt_str) => Ok(FhirPathValue::Date(
                dt_str.split('T').next().unwrap_or(dt_str).to_string(),
            )),
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// timeOf(datetime) - Extract time component from DateTime  
    /// Returns the time part as a Time value
    pub fn time_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() == 1 =>
            {
                Self::time_of(&items[0])
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty() =>
            {
                Ok(FhirPathValue::Empty)
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Err(FhirPathError::TypeError {
                    message: format!("timeOf() requires a single item, found {}", items.len()),
                })
            }
            FhirPathValue::DateTime(dt_str) => {
                if let Some(time) = dt_str.split_once('T').map(|(_, time)| time) {
                    Ok(FhirPathValue::Time(format!("T{}", strip_timezone(time))))
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            _ => Ok(FhirPathValue::Empty),
        }
    }
}

fn strip_timezone(time: &str) -> &str {
    if let Some(stripped) = time.strip_suffix('Z') {
        return stripped;
    }
    let plus = time.rfind('+');
    let minus = time.rfind('-');
    match (plus, minus) {
        (Some(pos), _) | (_, Some(pos)) => &time[..pos],
        _ => time,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

    #[test]
    fn test_now() {
        let result = DateTimeEvaluator::now().unwrap();
        match result {
            FhirPathValue::DateTime(dt_str) => {
                assert!(
                    dt_str.len() >= 20,
                    "DateTime should be at least 20 characters"
                );

                // Try to parse it back to verify RFC3339 format with a timezone.
                let parsed = DateTime::parse_from_rfc3339(&dt_str);
                assert!(parsed.is_ok(), "DateTime should be valid RFC3339: {dt_str}");
            }
            _ => panic!("now() should return a DateTime value"),
        }
    }

    #[test]
    fn test_today() {
        let result = DateTimeEvaluator::today().unwrap();
        match result {
            FhirPathValue::Date(date_str) => {
                // Verify it's a valid ISO 8601 date
                assert_eq!(
                    date_str.len(),
                    10,
                    "Date should be 10 characters (YYYY-MM-DD)"
                );

                // Try to parse it back to verify format
                let parsed = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");
                assert!(
                    parsed.is_ok(),
                    "Date should be valid YYYY-MM-DD: {date_str}"
                );
            }
            _ => panic!("today() should return a Date value"),
        }
    }

    #[test]
    fn test_time_of_day() {
        let result = DateTimeEvaluator::time_of_day().unwrap();
        match result {
            FhirPathValue::Time(time_str) => {
                // Verify it's a valid ISO 8601 time
                assert!(
                    time_str.len() >= 8,
                    "Time should be at least 8 characters (HH:MM:SS)"
                );

                // Try to parse it back to verify format
                let parsed = NaiveTime::parse_from_str(&time_str, "%H:%M:%S%.3f");
                assert!(
                    parsed.is_ok(),
                    "Time should be valid HH:MM:SS.sss: {time_str}"
                );
            }
            _ => panic!("timeOfDay() should return a Time value"),
        }
    }

    #[test]
    fn test_values_are_current() {
        // Test that the functions return current values (within reasonable bounds)
        let now_result = DateTimeEvaluator::now().unwrap();
        let today_result = DateTimeEvaluator::today().unwrap();
        let time_result = DateTimeEvaluator::time_of_day().unwrap();

        // Verify we got the expected types
        assert!(matches!(now_result, FhirPathValue::DateTime(_)));
        assert!(matches!(today_result, FhirPathValue::Date(_)));
        assert!(matches!(time_result, FhirPathValue::Time(_)));

        // Extract the actual time from now() to compare with system time
        if let FhirPathValue::DateTime(dt_str) = now_result {
            let parsed = DateTime::parse_from_rfc3339(&dt_str).unwrap();
            let system_now = Utc::now();

            // Convert both to UTC for comparison
            let parsed_utc = parsed.with_timezone(&Utc);

            // Should be within 1 second of each other
            let diff = (system_now - parsed_utc).num_seconds().abs();
            assert!(
                diff <= 1,
                "now() should return current time within 1 second, diff: {diff} seconds"
            );
        }
    }

    #[test]
    fn test_now_and_today_use_same_calendar_basis() {
        let now = DateTimeEvaluator::now().unwrap();
        let today = DateTimeEvaluator::today().unwrap();
        match (now, today) {
            (FhirPathValue::DateTime(dt), FhirPathValue::Date(date)) => {
                assert_eq!(&dt[..10], date);
            }
            _ => panic!("now() and today() should return DateTime and Date values"),
        }
    }

    #[test]
    fn test_year_of() {
        // Test with Date value
        let date_value = FhirPathValue::Date("2023-07-15".to_string());
        let result = DateTimeEvaluator::year_of(&date_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(2023));

        // Test with DateTime value
        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::year_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(2023));

        // Test with timezone offset
        let datetime_tz = FhirPathValue::DateTime("2023-07-15T14:30:25.123-05:00".to_string());
        let result = DateTimeEvaluator::year_of(&datetime_tz).unwrap();
        assert_eq!(result, FhirPathValue::Integer(2023));
    }

    #[test]
    fn test_month_of() {
        let date_value = FhirPathValue::Date("2023-07-15".to_string());
        let result = DateTimeEvaluator::month_of(&date_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(7));

        let datetime_value = FhirPathValue::DateTime("2023-12-25T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::month_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(12));
    }

    #[test]
    fn test_day_of() {
        let date_value = FhirPathValue::Date("2023-07-15".to_string());
        let result = DateTimeEvaluator::day_of(&date_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(15));

        let datetime_value = FhirPathValue::DateTime("2023-12-01T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::day_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(1));
    }

    #[test]
    fn test_hour_of() {
        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::hour_of(&time_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(14));

        let datetime_value = FhirPathValue::DateTime("2023-07-15T09:30:25.123Z".to_string());
        let result = DateTimeEvaluator::hour_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(9));
    }

    #[test]
    fn test_minute_of() {
        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::minute_of(&time_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(30));

        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:45:25.123Z".to_string());
        let result = DateTimeEvaluator::minute_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(45));
    }

    #[test]
    fn test_second_of() {
        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::second_of(&time_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(25));

        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:30:59.123Z".to_string());
        let result = DateTimeEvaluator::second_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(59));
    }

    #[test]
    fn test_millisecond_of() {
        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::millisecond_of(&time_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(123));

        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:30:25.999Z".to_string());
        let result = DateTimeEvaluator::millisecond_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Integer(999));

        // Test with no milliseconds
        let time_no_ms = FhirPathValue::Time("14:30:25.000".to_string());
        let result = DateTimeEvaluator::millisecond_of(&time_no_ms).unwrap();
        assert_eq!(result, FhirPathValue::Integer(0));
    }

    #[test]
    fn test_timezone_offset_of() {
        // UTC timezone
        let datetime_utc = FhirPathValue::DateTime("2023-07-15T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&datetime_utc).unwrap();
        assert_eq!(result, FhirPathValue::Number(Decimal::ZERO));

        // EST timezone (-5 hours)
        let datetime_est = FhirPathValue::DateTime("2023-07-15T14:30:25.123-05:00".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&datetime_est).unwrap();
        assert_eq!(result, FhirPathValue::Number(Decimal::from(-5)));

        // CEST timezone (+2 hours)
        let datetime_cest = FhirPathValue::DateTime("2023-07-15T14:30:25.123+02:00".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&datetime_cest).unwrap();
        assert_eq!(result, FhirPathValue::Number(Decimal::from(2)));
    }

    #[test]
    fn test_date_of() {
        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::date_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Date("2023-07-15".to_string()));

        // With timezone
        let datetime_tz = FhirPathValue::DateTime("2023-12-25T23:59:59.999-05:00".to_string());
        let result = DateTimeEvaluator::date_of(&datetime_tz).unwrap();
        assert_eq!(result, FhirPathValue::Date("2023-12-25".to_string()));
    }

    #[test]
    fn test_time_of() {
        let datetime_value = FhirPathValue::DateTime("2023-07-15T14:30:25.123Z".to_string());
        let result = DateTimeEvaluator::time_of(&datetime_value).unwrap();
        assert_eq!(result, FhirPathValue::Time("T14:30:25.123".to_string()));

        // With timezone
        let datetime_tz = FhirPathValue::DateTime("2023-12-25T09:15:30.500+02:00".to_string());
        let result = DateTimeEvaluator::time_of(&datetime_tz).unwrap();
        assert_eq!(result, FhirPathValue::Time("T09:15:30.500".to_string()));
    }

    #[test]
    fn test_component_extraction_errors() {
        let string_value = FhirPathValue::String("not a date".to_string());

        // Non-date/time values return empty per FHIRPath component extraction semantics.
        let result = DateTimeEvaluator::year_of(&string_value);
        assert_eq!(result.unwrap(), FhirPathValue::Empty);

        let result = DateTimeEvaluator::hour_of(&string_value);
        assert_eq!(result.unwrap(), FhirPathValue::Empty);

        let date_value = FhirPathValue::Date("2023-07-15".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&date_value);
        assert_eq!(result.unwrap(), FhirPathValue::Empty);

        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::date_of(&time_value);
        assert_eq!(result.unwrap(), FhirPathValue::Empty);
    }
}
