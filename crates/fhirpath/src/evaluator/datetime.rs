//! Date and time functions for FHIRPath expressions
//!
//! This module implements FHIRPath date/time functions like now(), today(), and timeOfDay().
//! These functions return current system date/time values and don't require a target value.

use crate::error::*;
use crate::evaluator::values::FhirPathValue;
use chrono::{Local, Utc};

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
                assert!(dt_str.len() >= 20, "DateTime should be at least 20 characters");
                
                // Try to parse it back to verify format
                let parsed = DateTime::parse_from_rfc3339(&dt_str);
                assert!(parsed.is_ok(), "DateTime should be valid RFC3339: {}", dt_str);
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
                assert_eq!(date_str.len(), 10, "Date should be 10 characters (YYYY-MM-DD)");
                
                // Try to parse it back to verify format
                let parsed = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");
                assert!(parsed.is_ok(), "Date should be valid YYYY-MM-DD: {}", date_str);
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
                assert!(time_str.len() >= 8, "Time should be at least 8 characters (HH:MM:SS)");
                
                // Try to parse it back to verify format  
                let parsed = NaiveTime::parse_from_str(&time_str, "%H:%M:%S%.3f");
                assert!(parsed.is_ok(), "Time should be valid HH:MM:SS.sss: {}", time_str);
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
            assert!(diff <= 1, "now() should return current time within 1 second, diff: {} seconds", diff);
        }
    }
}
