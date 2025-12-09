//! XML parsing support for ModelInfo.
//!
//! ModelInfo in FHIR packages is typically stored as XML embedded in Library resources.
//! This module provides parsing from XML format to our ModelInfo structures.
//!
//! The XML uses `xsi:type` attributes to discriminate polymorphic types like TypeInfo
//! and TypeSpecifier variants.

use crate::modelinfo::{
    ChoiceTypeInfo, ChoiceTypeSpecifier, ClassInfo, ClassInfoElement, ContextInfo, ConversionInfo,
    IntervalTypeInfo, IntervalTypeSpecifier, ListTypeInfo, ListTypeSpecifier, ModelInfo,
    ModelSpecifier, NamedTypeSpecifier, ProfileInfo, RelationshipInfo, SimpleTypeInfo,
    TupleTypeInfo, TupleTypeInfoElement, TupleTypeSpecifier, TypeInfo, TypeParameterInfo,
    TypeSpecifier,
};
use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::io::BufRead;

impl ModelInfo {
    /// Parse ModelInfo from XML string.
    pub fn from_xml(xml: &str) -> Result<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        parse_model_info(&mut reader)
    }

    /// Parse ModelInfo from XML reader.
    pub fn from_xml_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);
        parse_model_info(&mut xml_reader)
    }
}

fn parse_model_info<R: BufRead>(reader: &mut Reader<R>) -> Result<ModelInfo> {
    let mut buf = Vec::new();
    let mut model_info = ModelInfo::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"modelInfo" => {
                parse_model_info_attrs(&e, &mut model_info)?;
                parse_model_info_children(reader, &mut model_info)?;
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(model_info)
}

fn parse_model_info_attrs(e: &BytesStart, model_info: &mut ModelInfo) -> Result<()> {
    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => model_info.name = Some(value),
            "version" => model_info.version = Some(value),
            "url" => model_info.url = Some(value),
            "schemaLocation" => model_info.schema_location = Some(value),
            "targetQualifier" => model_info.target_qualifier = Some(value),
            "patientClassName" => model_info.patient_class_name = Some(value),
            "patientClassIdentifier" => model_info.patient_class_identifier = Some(value),
            "patientBirthDatePropertyName" => {
                model_info.patient_birth_date_property_name = Some(value)
            }
            "caseSensitive" => model_info.case_sensitive = Some(value == "true"),
            "strictRetrievable" => model_info.strict_retrievable = Some(value == "true"),
            "defaultContext" => model_info.default_context = Some(value),
            _ => {}
        }
    }
    Ok(())
}

fn parse_model_info_children<R: BufRead>(
    reader: &mut Reader<R>,
    model_info: &mut ModelInfo,
) -> Result<()> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"requiredModelInfo" => {
                        let spec = parse_required_model_info(&e)?;
                        model_info.required_model_info.push(spec);
                    }
                    b"typeInfo" => {
                        if let Some(type_info) = parse_type_info(reader, &e)? {
                            model_info.type_info.push(type_info);
                        }
                    }
                    b"conversionInfo" => {
                        let conv = parse_conversion_info(&e)?;
                        model_info.conversion_info.push(conv);
                    }
                    b"contextInfo" => {
                        let ctx = parse_context_info(&e)?;
                        model_info.context_info.push(ctx);
                    }
                    _ => skip_element(reader)?,
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"requiredModelInfo" => {
                        let spec = parse_required_model_info(&e)?;
                        model_info.required_model_info.push(spec);
                    }
                    b"conversionInfo" => {
                        let conv = parse_conversion_info(&e)?;
                        model_info.conversion_info.push(conv);
                    }
                    b"contextInfo" => {
                        let ctx = parse_context_info(&e)?;
                        model_info.context_info.push(ctx);
                    }
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"modelInfo" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

fn parse_required_model_info(e: &BytesStart) -> Result<ModelSpecifier> {
    let mut spec = ModelSpecifier::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => spec.name = Some(value),
            "version" => spec.version = Some(value),
            _ => {}
        }
    }

    Ok(spec)
}

fn parse_conversion_info(e: &BytesStart) -> Result<ConversionInfo> {
    let mut conv = ConversionInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "fromType" => conv.from_type = Some(value),
            "toType" => conv.to_type = Some(value),
            "functionName" => conv.function_name = Some(value),
            _ => {}
        }
    }

    Ok(conv)
}

fn parse_context_info(e: &BytesStart) -> Result<ContextInfo> {
    let mut ctx = ContextInfo::default();

    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = attr.unescape_value()?.into_owned();

        match key {
            "name" => ctx.name = Some(value),
            "keyElement" => ctx.key_element = Some(value),
            "birthDateElement" => ctx.birth_date_element = Some(value),
            _ => {}
        }
    }

    Ok(ctx)
}

fn get_xsi_type(e: &BytesStart) -> Option<String> {
    for attr in e.attributes().flatten() {
        let key = attr.key.as_ref();
        // Check for xsi:type or just type
        if key == b"xsi:type" || key.ends_with(b":type") {
            if let Ok(value) = attr.unescape_value() {
                return Some(value.into_owned());
            }
        }
    }
    None
}

fn parse_type_info<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Option<TypeInfo>> {
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
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
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
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                match name.as_ref() {
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
                }
            }
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
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
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
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                match name.as_ref() {
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
                }
            }
            Event::End(e) if e.name().as_ref() == b"element" => break,
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

fn parse_type_specifier_element<R: BufRead>(
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

fn parse_type_specifier_attrs(e: &BytesStart) -> Result<Option<TypeSpecifier>> {
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

fn parse_named_type_specifier_attrs(e: &BytesStart) -> Result<NamedTypeSpecifier> {
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

fn parse_list_type_specifier<R: BufRead>(
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
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_element(reader, &e)? {
                        spec.element_type_specifier = Some(Box::new(inner));
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
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

fn parse_interval_type_specifier<R: BufRead>(
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
                let name = e.name();
                if name.as_ref() == b"pointTypeSpecifier" {
                    if let Some(inner) = parse_type_specifier_element(reader, &e)? {
                        spec.point_type_specifier = Some(Box::new(inner));
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"pointTypeSpecifier" {
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

fn parse_choice_type_specifier<R: BufRead>(
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
                } else if name.as_ref() == b"type" {
                    // Read text content for deprecated type element
                    skip_element(reader)?;
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"choice" {
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

fn parse_tuple_type_specifier<R: BufRead>(
    reader: &mut Reader<R>,
    _e: &BytesStart,
) -> Result<TupleTypeSpecifier> {
    let mut spec = TupleTypeSpecifier::default();

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                if name.as_ref() == b"element" {
                    // TupleElementDefinition
                    let elem = parse_tuple_element_definition(reader, &e)?;
                    spec.element.push(elem.into());
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"element" {
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

fn parse_type_parameter_info<R: BufRead>(
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

fn parse_relationship_info(e: &BytesStart) -> Result<RelationshipInfo> {
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

    // Parse children - similar to ClassInfo
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"element" => {
                        let elem = parse_class_info_element(reader, &e)?;
                        info.element.push(elem);
                    }
                    _ => skip_element(reader)?,
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"element" {
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
                let name = e.name();
                if name.as_ref() == b"element" {
                    let elem = parse_tuple_type_info_element(reader, &e)?;
                    info.element.push(elem);
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"element" {
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
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
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

// Helper struct for conversion
struct TupleElementDef {
    name: Option<String>,
    element_type: Option<String>,
    element_type_specifier: Option<TypeSpecifier>,
    type_name: Option<String>,
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

fn parse_tuple_element_definition<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<TupleElementDef> {
    let mut elem = parse_tuple_element_attrs(e)?;

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
                    if let Some(spec) = parse_type_specifier_element(reader, &e)? {
                        elem.element_type_specifier = Some(spec);
                    }
                } else {
                    skip_element(reader)?;
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"elementTypeSpecifier" {
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

fn parse_tuple_element_attrs(e: &BytesStart) -> Result<TupleElementDef> {
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

fn skip_element<R: BufRead>(reader: &mut Reader<R>) -> Result<()> {
    let mut depth = 1;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_model_info() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" 
           name="FHIR" 
           version="4.0.1" 
           url="http://hl7.org/fhir"
           patientClassName="FHIR.Patient"
           patientBirthDatePropertyName="birthDate.value">
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        assert_eq!(model_info.name, Some("FHIR".to_string()));
        assert_eq!(model_info.version, Some("4.0.1".to_string()));
        assert_eq!(model_info.url, Some("http://hl7.org/fhir".to_string()));
        assert_eq!(
            model_info.patient_class_name,
            Some("FHIR.Patient".to_string())
        );
        assert_eq!(
            model_info.patient_birth_date_property_name,
            Some("birthDate.value".to_string())
        );
    }

    #[test]
    fn test_parse_required_model_info() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" name="FHIR" version="4.0.1">
    <requiredModelInfo name="System" version="1.0.0"/>
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        assert_eq!(model_info.required_model_info.len(), 1);
        assert_eq!(
            model_info.required_model_info[0].name,
            Some("System".to_string())
        );
        assert_eq!(
            model_info.required_model_info[0].version,
            Some("1.0.0".to_string())
        );
    }

    #[test]
    fn test_parse_conversion_info() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" name="FHIR" version="4.0.1">
    <conversionInfo functionName="FHIRHelpers.ToCode" fromType="FHIR.Coding" toType="System.Code"/>
    <conversionInfo functionName="FHIRHelpers.ToConcept" fromType="FHIR.CodeableConcept" toType="System.Concept"/>
    <conversionInfo functionName="FHIRHelpers.ToString" fromType="FHIR.string" toType="System.String"/>
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        assert_eq!(model_info.conversion_info.len(), 3);

        let conv = &model_info.conversion_info[0];
        assert_eq!(conv.function_name, Some("FHIRHelpers.ToCode".to_string()));
        assert_eq!(conv.from_type, Some("FHIR.Coding".to_string()));
        assert_eq!(conv.to_type, Some("System.Code".to_string()));
    }

    #[test]
    fn test_parse_class_info() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" 
           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           name="FHIR" version="4.0.1">
    <typeInfo baseType="FHIR.DomainResource" 
              namespace="FHIR" 
              name="Patient" 
              identifier="http://hl7.org/fhir/StructureDefinition/Patient" 
              label="Patient" 
              retrievable="true" 
              xsi:type="ClassInfo">
        <element name="identifier">
            <elementTypeSpecifier elementType="FHIR.Identifier" xsi:type="ListTypeSpecifier"/>
        </element>
        <element name="active" elementType="FHIR.boolean"/>
        <contextRelationship context="Practitioner" relatedKeyElement="generalPractitioner"/>
    </typeInfo>
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        assert_eq!(model_info.type_info.len(), 1);

        if let TypeInfo::ClassInfo(class_info) = &model_info.type_info[0] {
            assert_eq!(class_info.name, Some("Patient".to_string()));
            assert_eq!(class_info.namespace, Some("FHIR".to_string()));
            assert_eq!(class_info.retrievable, Some(true));
            assert_eq!(
                class_info.base_type,
                Some("FHIR.DomainResource".to_string())
            );

            assert_eq!(class_info.element.len(), 2);
            assert_eq!(class_info.element[0].name, Some("identifier".to_string()));
            assert_eq!(class_info.element[1].name, Some("active".to_string()));
            assert_eq!(
                class_info.element[1].element_type,
                Some("FHIR.boolean".to_string())
            );

            assert_eq!(class_info.context_relationship.len(), 1);
            assert_eq!(
                class_info.context_relationship[0].context,
                Some("Practitioner".to_string())
            );
        } else {
            panic!("Expected ClassInfo");
        }
    }

    #[test]
    fn test_parse_list_type_specifier() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" 
           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           name="FHIR" version="4.0.1">
    <typeInfo namespace="FHIR" name="Account" xsi:type="ClassInfo">
        <element name="identifier">
            <elementTypeSpecifier elementType="FHIR.Identifier" xsi:type="ListTypeSpecifier"/>
        </element>
    </typeInfo>
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        if let TypeInfo::ClassInfo(class_info) = &model_info.type_info[0] {
            let elem = &class_info.element[0];
            assert!(elem.element_type_specifier.is_some());
            if let Some(TypeSpecifier::ListTypeSpecifier(list)) = &elem.element_type_specifier {
                assert_eq!(list.element_type, Some("FHIR.Identifier".to_string()));
            } else {
                panic!("Expected ListTypeSpecifier");
            }
        }
    }

    #[test]
    fn test_parse_choice_type_specifier() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<modelInfo xmlns="urn:hl7-org:elm-modelinfo:r1" 
           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           name="FHIR" version="4.0.1">
    <typeInfo namespace="FHIR" name="ActivityDefinition" xsi:type="ClassInfo">
        <element name="subject">
            <elementTypeSpecifier xsi:type="ChoiceTypeSpecifier">
                <choice namespace="FHIR" name="CodeableConcept" xsi:type="NamedTypeSpecifier"/>
                <choice namespace="FHIR" name="Reference" xsi:type="NamedTypeSpecifier"/>
            </elementTypeSpecifier>
        </element>
    </typeInfo>
</modelInfo>"#;

        let model_info = ModelInfo::from_xml(xml).unwrap();
        if let TypeInfo::ClassInfo(class_info) = &model_info.type_info[0] {
            let elem = &class_info.element[0];
            if let Some(TypeSpecifier::ChoiceTypeSpecifier(choice)) = &elem.element_type_specifier {
                assert_eq!(choice.choice.len(), 2);
                if let TypeSpecifier::NamedTypeSpecifier(named) = &choice.choice[0] {
                    assert_eq!(named.namespace, Some("FHIR".to_string()));
                    assert_eq!(named.name, Some("CodeableConcept".to_string()));
                } else {
                    panic!("Expected NamedTypeSpecifier");
                }
            } else {
                panic!("Expected ChoiceTypeSpecifier");
            }
        }
    }

    #[test]
    fn test_parse_fhir_modelinfo_file() {
        // Test with actual FHIR ModelInfo file if available
        let path = std::path::Path::new("/tmp/fhir-modelinfo.xml");
        if path.exists() {
            let xml = std::fs::read_to_string(path).unwrap();
            let model_info = ModelInfo::from_xml(&xml).unwrap();

            assert_eq!(model_info.name, Some("FHIR".to_string()));
            assert_eq!(model_info.version, Some("4.0.1".to_string()));

            // Should have many types (FHIR has 100s)
            assert!(
                model_info.type_info.len() > 100,
                "Expected > 100 types, got {}",
                model_info.type_info.len()
            );

            // Should have conversion info
            assert!(
                model_info.conversion_info.len() > 200,
                "Expected > 200 conversions, got {}",
                model_info.conversion_info.len()
            );

            // Verify a few known conversions
            let to_code = model_info
                .conversion_info
                .iter()
                .find(|c| c.function_name.as_deref() == Some("FHIRHelpers.ToCode"));
            assert!(to_code.is_some());

            // Verify Patient type exists
            let patient = model_info.type_info.iter().find(|t| match t {
                TypeInfo::ClassInfo(c) => c.name.as_deref() == Some("Patient"),
                _ => false,
            });
            assert!(patient.is_some());
        }
    }
}
