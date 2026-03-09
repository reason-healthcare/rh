//! Shared XML parsing utilities for the modelinfo_xml module.

use crate::modelinfo::{RelationshipInfo, TypeParameterInfo, TypeSpecifier};
use anyhow::Result;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::io::BufRead;

/// Extract the `xsi:type` attribute value from a start element.
pub(super) fn get_xsi_type(e: &BytesStart) -> Option<String> {
    for attr in e.attributes().flatten() {
        let key = attr.key.as_ref();
        // Accept `xsi:type` or any `*:type` Clark-notation variant.
        if key == b"xsi:type" || key.ends_with(b":type") {
            if let Ok(value) = attr.unescape_value() {
                return Some(value.into_owned());
            }
        }
    }
    None
}

/// Consume and discard the remainder of the current element, including all
/// nested descendant elements.  Must be called immediately after reading the
/// opening `Event::Start`.
pub(super) fn skip_element<R: BufRead>(reader: &mut Reader<R>) -> Result<()> {
    let mut depth = 1;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            quick_xml::events::Event::Start(_) => depth += 1,
            quick_xml::events::Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parse a `<parameter>` element into a [`TypeParameterInfo`].
pub(super) fn parse_type_parameter_info<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<TypeParameterInfo> {
    let mut info = TypeParameterInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => info.name = Some(value),
            "constraint" => info.constraint = Some(value),
            "constraintType" => info.constraint_type = Some(value),
            _ => {}
        }
    }

    skip_element(reader)?;
    Ok(info)
}

/// Parse a `<contextRelationship>` or `<targetContextRelationship>` element.
pub(super) fn parse_relationship_info(e: &BytesStart) -> Result<RelationshipInfo> {
    let mut info = RelationshipInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "context" => info.context = Some(value),
            "relatedKeyElement" => info.related_key_element = Some(value),
            _ => {}
        }
    }

    Ok(info)
}

// ---------------------------------------------------------------------------
// TupleElementDefinition helper (used by both type_specifier and type_info)
// ---------------------------------------------------------------------------

/// Intermediate representation for a tuple element before it is converted to
/// the canonical [`crate::modelinfo::TupleElementDefinition`].
pub(super) struct TupleElementDef {
    pub(super) name: Option<String>,
    pub(super) element_type: Option<String>,
    pub(super) element_type_specifier: Option<TypeSpecifier>,
    pub(super) type_name: Option<String>,
}

impl From<TupleElementDef> for crate::modelinfo::TupleElementDefinition {
    fn from(def: TupleElementDef) -> Self {
        crate::modelinfo::TupleElementDefinition {
            name: def.name,
            element_type: def.element_type,
            element_type_specifier: def.element_type_specifier.map(Box::new),
            type_name: def.type_name,
        }
    }
}

/// Parse attributes of a `<element>` (TupleElementDefinition) into a
/// [`TupleElementDef`] without consuming child events.
pub(super) fn parse_tuple_element_attrs(e: &BytesStart) -> Result<TupleElementDef> {
    let mut elem = TupleElementDef {
        name: None,
        element_type: None,
        element_type_specifier: None,
        type_name: None,
    };

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
