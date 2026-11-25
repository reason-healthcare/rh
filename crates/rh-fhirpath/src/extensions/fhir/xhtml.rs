//! Check the namespace, elements, nesting, attributes and active-content restrictions
//! of FHIR narrative XHTML without resolving external resources.
use quick_xml::{events::Event, name::ResolveResult, reader::NsReader};
use std::{collections::HashMap, iter::Peekable, str::Chars, sync::LazyLock};

struct Element {
    attributes: &'static str,
    children: &'static str,
    required: &'static str,
}

static ELEMENTS: LazyLock<HashMap<&str, Element>> = LazyLock::new(|| {
    include_str!("../../evaluator/metadata/xhtml.tsv")
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some((
                parts.next()?,
                Element {
                    attributes: parts.next()?,
                    children: parts.next()?,
                    required: parts.next()?,
                },
            ))
        })
        .collect()
});

pub(super) fn valid(source: &str) -> bool {
    let mut reader = NsReader::from_str(source);
    reader.config_mut().expand_empty_elements = true;
    let mut stack: Vec<&Element> = Vec::new();
    let mut root_seen = false;
    let mut content = false;
    loop {
        let Ok((namespace, event)) = reader.read_resolved_event() else {
            return false;
        };
        match event {
            Event::Start(tag) => {
                if !matches!(namespace,ResolveResult::Bound(ns) if ns.as_ref()=="http://www.w3.org/1999/xhtml")
                {
                    return false;
                }
                let local = tag.local_name();
                let name = local.as_ref();
                if let Some(parent) = stack.last() {
                    if !parent.children.split_ascii_whitespace().any(|n| n == name) {
                        return false;
                    }
                } else {
                    if root_seen || name != "div" {
                        return false;
                    }
                    root_seen = true;
                }
                let Some(element) = ELEMENTS.get(name) else {
                    return false;
                };
                let mut required: Vec<_> = element.required.split_ascii_whitespace().collect();
                for attr in tag.attributes() {
                    let Ok(attr) = attr else { return false };
                    let key = attr.key.as_ref();
                    if key == "xmlns" || key.starts_with("xmlns:") {
                        continue;
                    }
                    if !element
                        .attributes
                        .split_ascii_whitespace()
                        .any(|k| k == key)
                    {
                        return false;
                    }
                    required.retain(|k| *k != key);
                    let Ok(value) = attr.normalized_value(quick_xml::XmlVersion::Implicit1_0)
                    else {
                        return false;
                    };
                    if !valid_attribute_value(key, &value) {
                        return false;
                    }
                }
                if !required.is_empty() {
                    return false;
                }
                content |= name == "img";
                stack.push(element);
            }
            Event::End(_) => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            Event::Text(text) => {
                if text.as_ref().bytes().any(|b| !b.is_ascii_whitespace()) {
                    if stack.is_empty() {
                        return false;
                    }
                    content = true;
                }
            }
            Event::CData(text) => {
                if stack.is_empty() {
                    return false;
                }
                content |= text.as_ref().bytes().any(|b| !b.is_ascii_whitespace());
            }
            Event::GeneralRef(reference) => {
                if stack.is_empty() {
                    return false;
                }
                if !matches!(reference.resolve_char_ref(), Ok(Some(_)))
                    && !matches!(reference.as_ref(), "amp" | "quot" | "apos" | "lt" | "gt")
                {
                    return false;
                }
                content = true;
            }
            Event::Comment(_) => (),
            Event::Eof => return root_seen && stack.is_empty() && content,
            _ => return false,
        }
    }
}

fn valid_attribute_value(key: &str, value: &str) -> bool {
    match key {
        "href" | "src" | "usemap" | "longdesc" => {
            let Some((scheme, _)) = value.split_once(':') else {
                return true;
            };
            match normalized_for_matching(scheme).as_str() {
                "javascript" | "vbscript" => false,
                "data" => key == "src" && allowed_image_data_url(value),
                _ => true,
            }
        }
        "style" => {
            let Some(normalized) = style_for_matching(value) else {
                return false;
            };
            [
                "expression(",
                "url(",
                "@import",
                "behavior:",
                "-moz-binding",
            ]
            .iter()
            .all(|pattern| !normalized.contains(pattern))
        }
        _ => true,
    }
}

fn normalized_for_matching(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_control())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn style_for_matching(value: &str) -> Option<String> {
    let mut input = value.chars().peekable();
    let mut decoded = String::with_capacity(value.len());
    let mut quote = None;
    while let Some(character) = input.next() {
        match character {
            '\\' => decoded.push(escaped_css_character(&mut input)?),
            '/' if quote.is_none() && input.next_if_eq(&'*').is_some() => {
                while input.next()? != '*' || input.next_if_eq(&'/').is_none() {}
            }
            '"' | '\'' if quote.is_none() => {
                quote = Some(character);
                decoded.push(character);
            }
            character if quote == Some(character) => {
                quote = None;
                decoded.push(character);
            }
            '\n' | '\r' | '\x0c' => {
                quote = None;
                decoded.push(character);
            }
            character => decoded.push(character),
        }
    }
    Some(normalized_for_matching(&decoded))
}

fn escaped_css_character(input: &mut Peekable<Chars<'_>>) -> Option<char> {
    let first = input.next()?;
    if first == '\r' {
        input.next_if_eq(&'\n');
    }
    let Some(mut code_point) = first.to_digit(16) else {
        return Some(first);
    };
    for _ in 1..6 {
        let Some(digit) = input.peek().and_then(|c| c.to_digit(16)) else {
            break;
        };
        input.next();
        code_point = code_point * 16 + digit;
    }
    if input.next_if(|c| c.is_ascii_whitespace()) == Some('\r') {
        input.next_if_eq(&'\n');
    }
    Some(
        char::from_u32(code_point)
            .filter(|&c| c != '\0')
            .unwrap_or('\u{fffd}'),
    )
}

fn allowed_image_data_url(value: &str) -> bool {
    let Some((header, _)) = value.split_once(';') else {
        return false;
    };
    let header = header.trim_start().replace(['\t', '\n', '\r'], "");
    ["data:image/png", "data:image/jpeg", "data:image/gif"]
        .iter()
        .any(|allowed| header.eq_ignore_ascii_case(allowed))
}
