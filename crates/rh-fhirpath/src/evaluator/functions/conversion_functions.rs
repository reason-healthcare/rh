//! Conversion function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::temporal::parse_temporal;
use crate::evaluator::operations::units::UnitConverter;
use crate::evaluator::types::FhirPathValue;
use regex::Regex;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all conversion functions
pub fn register_conversion_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    type Conversion = fn(&FhirPathValue) -> FhirPathResult<FhirPathValue>;
    let unary: [(&str, Conversion); 12] = [
        ("toBoolean", to_boolean),
        ("convertsToBoolean", converts_to_boolean),
        ("toInteger", to_integer),
        ("convertsToInteger", converts_to_integer),
        ("toLong", to_long),
        ("convertsToLong", converts_to_long),
        ("toString", to_string),
        ("convertsToString", converts_to_string),
        ("toTime", to_time),
        ("convertsToTime", converts_to_time),
        ("toDecimal", to_decimal),
        ("convertsToDecimal", converts_to_decimal),
    ];
    for (name, convert) in unary {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: format!("{name}() takes no parameters"),
                    });
                }
                convert(target)
            }),
        );
    }

    for (name, datetime, predicate) in [
        ("toDate", false, false),
        ("convertsToDate", false, true),
        ("toDateTime", true, false),
        ("convertsToDateTime", true, true),
    ] {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                temporal_conversion(target, params, datetime, predicate)
            }),
        );
    }

    functions.insert(
        "toQuantity".to_owned(),
        Box::new(|target, params| {
            to_quantity(target, optional_string_param(params, "toQuantity()")?)
        }),
    );
    functions.insert(
        "convertsToQuantity".to_owned(),
        Box::new(|target, params| {
            converts_to_quantity(
                target,
                optional_string_param(params, "convertsToQuantity()")?,
            )
        }),
    );
}

fn to_decimal(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    let Some(value) = value
        .singleton("toDecimal()")?
        .map(FhirPathValue::system_value)
    else {
        return Ok(FhirPathValue::Empty);
    };
    let converted = match value {
        FhirPathValue::Number(value) => Some(*value),
        FhirPathValue::Integer(value) | FhirPathValue::Long(value) => Some(Decimal::from(*value)),
        FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => {
            Some(Decimal::from(i64::from(*value)))
        }
        FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. } => {
            if let Some(value) = value.strip_suffix('L') {
                value.parse::<i64>().ok().map(Decimal::from)
            } else {
                parse_decimal_string(value)
            }
        }
        _ => None,
    };
    Ok(converted.map_or(FhirPathValue::Empty, FhirPathValue::Number))
}

fn parse_decimal_string(value: &str) -> Option<Decimal> {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    let valid_digits = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
    let valid = digits.split_once('.').map_or_else(
        || valid_digits(digits),
        |(whole, fraction)| valid_digits(whole) && valid_digits(fraction),
    );
    valid.then(|| Decimal::from_str_exact(value).ok()).flatten()
}

fn converts_to_decimal(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToDecimal()", to_decimal)
}

fn to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    let converted = match value
        .singleton("toBoolean()")?
        .map(FhirPathValue::system_value)
    {
        Some(FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. }) => {
            Some(*value)
        }
        Some(FhirPathValue::Integer(1) | FhirPathValue::Long(1)) => Some(true),
        Some(FhirPathValue::Integer(0) | FhirPathValue::Long(0)) => Some(false),
        Some(FhirPathValue::Number(value)) if *value == Decimal::ONE => Some(true),
        Some(FhirPathValue::Number(value)) if value.is_zero() => Some(false),
        Some(FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. }) => {
            match value.to_ascii_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "1" | "1.0" => Some(true),
                "false" | "f" | "no" | "n" | "0" | "0.0" => Some(false),
                _ => None,
            }
        }
        _ => None,
    };
    Ok(converted.map_or(FhirPathValue::Empty, FhirPathValue::Boolean))
}

fn converts_to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToBoolean()", to_boolean)
}

fn to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    Ok(integral_value(value, "toInteger()")?
        .and_then(|value| i32::try_from(value).ok())
        .map_or(FhirPathValue::Empty, |value| {
            FhirPathValue::Integer(i64::from(value))
        }))
}

fn converts_to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToInteger()", to_integer)
}

fn to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    Ok(integral_value(value, "toLong()")?.map_or(FhirPathValue::Empty, FhirPathValue::Long))
}

fn converts_to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToLong()", to_long)
}

fn integral_value(value: &FhirPathValue, name: &str) -> FhirPathResult<Option<i64>> {
    Ok(
        match value.singleton(name)?.map(FhirPathValue::system_value) {
            Some(FhirPathValue::Integer(value) | FhirPathValue::Long(value)) => Some(*value),
            Some(FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. }) => {
                Some(i64::from(*value))
            }
            Some(FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. }) => {
                value.parse::<i64>().ok()
            }
            _ => None,
        },
    )
}

fn can_convert(
    value: &FhirPathValue,
    name: &str,
    convert: impl FnOnce(&FhirPathValue) -> FhirPathResult<FhirPathValue>,
) -> FhirPathResult<FhirPathValue> {
    let Some(value) = value.singleton(name)?.map(FhirPathValue::system_value) else {
        return Ok(FhirPathValue::Empty);
    };
    if matches!(value, FhirPathValue::Empty) {
        return Ok(FhirPathValue::Empty);
    }
    Ok(FhirPathValue::Boolean(!matches!(
        convert(value)?,
        FhirPathValue::Empty
    )))
}

fn temporal_conversion(
    value: &FhirPathValue,
    params: &[FhirPathValue],
    datetime: bool,
    predicate: bool,
) -> FhirPathResult<FhirPathValue> {
    let name = match (datetime, predicate) {
        (true, true) => "convertsToDateTime()",
        (true, false) => "toDateTime()",
        (false, true) => "convertsToDate()",
        (false, false) => "toDate()",
    };
    let format = optional_string_param(params, name)?;
    let convert = |value: &FhirPathValue| convert_temporal(value, format, datetime);
    if predicate {
        return can_convert(value, name, convert);
    }
    match value.singleton(name)?.map(FhirPathValue::system_value) {
        Some(value) => convert(value),
        None => Ok(FhirPathValue::Empty),
    }
}

fn convert_temporal(
    value: &FhirPathValue,
    format: Option<&str>,
    datetime: bool,
) -> FhirPathResult<FhirPathValue> {
    let text = match value {
        FhirPathValue::Date(text)
        | FhirPathValue::DateTime(text)
        | FhirPathValue::TypedDateTime { value: text, .. } => text.clone(),
        FhirPathValue::String(text) | FhirPathValue::TypedString { value: text, .. } => {
            if let Some(format) = format {
                let Some(result) = parse_formatted_temporal(text, format) else {
                    return Ok(FhirPathValue::Empty);
                };
                result
            } else if parse_temporal(text).is_some_and(|parts| parts.year.is_some()) {
                text.clone()
            } else {
                return Ok(FhirPathValue::Empty);
            }
        }
        _ => return Ok(FhirPathValue::Empty),
    };
    if datetime {
        Ok(FhirPathValue::DateTime(text))
    } else {
        let date = text.split_once('T').map_or(text.as_str(), |(date, _)| date);
        Ok(FhirPathValue::Date(date.to_owned()))
    }
}

fn to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    let Some(value) = value
        .singleton("toString()")?
        .map(FhirPathValue::system_value)
    else {
        return Ok(FhirPathValue::Empty);
    };
    Ok(FhirPathValue::String(match value {
        FhirPathValue::String(value)
        | FhirPathValue::TypedString { value, .. }
        | FhirPathValue::Date(value) => value.clone(),
        FhirPathValue::DateTime(value) | FhirPathValue::TypedDateTime { value, .. } => {
            value.strip_suffix('T').unwrap_or(value).to_owned()
        }
        FhirPathValue::Time(value) => value.strip_prefix('T').unwrap_or(value).to_owned(),
        FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => {
            value.to_string()
        }
        FhirPathValue::Integer(value) | FhirPathValue::Long(value) => value.to_string(),
        FhirPathValue::Number(value) => {
            if value.scale() == 0 {
                format!("{value}.0")
            } else {
                value.to_string()
            }
        }
        FhirPathValue::Quantity { value, unit } => match unit.as_deref() {
            Some(unit) if is_calendar_duration_unit(unit) => format!("{value} {unit}"),
            Some(unit) => format!("{value} '{unit}'"),
            None => value.to_string(),
        },
        _ => return Ok(FhirPathValue::Empty),
    }))
}

fn converts_to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToString()", to_string)
}

fn to_time(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    let Some(value) = value
        .singleton("toTime()")?
        .map(FhirPathValue::system_value)
    else {
        return Ok(FhirPathValue::Empty);
    };
    Ok(match value {
        FhirPathValue::Time(value) => FhirPathValue::Time(value.clone()),
        FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. }
            if is_valid_time_string(value) =>
        {
            FhirPathValue::Time(value.strip_prefix('T').unwrap_or(value).to_owned())
        }
        FhirPathValue::DateTime(value) | FhirPathValue::TypedDateTime { value, .. } => {
            extract_time_from_datetime(value).map_or(FhirPathValue::Empty, FhirPathValue::Time)
        }
        _ => FhirPathValue::Empty,
    })
}

fn converts_to_time(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToTime()", to_time)
}

fn to_quantity(value: &FhirPathValue, target: Option<&str>) -> FhirPathResult<FhirPathValue> {
    let Some(value) = value
        .singleton("toQuantity()")?
        .map(FhirPathValue::system_value)
    else {
        return Ok(FhirPathValue::Empty);
    };
    let parsed = match value {
        FhirPathValue::Quantity { value, unit } => Some((*value, unit.clone())),
        FhirPathValue::Integer(value) | FhirPathValue::Long(value) => {
            Some((Decimal::from(*value), None))
        }
        FhirPathValue::Number(value) => Some((*value, None)),
        FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => {
            Some((Decimal::from(i64::from(*value)), None))
        }
        FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. } => {
            parse_quantity_string(value)
        }
        _ => None,
    };
    let Some((value, unit)) = parsed else {
        return Ok(FhirPathValue::Empty);
    };
    if let Some(target) = target {
        UnitConverter::new().convert_quantity(value, &unit, &Some(target.to_owned()))
    } else {
        Ok(FhirPathValue::Quantity {
            value,
            unit: Some(unit.unwrap_or_else(|| "1".to_string())),
        })
    }
}

fn converts_to_quantity(
    value: &FhirPathValue,
    unit: Option<&str>,
) -> FhirPathResult<FhirPathValue> {
    can_convert(value, "convertsToQuantity()", |value| {
        to_quantity(value, unit)
    })
}

/// Parse a partial time in the ISO format used by FHIRPath.
fn is_valid_time_string(value: &str) -> bool {
    let raw = value.strip_prefix('T').unwrap_or(value);
    parse_temporal(&format!("T{raw}")).is_some_and(|parts| {
        parts.hour.is_some() && parts.tz_offset_min.is_none() && parts.second.is_none_or(|s| s < 60)
    })
}

fn optional_string_param<'a>(
    params: &'a [FhirPathValue],
    name: &str,
) -> FhirPathResult<Option<&'a str>> {
    let parameter_error = || FhirPathError::TypeError {
        message: format!("{name} accepts at most one optional String argument"),
    };
    match params {
        [] => Ok(None),
        [value] => match value.singleton(name)?.map(FhirPathValue::system_value) {
            None | Some(FhirPathValue::Empty) => Ok(None),
            Some(FhirPathValue::String(value) | FhirPathValue::TypedString { value, .. }) => {
                Ok(Some(value))
            }
            _ => Err(parameter_error()),
        },
        _ => Err(parameter_error()),
    }
}

fn parse_formatted_temporal(text: &str, mut format: &str) -> Option<String> {
    // Longest matching token wins. Matches are case-sensitive except for the
    // English month and meridiem values accepted by those individual tokens.
    let tokens = [
        ("yyyy", "year", "[0-9]{4}"), ("yy", "short_year", "[0-9]{2}"),
        ("MMMM", "month_long", "(?i:january|february|march|april|may|june|july|august|september|october|november|december)"),
        ("MMM", "month_short", "(?i:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)"),
        ("MM", "month", "[0-9]{2}"), ("M", "month", "[0-9]{1,2}"),
        ("dd", "day", "[0-9]{2}"), ("d", "day", "[0-9]{1,2}"),
        ("HH", "hour", "[0-9]{2}"), ("H", "hour", "[0-9]{1,2}"),
        ("hh", "hour12", "[0-9]{2}"), ("h", "hour12", "[0-9]{1,2}"),
        ("mm", "minute", "[0-9]{2}"), ("m", "minute", "[0-9]{1,2}"),
        ("ss", "second", "[0-9]{2}"), ("s", "second", "[0-9]{1,2}"),
        ("a", "meridiem", "(?i:[ap]m?)"),
        ("Z", "zone", "Z|[+-][0-9]{2}:?[0-9]{2}"),
    ];
    let mut pattern = r"\A".to_string();
    while !format.is_empty() {
        if format.starts_with('S') {
            let length = format.bytes().take_while(|b| *b == b'S').count();
            if length > 9 {
                return None;
            }
            pattern.push_str(&format!("(?P<fraction>[0-9]{{{length}}})"));
            format = &format[length..];
        } else if let Some((token, field, expression)) = tokens
            .iter()
            .find(|(token, _, _)| format.starts_with(token))
        {
            pattern.push_str(&format!("(?P<{field}>{expression})"));
            format = &format[token.len()..];
        } else {
            let literal = format.chars().next()?;
            pattern.push_str(&regex::escape(&literal.to_string()));
            format = &format[literal.len_utf8()..];
        }
    }
    pattern.push_str(r"\z");
    let regex = Regex::new(&pattern).ok()?;
    let captures = regex.captures(text)?;
    let field = |name| captures.name(name).map(|capture| capture.as_str());
    let number = |name| field(name).and_then(|value| value.parse::<u32>().ok());
    let year = number("year")
        .or_else(|| number("short_year").map(|year| year + if year < 50 { 2000 } else { 1900 }))?;
    let mut date_time = format!("{year:04}");
    let month = number("month").or_else(|| {
        let month_name = field("month_short").or_else(|| field("month_long"))?;
        [
            "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
        ]
        .iter()
        .position(|month| month_name[..3].eq_ignore_ascii_case(month))
        .map(|index| index as u32 + 1)
    });
    if let Some(month) = month {
        date_time.push_str(&format!("-{month:02}"));
    }
    if let Some(day) = number("day") {
        month?;
        date_time.push_str(&format!("-{day:02}"));
    }
    let mut hour = number("hour");
    if let Some(hour12) = number("hour12") {
        if hour.is_some() {
            return None;
        }
        hour = Some(if let Some(meridiem) = field("meridiem") {
            if !(1..=12).contains(&hour12) {
                return None;
            }
            hour12 % 12
                + if meridiem.starts_with(['P', 'p']) {
                    12
                } else {
                    0
                }
        } else {
            // Historically accepted hh:mm as 24-hour input when no meridiem is supplied.
            hour12
        });
    } else if field("meridiem").is_some() {
        return None;
    }
    if let Some(hour) = hour {
        date_time.push_str(&format!("T{hour:02}"));
    }
    if let Some(minute) = number("minute") {
        hour?;
        date_time.push_str(&format!(":{minute:02}"));
    }
    if let Some(second) = number("second") {
        number("minute")?;
        date_time.push_str(&format!(":{second:02}"));
    }
    if let Some(fraction) = field("fraction") {
        number("second")?;
        date_time.push('.');
        date_time.push_str(fraction);
    }
    if let Some(zone) = field("zone") {
        hour?;
        if zone.len() == 5 {
            date_time.push_str(&format!("{}:{}", &zone[..3], &zone[3..]));
        } else {
            date_time.push_str(zone);
        }
    }
    parse_temporal(&date_time)?;
    Some(date_time)
}

/// Extract time part from datetime string
fn extract_time_from_datetime(dt: &str) -> Option<String> {
    // Look for 'T' separator in datetime
    if let Some(t_pos) = dt.find('T') {
        let time_part = &dt[t_pos + 1..];

        // Remove timezone information if present
        let time_clean = if let Some(tz_pos) = time_part.find('+').or_else(|| time_part.find('-')) {
            &time_part[..tz_pos]
        } else if let Some(z_pos) = time_part.find('Z') {
            &time_part[..z_pos]
        } else {
            time_part
        };

        // Validate the extracted time
        if is_valid_time_string(time_clean) {
            Some(time_clean.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Parse a string that might represent a quantity (e.g., "5 'mg'", "10.5'kg'")
/// Returns (value, unit) if successful
fn parse_quantity_string(text: &str) -> Option<(Decimal, Option<String>)> {
    let text = text.trim();
    if let Some((value, quoted)) = text.split_once('\'') {
        let unit = quoted.strip_suffix('\'')?;
        if unit.is_empty() || unit.contains('\'') {
            return None;
        }
        return Some((
            parse_decimal_string(value.trim_end())?,
            Some(unit.to_owned()),
        ));
    }
    // Calendar names can follow the number with or without whitespace.
    if let Some(index) = text.find(|ch: char| ch.is_ascii_alphabetic()) {
        let (value, unit) = text.split_at(index);
        return is_calendar_duration_unit(unit)
            .then(|| {
                Some((
                    parse_decimal_string(value.trim_end())?,
                    Some(unit.to_owned()),
                ))
            })
            .flatten();
    }
    Some((parse_decimal_string(text)?, None))
}

/// FHIRPath calendar-duration vocabulary (used by Quantity.toString to decide
/// whether the unit gets quoted). Both singular and plural forms accepted.
fn is_calendar_duration_unit(u: &str) -> bool {
    matches!(
        u,
        "year"
            | "years"
            | "month"
            | "months"
            | "week"
            | "weeks"
            | "day"
            | "days"
            | "hour"
            | "hours"
            | "minute"
            | "minutes"
            | "second"
            | "seconds"
            | "millisecond"
            | "milliseconds"
    )
}
