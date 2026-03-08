//! CQL string operators (CQL §15).
//!
//! Exports: [`concatenate`], [`combine`], [`split`], [`split_on_matches`],
//! [`length_str`], [`upper`], [`lower`], [`starts_with`], [`ends_with`],
//! [`matches_regex`], [`replace_matches`], [`indexer_str`], [`position_of`],
//! [`last_position_of`], [`substring`].

use regex::Regex;

use super::super::context::EvalError;
use super::super::value::Value;
use super::utils::{err, null1, null2};

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::value::Value;

    #[test]
    fn concatenate_two_strings() {
        let a = Value::String("Hello, ".into());
        let b = Value::String("World!".into());
        assert_eq!(
            concatenate(&a, &b).unwrap(),
            Value::String("Hello, World!".into())
        );
    }

    #[test]
    fn concatenate_null_propagates() {
        assert_eq!(
            concatenate(&Value::Null, &Value::String("x".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn combine_list_with_separator() {
        let list = Value::List(vec![
            Value::String("a".into()),
            Value::String("b".into()),
            Value::String("c".into()),
        ]);
        assert_eq!(
            combine(&list, Some(&Value::String(", ".into()))).unwrap(),
            Value::String("a, b, c".into())
        );
    }

    #[test]
    fn combine_skips_nulls() {
        let list = Value::List(vec![
            Value::String("a".into()),
            Value::Null,
            Value::String("c".into()),
        ]);
        assert_eq!(
            combine(&list, Some(&Value::String("-".into()))).unwrap(),
            Value::String("a-c".into())
        );
    }

    #[test]
    fn combine_null_separator_means_empty_string() {
        let list = Value::List(vec![Value::String("x".into()), Value::String("y".into())]);
        assert_eq!(combine(&list, None).unwrap(), Value::String("xy".into()));
    }

    #[test]
    fn split_basic() {
        let result = split(&Value::String("a,b,c".into()), &Value::String(",".into())).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".into()),
                Value::String("b".into()),
                Value::String("c".into()),
            ])
        );
    }

    #[test]
    fn length_str_unicode() {
        // "café" has 4 Unicode characters
        assert_eq!(
            length_str(&Value::String("café".into())).unwrap(),
            Value::Integer(4)
        );
    }

    #[test]
    fn upper_lower_round_trip() {
        let s = Value::String("Hello World".into());
        assert_eq!(upper(&s).unwrap(), Value::String("HELLO WORLD".into()));
        assert_eq!(lower(&s).unwrap(), Value::String("hello world".into()));
    }

    #[test]
    fn starts_ends_with() {
        let s = Value::String("foobar".into());
        assert_eq!(
            starts_with(&s, &Value::String("foo".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            starts_with(&s, &Value::String("bar".into())).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            ends_with(&s, &Value::String("bar".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            ends_with(&s, &Value::String("foo".into())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn matches_regex_full_match() {
        let s = Value::String("hello123".into());
        // Must match full string
        assert_eq!(
            matches_regex(&s, &Value::String("[a-z]+\\d+".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            matches_regex(&s, &Value::String("[a-z]+".into())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn replace_matches_substitution() {
        let result = replace_matches(
            &Value::String("foo123bar".into()),
            &Value::String("\\d+".into()),
            &Value::String("NUM".into()),
        )
        .unwrap();
        assert_eq!(result, Value::String("fooNUMbar".into()));
    }

    #[test]
    fn indexer_str_zero_based() {
        let s = Value::String("abc".into());
        assert_eq!(
            indexer_str(&s, &Value::Integer(0)).unwrap(),
            Value::String("a".into())
        );
        assert_eq!(
            indexer_str(&s, &Value::Integer(2)).unwrap(),
            Value::String("c".into())
        );
        assert_eq!(indexer_str(&s, &Value::Integer(5)).unwrap(), Value::Null);
        assert_eq!(indexer_str(&s, &Value::Integer(-1)).unwrap(), Value::Null);
    }

    #[test]
    fn position_of_found_and_not_found() {
        let s = Value::String("abcabc".into());
        let pat = Value::String("bc".into());
        // first occurrence at index 1 (0-based)
        assert_eq!(position_of(&pat, &s).unwrap(), Value::Integer(1));
        // not found: -1
        assert_eq!(
            position_of(&Value::String("xyz".into()), &s).unwrap(),
            Value::Integer(-1)
        );
    }

    #[test]
    fn last_position_of_found() {
        let s = Value::String("abcabc".into());
        let pat = Value::String("bc".into());
        assert_eq!(last_position_of(&pat, &s).unwrap(), Value::Integer(4));
    }

    #[test]
    fn substring_from_start() {
        let s = Value::String("Hello, World!".into());
        // 0-based
        assert_eq!(
            substring(&s, &Value::Integer(7), None).unwrap(),
            Value::String("World!".into())
        );
    }

    #[test]
    fn substring_with_length() {
        let s = Value::String("Hello, World!".into());
        assert_eq!(
            substring(&s, &Value::Integer(7), Some(&Value::Integer(5))).unwrap(),
            Value::String("World".into())
        );
    }

    #[test]
    fn substring_out_of_bounds_null() {
        let s = Value::String("Hi".into());
        assert_eq!(
            substring(&s, &Value::Integer(10), None).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn split_on_matches_basic() {
        let result = split_on_matches(
            &Value::String("a1b2c3".into()),
            &Value::String("\\d".into()),
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".into()),
                Value::String("b".into()),
                Value::String("c".into()),
                Value::String("".into()),
            ])
        );
    }

    #[test]
    fn split_on_matches_no_match_returns_single_element() {
        let result = split_on_matches(
            &Value::String("hello".into()),
            &Value::String("\\d+".into()),
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![Value::String("hello".into())]));
    }

    #[test]
    fn split_on_matches_null_propagates() {
        assert_eq!(
            split_on_matches(&Value::Null, &Value::String("x".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            split_on_matches(&Value::String("x".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn starts_with_null_propagates() {
        assert_eq!(
            starts_with(&Value::Null, &Value::String("foo".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            starts_with(&Value::String("foo".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn ends_with_null_propagates() {
        assert_eq!(
            ends_with(&Value::Null, &Value::String("bar".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            ends_with(&Value::String("foobar".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn upper_null_propagates() {
        assert_eq!(upper(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn lower_null_propagates() {
        assert_eq!(lower(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn length_str_null_propagates() {
        assert_eq!(length_str(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn split_null_propagates() {
        assert_eq!(
            split(&Value::Null, &Value::String(",".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            split(&Value::String("a,b".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }
}
