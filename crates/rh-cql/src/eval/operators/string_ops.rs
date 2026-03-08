//! CQL string operators (CQL §15).
//!
//! Exports: [`concatenate`], [`combine`], [`split`], [`split_on_matches`],
//! [`length_str`], [`upper`], [`lower`], [`starts_with`], [`ends_with`],
//! [`matches_regex`], [`replace_matches`], [`indexer_str`], [`position_of`],
//! [`last_position_of`], [`substring`].

use regex::Regex;

use super::super::context::EvalError;
use super::super::value::Value;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

macro_rules! null1 {
    ($a:expr) => {
        if matches!($a, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

macro_rules! null2 {
    ($a:expr, $b:expr) => {
        if matches!($a, Value::Null) || matches!($b, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

// ---------------------------------------------------------------------------
// String operators (9.12)
// ---------------------------------------------------------------------------

/// Concatenate two strings.
///
/// Returns `Null` if either argument is `Null`.
pub fn concatenate(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{s1}{s2}"))),
        _ => Err(err("Concatenate", "expected String arguments")),
    }
}

/// Combine a list of strings with an optional separator.
///
/// Null items in the list are skipped.  If `separator` is `None` or `Null`
/// an empty string is used.  Returns `Null` if `list` is `Null`.
pub fn combine(list: &Value, separator: Option<&Value>) -> Result<Value, EvalError> {
    null1!(list);
    let sep: String = match separator {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(err("Combine", "separator must be a String")),
    };
    match list {
        Value::List(items) => {
            let mut parts: Vec<&str> = Vec::new();
            for item in items {
                match item {
                    Value::Null => {}
                    Value::String(s) => parts.push(s.as_str()),
                    _ => return Err(err("Combine", "list elements must be String")),
                }
            }
            Ok(Value::String(parts.join(&sep)))
        }
        _ => Err(err("Combine", "expected List argument")),
    }
}

/// Split `s` on every occurrence of `separator`, returning a `List<String>`.
///
/// Returns `Null` if either argument is `Null`.
pub fn split(s: &Value, separator: &Value) -> Result<Value, EvalError> {
    null2!(s, separator);
    match (s, separator) {
        (Value::String(str_val), Value::String(sep)) => {
            let parts: Vec<Value> = str_val
                .split(sep.as_str())
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        _ => Err(err("Split", "expected String arguments")),
    }
}

/// Split `s` on every match of the regex `pattern`, returning a `List<String>`.
///
/// Returns `Null` if either argument is `Null`.
pub fn split_on_matches(s: &Value, pattern: &Value) -> Result<Value, EvalError> {
    null2!(s, pattern);
    match (s, pattern) {
        (Value::String(str_val), Value::String(pat)) => {
            let re = Regex::new(pat)
                .map_err(|e| err("SplitOnMatches", &format!("invalid pattern: {e}")))?;
            let parts: Vec<Value> = re
                .split(str_val)
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        _ => Err(err("SplitOnMatches", "expected String arguments")),
    }
}

/// Return the number of characters in `s`.
///
/// Returns `Null` if `s` is `Null`.
pub fn length_str(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::Integer(str_val.chars().count() as i64)),
        _ => Err(err("Length", "expected String")),
    }
}

/// Convert `s` to upper case.
pub fn upper(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::String(str_val.to_uppercase())),
        _ => Err(err("Upper", "expected String")),
    }
}

/// Convert `s` to lower case.
pub fn lower(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::String(str_val.to_lowercase())),
        _ => Err(err("Lower", "expected String")),
    }
}

/// Return `true` if `s` starts with `prefix`.
pub fn starts_with(s: &Value, prefix: &Value) -> Result<Value, EvalError> {
    null2!(s, prefix);
    match (s, prefix) {
        (Value::String(str_val), Value::String(pfx)) => {
            Ok(Value::Boolean(str_val.starts_with(pfx.as_str())))
        }
        _ => Err(err("StartsWith", "expected String arguments")),
    }
}

/// Return `true` if `s` ends with `suffix`.
pub fn ends_with(s: &Value, suffix: &Value) -> Result<Value, EvalError> {
    null2!(s, suffix);
    match (s, suffix) {
        (Value::String(str_val), Value::String(sfx)) => {
            Ok(Value::Boolean(str_val.ends_with(sfx.as_str())))
        }
        _ => Err(err("EndsWith", "expected String arguments")),
    }
}

/// Return `true` if `s` matches the full regex `pattern` (CQL §15.16).
///
/// The pattern is matched against the entire string (anchored).
pub fn matches_regex(s: &Value, pattern: &Value) -> Result<Value, EvalError> {
    null2!(s, pattern);
    match (s, pattern) {
        (Value::String(str_val), Value::String(pat)) => {
            let anchored = format!("^(?:{pat})$");
            let re = Regex::new(&anchored)
                .map_err(|e| err("Matches", &format!("invalid pattern: {e}")))?;
            Ok(Value::Boolean(re.is_match(str_val)))
        }
        _ => Err(err("Matches", "expected String arguments")),
    }
}

/// Replace all regex matches of `pattern` in `s` with `substitution`.
pub fn replace_matches(
    s: &Value,
    pattern: &Value,
    substitution: &Value,
) -> Result<Value, EvalError> {
    null2!(s, pattern);
    null1!(substitution);
    match (s, pattern, substitution) {
        (Value::String(str_val), Value::String(pat), Value::String(sub)) => {
            let re = Regex::new(pat)
                .map_err(|e| err("ReplaceMatches", &format!("invalid pattern: {e}")))?;
            Ok(Value::String(
                re.replace_all(str_val, sub.as_str()).to_string(),
            ))
        }
        _ => Err(err("ReplaceMatches", "expected String arguments")),
    }
}

/// Return the character at 0-based `index` in `s`, or `Null` if out of range.
///
/// CQL `Indexer` (the `[]` operator) uses 0-based indexing per CQL §15.5.
pub fn indexer_str(s: &Value, index: &Value) -> Result<Value, EvalError> {
    null2!(s, index);
    match (s, index) {
        (Value::String(str_val), Value::Integer(idx)) => {
            if *idx < 0 {
                return Ok(Value::Null);
            }
            let ch = str_val.chars().nth(*idx as usize);
            Ok(ch
                .map(|c| Value::String(c.to_string()))
                .unwrap_or(Value::Null))
        }
        _ => Err(err("Indexer", "expected String and Integer arguments")),
    }
}

/// Return the 0-based index of the first occurrence of `pattern` in `s`,
/// or `-1` if `s` does not contain `pattern` (CQL §15.17).
pub fn position_of(pattern: &Value, s: &Value) -> Result<Value, EvalError> {
    null2!(pattern, s);
    match (pattern, s) {
        (Value::String(pat), Value::String(str_val)) => {
            let pos = str_val
                .find(pat.as_str())
                .map(|byte_idx| str_val[..byte_idx].chars().count() as i64)
                .unwrap_or(-1);
            Ok(Value::Integer(pos))
        }
        _ => Err(err("PositionOf", "expected String arguments")),
    }
}

/// Return the 0-based index of the last occurrence of `pattern` in `s`,
/// or `-1` if not found.
pub fn last_position_of(pattern: &Value, s: &Value) -> Result<Value, EvalError> {
    null2!(pattern, s);
    match (pattern, s) {
        (Value::String(pat), Value::String(str_val)) => {
            let pos = str_val
                .rfind(pat.as_str())
                .map(|byte_idx| str_val[..byte_idx].chars().count() as i64)
                .unwrap_or(-1);
            Ok(Value::Integer(pos))
        }
        _ => Err(err("LastPositionOf", "expected String arguments")),
    }
}

/// Return the substring of `s` starting at 0-based `start_index`.
///
/// If `length` is provided, at most `length` characters are returned.
/// Returns `Null` if `start_index` is out of range.  CQL §15.22.
pub fn substring(
    s: &Value,
    start_index: &Value,
    length: Option<&Value>,
) -> Result<Value, EvalError> {
    null2!(s, start_index);
    match (s, start_index) {
        (Value::String(str_val), Value::Integer(start)) => {
            if *start < 0 {
                return Ok(Value::Null);
            }
            let start = *start as usize;
            let chars: Vec<char> = str_val.chars().collect();
            if start >= chars.len() {
                return Ok(Value::Null);
            }
            let result = match length {
                None | Some(Value::Null) => chars[start..].iter().collect(),
                Some(Value::Integer(len)) if *len <= 0 => String::new(),
                Some(Value::Integer(len)) => {
                    let end = (start + *len as usize).min(chars.len());
                    chars[start..end].iter().collect()
                }
                _ => return Err(err("Substring", "length must be an Integer")),
            };
            Ok(Value::String(result))
        }
        _ => Err(err("Substring", "expected String and Integer arguments")),
    }
}
