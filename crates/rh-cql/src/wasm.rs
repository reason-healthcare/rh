//! WASM bindings for CQL compilation and evaluation.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::eval::value::{CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};
use crate::{
    compile_to_elm_with_sourcemap, compile_to_json, evaluate_elm, CompilerOptions,
    EvalContextBuilder, FixedClock, InMemoryDataProvider,
};

pub use rh_foundation::wasm::WasmResult;

#[wasm_bindgen(start)]
pub fn init() {
    rh_foundation::wasm::init_panic_hook();
}

/// Compile options for CQL WASM calls.
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct CompileOptions {
    pretty: bool,
    source_map: bool,
}

#[wasm_bindgen]
impl CompileOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            pretty: true,
            source_map: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn pretty(&self) -> bool {
        self.pretty
    }

    #[wasm_bindgen(setter)]
    pub fn set_pretty(&mut self, pretty: bool) {
        self.pretty = pretty;
    }

    #[wasm_bindgen(getter)]
    pub fn source_map(&self) -> bool {
        self.source_map
    }

    #[wasm_bindgen(setter)]
    pub fn set_source_map(&mut self, source_map: bool) {
        self.source_map = source_map;
    }
}

/// Evaluation options for CQL WASM calls.
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct EvaluateOptions {
    expression: String,
    data_json: String,
    parameters_json: String,
}

#[wasm_bindgen]
impl EvaluateOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(expression: String) -> Self {
        Self {
            expression,
            data_json: "{}".to_string(),
            parameters_json: "{}".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> String {
        self.expression.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expression(&mut self, expression: String) {
        self.expression = expression;
    }

    #[wasm_bindgen(getter)]
    pub fn data_json(&self) -> String {
        self.data_json.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data_json(&mut self, data_json: String) {
        self.data_json = data_json;
    }

    #[wasm_bindgen(getter)]
    pub fn parameters_json(&self) -> String {
        self.parameters_json.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_parameters_json(&mut self, parameters_json: String) {
        self.parameters_json = parameters_json;
    }
}

#[wasm_bindgen]
pub fn compile_cql(source: &str, options: &CompileOptions) -> WasmResult {
    if options.source_map {
        match compile_to_elm_with_sourcemap(source, Some(CompilerOptions::default()), None) {
            Ok(result) => {
                let response = json!({
                    "library": result.library,
                    "sourceMap": result.source_map,
                    "errors": result.errors.iter().map(diagnostic_to_json).collect::<Vec<_>>(),
                    "warnings": result.warnings.iter().map(diagnostic_to_json).collect::<Vec<_>>(),
                    "messages": result.messages.iter().map(diagnostic_to_json).collect::<Vec<_>>(),
                    "success": result.is_success()
                });
                json_result(response, options.pretty)
            }
            Err(err) => WasmResult::err(format!("Failed to compile CQL: {err}")),
        }
    } else {
        match compile_to_json(source, Some(CompilerOptions::default()), options.pretty) {
            Ok(elm_json) => WasmResult::ok(elm_json),
            Err(err) => WasmResult::err(format!("Failed to compile CQL: {err}")),
        }
    }
}

#[wasm_bindgen]
pub fn evaluate_cql_elm(elm_json: &str, options: &EvaluateOptions) -> WasmResult {
    let library = match serde_json::from_str(elm_json) {
        Ok(library) => library,
        Err(err) => return WasmResult::err(format!("Failed to parse ELM JSON: {err}")),
    };

    let data_provider = match data_provider_from_json(&options.data_json) {
        Ok(provider) => provider,
        Err(err) => return WasmResult::err(err),
    };

    let parameters = match parameters_from_json(&options.parameters_json) {
        Ok(parameters) => parameters,
        Err(err) => return WasmResult::err(err),
    };

    let ctx = EvalContextBuilder::new(default_clock())
        .parameters(parameters)
        .data_provider(data_provider)
        .build();

    match evaluate_elm(&library, &options.expression, &ctx) {
        Ok(value) => json_result(
            json!({
                "expression": options.expression,
                "result": value_to_json(&value),
                "type": value_type(&value)
            }),
            true,
        ),
        Err(err) => WasmResult::err(format!("Failed to evaluate CQL ELM: {err}")),
    }
}

#[wasm_bindgen]
pub fn compile_cql_simple(source: &str) -> WasmResult {
    let options = CompileOptions::new();
    compile_cql(source, &options)
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn json_result(value: serde_json::Value, pretty: bool) -> WasmResult {
    let result = if pretty {
        serde_json::to_string_pretty(&value)
    } else {
        serde_json::to_string(&value)
    };
    match result {
        Ok(json) => WasmResult::ok(json),
        Err(err) => WasmResult::err(format!("Failed to serialize JSON: {err}")),
    }
}

fn diagnostic_to_json(diagnostic: &crate::Diagnostic) -> serde_json::Value {
    json!({
        "severity": format!("{:?}", diagnostic.severity),
        "code": format!("{:?}", diagnostic.code),
        "message": diagnostic.message,
        "span": diagnostic.span.as_ref().map(|span| {
            json!({
                "start": span.start,
                "end": span.end
            })
        }),
        "stage": format!("{:?}", diagnostic.stage)
    })
}

fn default_clock() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2024,
        month: Some(1),
        day: Some(1),
        hour: Some(0),
        minute: Some(0),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0),
    })
}

fn data_provider_from_json(data_json: &str) -> Result<InMemoryDataProvider, String> {
    let data: serde_json::Value = serde_json::from_str(data_json)
        .map_err(|err| format!("Failed to parse data JSON: {err}"))?;
    let mut provider = InMemoryDataProvider::new();

    let serde_json::Value::Object(map) = data else {
        return Err("Data JSON must be an object keyed by CQL data type".to_string());
    };

    for (data_type, resources) in map {
        match resources {
            serde_json::Value::Array(items) => {
                for item in items {
                    provider.add_resource(data_type.clone(), json_to_value(item));
                }
            }
            item => provider.add_resource(data_type, json_to_value(item)),
        }
    }

    Ok(provider)
}

fn parameters_from_json(parameters_json: &str) -> Result<HashMap<String, Value>, String> {
    let parameters: serde_json::Value = serde_json::from_str(parameters_json)
        .map_err(|err| format!("Failed to parse parameters JSON: {err}"))?;

    let serde_json::Value::Object(map) = parameters else {
        return Err("Parameters JSON must be an object".to_string());
    };

    Ok(map
        .into_iter()
        .map(|(key, value)| (key, json_to_value(value)))
        .collect())
}

fn json_to_value(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(value) => Value::Boolean(value),
        serde_json::Value::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Value::Integer(integer)
            } else {
                Value::Decimal(value.as_f64().unwrap_or_default())
            }
        }
        serde_json::Value::String(value) => Value::String(value),
        serde_json::Value::Array(items) => {
            Value::List(items.into_iter().map(json_to_value).collect())
        }
        serde_json::Value::Object(map) => Value::Tuple(
            map.into_iter()
                .map(|(key, value)| (key, json_to_value(value)))
                .collect(),
        ),
    }
}

fn value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Boolean(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Long(_) => "long",
        Value::Decimal(_) => "decimal",
        Value::String(_) => "string",
        Value::Date(_) => "date",
        Value::DateTime(_) => "datetime",
        Value::Time(_) => "time",
        Value::Quantity(_) => "quantity",
        Value::Ratio { .. } => "ratio",
        Value::Code(_) => "code",
        Value::Concept(_) => "concept",
        Value::List(_) => "list",
        Value::Tuple(_) => "tuple",
        Value::Interval { .. } => "interval",
    }
}

fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(value) => json!(value),
        Value::Integer(value) => json!(value),
        Value::Long(value) => json!(value.to_string()),
        Value::Decimal(value) => json!(value),
        Value::String(value) => json!(value),
        Value::Date(value) => cql_date_to_json(value),
        Value::DateTime(value) => cql_datetime_to_json(value),
        Value::Time(value) => cql_time_to_json(value),
        Value::Quantity(value) => cql_quantity_to_json(value),
        Value::Ratio {
            numerator,
            denominator,
        } => json!({
            "numerator": cql_quantity_to_json(numerator),
            "denominator": cql_quantity_to_json(denominator)
        }),
        Value::Code(value) => cql_code_to_json(value),
        Value::Concept(value) => cql_concept_to_json(value),
        Value::List(items) => serde_json::Value::Array(items.iter().map(value_to_json).collect()),
        Value::Tuple(fields) => serde_json::Value::Object(
            fields
                .iter()
                .map(|(key, value)| (key.clone(), value_to_json(value)))
                .collect(),
        ),
        Value::Interval {
            low,
            high,
            low_closed,
            high_closed,
        } => json!({
            "low": low.as_deref().map(value_to_json),
            "high": high.as_deref().map(value_to_json),
            "lowClosed": low_closed,
            "highClosed": high_closed
        }),
    }
}

fn cql_date_to_json(value: &CqlDate) -> serde_json::Value {
    json!({
        "year": value.year,
        "month": value.month,
        "day": value.day
    })
}

fn cql_datetime_to_json(value: &CqlDateTime) -> serde_json::Value {
    json!({
        "year": value.year,
        "month": value.month,
        "day": value.day,
        "hour": value.hour,
        "minute": value.minute,
        "second": value.second,
        "millisecond": value.millisecond,
        "offsetSeconds": value.offset_seconds
    })
}

fn cql_time_to_json(value: &CqlTime) -> serde_json::Value {
    json!({
        "hour": value.hour,
        "minute": value.minute,
        "second": value.second,
        "millisecond": value.millisecond
    })
}

fn cql_quantity_to_json(value: &CqlQuantity) -> serde_json::Value {
    json!({
        "value": value.value,
        "unit": value.unit
    })
}

fn cql_code_to_json(value: &CqlCode) -> serde_json::Value {
    json!({
        "code": value.code,
        "system": value.system,
        "display": value.display,
        "version": value.version
    })
}

fn cql_concept_to_json(value: &CqlConcept) -> serde_json::Value {
    json!({
        "codes": value.codes.iter().map(cql_code_to_json).collect::<Vec<_>>(),
        "display": value.display
    })
}
