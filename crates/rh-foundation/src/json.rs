//! JSON utility functions.
//!
//! This module provides convenience functions for working with JSON.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Parse a JSON string into a typed value.
pub fn parse<T>(json: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(json).map_err(Into::into)
}

/// Serialize a value to a JSON string.
pub fn stringify<T>(value: &T, pretty: bool) -> Result<String>
where
    T: Serialize,
{
    if pretty {
        serde_json::to_string_pretty(value).map_err(Into::into)
    } else {
        serde_json::to_string(value).map_err(Into::into)
    }
}

/// Parse a JSON value from bytes.
pub fn from_bytes<T>(bytes: &[u8]) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(bytes).map_err(Into::into)
}

/// Serialize a value to JSON bytes.
pub fn to_bytes<T>(value: &T, pretty: bool) -> Result<Vec<u8>>
where
    T: Serialize,
{
    if pretty {
        serde_json::to_vec_pretty(value).map_err(Into::into)
    } else {
        serde_json::to_vec(value).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        count: i32,
    }

    #[test]
    fn test_parse_and_stringify() {
        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        let json = stringify(&data, false).unwrap();
        let parsed: TestData = parse(&json).unwrap();

        assert_eq!(data, parsed);
    }

    #[test]
    fn test_bytes_roundtrip() {
        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        let bytes = to_bytes(&data, false).unwrap();
        let parsed: TestData = from_bytes(&bytes).unwrap();

        assert_eq!(data, parsed);
    }

    #[test]
    fn test_pretty_print() {
        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        let pretty = stringify(&data, true).unwrap();
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  "));
    }
}
