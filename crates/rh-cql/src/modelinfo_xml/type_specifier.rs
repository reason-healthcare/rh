//! Parsing of `TypeSpecifier` variants from ModelInfo XML.
//!
//! Each `parse_*` function corresponds to a concrete `TypeSpecifier` subtype
//! discriminated by the `xsi:type` attribute on the enclosing element.

use super::util::{get_xsi_type, parse_tuple_element_attrs, skip_element, TupleElementDef};
use crate::modelinfo::{
    ChoiceTypeSpecifier, IntervalTypeSpecifier, ListTypeSpecifier, NamedTypeSpecifier,
    TupleTypeSpecifier, TypeSpecifier,
};
use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::io::BufRead;

/// Parse a container element whose `xsi:type` identifies a `TypeSpecifier`
/// variant.  Expects to consume through the matching closing tag.
pub(super) fn parse_type_specifier_element<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<Option<TypeSpecifier>> {
    let xsi_type = get_xsi_type(e);

    match xsi_type.as_deref() {
        Some("NamedTypeSpecifier") => {
            let spec = parse_named_type_specifier_attrs(e)?;
            skip_element(reader)?;
            Ok(Some(TypeSpecifier::NamedTypeSpecifier(spec)))
        }
        Some("ListTypeSpecifier") => {
            let spec = parse_list_type_specifier(reader, e)?;
            Ok(Some(TypeSpecifier::ListTypeSpecifier(spec)))
        }
        Some("IntervalTypeSpecifier") => {
            let spec = parse_interval_type_specifier(reader, e)?;
            Ok(Some(TypeSpecifier::IntervalTypeSpecifier(spec)))
        }
        Some("ChoiceTypeSpecifier") => {
            let spec = parse_choice_type_specifier(reader, e)?;
            Ok(Some(TypeSpecifier::ChoiceTypeSpecifier(spec)))
        }
        Some("TupleTypeSpecifier") => {
            let spec = parse_tuple_type_specifier(reader, e)?;
            Ok(Some(TypeSpecifier::TupleTypeSpecifier(spec)))
        }
        _ => {
            skip_element(reader)?;
            Ok(None)
        }
    }
}

/// Parse a `TypeSpecifier` from an *empty* element (attributes only, no
/// children).  Used when the element carries `xsi:type` inline.
pub(super) fn parse_type_specifier_attrs(e: &BytesStart) -> Result<Option<TypeSpecifier>> {
    let xsi_type = get_xsi_type(e);

    match xsi_type.as_deref() {
        Some("NamedTypeSpecifier") => {
            let spec = parse_named_type_specifier_attrs(e)?;
            Ok(Some(TypeSpecifier::NamedTypeSpecifier(spec)))
        }
        Some("ListTypeSpecifier") => {
            let mut spec = ListTypeSpecifier::default();
            for attr in e.attributes().flatten() {
                let key = std::str::from_utf8(attr.key.as_ref())?;
                let value = attr.unescape_value()?.into_owned();
                if key == "elementType" {
                    spec.element_type = Some(value);
                }
            }
            Ok(Some(TypeSpecifier::ListTypeSpecifier(spec)))
        }
        Some("IntervalTypeSpecifier") => {
            let mut spec = IntervalTypeSpecifier::default();
            for attr in e.attributes().flatten() {
                let key = std::str::from_utf8(attr.key.as_ref())?;
                let value = attr.unescape_value()?.into_owned();
                if key == "pointType" {
                    spec.point_type = Some(value);
                }
            }
            Ok(Some(TypeSpecifier::IntervalTypeSpecifier(spec)))
        }
        _ => Ok(None),
    }
}

/// Parse attributes of a `NamedTypeSpecifier` element.
pub(super) fn parse_named_type_specifier_attrs(e: &BytesStart) -> Result<NamedTypeSpecifier> {
    let mut spec = NamedTypeSpecifier::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "modelName" => spec.model_name = Some(value),
            "namespace" => spec.namespace = Some(value),
            "name" => spec.name = Some(value),
            _ => {}
        }
    }

    Ok(spec)
}

/// Parse a `ListTypeSpecifier` element and its optional `elementTypeSpecifier`
/// child.
pub(super) fn parse_list_type_specifier<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<ListTypeSpecifier> {
    let mut spec = ListTypeSpecifier::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        if key == "elementType" {
            spec.element_type = Some(value);
        }
    }

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"elementTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_element(reader, &e)? {
                        spec.element_type_specifier = Some(Box::new(inner));
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"elementTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_attrs(&e)? {
                        spec.element_type_specifier = Some(Box::new(inner));
                    }
                }
            }
            Event::End(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier"
                    || name.as_ref() == b"typeSpecifier"
                    || name.as_ref().ends_with(b"TypeSpecifier")
                {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(spec)
}

/// Parse an `IntervalTypeSpecifier` element and its optional
/// `pointTypeSpecifier` child.
pub(super) fn parse_interval_type_specifier<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<IntervalTypeSpecifier> {
    let mut spec = IntervalTypeSpecifier::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        if key == "pointType" {
            spec.point_type = Some(value);
        }
    }

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"pointTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_element(reader, &e)? {
                        spec.point_type_specifier = Some(Box::new(inner));
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"pointTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_attrs(&e)? {
                        spec.point_type_specifier = Some(Box::new(inner));
                    }
                }
            }
            Event::End(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier"
                    || name.as_ref() == b"typeSpecifier"
                    || name.as_ref().ends_with(b"TypeSpecifier")
                {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(spec)
}

/// Parse a `ChoiceTypeSpecifier` element and its `<choice>` children.
pub(super) fn parse_choice_type_specifier<R: BufRead>(
    reader: &mut Reader<R>,
    _e: &BytesStart,
) -> Result<ChoiceTypeSpecifier> {
    let mut spec = ChoiceTypeSpecifier::default();

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                if name.as_ref() == b"choice" {
                    if let Some(inner) = parse_type_specifier_element(reader, &e)? {
                        spec.choice.push(inner);
                    }
                } else {
                    // Deprecated `<type>` child or unknown element.
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"choice" {
                    if let Some(inner) = parse_type_specifier_attrs(&e)? {
                        spec.choice.push(inner);
                    }
                }
            }
            Event::End(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier"
                    || name.as_ref() == b"typeSpecifier"
                    || name.as_ref().ends_with(b"TypeSpecifier")
                {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(spec)
}

/// Parse a `TupleTypeSpecifier` element and its `<element>` children.
pub(super) fn parse_tuple_type_specifier<R: BufRead>(
    reader: &mut Reader<R>,
    _e: &BytesStart,
) -> Result<TupleTypeSpecifier> {
    let mut spec = TupleTypeSpecifier::default();

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_tuple_element_definition(reader, &e)?;
                    spec.element.push(elem.into());
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_tuple_element_attrs(&e)?;
                    spec.element.push(elem.into());
                }
            }
            Event::End(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier"
                    || name.as_ref() == b"typeSpecifier"
                    || name.as_ref().ends_with(b"TypeSpecifier")
                {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(spec)
}

/// Parse a `<element>` (TupleElementDefinition) that may carry an
/// `<elementTypeSpecifier>` child.
pub(super) fn parse_tuple_element_definition<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<TupleElementDef> {
    let mut elem = parse_tuple_element_attrs(e)?;

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"elementTypeSpecifier" {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"elementTypeSpecifier" {
                    if let Some(spec) = parse_type_specifier_attrs(&e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                }
            }
            Event::End(e) if e.name().as_ref() == b"element" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(elem)
}
