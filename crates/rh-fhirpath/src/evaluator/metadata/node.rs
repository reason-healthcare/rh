//! Source identity carried through navigation, filters, and collection operations.
use super::{typed_value_from_string, FhirModel};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::{EvaluationContext, FhirPathValue as V};
use crate::json::{number_text, parse_decimal, JsonData, JsonNumbers};
use rh_hl7_fhir_r4_core::metadata::{FhirFieldType, FhirPrimitiveType as P};
use serde_json::{Map, Value};
use std::{borrow::Cow, sync::Arc};

#[derive(Clone, Debug, PartialEq)]
pub struct FhirNode {
    pub inner: V,
    pub model: FhirModel,
    pub fhir_type: String,
    pub raw: Value,
    pub(crate) numbers: Option<Arc<JsonNumbers>>,
    pub metadata: Option<Value>,
    metadata_numbers: Option<Arc<JsonNumbers>>,
    /// Absolute path within the original resource. Detached nodes have no path.
    pub path: Option<String>,
    pub short_path: Option<String>,
    pub definition: String,
    /// StructureDefinition path relative to the enclosing resource or datatype.
    pub(crate) element_path: String,
    resource_pointer: String,
    external_root: Option<Arc<JsonData>>,
    pointer: String,
}

pub fn primitive_from_json(ty: &str, raw: &Value) -> V {
    primitive_value(ty, raw, None)
}

fn primitive_value(ty: &str, raw: &Value, numbers: Option<&JsonNumbers>) -> V {
    if ty == "decimal" {
        return number_text(raw, numbers)
            .and_then(|literal| parse_decimal(&literal))
            .map(V::Number)
            .unwrap_or(V::Empty);
    }
    if let Some(text) = numbers.and_then(JsonNumbers::number) {
        return V::from_number_text(text);
    }
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

fn typed_object(raw: &Value, ty: &str) -> V {
    V::TypedObject {
        value: raw.clone(),
        fhir_type: ty.to_owned(),
    }
}

fn member_keys(fields: &Map<String, Value>) -> impl Iterator<Item = &str> {
    fields.keys().filter_map(|key| match key.strip_prefix('_') {
        Some(name) if fields.contains_key(name) => None,
        Some(name) => Some(name),
        None => Some(key.as_str()),
    })
}

impl FhirNode {
    fn new(raw: &Value, ty: &str, model: FhirModel, inner: V) -> Self {
        Self {
            inner,
            model,
            fhir_type: ty.to_owned(),
            raw: raw.clone(),
            numbers: None,
            metadata: None,
            metadata_numbers: None,
            path: None,
            short_path: None,
            definition: ty.to_owned(),
            element_path: ty.to_owned(),
            pointer: String::new(),
            external_root: None,
            resource_pointer: String::new(),
        }
    }

    pub fn root(raw: &Value, model: FhirModel, located: bool) -> V {
        Self::root_with_numbers(raw, None, model, located)
    }

    pub(crate) fn root_with_numbers(
        raw: &Value,
        numbers: Option<&Arc<JsonNumbers>>,
        model: FhirModel,
        located: bool,
    ) -> V {
        let Some(name) = raw.get("resourceType").and_then(Value::as_str) else {
            return V::from_json_with_numbers(raw, numbers, model);
        };
        let mut node = Self::new(raw, name, model, V::Object(raw.clone()));
        node.numbers = numbers.cloned();
        if located {
            node.path = Some(name.to_owned());
            node.short_path = Some(name.to_owned());
        } else {
            node.external_root = Some(Arc::new(JsonData {
                value: raw.clone(),
                numbers: numbers.cloned(),
            }));
        }
        V::Node(Box::new(node))
    }

    pub fn detached(raw: &Value, ty: &str, model: FhirModel) -> Self {
        Self::new(raw, ty, model, typed_object(raw, ty))
    }

    pub(crate) fn detached_json(raw: &JsonData, ty: &str, model: FhirModel) -> Self {
        Self {
            numbers: raw.numbers.clone(),
            ..Self::detached(&raw.value, ty, model)
        }
    }

    pub(crate) fn untyped_json(
        raw: &Value,
        numbers: &Arc<JsonNumbers>,
        model: FhirModel,
        inner: V,
    ) -> V {
        let ty = if numbers.number().is_some() {
            match inner.system_scalar_name() {
                Some("Integer") => "System.Integer",
                Some("Long") => "System.Long",
                _ => "System.Decimal",
            }
        } else {
            ""
        };
        V::Node(Box::new(Self {
            numbers: Some(Arc::clone(numbers)),
            ..Self::new(raw, ty, model, inner)
        }))
    }

    pub(crate) fn is_untyped_object(&self) -> bool {
        self.fhir_type.is_empty()
    }

    /// Construct a FHIR primitive without a source path, including optional id and
    /// extensions, as in the FHIR logical model.
    pub fn primitive(ty: &str, raw: &Value, metadata: Option<Value>, model: FhirModel) -> Self {
        Self {
            metadata,
            ..Self::new(raw, ty, model, primitive_from_json(ty, raw))
        }
    }

    pub(crate) fn primitive_json(
        ty: &str,
        raw: &JsonData,
        metadata: Option<JsonData>,
        model: FhirModel,
    ) -> Self {
        let (metadata, metadata_numbers) =
            metadata.map_or((None, None), |data| (Some(data.value), data.numbers));
        Self {
            numbers: raw.numbers.clone(),
            metadata,
            metadata_numbers,
            ..Self::new(
                &raw.value,
                ty,
                model,
                primitive_value(ty, &raw.value, raw.numbers.as_deref()),
            )
        }
    }

    pub(crate) fn has_raw_value(&self) -> bool {
        !self.raw.is_null()
            || self
                .numbers
                .as_deref()
                .and_then(JsonNumbers::number)
                .is_some()
    }

    pub(crate) fn number_text(&self) -> Option<Cow<'_, str>> {
        number_text(&self.raw, self.numbers.as_deref())
    }

    pub(crate) fn field_number_text(&self, key: &str) -> Option<Cow<'_, str>> {
        number_text(self.raw.get(key)?, self.field_numbers(key))
    }

    fn field_numbers(&self, key: &str) -> Option<&JsonNumbers> {
        self.numbers.as_ref()?.field(key).map(Arc::as_ref)
    }

    pub(crate) fn preserves_exact_output(&self) -> bool {
        self.numbers.is_some()
            && (matches!(self.inner, V::Object(_) | V::TypedObject { .. })
                || self.extreme_numeric().is_some())
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

    pub(crate) fn matches_decimal_projection(&self, lower_name: &str) -> bool {
        self.fhir_type == "System.Decimal" && matches!(lower_name, "any" | "decimal" | "number")
    }

    fn value_child(&self) -> V {
        let inner = self.system_value();
        let extreme_decimal = self.fhir_type == "decimal" && self.extreme_numeric().is_some();
        let Some(name) = inner
            .system_scalar_name()
            .or(extreme_decimal.then_some("Decimal"))
        else {
            return inner;
        };
        if self.path.is_none() && !extreme_decimal {
            return inner;
        }
        V::Node(Box::new(Self {
            fhir_type: format!("System.{name}"),
            raw: if self.fhir_type == "decimal" {
                self.raw.clone()
            } else {
                inner.to_json()
            },
            numbers: (self.fhir_type == "decimal")
                .then(|| self.numbers.clone())
                .flatten(),
            inner,
            metadata: None,
            metadata_numbers: None,
            path: self.path.as_ref().map(|p| format!("{p}.value[0]")),
            short_path: self.short_path.as_ref().map(|p| format!("{p}.value")),
            element_path: format!("{}.value", self.element_path),
            definition: format!("{}.value", self.fhir_type),
            model: self.model,
            resource_pointer: self.resource_pointer.clone(),
            external_root: self.external_root.clone(),
            pointer: self.pointer.clone(),
        }))
    }

    pub(crate) fn primitive_system_value(&self) -> V {
        if self.fhir_type == "decimal" && self.extreme_numeric().is_some() {
            self.value_child()
        } else {
            self.system_value()
        }
    }

    fn source_fields(&self) -> Option<(&Map<String, Value>, Option<&JsonNumbers>)> {
        if self.is_primitive() {
            Some((
                self.metadata.as_ref()?.as_object()?,
                self.metadata_numbers.as_deref(),
            ))
        } else {
            Some((self.raw.as_object()?, self.numbers.as_deref()))
        }
    }

    fn member_value(
        &self,
        value: &Value,
        ty: &str,
        choice: bool,
        numbers: Option<&Arc<JsonNumbers>>,
    ) -> V {
        if !value.is_object() {
            return if ty == "string" && !choice {
                V::from_json_with_numbers(value, numbers, self.model)
            } else {
                primitive_value(ty, value, numbers.map(Arc::as_ref))
            };
        }
        if self.model.is_subtype(ty, "Quantity")
            && value.get("comparator").is_none()
            && value["system"] == "http://unitsofmeasure.org"
        {
            let amount_numbers = numbers.and_then(|n| n.field("value")).map(Arc::as_ref);
            let amount = &value["value"];
            match (
                primitive_value("decimal", amount, amount_numbers),
                value["code"].as_str(),
            ) {
                (V::Number(amount), Some(code)) => {
                    let unit = match code {
                        "a" => "year",
                        "mo" => "month",
                        "d" => "day",
                        "h" => "hour",
                        "min" => "minute",
                        "s" => "second",
                        _ => code,
                    };
                    return V::Quantity {
                        value: amount,
                        unit: Some(unit.to_owned()),
                    };
                }
                (V::Empty, Some(_)) if amount.is_number() || amount_numbers.is_some() => {
                    return V::Empty
                }
                _ => (),
            }
        }
        typed_object(value, ty)
    }

    pub fn member(&self, key: &str, strict: bool) -> FhirPathResult<V> {
        let owner_type = self.raw["resourceType"].as_str();
        if owner_type.is_some_and(|ty| self.model.is_subtype(ty, key)) {
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
        let Some((map, source_numbers)) = self.source_fields() else {
            return Ok(V::Empty);
        };
        if key.starts_with('_') {
            return Ok(V::Empty);
        }
        let owner_path = owner_type.unwrap_or(&self.element_path);
        let mut found = Vec::new();
        for json_key in member_keys(map) {
            let info = self.model.field(&self.fhir_type, json_key);
            let (logical, declared) = info.unwrap_or((json_key, ""));
            if key != logical && key != json_key {
                continue;
            }
            let choice = logical != json_key;
            if strict && key == json_key && choice {
                return Err(FhirPathError::EvaluationError {
                    message: format!(
                        "Invalid choice-type access: use '{logical}' instead of '{key}'"
                    ),
                });
            }
            let raw = map.get(json_key).unwrap_or(&Value::Null);
            let raw_numbers = source_numbers.and_then(|numbers| numbers.field(json_key));
            let meta_key = format!("_{json_key}");
            let meta = map.get(&meta_key);
            let meta_numbers = source_numbers.and_then(|numbers| numbers.field(&meta_key));
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
                let numbers = if array {
                    raw_numbers.and_then(|numbers| numbers.index(i))
                } else {
                    raw_numbers
                };
                let meta = if array {
                    meta.and_then(|v| v.get(i))
                } else {
                    meta
                };
                let meta_numbers = if array {
                    meta_numbers.and_then(|numbers| numbers.index(i))
                } else {
                    meta_numbers
                };
                if value.is_null() && numbers.is_none() && meta.is_none_or(Value::is_null) {
                    continue;
                }
                let Some(ty) = (!declared.is_empty()).then_some(declared) else {
                    found.push(
                        if numbers.is_some() || self.model.is_type(&self.fhir_type) {
                            V::from_json_with_numbers(value, numbers, self.model)
                        } else {
                            super::apply_fhir_typing(value, &self.fhir_type, json_key)
                        },
                    );
                    continue;
                };
                let resource = value.get("resourceType").and_then(Value::as_str);
                let ty = resource.unwrap_or(ty);
                let inner = if resource.is_some() {
                    V::Object(value.clone())
                } else {
                    self.member_value(value, ty, choice, numbers)
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
                let element_name = if choice {
                    format!("{logical}[x]")
                } else {
                    logical.to_owned()
                };
                found.push(V::Node(Box::new(Self {
                    inner,
                    model: self.model,
                    fhir_type: ty.to_owned(),
                    raw: value.clone(),
                    numbers: numbers.cloned(),
                    metadata: meta.cloned(),
                    metadata_numbers: meta_numbers.cloned(),
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
        if self.is_untyped_object() {
            return Ok(crate::evaluator::CollectionEvaluator::json_children(
                &self.raw,
                self.numbers.as_deref(),
                self.model,
            ));
        }
        let mut children = Vec::new();
        if self.is_primitive() {
            let value = self.value_child();
            if !value.is_empty() {
                children.push(value);
            }
        }
        if let Some((fields, _)) = self.source_fields() {
            for name in member_keys(fields).filter(|&name| name != "resourceType") {
                children.extend(self.member(name, false)?.into_items());
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
            FhirNode::root_with_numbers(&raw.value, raw.numbers.as_ref(), self.model, false)
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
    FhirNode::root_with_numbers(&context.root, context.root_numbers(), context.model, true)
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
                next.extend(node.member(key, false)?.into_items());
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
    fn quantity(v: &V) -> Option<&FhirNode> {
        match v {
            V::Node(node) if node.model.is_subtype(&node.fhir_type, "Quantity") => Some(node),
            _ => None,
        }
    }
    let (left, right) = (quantity(a)?, quantity(b)?);
    let (a, b) = (&left.raw, &right.raw);
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
        primitive_value("decimal", &a["value"], left.field_numbers("value")),
        primitive_value("decimal", &b["value"], right.field_numbers("value")),
    ) {
        (V::Number(a), V::Number(b)) => Some(Some(a.cmp(&b))),
        _ => Some(None),
    }
}
