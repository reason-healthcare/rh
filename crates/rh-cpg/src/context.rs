use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Datelike, FixedOffset, Timelike};
use rh_cql::eval::{CqlDateTime, Value as CqlValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{CpgError, CpgResult};
use crate::resolver::ContentResolver;

/// A fixed evaluation period shared by CQL evaluation and MeasureReport output.
///
/// The bounds retain their original FHIR date or dateTime representation in the
/// resulting MeasureReport. CQL receives the same values as an interval under
/// the standard `Measurement Period` parameter name.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementPeriod {
    pub start: String,
    pub end: String,
    #[serde(default = "default_true")]
    pub start_inclusive: bool,
    #[serde(default = "default_true")]
    pub end_inclusive: bool,
}

impl MeasurementPeriod {
    /// Convert this FHIR-facing period to the CQL interval used by a library.
    pub fn as_cql_interval(&self) -> CpgResult<CqlValue> {
        Ok(CqlValue::Interval {
            low: Some(Box::new(CqlValue::DateTime(parse_cql_datetime(
                &self.start,
            )?))),
            high: Some(Box::new(CqlValue::DateTime(parse_cql_datetime(&self.end)?))),
            low_closed: self.start_inclusive,
            high_closed: self.end_inclusive,
        })
    }

    fn validated_bounds(&self) -> CpgResult<(DateTime<FixedOffset>, DateTime<FixedOffset>)> {
        let start = parse_fixed_offset_datetime(&self.start)?;
        let end = parse_fixed_offset_datetime(&self.end)?;
        if start > end {
            return Err(CpgError::EvaluationError(format!(
                "measurement period start '{}' is after end '{}'",
                self.start, self.end
            )));
        }
        Ok((start, end))
    }
}

fn default_true() -> bool {
    true
}

pub struct ApplyContext {
    pub content_resolver: Arc<dyn ContentResolver>,
    /// Subject reference, e.g. "Patient/123".
    pub subject: String,
    pub encounter: Option<String>,
    pub practitioner: Option<String>,
    pub organization: Option<String>,
    /// FHIR Bundle of patient data (may be a collection bundle).
    pub data: Option<Value>,
    /// Optional deterministic clock for CQL `Now()` and related operations.
    /// RFC 3339 date-time and FHIR date values are accepted.
    pub evaluation_date: Option<String>,
    /// Optional period supplied to CQL as `Measurement Period` and emitted by
    /// individual MeasureReports.
    pub measurement_period: Option<MeasurementPeriod>,
    /// Additional CQL parameters keyed by their declared CQL names.
    pub parameters: HashMap<String, Value>,
}

impl ApplyContext {
    pub fn new(content_resolver: Arc<dyn ContentResolver>, subject: impl Into<String>) -> Self {
        Self {
            content_resolver,
            subject: subject.into(),
            encounter: None,
            practitioner: None,
            organization: None,
            data: None,
            evaluation_date: None,
            measurement_period: None,
            parameters: HashMap::new(),
        }
    }

    /// Return the deterministic CQL clock for this evaluation.
    pub fn cql_evaluation_date(&self) -> CpgResult<CqlDateTime> {
        self.evaluation_date
            .as_deref()
            .map(parse_cql_datetime)
            .transpose()?
            .map_or_else(|| Ok(default_cql_evaluation_date()), Ok)
    }

    /// Validate supplied temporal context before an operation emits output.
    pub fn validate_execution_context(&self) -> CpgResult<()> {
        if let Some(evaluation_date) = &self.evaluation_date {
            parse_cql_datetime(evaluation_date)?;
        }
        if let Some(measurement_period) = &self.measurement_period {
            measurement_period.validated_bounds()?;
        }
        Ok(())
    }

    pub fn data_resources(&self) -> Vec<Value> {
        self.data
            .as_ref()
            .map(|bundle| {
                bundle
                    .get("entry")
                    .and_then(Value::as_array)
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(|entry| entry.get("resource"))
                            .filter(|resource| resource.is_object())
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn resolve_context_resource(&self, reference: &str) -> Option<Value> {
        let canonical_reference = canonical_reference(reference);
        self.data_resources()
            .into_iter()
            .find(|resource| {
                matches_context_reference(
                    &canonical_reference,
                    resource.get("resourceType").and_then(Value::as_str),
                    resource.get("id").and_then(Value::as_str),
                )
            })
            .or_else(|| {
                self.content_resolver
                    .resolve_reference(reference)
                    .ok()
                    .flatten()
            })
    }
}

fn default_cql_evaluation_date() -> CqlDateTime {
    CqlDateTime {
        year: 2026,
        month: Some(1),
        day: Some(1),
        hour: Some(0),
        minute: Some(0),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0),
    }
}

/// Parse an RFC 3339 date-time or FHIR date into a CQL clock value.
pub fn parse_cql_datetime(value: &str) -> CpgResult<CqlDateTime> {
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(cql_datetime_from_rfc3339(datetime));
    }

    let date = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|error| {
        CpgError::EvaluationError(format!(
            "expected an RFC 3339 date-time or FHIR date, got '{value}': {error}"
        ))
    })?;
    Ok(CqlDateTime {
        year: date.year(),
        month: Some(date.month() as u8),
        day: Some(date.day() as u8),
        hour: None,
        minute: None,
        second: None,
        millisecond: None,
        offset_seconds: None,
    })
}

fn parse_fixed_offset_datetime(value: &str) -> CpgResult<DateTime<FixedOffset>> {
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime);
    }

    let date = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|error| {
        CpgError::EvaluationError(format!(
            "expected an RFC 3339 date-time or FHIR date, got '{value}': {error}"
        ))
    })?;
    date.and_hms_opt(0, 0, 0)
        .and_then(|datetime| {
            FixedOffset::east_opt(0).map(|offset| datetime.and_local_timezone(offset))
        })
        .and_then(chrono::LocalResult::single)
        .ok_or_else(|| CpgError::EvaluationError(format!("could not normalize date '{value}'")))
}

fn cql_datetime_from_rfc3339(datetime: DateTime<FixedOffset>) -> CqlDateTime {
    CqlDateTime {
        year: datetime.year(),
        month: Some(datetime.month() as u8),
        day: Some(datetime.day() as u8),
        hour: Some(datetime.hour() as u8),
        minute: Some(datetime.minute() as u8),
        second: Some(datetime.second() as u8),
        millisecond: Some(datetime.timestamp_subsec_millis()),
        offset_seconds: Some(datetime.offset().local_minus_utc()),
    }
}

fn canonical_reference(reference: &str) -> String {
    reference
        .split(['?', '#'])
        .next()
        .unwrap_or(reference)
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/")
}

fn matches_context_reference(
    reference: &str,
    resource_type: Option<&str>,
    resource_id: Option<&str>,
) -> bool {
    let Some((resource_type, resource_id)) = resource_type.zip(resource_id) else {
        return false;
    };

    reference == format!("{resource_type}/{resource_id}")
}
