//! Date and time functions for FHIRPath expressions
//!
//! This module implements FHIRPath date/time functions like now(), today(), and tim                let _time = NaiveTime::parse_from_str(time_part, "%H:%M:%S")?;OfDay().
//! These functions return current system date/time values and don't require a target value.
//! Also includes component extraction functions for Date, DateTime, and Time values.

use crate::error::*;
use crate::evaluator::values::FhirPathValue;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, Timelike, Utc};

/// Helper function to parse datetime strings with timezone handling
fn parse_datetime_str(dt_str: &str) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
    if dt_str.ends_with('Z') || dt_str.contains('+') || dt_str.contains('-') {
        DateTime::parse_from_rfc3339(dt_str)
    } else {
        DateTime::parse_from_rfc3339(&format!("{dt_str}Z"))
    }
}

/// Helper function to parse time strings with optional T prefix
fn parse_time_str(time_str: &str) -> Result<NaiveTime, chrono::ParseError> {
    let time_part = if let Some(stripped) = time_str.strip_prefix('T') {
        stripped
    } else {
        time_str
    };
    // Try parsing with milliseconds first, then fall back to seconds only
    NaiveTime::parse_from_str(time_part, "%H:%M:%S%.3f")
        .or_else(|_| NaiveTime::parse_from_str(time_part, "%H:%M:%S"))
}

/// Date/time function evaluator
pub struct DateTimeEvaluator;

impl DateTimeEvaluator {
    /// now() - Returns the current date and time (with timezone)
    /// Returns a DateTime value in ISO 8601 format
    pub fn now() -> FhirPathResult<FhirPathValue> {
        let now = Utc::now();
        let datetime_string = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
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
        let time_string = now.format("%H:%M:%S%.3f").to_string();
        Ok(FhirPathValue::Time(time_string))
    }

    /// yearOf(date) - Extract year component from Date or DateTime
    /// Returns the year as an Integer value
    pub fn year_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Date(date_str) => {
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                    FhirPathError::TypeError {
                        message: format!("Invalid date format: {date_str}"),
                    }
                })?;
                Ok(FhirPathValue::Integer(date.year() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.year() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("yearOf() requires Date or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// monthOf(date) - Extract month component from Date or DateTime  
    /// Returns the month as an Integer value (1-12)
    pub fn month_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Date(date_str) => {
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                    FhirPathError::TypeError {
                        message: format!("Invalid date format: {date_str}"),
                    }
                })?;
                Ok(FhirPathValue::Integer(date.month() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.month() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("monthOf() requires Date or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// dayOf(date) - Extract day component from Date or DateTime
    /// Returns the day as an Integer value (1-31)
    pub fn day_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Date(date_str) => {
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                    FhirPathError::TypeError {
                        message: format!("Invalid date format: {date_str}"),
                    }
                })?;
                Ok(FhirPathValue::Integer(date.day() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.day() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("dayOf() requires Date or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// hourOf(time) - Extract hour component from Time or DateTime
    /// Returns the hour as an Integer value (0-23)
    pub fn hour_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Time(time_str) => {
                let time = parse_time_str(time_str).map_err(|_| FhirPathError::TypeError {
                    message: format!("Invalid time format: {time_str}"),
                })?;
                Ok(FhirPathValue::Integer(time.hour() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.hour() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("hourOf() requires Time or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// minuteOf(time) - Extract minute component from Time or DateTime
    /// Returns the minute as an Integer value (0-59)
    pub fn minute_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Time(time_str) => {
                // Parse time with T prefix (format: "T14:30:25")
                let time = parse_time_str(time_str).map_err(|_| FhirPathError::TypeError {
                    message: format!("Invalid time format: {time_str}"),
                })?;
                Ok(FhirPathValue::Integer(time.minute() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.minute() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("minuteOf() requires Time or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// secondOf(time) - Extract second component from Time or DateTime
    /// Returns the second as an Integer value (0-59)
    pub fn second_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Time(time_str) => {
                // Parse time with T prefix (format: "T14:30:25")
                let time = parse_time_str(time_str).map_err(|_| FhirPathError::TypeError {
                    message: format!("Invalid time format: {time_str}"),
                })?;
                Ok(FhirPathValue::Integer(time.second() as i64))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(datetime.second() as i64))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("secondOf() requires Time or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// millisecondOf(time) - Extract millisecond component from Time or DateTime
    /// Returns the millisecond as an Integer value (0-999)
    pub fn millisecond_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Time(time_str) => {
                let time = parse_time_str(time_str).map_err(|_| FhirPathError::TypeError {
                    message: format!("Invalid time format: {time_str}"),
                })?;
                // Extract milliseconds from nanoseconds (1 millisecond = 1,000,000 nanoseconds)
                Ok(FhirPathValue::Integer(
                    (time.nanosecond() / 1_000_000) as i64,
                ))
            }
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                Ok(FhirPathValue::Integer(
                    (datetime.nanosecond() / 1_000_000) as i64,
                ))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("millisecondOf() requires Time or DateTime value, got: {value:?}"),
            }),
        }
    }

    /// timezoneOffsetOf(datetime) - Extract timezone offset from DateTime
    /// Returns the timezone offset as a Number in hours (e.g., -5.0 for EST, 0.0 for UTC)
    pub fn timezone_offset_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                let offset_seconds = datetime.offset().local_minus_utc() as f64;
                let offset_hours = offset_seconds / 3600.0;
                Ok(FhirPathValue::Number(offset_hours))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("timezoneOffsetOf() requires DateTime value, got: {value:?}"),
            }),
        }
    }

    /// dateOf(datetime) - Extract date component from DateTime
    /// Returns the date part as a Date value
    pub fn date_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                let date_string = datetime.format("%Y-%m-%d").to_string();
                Ok(FhirPathValue::Date(date_string))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("dateOf() requires DateTime value, got: {value:?}"),
            }),
        }
    }

    /// timeOf(datetime) - Extract time component from DateTime  
    /// Returns the time part as a Time value
    pub fn time_of(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::DateTime(dt_str) => {
                let datetime =
                    parse_datetime_str(dt_str).map_err(|_| FhirPathError::TypeError {
                        message: format!("Invalid datetime format: {dt_str}"),
                    })?;
                let time_string = datetime.format("%H:%M:%S%.3f").to_string();
                Ok(FhirPathValue::Time(time_string))
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("timeOf() requires DateTime value, got: {value:?}"),
            }),
        }
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
                // Verify it's a valid ISO 8601 datetime with UTC timezone
                assert!(dt_str.ends_with('Z'), "DateTime should end with Z for UTC");
                assert!(
                    dt_str.len() >= 20,
                    "DateTime should be at least 20 characters"
                );

                // Try to parse it back to verify format
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
        assert_eq!(result, FhirPathValue::Number(0.0));

        // EST timezone (-5 hours)
        let datetime_est = FhirPathValue::DateTime("2023-07-15T14:30:25.123-05:00".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&datetime_est).unwrap();
        assert_eq!(result, FhirPathValue::Number(-5.0));

        // CEST timezone (+2 hours)
        let datetime_cest = FhirPathValue::DateTime("2023-07-15T14:30:25.123+02:00".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&datetime_cest).unwrap();
        assert_eq!(result, FhirPathValue::Number(2.0));
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
        assert_eq!(result, FhirPathValue::Time("14:30:25.123".to_string()));

        // With timezone
        let datetime_tz = FhirPathValue::DateTime("2023-12-25T09:15:30.500+02:00".to_string());
        let result = DateTimeEvaluator::time_of(&datetime_tz).unwrap();
        assert_eq!(result, FhirPathValue::Time("09:15:30.500".to_string()));
    }

    #[test]
    fn test_component_extraction_errors() {
        let string_value = FhirPathValue::String("not a date".to_string());

        // yearOf should reject non-date/datetime values
        let result = DateTimeEvaluator::year_of(&string_value);
        assert!(result.is_err());

        // hourOf should reject non-time/datetime values
        let result = DateTimeEvaluator::hour_of(&string_value);
        assert!(result.is_err());

        // timezoneOffsetOf should reject non-datetime values
        let date_value = FhirPathValue::Date("2023-07-15".to_string());
        let result = DateTimeEvaluator::timezone_offset_of(&date_value);
        assert!(result.is_err());

        // dateOf should reject non-datetime values
        let time_value = FhirPathValue::Time("14:30:25.123".to_string());
        let result = DateTimeEvaluator::date_of(&time_value);
        assert!(result.is_err());
    }
}
