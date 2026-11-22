//! Context-dependent FHIR functions. Hosts opt in to external services explicitly.
use crate::error::{FhirPathError, FhirPathResult as Result};
use crate::evaluator::metadata::{root_node, FhirModel, FhirNode};
use crate::evaluator::{CollectionEvaluator, EvaluationContext, FhirPathValue as V};
use serde_json::Value;
use std::borrow::Cow;

/// Host callbacks return None when a reference, profile, or terminology operation
/// cannot be resolved. The evaluator applies the selected release's result rules.
pub trait FhirHost: std::fmt::Debug + Send + Sync {
    fn resolve(&self, _reference: &str) -> Result<Option<Value>> {
        Ok(None)
    }
    fn conforms_to(
        &self,
        _element: &Value,
        _declared_type: Option<&str>,
        _profile: &str,
    ) -> Result<Option<bool>> {
        Ok(None)
    }
    fn member_of(&self, _element: &Value, _value_set: &str) -> Result<Option<bool>> {
        Ok(None)
    }
    fn subsumes(&self, _parent: &Value, _child: &Value) -> Result<Option<bool>> {
        Ok(None)
    }
    fn element_definition(&self, _path: &str) -> Result<Option<Value>> {
        Ok(None)
    }
    /// Candidate StructureDefinition paths are supplied in resource-relative and
    /// datatype-relative form, without collection indexes.
    fn matches_slice(
        &self,
        _element: &Value,
        _source_paths: &[&str],
        _structure: &str,
        _name: &str,
    ) -> Result<Option<bool>> {
        Ok(None)
    }
}

pub(crate) fn handles(name: &str) -> bool {
    matches!(
        name,
        "resolve"
            | "conformsTo"
            | "memberOf"
            | "subsumes"
            | "subsumedBy"
            | "elementDefinition"
            | "slice"
            | "checkModifiers"
            | "htmlChecks"
            | "hasValue"
            | "getValue"
            | "extension"
            | "pathname"
            | "children"
            | "descendants"
    )
}

fn error(message: impl Into<String>) -> FhirPathError {
    FhirPathError::EvaluationError {
        message: message.into(),
    }
}

fn unresolved(name: &str, context: &EvaluationContext) -> Result<V> {
    if context.model == FhirModel::R5 {
        Ok(V::Empty)
    } else {
        Err(error(format!(
            "{name}(): unresolved argument or unavailable service"
        )))
    }
}

fn raw_json(value: &V) -> Cow<'_, Value> {
    match value {
        V::Node(node) => Cow::Borrowed(&node.raw),
        V::Object(raw) | V::TypedObject { value: raw, .. } => Cow::Borrowed(raw),
        _ => Cow::Owned(value.to_json()),
    }
}

fn navigate(value: &V, name: &str, model: FhirModel) -> Result<V> {
    match value {
        V::Node(n) => n.member(name, false),
        V::TypedObject { value, fhir_type } => {
            FhirNode::detached(value, fhir_type, model).member(name, false)
        }
        V::Object(obj) => Ok(obj.get(name).map(V::from_json).unwrap_or(V::Empty)),
        V::FhirPrimitive { extensions, .. } => {
            Ok(extensions.get(name).map(V::from_json).unwrap_or(V::Empty))
        }
        _ => Ok(V::Empty),
    }
}

fn local_reference(reference: &str, item: &V, context: &EvaluationContext) -> Result<Option<V>> {
    let node = match item {
        V::Node(n) => Some(n.as_ref()),
        _ => None,
    };
    let root = node.map_or_else(|| root_node(context), |n| n.origin_root(context));
    if let Some(id) = reference.strip_prefix('#') {
        let parent = node.and_then(|n| n.reference_container(context));
        if id.is_empty() {
            return Ok(parent);
        }
        let container = parent.unwrap_or_else(|| {
            node.map_or_else(|| root.clone(), |n| n.containing_resource(context))
        });
        return Ok(navigate(&container, "contained", context.model)?
            .into_items()
            .into_iter()
            .find(|candidate| raw_json(candidate)["id"].as_str() == Some(id)));
    }
    let raw_root = if let V::Node(n) = &root {
        &n.raw
    } else {
        &context.root
    };
    if raw_root["resourceType"] != "Bundle" {
        return Ok(None);
    }
    let (url, version) = reference
        .split_once("/_history/")
        .map_or((reference, None), |(url, v)| (url, Some(v)));
    let (url, canonical_version) = url
        .split_once('|')
        .map_or((url, None), |(url, v)| (url, Some(v)));
    let absolute = node
        .and_then(FhirNode::entry_index)
        .and_then(|i| raw_root["entry"][i]["fullUrl"].as_str())
        .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
        .filter(|_| !url.contains(':'))
        .and_then(|s| s.rsplit_once('/')?.0.rsplit_once('/'))
        .map(|(server, _)| format!("{server}/{url}"));
    let mut found = None;
    for entry in V::items(&navigate(&root, "entry", context.model)?) {
        let resource = navigate(entry, "resource", context.model)?;
        let V::Node(n) = &resource else { continue };
        let raw = &n.raw;
        if version.is_some_and(|v| raw["meta"]["versionId"] != v)
            || canonical_version.is_some_and(|v| raw["version"] != v)
        {
            continue;
        }
        let physical = raw_json(entry);
        let full_url = physical["fullUrl"].as_str();
        let relative = absolute.is_none()
            && raw["resourceType"]
                .as_str()
                .zip(raw["id"].as_str())
                .is_some_and(|(ty, id)| url == format!("{ty}/{id}"));
        if full_url == Some(absolute.as_deref().unwrap_or(url))
            || relative
            || raw["url"].as_str() == Some(url)
        {
            if found.is_some() {
                return Ok(None);
            }
            found = Some(resource);
        }
    }
    Ok(found)
}

fn children(value: &V) -> Result<Vec<V>> {
    match value {
        V::Node(n) => n.children(),
        _ => Ok(CollectionEvaluator::children(value)?.into_items()),
    }
}

pub(crate) fn invoke(name: &str, target: &V, params: &[V], ctx: &EvaluationContext) -> Result<V> {
    let host = ctx.host.as_deref();
    let values = target.items();
    let arity = match name {
        "extension" | "memberOf" | "subsumes" | "subsumedBy" | "conformsTo" => 1..=1,
        "slice" => 2..=2,
        "checkModifiers" if ctx.model == FhirModel::R5 => 1..=1,
        "checkModifiers" => 0..=usize::MAX,
        "pathname" => 0..=1,
        _ => 0..=0,
    };
    if !arity.contains(&params.len()) {
        return Err(error(format!("Invalid argument count for {name}()")));
    }
    let string_arg = |i: usize| -> Result<Option<&str>> {
        match params.get(i).map(V::items).unwrap_or(&[]) {
            [] => Ok(None),
            [v] => v
                .as_str()
                .map(Some)
                .ok_or_else(|| error(format!("{name} expects a String"))),
            _ => Err(error(format!("{name} expects a singleton String"))),
        }
    };
    match name {
        "children" | "descendants" => {
            let mut result = Vec::new();
            for value in values {
                result.extend(children(value)?);
            }
            if name == "descendants" {
                let mut i = 0;
                while i < result.len() {
                    let next = children(&result[i])?;
                    result.extend(next);
                    i += 1;
                }
            }
            Ok(if result.is_empty() {
                V::Empty
            } else {
                V::UnorderedCollection(result)
            })
        }
        "pathname" => {
            let short = match params.first().map(V::items) {
                None => false,
                Some([V::Boolean(b)]) => *b,
                _ => return Err(error("pathname expects a Boolean")),
            };
            Ok(V::from_items(
                values
                    .iter()
                    .filter_map(|value| {
                        let V::Node(node) = value else { return None };
                        if short {
                            node.short_path.clone()
                        } else {
                            node.path.clone()
                        }
                    })
                    .map(V::String)
                    .collect(),
            ))
        }
        "extension" => {
            let Some(url) = string_arg(0)? else {
                return Ok(V::Empty);
            };
            let mut result = Vec::new();
            for v in values {
                let extensions = navigate(v, "extension", ctx.model)?.into_items();
                result.extend(
                    extensions
                        .into_iter()
                        .filter(|ext| raw_json(ext)["url"].as_str() == Some(url)),
                );
            }
            Ok(V::Collection(result))
        }
        "hasValue" | "getValue" => {
            let primitive = match values {
                [V::Node(node)] if node.is_primitive() => Some(node.as_ref()),
                _ => None,
            };
            if name == "hasValue" {
                let has_value = primitive.is_some_and(|node| !node.raw.is_null());
                Ok(if has_value || ctx.model == FhirModel::R4 {
                    V::Boolean(has_value)
                } else {
                    V::Empty
                })
            } else {
                Ok(primitive.map_or(V::Empty, FhirNode::system_value))
            }
        }
        "resolve" => {
            let mut result = Vec::new();
            for value in values {
                let raw = raw_json(value);
                let Some(reference) = raw.as_str().or_else(|| raw["reference"].as_str()) else {
                    continue;
                };
                if let Some(resource) = local_reference(reference, value, ctx)? {
                    result.push(resource);
                    continue;
                }
                let Some(host) = host else { continue };
                if let Some(raw) = host.resolve(reference)? {
                    result.push(FhirNode::root(&raw, ctx.model, false));
                }
            }
            Ok(V::from_items(result))
        }
        "elementDefinition" => {
            let Some(host) = host else {
                return Ok(V::Empty);
            };
            let mut result = Vec::new();
            for value in values {
                let V::Node(node) = value else { continue };
                if let Some(def) = host.element_definition(&node.definition)? {
                    result.push(V::TypedObject {
                        value: def,
                        fhir_type: "ElementDefinition".into(),
                    });
                }
            }
            Ok(V::from_items(result))
        }
        "slice" => {
            let (Some(url), Some(slice), Some(host)) = (string_arg(0)?, string_arg(1)?, host)
            else {
                return Ok(V::Empty);
            };
            let mut result = Vec::new();
            for value in values {
                let source_paths: &[&str] = match value {
                    V::Node(n) => &[&n.element_path, &n.definition],
                    _ => &[],
                };
                if host.matches_slice(&raw_json(value), source_paths, url, slice)? == Some(true) {
                    result.push(value.clone());
                }
            }
            Ok(V::from_items(result))
        }
        "checkModifiers" => {
            let mut allowed = Vec::new();
            for i in 0..params.len() {
                if let Some(arg) = string_arg(i)? {
                    allowed.extend(arg.split(',').map(str::trim).filter(|s| !s.is_empty()));
                }
            }
            if ctx.model == FhirModel::R5 && allowed.is_empty() {
                return Err(error("checkModifiers requires allowed modifier URLs"));
            }
            for value in values {
                for ext in V::items(&navigate(value, "modifierExtension", ctx.model)?) {
                    if !raw_json(ext)["url"]
                        .as_str()
                        .is_some_and(|url| allowed.contains(&url))
                    {
                        return Err(error("Unrecognized modifierExtension"));
                    }
                }
            }
            Ok(target.clone())
        }
        "htmlChecks" => Ok(match values {
            [V::Node(n)] if n.fhir_type == "xhtml" => n
                .raw
                .as_str()
                .map(|s| V::Boolean(super::xhtml::valid(s)))
                .unwrap_or(V::Empty),
            _ => V::Empty,
        }),
        "conformsTo" | "memberOf" => {
            let [element] = values else {
                return if values.is_empty() {
                    Ok(V::Empty)
                } else {
                    unresolved(name, ctx)
                };
            };
            let Some(url) = string_arg(0)? else {
                return unresolved(name, ctx);
            };
            let Some(host) = host else {
                return unresolved(name, ctx);
            };
            let raw = raw_json(element);
            let result = if name == "conformsTo" {
                let declared_type = match element {
                    V::Node(n) => Some(n.fhir_type.as_str()),
                    V::TypedObject { fhir_type, .. } => Some(fhir_type.as_str()),
                    _ => None,
                };
                host.conforms_to(&raw, declared_type, url)?
            } else {
                host.member_of(&raw, url)?
            };
            match result {
                Some(b) => Ok(V::Boolean(b)),
                None => unresolved(name, ctx),
            }
        }
        "subsumes" | "subsumedBy" => {
            let ([lhs], [rhs]) = (values, V::items(&params[0])) else {
                return Ok(V::Empty);
            };
            let (l, r) = (raw_json(lhs), raw_json(rhs));
            let (parent, child) = if name == "subsumes" {
                (&l, &r)
            } else {
                (&r, &l)
            };
            fn codings(v: &Value) -> &[Value] {
                if let Some(a) = v["coding"].as_array() {
                    a.as_slice()
                } else if v["code"].is_string() {
                    std::slice::from_ref(v)
                } else {
                    &[]
                }
            }
            let (parents, children) = (codings(parent), codings(child));
            if parents.is_empty() || children.is_empty() {
                return Ok(V::Empty);
            }
            let mut unknown = false;
            let mut cross_system_unknown = false;
            for a in parents {
                for b in children {
                    if a["system"].is_string()
                        && a["system"] == b["system"]
                        && a["code"] == b["code"]
                    {
                        return Ok(V::Boolean(true));
                    }
                    match host.map(|h| h.subsumes(a, b)).transpose()?.flatten() {
                        Some(true) => return Ok(V::Boolean(true)),
                        Some(false) => (),
                        None => {
                            unknown = true;
                            cross_system_unknown |= a["system"] != b["system"];
                        }
                    }
                }
            }
            if unknown {
                if name == "subsumedBy" && cross_system_unknown {
                    return Err(error("Unresolved cross-system subsumption"));
                }
                unresolved(name, ctx)
            } else {
                Ok(V::Boolean(false))
            }
        }
        _ => unreachable!(),
    }
}
