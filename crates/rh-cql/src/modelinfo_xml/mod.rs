//! XML parsing support for ModelInfo.
//!
//! ModelInfo in FHIR packages is typically stored as XML embedded in Library
//! resources.  This module provides parsing from XML format to the
//! [`crate::modelinfo::ModelInfo`] structures.
//!
//! The XML uses `xsi:type` attributes to discriminate polymorphic types like
//! [`crate::modelinfo::TypeInfo`] and [`crate::modelinfo::TypeSpecifier`]
//! variants.
//!
//! ## Submodule layout
//!
//! | Module | Responsibility |
//! |---|---|
//! | [`type_info`] | `TypeInfo` variant parsers (`ClassInfo`, `SimpleTypeInfo`, …) |
//! | [`type_specifier`] | `TypeSpecifier` variant parsers (`NamedTypeSpecifier`, `ListTypeSpecifier`, …) |
//! | [`util`] | Shared helpers: `skip_element`, `get_xsi_type`, `TupleElementDef`, … |

mod type_info;
mod type_specifier;
mod util;

use type_info::parse_type_info;
use util::skip_element;

use crate::modelinfo::{ContextInfo, ConversionInfo, ModelInfo, ModelSpecifier};
use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::io::BufRead;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl ModelInfo {
    /// Parse a [`ModelInfo`] from an XML string.
    pub fn from_xml(xml: &str) -> Result<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        parse_model_info(&mut reader)
    }

    /// Parse a [`ModelInfo`] from any [`BufRead`] source.
    pub fn from_xml_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);
        parse_model_info(&mut xml_reader)
    }
}

// ---------------------------------------------------------------------------
// Top-level model parsing
// ---------------------------------------------------------------------------

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
            Event::Start(e) => match e.name().as_ref() {
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
            },
            Event::Empty(e) => match e.name().as_ref() {
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
            },
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelinfo::TypeInfo;
    use crate::modelinfo::TypeSpecifier;

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
        // Test with actual FHIR ModelInfo file if available.
        let path = std::path::Path::new("/tmp/fhir-modelinfo.xml");
        if path.exists() {
            let xml = std::fs::read_to_string(path).unwrap();
            let model_info = ModelInfo::from_xml(&xml).unwrap();

            assert_eq!(model_info.name, Some("FHIR".to_string()));
            assert_eq!(model_info.version, Some("4.0.1".to_string()));

            assert!(
                model_info.type_info.len() > 100,
                "Expected > 100 types, got {}",
                model_info.type_info.len()
            );

            assert!(
                model_info.conversion_info.len() > 200,
                "Expected > 200 conversions, got {}",
                model_info.conversion_info.len()
            );

            let to_code = model_info
                .conversion_info
                .iter()
                .find(|c| c.function_name.as_deref() == Some("FHIRHelpers.ToCode"));
            assert!(to_code.is_some());

            let patient = model_info.type_info.iter().find(|t| match t {
                TypeInfo::ClassInfo(c) => c.name.as_deref() == Some("Patient"),
                _ => false,
            });
            assert!(patient.is_some());
        }
    }
}
