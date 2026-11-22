//! String function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::strings::StringEvaluator;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all string manipulation functions
pub fn register_string_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // length() function
    functions.insert(
        "length".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::length(target)
        }),
    );

    // upper() function
    functions.insert(
        "upper".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::upper(target)
        }),
    );

    // lower() function
    functions.insert(
        "lower".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::lower(target)
        }),
    );

    // trim() function
    functions.insert(
        "trim".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| StringEvaluator::trim(target)),
    );

    // substring() function
    functions.insert(
        "substring".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "substring() requires at least one parameter (start index)"
                        .to_string(),
                });
            }
            let start = &params[0];
            let length = params.get(1);
            StringEvaluator::substring(target, start, length)
        }),
    );

    // startsWith() function
    functions.insert(
        "startsWith".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "startsWith() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::starts_with(target, &params[0])
        }),
    );

    // endsWith() function
    functions.insert(
        "endsWith".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "endsWith() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::ends_with(target, &params[0])
        }),
    );

    // indexOf() function
    functions.insert(
        "indexOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "indexOf() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::index_of(target, &params[0])
        }),
    );

    // lastIndexOf() function
    functions.insert(
        "lastIndexOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "lastIndexOf() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::last_index_of(target, &params[0])
        }),
    );

    // replace() function
    functions.insert(
        "replace".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 2 {
                return Err(FhirPathError::InvalidOperation {
                    message: "replace() requires exactly two parameters (pattern, replacement)"
                        .to_string(),
                });
            }
            StringEvaluator::replace(target, &params[0], &params[1])
        }),
    );

    // replaceMatches() function
    functions.insert(
        "replaceMatches".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !(2..=3).contains(&params.len()) {
                return Err(FhirPathError::InvalidOperation {
                    message:
                        "replaceMatches() requires two or three parameters (regex, substitution, flags)"
                            .to_string(),
                });
            }
            StringEvaluator::replace_matches(target, &params[0], &params[1], params.get(2))
        }),
    );

    // split() function
    functions.insert(
        "split".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "split() requires exactly one parameter (delimiter)".to_string(),
                });
            }
            StringEvaluator::split(target, &params[0])
        }),
    );

    // join() function
    functions.insert(
        "join".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() > 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "join() accepts at most one parameter (delimiter)".to_string(),
                });
            }
            let empty = FhirPathValue::String(String::new());
            StringEvaluator::join(target, params.first().unwrap_or(&empty))
        }),
    );

    // matches() function
    functions.insert(
        "matches".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !(1..=2).contains(&params.len()) {
                return Err(FhirPathError::InvalidOperation {
                    message: "matches() requires one or two parameters (pattern, flags)"
                        .to_string(),
                });
            }
            StringEvaluator::matches(target, &params[0], params.get(1))
        }),
    );

    // matchesFull() function
    functions.insert(
        "matchesFull".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !(1..=2).contains(&params.len()) {
                return Err(FhirPathError::InvalidOperation {
                    message: "matchesFull() requires one or two parameters (pattern, flags)"
                        .to_string(),
                });
            }
            StringEvaluator::matches_full(target, &params[0], params.get(1))
        }),
    );

    // contains() function
    functions.insert(
        "contains".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "contains() requires exactly one parameter (substring)".to_string(),
                });
            }
            StringEvaluator::contains(target, &params[0])
        }),
    );

    // toChars() function
    functions.insert(
        "toChars".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::to_chars(target)
        }),
    );

    // Encode and decode strings in the specified format.
    functions.insert(
        "encode".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            with_string_arg(target, params, "encode", encode_string)
        }),
    );
    functions.insert(
        "decode".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            with_string_arg(target, params, "decode", decode_string)
        }),
    );

    // escape(target) / unescape(target) — html, json
    functions.insert(
        "escape".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            with_string_arg(target, params, "escape", escape_string)
        }),
    );
    functions.insert(
        "unescape".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            with_string_arg(target, params, "unescape", unescape_string)
        }),
    );
}

/// Helper: apply a (string, format) -> string transformation, propagating
/// empty and validating argument shape.
fn with_string_arg(
    target: &FhirPathValue,
    params: &[FhirPathValue],
    name: &str,
    apply: fn(&str, &str) -> FhirPathResult<Option<String>>,
) -> FhirPathResult<FhirPathValue> {
    let [parameter] = params else {
        if params.is_empty() {
            return Ok(FhirPathValue::Empty);
        }
        return Err(FhirPathError::function_error(format!(
            "{name}() takes one format parameter"
        )));
    };
    let Some(value) = target.singleton(name)? else {
        return Ok(FhirPathValue::Empty);
    };
    let Some(format) = parameter.singleton(name)? else {
        return Ok(FhirPathValue::Empty);
    };
    let s = value
        .as_str()
        .ok_or_else(|| FhirPathError::type_error(format!("{name}() requires a String input")))?;
    let format = format.as_str().ok_or_else(|| {
        FhirPathError::type_error(format!("{name}() requires a String format parameter"))
    })?;
    match apply(s, format)? {
        Some(out) => Ok(FhirPathValue::String(out)),
        None => Ok(FhirPathValue::Empty),
    }
}

fn encode_string(s: &str, format: &str) -> FhirPathResult<Option<String>> {
    use base64::Engine as _;
    Ok(match format {
        "base64" => Some(base64::engine::general_purpose::STANDARD.encode(s.as_bytes())),
        "urlbase64" => Some(base64::engine::general_purpose::URL_SAFE.encode(s.as_bytes())),
        "hex" => Some(s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()),
        "ascii" => Some(
            s.chars()
                .map(|c| if c.is_ascii() { c } else { '?' })
                .collect(),
        ),
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("encode(): unsupported format '{format}'"),
            })
        }
    })
}

fn decode_string(s: &str, format: &str) -> FhirPathResult<Option<String>> {
    use base64::Engine as _;
    let bytes = match format {
        "base64" => base64::engine::general_purpose::STANDARD
            .decode(s.as_bytes())
            .ok(),
        "urlbase64" => base64::engine::general_purpose::URL_SAFE
            .decode(s.as_bytes())
            .ok(),
        "hex" if s.len().is_multiple_of(2) && s.bytes().all(|b| b.is_ascii_hexdigit()) => (0..s
            .len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&s[index..index + 2], 16).ok())
            .collect::<Option<Vec<_>>>(),
        "hex" => None,
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("decode(): unsupported format '{format}'"),
            })
        }
    };
    Ok(bytes.and_then(|b| String::from_utf8(b).ok()))
}

fn escape_string(s: &str, format: &str) -> FhirPathResult<Option<String>> {
    Ok(match format {
        "html" => Some(
            s.chars()
                .map(|ch| match ch {
                    '&' => "&amp;".to_string(),
                    '<' => "&lt;".to_string(),
                    '>' => "&gt;".to_string(),
                    '"' => "&quot;".to_string(),
                    '\'' => "&#39;".to_string(),
                    ch if !ch.is_ascii() => format!("&#{};", ch as u32),
                    ch => ch.to_string(),
                })
                .collect(),
        ),
        "json" => {
            let json = serde_json::to_string(s)?;
            Some(json[1..json.len() - 1].to_owned())
        }
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("escape(): unsupported format '{format}'"),
            })
        }
    })
}

fn unescape_string(s: &str, format: &str) -> FhirPathResult<Option<String>> {
    Ok(match format {
        "html" => Some(unescape_html(s)),
        "json" => unescape_json(s),
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("unescape(): unsupported format '{format}'"),
            })
        }
    })
}

fn unescape_json(text: &str) -> Option<String> {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push('"');
    let mut after_backslash = false;
    for character in text.chars() {
        match character {
            '\\' => {
                quoted.push(character);
                after_backslash = !after_backslash;
                continue;
            }
            '"' if !after_backslash => quoted.push_str("\\\""),
            character if character < ' ' && !after_backslash => {
                use std::fmt::Write;
                write!(&mut quoted, "\\u{:04x}", u32::from(character)).ok()?;
            }
            _ => quoted.push(character),
        }
        after_backslash = false;
    }
    quoted.push('"');
    serde_json::from_str(&quoted).ok()
}

fn unescape_html(mut text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    while let Some((plain, after_ampersand)) = text.split_once('&') {
        output.push_str(plain);
        if let Some(end) = after_ampersand.find(['&', ';']) {
            if after_ampersand.as_bytes()[end] == b';' {
                let entity = &after_ampersand[..end];
                let value = match entity {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    "nbsp" => Some('\u{a0}'),
                    _ => entity.strip_prefix('#').and_then(|code| {
                        if let Some(hex) = code.strip_prefix(['x', 'X']) {
                            u32::from_str_radix(hex, 16).ok()
                        } else {
                            code.parse::<u32>().ok()
                        }
                        .and_then(char::from_u32)
                    }),
                };
                if let Some(value) = value {
                    output.push(value);
                    text = &after_ampersand[end + 1..];
                    continue;
                }
            }
        }
        output.push('&');
        text = after_ampersand;
    }
    output.push_str(text);
    output
}
