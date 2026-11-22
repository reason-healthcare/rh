use crate::error::FhirPathError;
use crate::evaluator::types::FhirPathValue;

fn string_value<'a>(
    value: &'a FhirPathValue,
    operation: &str,
) -> Result<Option<&'a str>, FhirPathError> {
    value
        .singleton(operation)?
        .map(|item| {
            item.as_str()
                .ok_or_else(|| FhirPathError::type_error(format!("{operation} requires a String")))
        })
        .transpose()
}

/// Evaluator for string manipulation functions in FHIRPath expressions
pub struct StringEvaluator;

fn regex_pattern_with_flags(
    pattern: &str,
    flags: Option<&FhirPathValue>,
) -> Result<String, FhirPathError> {
    let flags_str = match flags {
        Some(flags) => string_value(flags, "Regex flags")?.unwrap_or_default(),
        None => "",
    };
    let mut prefix = String::from("(?s");
    for flag in flags_str.chars() {
        match flag {
            'i' if !prefix.contains('i') => prefix.push('i'),
            'm' if !prefix.contains('m') => prefix.push('m'),
            'i' | 'm' => {}
            _ => {
                return Err(FhirPathError::EvaluationError {
                    message: format!("Invalid regex flag: {flag}"),
                })
            }
        }
    }
    prefix.push(')');
    Ok(format!("{prefix}{pattern}"))
}

impl StringEvaluator {
    /// Get the length of a string
    /// FHIRPath: String.length() -> Integer
    pub fn length(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        Ok(string_value(target, "length()")?
            .map(|s| FhirPathValue::Integer(s.chars().count() as i64))
            .unwrap_or(FhirPathValue::Empty))
    }

    /// Extract a substring from a string. Per FHIRPath §6.7, an out-of-range
    /// or negative `start` returns the empty collection (not an empty string),
    /// and an empty input collection propagates empty.
    /// FHIRPath: String.substring(start: Integer, length?: Integer) -> String
    pub fn substring(
        target: &FhirPathValue,
        start: &FhirPathValue,
        length: Option<&FhirPathValue>,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "substring()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(start) = start.singleton("substring() start")? else {
            return Ok(FhirPathValue::Empty);
        };
        let (FhirPathValue::Integer(start) | FhirPathValue::Long(start)) = start else {
            return Err(FhirPathError::type_error(
                "substring() start must be an Integer",
            ));
        };
        let Ok(start) = usize::try_from(*start) else {
            return Ok(FhirPathValue::Empty);
        };
        if start >= string.chars().count() {
            return Ok(FhirPathValue::Empty);
        }
        let length = match length {
            Some(value) => match value.singleton("substring() length")? {
                Some(FhirPathValue::Integer(length) | FhirPathValue::Long(length)) => {
                    usize::try_from((*length).max(0)).unwrap_or(usize::MAX)
                }
                None => usize::MAX,
                _ => {
                    return Err(FhirPathError::type_error(
                        "substring() length must be an Integer",
                    ))
                }
            },
            None => usize::MAX,
        };
        let result = string.chars().skip(start).take(length).collect();
        Ok(FhirPathValue::String(result))
    }

    /// Check if a string starts with a given prefix
    /// FHIRPath: String.startsWith(prefix: String) -> Boolean
    pub fn starts_with(
        target: &FhirPathValue,
        prefix: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "startsWith()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(prefix_str) = string_value(prefix, "startsWith() prefix parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        Ok(FhirPathValue::Boolean(string.starts_with(prefix_str)))
    }

    /// Check if a string ends with a given suffix
    /// FHIRPath: String.endsWith(suffix: String) -> Boolean
    pub fn ends_with(
        target: &FhirPathValue,
        suffix: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "endsWith()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(suffix_str) = string_value(suffix, "endsWith() suffix parameter")? else {
            return Ok(FhirPathValue::Empty);
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
        let Some(string) = string_value(target, "indexOf()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(substring_str) = string_value(substring, "indexOf() substring parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        match string.find(substring_str) {
            Some(index) => Ok(FhirPathValue::Integer(
                string[..index].chars().count() as i64
            )),
            None => Ok(FhirPathValue::Integer(-1)),
        }
    }

    /// Find the index of the last occurrence of a substring
    /// FHIRPath: String.lastIndexOf(substring: String) -> Integer
    /// Returns -1 if not found (following FHIRPath specification)
    pub fn last_index_of(
        target: &FhirPathValue,
        substring: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "lastIndexOf()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(substring_str) = string_value(substring, "lastIndexOf() substring parameter")?
        else {
            return Ok(FhirPathValue::Empty);
        };

        match string.rfind(substring_str) {
            Some(index) => Ok(FhirPathValue::Integer(
                string[..index].chars().count() as i64
            )),
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
        let Some(string) = string_value(target, "replace()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(pattern_str) = string_value(pattern, "replace() pattern parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(replacement_str) = string_value(replacement, "replace() replacement parameter")?
        else {
            return Ok(FhirPathValue::Empty);
        };

        Ok(FhirPathValue::String(
            string.replace(pattern_str, replacement_str),
        ))
    }

    /// Replace all matches of a regular expression pattern with a substitution
    /// string. Per FHIRPath §6.7 string functions, an empty input collection
    /// or an empty regex/substitution argument propagates empty. An empty
    /// regex string matches nothing, so the original input is returned.
    /// FHIRPath: String.replaceMatches(regex: String, substitution: String) -> String
    pub fn replace_matches(
        target: &FhirPathValue,
        regex_pattern: &FhirPathValue,
        substitution: &FhirPathValue,
        flags: Option<&FhirPathValue>,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "replaceMatches()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(pattern_str) = string_value(regex_pattern, "replaceMatches() regex parameter")?
        else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(substitution_str) =
            string_value(substitution, "replaceMatches() substitution parameter")?
        else {
            return Ok(FhirPathValue::Empty);
        };

        // An empty pattern matches the zero-width position before every
        // character — per the suite's expectation, treat this as no match
        // and return the input unchanged rather than splicing the
        // substitution between every character.
        if pattern_str.is_empty() {
            return Ok(FhirPathValue::String(string.to_owned()));
        }

        let pattern = regex_pattern_with_flags(pattern_str, flags)?;
        match regex::Regex::new(&pattern) {
            Ok(re) => Ok(FhirPathValue::String(
                re.replace_all(string, substitution_str).to_string(),
            )),
            Err(e) => Err(FhirPathError::EvaluationError {
                message: format!("Invalid regex pattern: {e}"),
            }),
        }
    }

    /// Convert a string to uppercase
    /// FHIRPath: String.upper() -> String
    pub fn upper(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        Ok(string_value(target, "upper()")?
            .map(|s| FhirPathValue::String(s.to_uppercase()))
            .unwrap_or(FhirPathValue::Empty))
    }

    /// Convert a string to lowercase
    /// FHIRPath: String.lower() -> String
    pub fn lower(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        Ok(string_value(target, "lower()")?
            .map(|s| FhirPathValue::String(s.to_lowercase()))
            .unwrap_or(FhirPathValue::Empty))
    }

    /// Remove leading and trailing whitespace from a string
    /// FHIRPath: String.trim() -> String
    pub fn trim(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        Ok(string_value(target, "trim()")?
            .map(|s| FhirPathValue::String(s.trim_matches([' ', '\t', '\r', '\n']).to_owned()))
            .unwrap_or(FhirPathValue::Empty))
    }

    /// Split a string by a delimiter
    /// FHIRPath: String.split(delimiter: String) -> Collection(String)
    pub fn split(
        target: &FhirPathValue,
        delimiter: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "split()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(delimiter_str) = string_value(delimiter, "split() delimiter parameter")? else {
            return Ok(FhirPathValue::Empty);
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
        let items = target.items();
        if items.is_empty() {
            return Ok(FhirPathValue::Empty);
        }
        let delimiter = string_value(delimiter, "join() delimiter")?.unwrap_or_default();
        let strings: Result<Vec<&str>, _> = items
            .iter()
            .map(|item| {
                item.as_str().ok_or_else(|| {
                    FhirPathError::type_error("join() requires a collection of Strings")
                })
            })
            .collect();
        Ok(FhirPathValue::String(strings?.join(delimiter)))
    }

    /// Check if a string matches a regular expression pattern
    /// FHIRPath: String.matches(pattern: String) -> Boolean
    pub fn matches(
        target: &FhirPathValue,
        pattern: &FhirPathValue,
        flags: Option<&FhirPathValue>,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "matches()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(pattern_str) = string_value(pattern, "matches() pattern parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        let pattern = regex_pattern_with_flags(pattern_str, flags)?;
        match regex::Regex::new(&pattern) {
            Ok(re) => Ok(FhirPathValue::Boolean(re.is_match(string))),
            Err(_) => {
                // If regex compilation fails, return false (invalid pattern)
                Ok(FhirPathValue::Boolean(false))
            }
        }
    }

    /// Check if a string matches a regular expression pattern exactly (full match with implied anchors)
    /// FHIRPath: String.matchesFull(pattern: String) -> Boolean
    /// This is equivalent to wrapping the pattern with ^ and $ anchors
    pub fn matches_full(
        target: &FhirPathValue,
        pattern: &FhirPathValue,
        flags: Option<&FhirPathValue>,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "matchesFull()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(pattern_str) = string_value(pattern, "matchesFull() pattern parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        let flagged_pattern = regex_pattern_with_flags(pattern_str, flags)?;
        let anchored_pattern = format!(r"\A(?:{flagged_pattern})\z");

        // Use regex for proper pattern matching with anchors
        match regex::Regex::new(&anchored_pattern) {
            Ok(re) => Ok(FhirPathValue::Boolean(re.is_match(string))),
            Err(_) => {
                // If regex compilation fails, return false (invalid pattern)
                Ok(FhirPathValue::Boolean(false))
            }
        }
    }

    /// Check if a string contains another string (helper function)
    /// This is not a standard FHIRPath function, but useful for implementation
    pub fn contains(
        target: &FhirPathValue,
        substring: &FhirPathValue,
    ) -> Result<FhirPathValue, FhirPathError> {
        let Some(string) = string_value(target, "contains()")? else {
            return Ok(FhirPathValue::Empty);
        };

        let Some(substring_str) = string_value(substring, "contains() substring parameter")? else {
            return Ok(FhirPathValue::Empty);
        };

        Ok(FhirPathValue::Boolean(string.contains(substring_str)))
    }

    /// Convert a string to a collection of single-character strings
    /// FHIRPath: String.toChars() -> Collection[String]
    pub fn to_chars(target: &FhirPathValue) -> Result<FhirPathValue, FhirPathError> {
        Ok(string_value(target, "toChars()")?
            .map(|s| {
                FhirPathValue::Collection(
                    s.chars()
                        .map(|c| FhirPathValue::String(c.to_string()))
                        .collect(),
                )
            })
            .unwrap_or(FhirPathValue::Empty))
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
    fn test_last_index_of_found() {
        let input = FhirPathValue::String("hello world world".to_string());
        let substring = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::last_index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(12));
    }

    #[test]
    fn test_last_index_of_not_found() {
        let input = FhirPathValue::String("hello world".to_string());
        let substring = FhirPathValue::String("foo".to_string());
        let result = StringEvaluator::last_index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-1));
    }

    #[test]
    fn test_last_index_of_single_occurrence() {
        let input = FhirPathValue::String("hello world".to_string());
        let substring = FhirPathValue::String("world".to_string());
        let result = StringEvaluator::last_index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(6));
    }

    #[test]
    fn test_last_index_of_empty_substring() {
        let input = FhirPathValue::String("hello".to_string());
        let substring = FhirPathValue::String("".to_string());
        let result = StringEvaluator::last_index_of(&input, &substring).unwrap();
        assert_eq!(result, FhirPathValue::Integer(5)); // Empty string is found at the end
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

    #[test]
    fn test_replace_matches() {
        // Test basic regex replacement
        let input =
            FhirPathValue::String("The year 2023 was great, and 2024 will be better!".to_string());
        let regex = FhirPathValue::String(r"\d{4}".to_string());
        let substitution = FhirPathValue::String("YYYY".to_string());
        let result = StringEvaluator::replace_matches(&input, &regex, &substitution, None).unwrap();
        assert_eq!(
            result,
            FhirPathValue::String("The year YYYY was great, and YYYY will be better!".to_string())
        );

        // Test with capture groups
        let input = FhirPathValue::String("john.doe@example.com".to_string());
        let regex = FhirPathValue::String(r"(\w+)\.(\w+)@(.+)".to_string());
        let substitution = FhirPathValue::String("$2, $1 from $3".to_string());
        let result = StringEvaluator::replace_matches(&input, &regex, &substitution, None).unwrap();
        assert_eq!(
            result,
            FhirPathValue::String("doe, john from example.com".to_string())
        );

        // Test with word boundaries
        let input = FhirPathValue::String("cat catastrophe scattered".to_string());
        let regex = FhirPathValue::String(r"\bcat\b".to_string());
        let substitution = FhirPathValue::String("dog".to_string());
        let result = StringEvaluator::replace_matches(&input, &regex, &substitution, None).unwrap();
        assert_eq!(
            result,
            FhirPathValue::String("dog catastrophe scattered".to_string())
        );

        // Test invalid regex
        let input = FhirPathValue::String("test".to_string());
        let invalid_regex = FhirPathValue::String("[".to_string()); // Invalid regex
        let substitution = FhirPathValue::String("replacement".to_string());
        let result = StringEvaluator::replace_matches(&input, &invalid_regex, &substitution, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_chars() {
        let input = FhirPathValue::String("hello".to_string());
        let result = StringEvaluator::to_chars(&input).unwrap();
        let expected = FhirPathValue::Collection(vec![
            FhirPathValue::String("h".to_string()),
            FhirPathValue::String("e".to_string()),
            FhirPathValue::String("l".to_string()),
            FhirPathValue::String("l".to_string()),
            FhirPathValue::String("o".to_string()),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_chars_empty_string() {
        let input = FhirPathValue::String("".to_string());
        let result = StringEvaluator::to_chars(&input).unwrap();
        let expected = FhirPathValue::Collection(vec![]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_chars_unicode() {
        let input = FhirPathValue::String("café".to_string());
        let result = StringEvaluator::to_chars(&input).unwrap();
        let expected = FhirPathValue::Collection(vec![
            FhirPathValue::String("c".to_string()),
            FhirPathValue::String("a".to_string()),
            FhirPathValue::String("f".to_string()),
            FhirPathValue::String("é".to_string()),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_chars_non_string() {
        let input = FhirPathValue::Integer(42);
        let result = StringEvaluator::to_chars(&input);
        assert!(result.is_err());
    }
}
