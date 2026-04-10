use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use std::collections::BTreeMap;

use crate::Result;
use crate::simple_type::simple_type_mapping;

#[derive(Default)]
pub(crate) struct ParsedXsd {
  pub target_namespace: String,
  pub root_elements: BTreeMap<String, ParsedComplexType>,
  pub complex_types: BTreeMap<String, ParsedComplexType>,
  pub simple_types: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Default)]
pub(crate) struct ParsedComplexType {
  pub _mixed: bool,
  pub children: Vec<ParsedChildElement>,
  pub attributes: Vec<ParsedAttribute>,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub(crate) struct ParsedChildElement {
  pub q_name: String,
  pub min_occurs: u64,
  pub max_occurs: u64,
  pub complex_type: Option<ParsedComplexType>,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub(crate) struct ParsedAttribute {
  pub field: String,
  pub q_name: String,
  pub r#type: String,
  pub required: bool,
}

pub(crate) fn parse_xsd(source: &str) -> Result<ParsedXsd> {
  let mut reader = Reader::from_str(source);
  reader.config_mut().trim_text(true);
  let mut parsed = ParsedXsd::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"schema" => {
          parsed.target_namespace = required_attr(&reader, &e, b"targetNamespace")?;
        }
        b"element" => {
          let (name, complex_type) = parse_element(&mut reader, e)?;
          parsed.root_elements.insert(name, complex_type);
        }
        b"complexType" => {
          let (name, complex_type) = parse_complex_type(&mut reader, e)?;
          parsed.complex_types.insert(name, complex_type);
        }
        b"simpleType" => {
          let (name, values) = parse_simple_type(&mut reader, e)?;
          parsed.simple_types.insert(name, values);
        }
        _ => skip_element(&mut reader, e.name().as_ref())?,
      },
      Event::Empty(_) | Event::Text(_) | Event::Comment(_) | Event::Decl(_) => {}
      Event::Eof => break,
      _ => {}
    }
  }

  Ok(parsed)
}

fn parse_complex_type(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, ParsedComplexType)> {
  let name = required_attr(reader, &start, b"name")?;
  let complex_type = parse_complex_type_body(reader, start)?;
  Ok((name, complex_type))
}

fn parse_complex_type_body(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<ParsedComplexType> {
  let mut complex_type = ParsedComplexType {
    _mixed: optional_attr(reader, &start, b"mixed")?.as_deref() == Some("true"),
    ..ParsedComplexType::default()
  };

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"sequence" | b"choice" | b"all" | b"simpleContent" | b"extension" => {}
        b"element" => complex_type
          .children
          .push(parse_child_element(reader, &e, false)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
          skip_element(reader, e.name().as_ref())?;
        }
        _ => skip_element(reader, e.name().as_ref())?,
      },
      Event::Empty(e) => match local_name(e.name().as_ref()) {
        b"element" => complex_type
          .children
          .push(parse_child_element(reader, &e, true)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
        }
        _ => {}
      },
      Event::End(e) if local_name(e.name().as_ref()) == b"complexType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => {
        return Err(
          format!(
            "unexpected EOF in complexType{}",
            required_attr(reader, &start, b"name")
              .ok()
              .map(|value| format!(" {}", value))
              .unwrap_or_default()
          )
          .into(),
        );
      }
      _ => {}
    }
  }

  Ok(complex_type)
}

fn parse_element(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, ParsedComplexType)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut complex_type = ParsedComplexType::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"complexType" => {
        complex_type = parse_complex_type_body(reader, e)?;
      }
      Event::Empty(e) if local_name(e.name().as_ref()) == b"complexType" => {
        complex_type = ParsedComplexType::default();
      }
      Event::End(e) if local_name(e.name().as_ref()) == b"element" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in element {}", name).into()),
      _ => {}
    }
  }

  Ok((name, complex_type))
}

fn parse_simple_type(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, Vec<String>)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut values = vec![];

  loop {
    match reader.read_event()? {
      Event::Empty(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
        skip_element(reader, e.name().as_ref())?;
      }
      Event::End(e) if local_name(e.name().as_ref()) == b"simpleType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in simpleType {}", name).into()),
      _ => {}
    }
  }

  Ok((name, values))
}

fn parse_child_element(
  reader: &mut Reader<&[u8]>,
  element: &BytesStart<'_>,
  empty: bool,
) -> Result<ParsedChildElement> {
  let q_name = if let Some(reference) = optional_attr(reader, element, b"ref")? {
    reference
  } else {
    let name = required_attr(reader, element, b"name")?;
    let prefix = element_prefix(reader, element)?;
    if prefix.is_empty() {
      name
    } else {
      format!("{prefix}:{name}")
    }
  };

  let mut complex_type = None;
  let mut depth = 1usize;
  let saw_body = !empty;

  while saw_body && depth > 0 {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"complexType" => {
        complex_type = Some(parse_complex_type_body(reader, e)?);
      }
      Event::Start(e) => {
        depth += 1;
        skip_element(reader, e.name().as_ref())?;
      }
      Event::Empty(e) if local_name(e.name().as_ref()) == b"complexType" => {
        complex_type = Some(ParsedComplexType::default());
      }
      Event::End(e) if e.name().as_ref() == element.name().as_ref() => {
        depth -= 1;
      }
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in element {}", q_name).into()),
      _ => {}
    }
  }

  Ok(ParsedChildElement {
    q_name,
    min_occurs: optional_attr(reader, element, b"minOccurs")?
      .as_deref()
      .unwrap_or("1")
      .parse()
      .unwrap_or(1),
    max_occurs: optional_attr(reader, element, b"maxOccurs")?
      .as_deref()
      .map(|value| {
        if value == "unbounded" {
          u64::MAX
        } else {
          value.parse().unwrap_or(1)
        }
      })
      .unwrap_or(1),
    complex_type,
  })
}

fn parse_attribute(
  reader: &Reader<&[u8]>,
  element: &BytesStart<'_>,
) -> Result<Option<ParsedAttribute>> {
  if let Some(reference) = optional_attr(reader, element, b"ref")? {
    let q_name = reference;
    return Ok(Some(ParsedAttribute {
      field: attribute_field_name(&q_name),
      q_name,
      r#type: "StringValue".to_string(),
      required: optional_attr(reader, element, b"use")?.as_deref() == Some("required"),
    }));
  }

  let Some(name) = optional_attr(reader, element, b"name")? else {
    return Ok(None);
  };

  Ok(Some(ParsedAttribute {
    field: attribute_field_name(&name),
    q_name: name,
    r#type: map_xsd_type_to_schema_type(
      optional_attr(reader, element, b"type")?
        .unwrap_or_else(|| "xsd:string".to_string())
        .as_str(),
    ),
    required: optional_attr(reader, element, b"use")?.as_deref() == Some("required"),
  }))
}

fn attribute_field_name(name: &str) -> String {
  match name {
    "TargetMode" => "target_mode",
    "Target" => "target",
    "Type" => "type",
    "Id" | "ID" => "id",
    "Extension" => "extension",
    "ContentType" => "content_type",
    "PartName" => "part_name",
    "SchemaRef" => "schema_reference",
    "SchemaLanguage" => "schema_language",
    "xml:lang" => "lang",
    _ => name,
  }
  .to_string()
}

fn map_xsd_type_to_schema_type(value: &str) -> String {
  match value {
    "ST_TargetMode" => "ST_TargetMode".to_string(),
    other => match simple_type_mapping(other) {
      mapped if mapped != other => mapped.to_string(),
      _ => "StringValue".to_string(),
    },
  }
}

fn element_prefix(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<String> {
  if let Some(reference) = optional_attr(reader, element, b"ref")?
    && let Some((prefix, _)) = reference.split_once(':')
  {
    return Ok(prefix.to_string());
  }

  let Some(type_name) = optional_attr(reader, element, b"type")? else {
    return Ok("cp".to_string());
  };

  Ok(
    match strip_prefix(type_name.as_str()) {
      "CT_Keyword" | "CT_Keywords" => "cp",
      _ => "cp",
    }
    .to_string(),
  )
}

fn required_attr(reader: &Reader<&[u8]>, start: &BytesStart<'_>, key: &[u8]) -> Result<String> {
  optional_attr(reader, start, key)?.ok_or_else(|| {
    format!(
      "missing attribute {} on {}",
      String::from_utf8_lossy(key),
      String::from_utf8_lossy(start.name().as_ref())
    )
    .into()
  })
}

fn optional_attr(
  reader: &Reader<&[u8]>,
  start: &BytesStart<'_>,
  key: &[u8],
) -> Result<Option<String>> {
  for attr in start.attributes().with_checks(false) {
    let attr = attr?;
    if attr.key == QName(key) {
      return Ok(Some(
        attr
          .decode_and_unescape_value(reader.decoder())?
          .into_owned(),
      ));
    }
  }

  Ok(None)
}

fn skip_element(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<()> {
  let mut depth = 1usize;

  while depth > 0 {
    match reader.read_event()? {
      Event::Start(e) if e.name().as_ref() == tag => depth += 1,
      Event::End(e) if e.name().as_ref() == tag => depth -= 1,
      Event::Eof => return Err("unexpected EOF while skipping element".into()),
      _ => {}
    }
  }

  Ok(())
}

fn local_name(name: &[u8]) -> &[u8] {
  match name.iter().rposition(|byte| *byte == b':') {
    Some(index) => &name[index + 1..],
    None => name,
  }
}

fn strip_prefix(value: &str) -> &str {
  match value.rsplit_once(':') {
    Some((_, suffix)) => suffix,
    None => value,
  }
}
