use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;

use rust_decimal::Decimal;
use serde::de::Error as _;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use serde_json::{Map, Number, Value};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JsonData {
    pub value: Value,
    pub numbers: Option<Arc<JsonNumbers>>,
}

pub(crate) fn number_text<'a>(
    raw: &Value,
    numbers: Option<&'a JsonNumbers>,
) -> Option<Cow<'a, str>> {
    if let Some(text) = numbers.and_then(JsonNumbers::number) {
        return Some(Cow::Borrowed(text));
    }
    raw.as_number().map(|number| Cow::Owned(number.to_string()))
}

pub(crate) fn parse_decimal(text: &str) -> Option<Decimal> {
    if let Some((mantissa, _)) = text.split_once(['e', 'E']) {
        Decimal::from_str_exact(mantissa).ok()?;
        Decimal::from_scientific(text).ok()
    } else {
        Decimal::from_str_exact(text).ok()
    }
}

#[derive(Debug)]
pub(crate) enum JsonNumbers {
    Number(Box<RawValue>),
    Array(BTreeMap<usize, Arc<Self>>),
    Object(BTreeMap<String, Arc<Self>>),
}

impl PartialEq for JsonNumbers {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.get() == right.get(),
            (Self::Array(left), Self::Array(right)) => left == right,
            (Self::Object(left), Self::Object(right)) => left == right,
            _ => false,
        }
    }
}

impl JsonNumbers {
    pub(crate) fn number(&self) -> Option<&str> {
        match self {
            Self::Number(raw) => Some(raw.get()),
            _ => None,
        }
    }

    pub(crate) fn field(&self, key: &str) -> Option<&Arc<Self>> {
        match self {
            Self::Object(fields) => fields.get(key),
            _ => None,
        }
    }

    pub(crate) fn index(&self, index: usize) -> Option<&Arc<Self>> {
        match self {
            Self::Array(elements) => elements.get(&index),
            _ => None,
        }
    }
}

impl JsonData {
    pub(crate) fn parse(input: &str) -> serde_json::Result<Self> {
        let raw: &RawValue = serde_json::from_str(input)?;
        ValidatedJson(raw.get()).value(0)
    }

    pub(crate) fn from_value(value: Value) -> Self {
        Self {
            value,
            numbers: None,
        }
    }

    pub(crate) fn number(input: &str) -> serde_json::Result<Self> {
        let raw: &RawValue = serde_json::from_str(input)?;
        if !is_number(raw.get()) {
            return Err(serde_json::Error::custom("expected a JSON number"));
        }
        Self::from_number_token(raw.get())
    }

    fn from_number_token(token: &str) -> serde_json::Result<Self> {
        let value = serde_json::from_str::<Number>(token).map_or(Value::Null, Value::Number);
        if value.as_number().is_some_and(|number| {
            number.to_string() == token && (number.is_i64() || parse_decimal(token).is_some())
        }) {
            return Ok(Self::from_value(value));
        }
        let raw = RawValue::from_string(token.to_owned())?;
        Ok(Self {
            value,
            numbers: Some(Arc::new(JsonNumbers::Number(raw))),
        })
    }

    pub(crate) fn array(elements: Vec<Self>) -> Self {
        let mut values = Vec::with_capacity(elements.len());
        let mut numbers = BTreeMap::new();
        for (index, element) in elements.into_iter().enumerate() {
            values.push(element.value);
            if let Some(element_numbers) = element.numbers {
                numbers.insert(index, element_numbers);
            }
        }
        Self {
            value: Value::Array(values),
            numbers: (!numbers.is_empty()).then(|| Arc::new(JsonNumbers::Array(numbers))),
        }
    }

    pub(crate) fn object(fields: BTreeMap<String, Self>) -> Self {
        let mut values = Map::with_capacity(fields.len());
        let mut numbers = BTreeMap::new();
        for (key, field) in fields {
            if let Some(field_numbers) = field.numbers {
                numbers.insert(key.clone(), field_numbers);
            }
            values.insert(key, field.value);
        }
        Self {
            value: Value::Object(values),
            numbers: (!numbers.is_empty()).then(|| Arc::new(JsonNumbers::Object(numbers))),
        }
    }
}

const MAX_CONTAINER_DEPTH: usize = 127;

struct ValidatedJson<'a>(&'a str);

impl<'a> ValidatedJson<'a> {
    fn value(&mut self, depth: usize) -> serde_json::Result<JsonData> {
        self.skip_whitespace();
        match self.0.as_bytes().first() {
            Some(b'[' | b'{') if depth == MAX_CONTAINER_DEPTH => {
                Err(serde_json::Error::custom("recursion limit exceeded"))
            }
            Some(b'[') => self.array(depth + 1),
            Some(b'{') => self.object(depth + 1),
            Some(b'"') => self
                .string()
                .map(|value| JsonData::from_value(Value::String(value))),
            _ => {
                let end = self.0.find(|character| {
                    json_whitespace(character) || matches!(character, ',' | ']' | '}')
                });
                let token = self.take(end.unwrap_or(self.0.len()));
                if is_number(token) {
                    JsonData::from_number_token(token)
                } else {
                    serde_json::from_str(token).map(JsonData::from_value)
                }
            }
        }
    }

    fn array(&mut self, depth: usize) -> serde_json::Result<JsonData> {
        self.take(1);
        let mut elements = Vec::new();
        if !self.consume(']') {
            loop {
                elements.push(self.value(depth)?);
                if self.consume(']') {
                    break;
                }
                self.expect(',')?;
            }
        }
        Ok(JsonData::array(elements))
    }

    fn object(&mut self, depth: usize) -> serde_json::Result<JsonData> {
        self.take(1);
        let mut fields = BTreeMap::new();
        if !self.consume('}') {
            loop {
                self.skip_whitespace();
                let key = self.string()?;
                self.expect(':')?;
                fields.insert(key, self.value(depth)?);
                if self.consume('}') {
                    break;
                }
                self.expect(',')?;
            }
        }
        Ok(JsonData::object(fields))
    }

    fn string(&mut self) -> serde_json::Result<String> {
        let mut stream = serde_json::Deserializer::from_str(self.0).into_iter::<String>();
        let value = stream
            .next()
            .transpose()?
            .ok_or_else(|| serde_json::Error::custom("expected a JSON string"))?;
        self.take(stream.byte_offset());
        Ok(value)
    }

    fn expect(&mut self, character: char) -> serde_json::Result<()> {
        if self.consume(character) {
            Ok(())
        } else {
            Err(serde_json::Error::custom(format!("expected '{character}'")))
        }
    }

    fn consume(&mut self, character: char) -> bool {
        self.skip_whitespace();
        if let Some(remaining) = self.0.strip_prefix(character) {
            self.0 = remaining;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        self.0 = self.0.trim_start_matches(json_whitespace);
    }

    fn take(&mut self, len: usize) -> &'a str {
        let (token, remaining) = self.0.split_at(len);
        self.0 = remaining;
        token
    }
}

fn json_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\n' | '\r' | '\t')
}

fn is_number(token: &str) -> bool {
    matches!(token.as_bytes().first(), Some(b'-' | b'0'..=b'9'))
}

pub(crate) struct JsonWithNumbers<'a> {
    pub value: &'a Value,
    pub numbers: Option<&'a JsonNumbers>,
}

impl Serialize for JsonWithNumbers<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match (self.value, self.numbers) {
            (_, Some(JsonNumbers::Number(raw))) => raw.serialize(serializer),
            (Value::Array(values), Some(JsonNumbers::Array(numbers))) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for (index, value) in values.iter().enumerate() {
                    seq.serialize_element(&Self {
                        value,
                        numbers: numbers.get(&index).map(Arc::as_ref),
                    })?;
                }
                seq.end()
            }
            (Value::Object(values), Some(JsonNumbers::Object(numbers))) => {
                let mut map = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values {
                    map.serialize_entry(
                        key,
                        &Self {
                            value,
                            numbers: numbers.get(key).map(Arc::as_ref),
                        },
                    )?;
                }
                map.end()
            }
            _ => self.value.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(data: &JsonData) -> String {
        serde_json::to_string(&JsonWithNumbers {
            value: &data.value,
            numbers: data.numbers.as_deref(),
        })
        .unwrap()
    }

    #[test]
    fn preserves_number_tokens_in_nested_json() {
        let input = r#"
            { "array": [null, 1000, { "exact": 0.12345678901234567890123456789 }, 1E+99999999999],
              "text": "quote \" and backslash \\ and brackets [,] and é",
              "zero": -0, "scale": 12.000000 }
        "#;
        let parsed = JsonData::parse(input).unwrap();
        let encoded = encode(&parsed);
        let array = parsed.numbers.as_deref().unwrap().field("array").unwrap();
        assert!(array.index(0).is_none());
        assert!(array.index(1).is_none());
        assert_eq!(
            array.index(3).and_then(|value| value.number()),
            Some("1E+99999999999")
        );
        assert_eq!(parsed.value["array"][1], 1000);
        assert!(encoded.contains(r#""exact":0.12345678901234567890123456789"#));
        assert!(encoded.contains(r#""scale":12.000000"#));
        assert!(encoded.contains(r#""zero":-0"#));
        assert!(encoded.contains(r#""text":"quote \" and backslash \\ and brackets [,] and é""#));
        assert_eq!(encode(&JsonData::parse(&encoded).unwrap()), encoded);
    }

    #[test]
    fn keeps_only_the_last_value_for_duplicate_keys() {
        let parsed = JsonData::parse(r#"{"a": 1E9999, "\u0061": false}"#).unwrap();
        assert!(parsed.numbers.is_none());
        assert_eq!(encode(&parsed), r#"{"a":false}"#);

        let parsed = JsonData::parse(r#"{"a": false, "\u0061": 1E9999}"#).unwrap();
        assert_eq!(encode(&parsed), r#"{"a":1E9999}"#);
    }

    #[test]
    fn assembles_json_with_exact_numbers() {
        let number = JsonData::number(" 4.200000000000000000000001 ").unwrap();
        let array = JsonData::array(vec![JsonData::from_value(Value::Null), number]);
        let object = JsonData::object(BTreeMap::from([("x".to_owned(), array)]));
        assert_eq!(
            encode(&object),
            r#"{"x":[null,4.200000000000000000000001]}"#
        );
        assert!(JsonData::number(r#""4.2""#).is_err());
    }

    #[test]
    fn rejects_invalid_json_and_limits_nesting() {
        for invalid in [
            "[1e9999,]",
            r#"{"n": 1e9999,}"#,
            r#"{"n" 1e9999}"#,
            "[01]",
            "[1e+]",
            "1e9999 true",
            r#"["\uD800"]"#,
        ] {
            assert!(JsonData::parse(invalid).is_err(), "{invalid}");
        }
        let nested = |depth| format!("{}1E9999{}", "[".repeat(depth), "]".repeat(depth));
        assert!(JsonData::parse(&nested(MAX_CONTAINER_DEPTH)).is_ok());
        assert!(JsonData::parse(&nested(MAX_CONTAINER_DEPTH + 1)).is_err());
    }
}
