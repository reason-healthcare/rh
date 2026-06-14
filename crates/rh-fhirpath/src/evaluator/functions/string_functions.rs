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
            if params.len() != 2 {
                return Err(FhirPathError::InvalidOperation {
                    message:
                        "replaceMatches() requires exactly two parameters (regex, substitution)"
                            .to_string(),
                });
            }
            StringEvaluator::replace_matches(target, &params[0], &params[1])
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
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "join() requires exactly one parameter (delimiter)".to_string(),
                });
            }
            StringEvaluator::join(target, &params[0])
        }),
    );

    // matches() function
    functions.insert(
        "matches".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "matches() requires exactly one parameter (pattern)".to_string(),
                });
            }
            StringEvaluator::matches(target, &params[0])
        }),
    );

    // matchesFull() function
    functions.insert(
        "matchesFull".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "matchesFull() requires exactly one parameter (pattern)".to_string(),
                });
            }
            StringEvaluator::matches_full(target, &params[0])
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

    // encode(format) / decode(format) — base64, urlbase64, hex
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
    let value = match target {
        FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
        FhirPathValue::Collection(items) if items.is_empty() => return Ok(FhirPathValue::Empty),
        FhirPathValue::Collection(items) if items.len() == 1 => &items[0],
        other => other,
    };
    let FhirPathValue::String(s) = value else {
        return Err(FhirPathError::FunctionError {
            message: format!("{name}() requires a string input"),
        });
    };
    let Some(FhirPathValue::String(format)) = params.first() else {
        return Err(FhirPathError::FunctionError {
            message: format!("{name}() requires a string format parameter"),
        });
    };
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
        "hex" => {
            if !s.len().is_multiple_of(2) {
                None
            } else {
                (0..s.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
                    .collect::<Option<Vec<u8>>>()
            }
        }
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
            s.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#39;"),
        ),
        "json" => Some(
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t"),
        ),
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("escape(): unsupported format '{format}'"),
            })
        }
    })
}

fn unescape_string(s: &str, format: &str) -> FhirPathResult<Option<String>> {
    Ok(match format {
        "html" => Some(
            s.replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .replace("&amp;", "&"),
        ),
        "json" => Some(
            s.replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
                .replace("\\\\", "\\"),
        ),
        _ => {
            return Err(FhirPathError::FunctionError {
                message: format!("unescape(): unsupported format '{format}'"),
            })
        }
    })
}
