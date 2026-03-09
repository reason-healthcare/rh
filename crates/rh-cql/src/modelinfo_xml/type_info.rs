//! Parsing of `TypeInfo` variants from ModelInfo XML.
//!
//! Each `parse_*` function handles a concrete `TypeInfo` subtype discriminated
//! by the `xsi:type` attribute on the enclosing `<typeInfo>` element.

use super::type_specifier::{parse_type_specifier_attrs, parse_type_specifier_element};
use super::util::{get_xsi_type, parse_relationship_info, parse_type_parameter_info, skip_element};
use crate::modelinfo::{
    ChoiceTypeInfo, ClassInfo, ClassInfoElement, IntervalTypeInfo, ListTypeInfo, ProfileInfo,
    SimpleTypeInfo, TupleTypeInfo, TupleTypeInfoElement, TypeInfo, TypeSpecifier,
};
use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::io::BufRead;

/// Dispatch on `xsi:type` and parse the appropriate `TypeInfo` variant.
/// Returns `None` for unrecognised or unsupported type values.
pub(super) fn parse_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<Option<TypeInfo>> {
    let xsi_type = get_xsi_type(e);

    match xsi_type.as_deref() {
        Some("ClassInfo") => {
            let class_info = parse_class_info(reader, e)?;
            Ok(Some(TypeInfo::ClassInfo(class_info)))
        }
        Some("SimpleTypeInfo") => {
            let simple = parse_simple_type_info(reader, e)?;
            Ok(Some(TypeInfo::SimpleTypeInfo(simple)))
        }
        Some("ProfileInfo") => {
            let profile = parse_profile_info(reader, e)?;
            Ok(Some(TypeInfo::ProfileInfo(profile)))
        }
        Some("IntervalTypeInfo") => {
            let interval = parse_interval_type_info(reader, e)?;
            Ok(Some(TypeInfo::IntervalTypeInfo(interval)))
        }
        Some("ListTypeInfo") => {
            let list = parse_list_type_info(reader, e)?;
            Ok(Some(TypeInfo::ListTypeInfo(list)))
        }
        Some("TupleTypeInfo") => {
            let tuple = parse_tuple_type_info(reader, e)?;
            Ok(Some(TypeInfo::TupleTypeInfo(tuple)))
        }
        Some("ChoiceTypeInfo") => {
            let choice = parse_choice_type_info(reader, e)?;
            Ok(Some(TypeInfo::ChoiceTypeInfo(choice)))
        }
        _ => {
            skip_element(reader)?;
            Ok(None)
        }
    }
}

// ---------------------------------------------------------------------------
// ClassInfo
// ---------------------------------------------------------------------------

fn parse_class_info<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<ClassInfo> {
    let mut info = ClassInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "namespace" => info.namespace = Some(value),
            "name" => info.name = Some(value),
            "identifier" => info.identifier = Some(value),
            "label" => info.label = Some(value),
            "baseType" => info.base_type = Some(value),
            "retrievable" => info.retrievable = Some(value == "true"),
            "primaryCodePath" => info.primary_code_path = Some(value),
            "primaryValueSetPath" => info.primary_value_set_path = Some(value),
            _ => {}
        }
    }

    parse_class_info_children(reader, &mut info)?;
    Ok(info)
}

fn parse_class_info_children<R: BufRead>(
    reader: &mut Reader<R>,
    info: &mut ClassInfo,
) -> Result<()> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"element" => {
                    let elem = parse_class_info_element(reader, &e)?;
                    info.element.push(elem);
                }
                b"parameter" => {
                    let param = parse_type_parameter_info(reader, &e)?;
                    info.parameter.push(param);
                }
                b"contextRelationship" => {
                    let rel = parse_relationship_info(&e)?;
                    info.context_relationship.push(rel);
                    skip_element(reader)?;
                }
                b"targetContextRelationship" => {
                    let rel = parse_relationship_info(&e)?;
                    info.target_context_relationship.push(rel);
                    skip_element(reader)?;
                }
                b"baseTypeSpecifier" => {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        info.base_type_specifier = Some(spec);
                    }
                }
                _ => skip_element(reader)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"element" => {
                    let elem = parse_class_info_element_attrs(&e)?;
                    info.element.push(elem);
                }
                b"contextRelationship" => {
                    let rel = parse_relationship_info(&e)?;
                    info.context_relationship.push(rel);
                }
                b"targetContextRelationship" => {
                    let rel = parse_relationship_info(&e)?;
                    info.target_context_relationship.push(rel);
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"typeInfo" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

fn parse_class_info_element<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<ClassInfoElement> {
    let mut elem = parse_class_info_element_attrs(e)?;
    parse_class_info_element_children(reader, &mut elem)?;
    Ok(elem)
}

fn parse_class_info_element_attrs(e: &BytesStart) -> Result<ClassInfoElement> {
    let mut elem = ClassInfoElement::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => elem.name = Some(value),
            "elementType" => elem.element_type = Some(value),
            "type" => elem.type_name = Some(value),
            "prohibited" => elem.prohibited = Some(value == "true"),
            "oneBased" => elem.one_based = Some(value == "true"),
            "target" => elem.target = Some(value),
            _ => {}
        }
    }

    Ok(elem)
}

fn parse_class_info_element_children<R: BufRead>(
    reader: &mut Reader<R>,
    elem: &mut ClassInfoElement,
) -> Result<()> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"elementTypeSpecifier" => {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                }
                b"typeSpecifier" => {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        elem.type_specifier = Some(spec);
                    }
                }
                _ => skip_element(reader)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"elementTypeSpecifier" => {
                    if let Some(spec) = parse_type_specifier_attrs(&e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                }
                b"typeSpecifier" => {
                    if let Some(spec) = parse_type_specifier_attrs(&e)? {
                        elem.type_specifier = Some(spec);
                    }
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"element" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// SimpleTypeInfo
// ---------------------------------------------------------------------------

fn parse_simple_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<SimpleTypeInfo> {
    let mut info = SimpleTypeInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "namespace" => info.namespace = Some(value),
            "name" => info.name = Some(value),
            "baseType" => info.base_type = Some(value),
            _ => {}
        }
    }

    skip_element(reader)?;
    Ok(info)
}

// ---------------------------------------------------------------------------
// ProfileInfo
// ---------------------------------------------------------------------------

fn parse_profile_info<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<ProfileInfo> {
    let mut info = ProfileInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "namespace" => info.namespace = Some(value),
            "name" => info.name = Some(value),
            "identifier" => info.identifier = Some(value),
            "label" => info.label = Some(value),
            "baseType" => info.base_type = Some(value),
            "retrievable" => info.retrievable = Some(value == "true"),
            "primaryCodePath" => info.primary_code_path = Some(value),
            _ => {}
        }
    }

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_class_info_element(reader, &e)?;
                    info.element.push(elem);
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_class_info_element_attrs(&e)?;
                    info.element.push(elem);
                }
            }
            Event::End(e) if e.name().as_ref() == b"typeInfo" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(info)
}

// ---------------------------------------------------------------------------
// IntervalTypeInfo / ListTypeInfo
// ---------------------------------------------------------------------------

fn parse_interval_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<IntervalTypeInfo> {
    let mut info = IntervalTypeInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "baseType" => info.base_type = Some(value),
            "pointType" => info.point_type = Some(value),
            _ => {}
        }
    }

    skip_element(reader)?;
    Ok(info)
}

fn parse_list_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<ListTypeInfo> {
    let mut info = ListTypeInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "baseType" => info.base_type = Some(value),
            "elementType" => info.element_type = Some(value),
            _ => {}
        }
    }

    skip_element(reader)?;
    Ok(info)
}

// ---------------------------------------------------------------------------
// TupleTypeInfo
// ---------------------------------------------------------------------------

fn parse_tuple_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<TupleTypeInfo> {
    let mut info = TupleTypeInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        if key == "baseType" {
            info.base_type = Some(value);
        }
    }

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_tuple_type_info_element(reader, &e)?;
                    info.element.push(elem);
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"element" {
                    let elem = parse_tuple_type_info_element_attrs(&e)?;
                    info.element.push(elem);
                }
            }
            Event::End(e) if e.name().as_ref() == b"typeInfo" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(info)
}

fn parse_tuple_type_info_element<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<TupleTypeInfoElement> {
    let mut elem = parse_tuple_type_info_element_attrs(e)?;

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

fn parse_tuple_type_info_element_attrs(e: &BytesStart) -> Result<TupleTypeInfoElement> {
    let mut elem = TupleTypeInfoElement::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => elem.name = Some(value),
            "elementType" => elem.element_type = Some(value),
            "type" => elem.type_name = Some(value),
            _ => {}
        }
    }

    Ok(elem)
}

// ---------------------------------------------------------------------------
// ChoiceTypeInfo
// ---------------------------------------------------------------------------

fn parse_choice_type_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<ChoiceTypeInfo> {
    let mut info = ChoiceTypeInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        if key == "baseType" {
            info.base_type = Some(value);
        }
    }

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                if name.as_ref() == b"choice" || name.as_ref() == b"type" {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        info.choice.push(spec);
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"choice" || name.as_ref() == b"type" {
                    if let Some(spec) = parse_type_specifier_attrs(&e)? {
                        info.choice.push(spec);
                    }
                }
            }
            Event::End(e) if e.name().as_ref() == b"typeInfo" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(info)
}

// Satisfy the unused-import lint: TypeSpecifier is used by element_type_specifier
// fields populated via parse_type_specifier_* calls above.
const _: Option<TypeSpecifier> = None;
