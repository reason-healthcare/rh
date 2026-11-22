//! Source identity carried through navigation, filters, and collection operations.
use super::{typed_value_from_string, FhirModel};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::{EvaluationContext, FhirPathValue as V};
use rh_hl7_fhir_r4_core::metadata::{FhirFieldType, FhirPrimitiveType as P};
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct FhirNode {
    pub inner: V,
    pub model: FhirModel,
    pub fhir_type: String,
    pub raw: Value,
    pub metadata: Option<Value>,
    /// Absolute path within the original resource. Detached nodes have no path.
    pub path: Option<String>,
    pub short_path: Option<String>,
    pub definition: String,
    /// StructureDefinition path relative to the enclosing resource or datatype.
    pub(crate) element_path: String,
    resource_pointer: String,
    external_root: Option<std::sync::Arc<Value>>,
    pointer: String,
}

pub fn primitive_from_json(ty: &str, raw: &Value) -> V {
    if raw.is_null() {
        return V::Empty;
    }
    if ty == "integer64" {
        return raw
            .as_str()
            .and_then(|s| s.parse().ok())
            .map(V::Long)
            .unwrap_or(V::Empty);
    }
    if ty == "decimal" {
        return raw
            .as_number()
            .and_then(|n| {
                let literal = n.to_string();
                Decimal::from_str_exact(&literal)
                    .or_else(|_| Decimal::from_scientific(&literal))
                    .ok()
            })
            .map(V::Number)
            .unwrap_or(V::Empty);
    }
    let primitive = match ty {
        "boolean" => P::Boolean,
        "code" => P::Code,
        "id" => P::Id,
        "canonical" => P::Canonical,
        "uri" | "uuid" => P::Uri,
        "url" => P::Url,
        "oid" => P::Oid,
        "markdown" => P::Markdown,
        "base64Binary" => P::Base64Binary,
        "date" => P::Date,
        "dateTime" => P::DateTime,
        "instant" => P::Instant,
        "time" => P::Time,
        "string" | "xhtml" => P::String,
        _ => return V::from_json(raw),
    };
    match raw {
        Value::String(s) => {
            typed_value_from_string(s.clone(), &FhirFieldType::Primitive(primitive))
        }
        Value::Bool(b) => V::TypedBoolean {
            value: *b,
            fhir_type: P::Boolean,
        },
        _ => V::from_json(raw),
    }
}

impl FhirNode {
    pub fn root(raw: &Value, model: FhirModel, located: bool) -> V {
        let Some(name) = raw.get("resourceType").and_then(Value::as_str) else {
            return V::from_json(raw);
        };
        V::Node(Box::new(Self {
            inner: V::Object(raw.clone()),
            model,
            fhir_type: name.to_owned(),
            raw: raw.clone(),
            metadata: None,
            path: located.then(|| name.to_owned()),
            short_path: located.then(|| name.to_owned()),
            definition: name.to_owned(),
            element_path: name.to_owned(),
            resource_pointer: String::new(),
            external_root: (!located).then(|| std::sync::Arc::new(raw.clone())),
            pointer: String::new(),
        }))
    }

    pub fn detached(raw: &Value, ty: &str, model: FhirModel) -> Self {
        Self {
            inner: V::TypedObject {
                value: raw.clone(),
                fhir_type: ty.to_string(),
            },
            model,
            fhir_type: ty.to_string(),
            raw: raw.clone(),
            metadata: None,
            path: None,
            short_path: None,
            definition: ty.to_owned(),
            element_path: ty.to_owned(),
            pointer: String::new(),
            external_root: None,
            resource_pointer: String::new(),
        }
    }

    /// Construct a FHIR primitive without a source path, including optional id and
    /// extensions, as in the FHIR logical model.
    pub fn primitive(ty: &str, raw: &Value, metadata: Option<Value>, model: FhirModel) -> Self {
        Self {
            inner: primitive_from_json(ty, raw),
            metadata,
            ..Self::detached(raw, ty, model)
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.fhir_type.starts_with(|c: char| c.is_ascii_lowercase())
    }

    pub fn system_value(&self) -> V {
        match &self.inner {
            V::TypedBoolean { value, .. } => V::Boolean(*value),
            V::TypedString { value, .. } => V::String(value.clone()),
            V::TypedDateTime { value, .. } => V::DateTime(value.clone()),
            _ => self.inner.clone(),
        }
    }

    pub(crate) fn is_system_projection(&self) -> bool {
        self.fhir_type.starts_with("System.")
    }

    fn value_child(&self) -> V {
        let inner = self.system_value();
        let Some(name) = inner.system_scalar_name() else {
            return inner;
        };
        if self.path.is_none() {
            return inner;
        }
        V::Node(Box::new(Self {
            fhir_type: format!("System.{name}"),
            raw: inner.to_json(),
            inner,
            metadata: None,
            path: self.path.as_ref().map(|p| format!("{p}.value[0]")),
            short_path: self.short_path.as_ref().map(|p| format!("{p}.value")),
            element_path: format!("{}.value", self.element_path),
            definition: format!("{}.value", self.fhir_type),
            ..self.clone()
        }))
    }

    pub fn member(&self, key: &str, strict: bool) -> FhirPathResult<V> {
        if self
            .raw
            .get("resourceType")
            .and_then(Value::as_str)
            .is_some_and(|rt| self.model.is_subtype(rt, key))
        {
            return Ok(V::Node(Box::new(self.clone())));
        }
        if self.is_primitive() && key == "value" {
            return Ok(self.value_child());
        }
        if strict
            && self.model.is_type(&self.fhir_type)
            && self.model.field_type(&self.fhir_type, key).is_none()
            && self
                .model
                .field_type(&self.fhir_type, &format!("{key}[x]"))
                .is_none()
        {
            return Err(FhirPathError::TypeError {
                message: format!("Unknown member '{key}' of FHIR.{}", self.fhir_type),
            });
        }
        let source = if self.is_primitive() {
            self.metadata.as_ref()
        } else {
            Some(&self.raw)
        };
        let Some(map) = source.and_then(Value::as_object) else {
            return Ok(V::Empty);
        };
        if key.starts_with('_') {
            return Ok(V::Empty);
        }
        let mut found = Vec::new();
        for property in map.keys() {
            let json_key = property.strip_prefix('_').unwrap_or(property);
            if property.starts_with('_') && map.contains_key(json_key) {
                continue;
            }
            let info = self.model.field(&self.fhir_type, json_key);
            let (logical, declared) = info.unwrap_or((json_key, ""));
            if key != logical && key != json_key {
                continue;
            }
            if strict && key == json_key && key != logical {
                return Err(FhirPathError::EvaluationError {
                    message: format!(
                        "Invalid choice-type access: use '{logical}' instead of '{key}'"
                    ),
                });
            }
            let raw = map.get(json_key).unwrap_or(&Value::Null);
            let meta = map.get(&format!("_{json_key}"));
            let array = raw.is_array() || meta.is_some_and(Value::is_array);
            let count = if array {
                raw.as_array()
                    .map_or(0, Vec::len)
                    .max(meta.and_then(Value::as_array).map_or(0, Vec::len))
            } else {
                1
            };
            for i in 0..count {
                let value = if array {
                    raw.get(i).unwrap_or(&Value::Null)
                } else {
                    raw
                };
                let meta = if array {
                    meta.and_then(|v| v.get(i))
                } else {
                    meta
                };
                if value.is_null() && meta.is_none_or(Value::is_null) {
                    continue;
                }
                let Some(mut ty) = (!declared.is_empty()).then_some(declared) else {
                    found.push(if self.model.is_type(&self.fhir_type) {
                        V::from_json(value)
                    } else {
                        super::apply_fhir_typing(value, &self.fhir_type, json_key)
                    });
                    continue;
                };
                let resource = value.get("resourceType").and_then(Value::as_str);
                if let Some(name) = resource {
                    ty = name;
                }
                let inner = if value.is_object() {
                    if resource.is_some() {
                        V::Object(value.clone())
                    } else if self.model.is_subtype(ty, "Quantity")
                        && value.get("comparator").is_none()
                        && value["system"].as_str() == Some("http://unitsofmeasure.org")
                    {
                        match (
                            primitive_from_json("decimal", &value["value"]),
                            value["code"].as_str(),
                        ) {
                            (V::Number(amount), Some(code)) => V::Quantity {
                                value: amount,
                                unit: Some(
                                    match code {
                                        "a" => "year",
                                        "mo" => "month",
                                        "d" => "day",
                                        "h" => "hour",
                                        "min" => "minute",
                                        "s" => "second",
                                        _ => code,
                                    }
                                    .to_owned(),
                                ),
                            },
                            (V::Empty, Some(_)) if value["value"].is_number() => V::Empty,
                            _ => V::TypedObject {
                                value: value.clone(),
                                fhir_type: ty.to_owned(),
                            },
                        }
                    } else {
                        V::TypedObject {
                            value: value.clone(),
                            fhir_type: ty.to_owned(),
                        }
                    }
                } else if ty == "string" && logical == json_key {
                    V::from_json(value)
                } else {
                    primitive_from_json(ty, value)
                };
                let pointer = format!(
                    "{}/{json_key}{}",
                    self.pointer,
                    if array {
                        format!("/{i}")
                    } else {
                        String::new()
                    }
                );
                let element_name = if logical != json_key {
                    format!("{logical}[x]")
                } else {
                    logical.to_owned()
                };
                let owner_path = self
                    .raw
                    .get("resourceType")
                    .and_then(Value::as_str)
                    .unwrap_or(&self.element_path);
                found.push(V::Node(Box::new(Self {
                    inner,
                    model: self.model,
                    fhir_type: ty.to_owned(),
                    raw: value.clone(),
                    metadata: meta.cloned(),
                    path: self.path.as_ref().map(|p| format!("{p}.{logical}[{i}]")),
                    short_path: self.short_path.as_ref().map(|p| {
                        if array {
                            format!("{p}.{logical}[{i}]")
                        } else {
                            format!("{p}.{logical}")
                        }
                    }),
                    definition: format!("{}.{element_name}", self.fhir_type),
                    element_path: format!("{owner_path}.{element_name}"),
                    external_root: self.external_root.clone(),
                    resource_pointer: if resource.is_some() {
                        pointer.clone()
                    } else {
                        self.resource_pointer.clone()
                    },
                    pointer,
                })));
            }
            if array && found.len() == 1 {
                return Ok(V::Collection(found));
            }
        }
        Ok(V::from_items(found))
    }

    pub fn children(&self) -> FhirPathResult<Vec<V>> {
        let mut children = Vec::new();
        if self.is_primitive() && self.inner != V::Empty {
            children.push(self.value_child());
        }
        let source = if self.is_primitive() {
            self.metadata.as_ref()
        } else {
            Some(&self.raw)
        };
        if let Some(map) = source.and_then(Value::as_object) {
            for key in map.keys() {
                let name = key.strip_prefix('_').unwrap_or(key);
                if name == "resourceType" || (key.starts_with('_') && map.contains_key(name)) {
                    continue;
                }
                children.extend_from_slice(V::items(&self.member(name, false)?));
            }
        }
        Ok(children)
    }

    pub(crate) fn containing_resource(&self, context: &EvaluationContext) -> V {
        let root = self.origin_root(context);
        at_pointer(root, &self.resource_pointer).unwrap_or(V::Empty)
    }

    pub(crate) fn origin_root(&self, context: &EvaluationContext) -> V {
        if let Some(raw) = &self.external_root {
            FhirNode::root(raw, self.model, false)
        } else {
            root_node(context)
        }
    }

    /// A contained reference resolves in the nearest enclosing DomainResource,
    /// including references from a contained resource to its siblings.
    pub(crate) fn reference_container(&self, context: &EvaluationContext) -> Option<V> {
        let (parent, index) = self.resource_pointer.rsplit_once("/contained/")?;
        index.parse::<usize>().ok()?;
        at_pointer(self.origin_root(context), parent)
    }

    pub(crate) fn entry_index(&self) -> Option<usize> {
        self.pointer
            .strip_prefix("/entry/")?
            .split('/')
            .next()?
            .parse()
            .ok()
    }
}

fn at_pointer(mut current: V, pointer: &str) -> Option<V> {
    if pointer.is_empty() {
        return Some(current);
    }
    let mut parts = pointer.trim_start_matches('/').split('/').peekable();
    while let Some(key) = parts.next() {
        let V::Node(node) = current else { return None };
        let value = node.member(key, false).ok()?;
        let index = parts.peek().and_then(|s| s.parse::<usize>().ok());
        if index.is_some() {
            parts.next();
        }
        current = V::items(&value).get(index.unwrap_or(0))?.clone();
    }
    Some(current)
}

pub fn root_node(context: &EvaluationContext) -> V {
    FhirNode::root(&context.root, context.model, true)
}
pub fn root_or_current_node(context: &EvaluationContext) -> V {
    if context.root == context.current {
        root_node(context)
    } else {
        FhirNode::root(&context.current, context.model, false)
    }
}

pub fn resource_for_focus(context: &EvaluationContext) -> V {
    if let Some(V::Node(n)) = context.initial_focus.as_deref() {
        n.containing_resource(context)
    } else {
        root_node(context)
    }
}

pub fn coding_equivalent(a: &V, b: &V) -> Option<bool> {
    fn typed(v: &V) -> Option<(&str, &Value)> {
        match v {
            V::Node(n) => Some((&n.fhir_type, &n.raw)),
            V::TypedObject { value, fhir_type } => Some((fhir_type, value)),
            _ => None,
        }
    }
    fn coding(a: &Value, b: &Value) -> bool {
        a.get("system") == b.get("system") && a.get("code") == b.get("code")
    }
    let (at, a) = typed(a)?;
    let (bt, b) = typed(b)?;
    match (at, bt) {
        ("Coding", "Coding") => Some(coding(a, b)),
        ("CodeableConcept", "CodeableConcept") => {
            Some(a["coding"].as_array().into_iter().flatten().any(|x| {
                b["coding"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|y| coding(x, y))
            }))
        }
        _ => None,
    }
}

/// Select nodes at a StructureDefinition element path for element-scoped invariants.
pub fn nodes_at_path(context: &EvaluationContext, path: &str) -> FhirPathResult<Vec<V>> {
    let mut selected = vec![root_node(context)];
    for key in path.split('.').skip(1) {
        let mut next = Vec::new();
        let key = key.strip_suffix("[x]").unwrap_or(key);
        for value in selected {
            if let V::Node(node) = value {
                next.extend_from_slice(V::items(&node.member(key, false)?));
            }
        }
        selected = next;
    }
    Ok(selected)
}

/// Compare FHIR quantities whose unit is expressed in text or a non-UCUM code
/// system. Leave UCUM conversion and comparisons of system values to the core.
pub(crate) fn compare_model_quantities(a: &V, b: &V) -> Option<Option<std::cmp::Ordering>> {
    if let Some(result) = super::magnitude::extreme_numeric_order(a, b) {
        return Some(result);
    }
    fn quantity(v: &V) -> Option<&Value> {
        match v {
            V::Node(node) if node.model.is_subtype(&node.fhir_type, "Quantity") => Some(&node.raw),
            _ => None,
        }
    }
    let (a, b) = (quantity(a)?, quantity(b)?);
    if a.get("comparator").is_some() || b.get("comparator").is_some() {
        return Some(None);
    }
    let (ac, bc) = (a["code"].as_str(), b["code"].as_str());
    if ac.is_some()
        && bc.is_some()
        && a["system"] == "http://unitsofmeasure.org"
        && b["system"] == "http://unitsofmeasure.org"
    {
        return None;
    }
    let comparable = if ac.is_some() && bc.is_some() {
        ac == bc && a["system"] == b["system"]
    } else {
        let (au, bu) = (a["unit"].as_str(), b["unit"].as_str());
        (au.is_some() && au == bu) || (au.is_none() && bu.is_none() && ac.is_none() && bc.is_none())
    };
    if !comparable {
        return Some(None);
    }
    match (
        primitive_from_json("decimal", &a["value"]),
        primitive_from_json("decimal", &b["value"]),
    ) {
        (V::Number(a), V::Number(b)) => Some(Some(a.cmp(&b))),
        _ => Some(None),
    }
}
