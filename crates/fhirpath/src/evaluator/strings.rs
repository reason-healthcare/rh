use crate::evaluator::values::FhirPathValue;
use crate::error::FhirPathError;

/// Evaluator for string manipulation functions in FHIRPath expressions
pub struct StringEvaluator;

impl StringEvaluator {
    /// Get the length of a string
    /// FHIRPath: String.length() -> Integer
    pub fn length(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        match target {
            FhirPathValue::String(s) => Ok(FhirPathValue::Integer(s.len() as i64)),
            _ => Err(FhirPathError::TypeError {
                message: "length() can only be called on String values".to_string(),
            }),
        }
    }

    /// Extract a substring from a string
    /// FHIRPath: String.substring(start: Integer, length?: Integer) -> String
    pub fn substring(
        target: &FhirPathValue,
        start: &FhirPathValue,
        length: Option<&FhirPathValue>,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "substring() can only be called on String values".to_string(),
                })
            }
        };

        let start_index = match start {
            FhirPathValue::Integer(i) => *i as usize,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "substring() start parameter must be an Integer".to_string(),
                })
            }
        };

        if start_index > string.len() {
            return Ok(FhirPathValue::String(String::new()));
        }

        let result = match length {
            Some(len_val) => {
                let length_val = match len_val {
                    FhirPathValue::Integer(i) => *i as usize,
                    _ => {
                        return Err(FhirPathError::TypeError {
                            message: "substring() length parameter must be an Integer".to_string(),
                        })
                    }
                };

                let end_index = std::cmp::min(start_index + length_val, string.len());
                string[start_index..end_index].to_string()
            }
            None => string[start_index..].to_string(),
        };

        Ok(FhirPathValue::String(result))
    }

    /// Check if a string starts with a given prefix
    /// FHIRPath: String.startsWith(prefix: String) -> Boolean
    pub fn starts_with(
        target: &FhirPathValue,
        prefix: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "startsWith() can only be called on String values".to_string(),
                })
            }
        };

        let prefix_str = match prefix {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "startsWith() prefix parameter must be a String".to_string(),
                })
            }
        };

        Ok(FhirPathValue::Boolean(string.starts_with(prefix_str)))
    }

    /// Check if a string ends with a given suffix
    /// FHIRPath: String.endsWith(suffix: String) -> Boolean
    pub fn ends_with(
        target: &FhirPathValue,
        suffix: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "endsWith() can only be called on String values".to_string(),
                })
            }
        };

        let suffix_str = match suffix {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "endsWith() suffix parameter must be a String".to_string(),
                })
            }
        };

        Ok(FhirPathValue::Boolean(string.ends_with(suffix_str)))
    }

    /// Find the index of the first occurrence of a substring
    /// FHIRPath: String.indexOf(substring: String) -> Integer
    /// Returns -1 if not found (following FHIRPath specification)
    pub fn index_of(
        target: &FhirPathValue,
        substring: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "indexOf() can only be called on String values".to_string(),
                })
            }
        };

        let substring_str = match substring {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "indexOf() substring parameter must be a String".to_string(),
                })
            }
        };

        match string.find(substring_str) {
            Some(index) => Ok(FhirPathValue::Integer(index as i64)),
            None => Ok(FhirPathValue::Integer(-1)),
        }
    }

    /// Replace all occurrences of a pattern with a replacement string
    /// FHIRPath: String.replace(pattern: String, replacement: String) -> String
    pub fn replace(
        target: &FhirPathValue,
        pattern: &FhirPathValue,
        replacement: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "replace() can only be called on String values".to_string(),
                })
            }
        };

        let pattern_str = match pattern {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "replace() pattern parameter must be a String".to_string(),
                })
            }
        };

        let replacement_str = match replacement {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "replace() replacement parameter must be a String".to_string(),
                })
            }
        };

        Ok(FhirPathValue::String(string.replace(pattern_str, replacement_str)))
    }

    /// Convert a string to uppercase
    /// FHIRPath: String.upper() -> String
    pub fn upper(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        match target {
            FhirPathValue::String(s) => Ok(FhirPathValue::String(s.to_uppercase())),
            _ => Err(FhirPathError::TypeError {
                message: "upper() can only be called on String values".to_string(),
            }),
        }
    }

    /// Convert a string to lowercase
    /// FHIRPath: String.lower() -> String
    pub fn lower(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        match target {
            FhirPathValue::String(s) => Ok(FhirPathValue::String(s.to_lowercase())),
            _ => Err(FhirPathError::TypeError {
                message: "lower() can only be called on String values".to_string(),
            }),
        }
    }

    /// Remove leading and trailing whitespace from a string
    /// FHIRPath: String.trim() -> String
    pub fn trim(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        match target {
            FhirPathValue::String(s) => Ok(FhirPathValue::String(s.trim().to_string())),
            _ => Err(FhirPathError::TypeError {
                message: "trim() can only be called on String values".to_string(),
            }),
        }
    }

    /// Split a string by a delimiter
    /// FHIRPath: String.split(delimiter: String) -> Collection(String)
    pub fn split(
        target: &FhirPathValue,
        delimiter: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "split() can only be called on String values".to_string(),
                })
            }
        };

        let delimiter_str = match delimiter {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "split() delimiter parameter must be a String".to_string(),
                })
            }
        };

        let parts: Vec<FhirPathValue> = string
            .split(delimiter_str)
            .map(|s| FhirPathValue::String(s.to_string()))
            .collect();

        Ok(FhirPathValue::Collection(parts))
    }

    /// Join a collection of strings with a delimiter
    /// FHIRPath: Collection(String).join(delimiter: String) -> String
    pub fn join(
        target: &FhirPathValue,
        delimiter: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let collection = match target {
            FhirPathValue::Collection(items) => items,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "join() can only be called on Collection values".to_string(),
                })
            }
        };

        let delimiter_str = match delimiter {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "join() delimiter parameter must be a String".to_string(),
                })
            }
        };

        let string_parts: Result<Vec<String>, FhirPathError> = collection
            .iter()
            .map(|item| match item {
                FhirPathValue::String(s) => Ok(s.clone()),
                _ => Err(FhirPathError::TypeError {
                    message: "join() can only be called on collections of String values".to_string(),
                }),
            })
            .collect();

        match string_parts {
            Ok(parts) => Ok(FhirPathValue::String(parts.join(delimiter_str))),
            Err(e) => Err(e),
        }
    }

    /// Check if a string matches a regular expression pattern
    /// FHIRPath: String.matches(pattern: String) -> Boolean
    pub fn matches(
        target: &FhirPathValue,
        pattern: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "matches() can only be called on String values".to_string(),
                })
            }
        };

        let pattern_str = match pattern {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "matches() pattern parameter must be a String".to_string(),
                })
            }
        };

        // For now, we'll implement a simple contains check
        // In a full implementation, you'd want to use a regex crate
        Ok(FhirPathValue::Boolean(string.contains(pattern_str)))
    }

    /// Check if a string contains another string (helper function)
    /// This is not a standard FHIRPath function, but useful for implementation
    pub fn contains(
        target: &FhirPathValue,
        substring: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let string = match target {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "contains() can only be called on String values".to_string(),
                })
            }
        };

        let substring_str = match substring {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "contains() substring parameter must be a String".to_string(),
                })
            }
        };

        Ok(FhirPathValue::Boolean(string.contains(substring_str)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_length() {
        let input = FhirPathValue::String("hello".to_string());
        let result = StringEvaluator::length(&input).unwrap();
        assert_eq!(result, FhirPathValue::Integer(5));
    }

    #[test]
    fn test_string_length_empty() {
        let input = FhirPathValue::String("".to_string());
        let result = StringEvaluator::length(&input).unwrap();
        assert_eq!(result, FhirPathValue::Integer(0));
    }

    #[test]
    fn test_substring_with_start_only() {
        let input = FhirPathValue::String("hello world".to_string());
        let start = FhirPathValue::Integer(6);
        let result = StringEvaluator::substring(&input, &start, None).unwrap();
        assert_eq!(result, FhirPathValue::String("world".to_string()));
    }

    #[test]
    fn test_substring_with_start_and_length() {
        let input = FhirPathValue::String("hello world".to_string());
        let start = FhirPathValue::Integer(0);
        let length = FhirPathValue::Integer(5);
        let result = StringEvaluator::substring(&input, &start, Some(&length)).unwrap();
        assert_eq!(result, FhirPathValue::String("hello".to_string()));
    }

    #[test]
    fn test_starts_with_true() {
        let input = FhirPathValue::String("hello world".to_string());
        let prefix = FhirPathValue::String("hello".to_string());
        let result = StringEvaluator::starts_with(&input, &prefix).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_starts_with_false() {
        let input = FhirPathValue::String("hello world".to_string());
        let prefix = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::starts_with(&input, &prefix).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_ends_with_true() {
        let input = FhirPathValue::String("hello world".to_string());
        let suffix = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::ends_with(&input, &suffix).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_index_of_found() {
        let input = FhirPathValue::String("hello world".to_string());
        let substring = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(6));
    }

    #[test]
    fn test_index_of_not_found() {
        let input = FhirPathValue::String("hello world".to_string());
        let substring = FhirPathValue::String("foo".to_string());
        let result = StringEvaluator::index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-1));
    }

    #[test]
    fn test_replace() {
        let input = FhirPathValue::String("hello world".to_string());
        let pattern = FhirPathValue::String("world".to_string());
        let replacement = FhirPathValue::String("universe".to_string());
        let result = StringEvaluator::replace(&input, &pattern, &replacement).unwrap();
        assert_eq!(result, FhirPathValue::String("hello universe".to_string()));
    }

    #[test]
    fn test_upper() {
        let input = FhirPathValue::String("hello world".to_string());
        let result = StringEvaluator::upper(&input).unwrap();
        assert_eq!(result, FhirPathValue::String("HELLO WORLD".to_string()));
    }

    #[test]
    fn test_lower() {
        let input = FhirPathValue::String("HELLO WORLD".to_string());
        let result = StringEvaluator::lower(&input).unwrap();
        assert_eq!(result, FhirPathValue::String("hello world".to_string()));
    }

    #[test]
    fn test_trim() {
        let input = FhirPathValue::String("  hello world  ".to_string());
        let result = StringEvaluator::trim(&input).unwrap();
        assert_eq!(result, FhirPathValue::String("hello world".to_string()));
    }

    #[test]
    fn test_split() {
        let input = FhirPathValue::String("a,b,c".to_string());
        let delimiter = FhirPathValue::String(",".to_string());
        let result = StringEvaluator::split(&input, &delimiter).unwrap();
        
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], FhirPathValue::String("a".to_string()));
                assert_eq!(items[1], FhirPathValue::String("b".to_string()));
                assert_eq!(items[2], FhirPathValue::String("c".to_string()));
            }
            _ => panic!("Expected Collection"),
        }
    }

    #[test]
    fn test_join() {
        let items = vec![
            FhirPathValue::String("a".to_string()),
            FhirPathValue::String("b".to_string()),
            FhirPathValue::String("c".to_string()),
        ];
        let input = FhirPathValue::Collection(items);
        let delimiter = FhirPathValue::String(",".to_string());
        let result = StringEvaluator::join(&input, &delimiter).unwrap();
        assert_eq!(result, FhirPathValue::String("a,b,c".to_string()));
    }

    #[test]
    fn test_contains() {
        let input = FhirPathValue::String("hello world".to_string());
        let substring = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::contains(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }
}
