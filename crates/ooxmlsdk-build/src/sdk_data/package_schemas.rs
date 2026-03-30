use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::sdk_data::sdk_data_model::{
  PackageAttribute, PackageChildField, PackageChildFieldKind, PackageChildVariant, PackageEnum,
  PackageEnumVariant, PackageFixedAttribute, PackageSchema, PackageTextChild, PackageType,
  PackageTypeKind, PackageXmlHeader,
};

use super::Result;

pub fn read_package_schemas(schemas_dir: &Path) -> Result<Vec<PackageSchema>> {
  let mut schemas = vec![];

  if !schemas_dir.exists() {
    return Ok(schemas);
  }

  for entry in fs::read_dir(schemas_dir)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("xsd")) {
      continue;
    }

    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
      continue;
    };

    let source = fs::read_to_string(&path)?;

    let schema = match file_name {
      "opc-relationships.xsd" => parse_opc_relationships_xsd(&source)?,
      "opc-contentTypes.xsd" => parse_opc_content_types_xsd(&source)?,
      "opc-coreProperties.xsd" => parse_opc_core_properties_xsd(&source)?,
      _ => continue,
    };

    schemas.push(schema);
  }

  schemas.sort_by(|left, right| left.module_name.cmp(&right.module_name));
  Ok(schemas)
}

fn parse_opc_relationships_xsd(source: &str) -> Result<PackageSchema> {
  let xsd = parse_xsd(source)?;
  let relationships = xsd
    .complex_types
    .get("CT_Relationships")
    .ok_or("missing CT_Relationships")?;
  let relationship = xsd
    .complex_types
    .get("CT_Relationship")
    .ok_or("missing CT_Relationship")?;
  let target_mode = xsd
    .simple_types
    .get("ST_TargetMode")
    .ok_or("missing ST_TargetMode")?;

  Ok(PackageSchema {
    module_name: "opc_relationships".to_string(),
    root: "Relationships".to_string(),
    xmlns_uri: xsd.target_namespace,
    xml_header: PackageXmlHeader::Plain,
    types: vec![
      PackageType {
        name: "Relationships".to_string(),
        tag: "Relationships".to_string(),
        prefix: "w".to_string(),
        kind: PackageTypeKind::Composite,
        has_xmlns_fields: true,
        child_fields: vec![PackageChildField {
          field: "relationship".to_string(),
          kind: PackageChildFieldKind::Vec,
          item_type: relationships
            .children
            .first()
            .ok_or("CT_Relationships missing child")?
            .to_string(),
          ..PackageChildField::default()
        }],
        ..PackageType::default()
      },
      PackageType {
        name: "Relationship".to_string(),
        tag: "Relationship".to_string(),
        prefix: "w".to_string(),
        kind: PackageTypeKind::Leaf,
        attributes: relationship.attributes.clone(),
        ..PackageType::default()
      },
    ],
    enums: vec![PackageEnum {
      name: "TargetMode".to_string(),
      variants: target_mode
        .iter()
        .enumerate()
        .map(|(index, value)| PackageEnumVariant {
          name: value.clone(),
          value: value.clone(),
          is_default: index == 0,
        })
        .collect(),
    }],
  })
}

fn parse_opc_content_types_xsd(source: &str) -> Result<PackageSchema> {
  let xsd = parse_xsd(source)?;
  let types = xsd
    .complex_types
    .get("CT_Types")
    .ok_or("missing CT_Types")?;
  let default_type = xsd
    .complex_types
    .get("CT_Default")
    .ok_or("missing CT_Default")?;
  let override_type = xsd
    .complex_types
    .get("CT_Override")
    .ok_or("missing CT_Override")?;

  Ok(PackageSchema {
    module_name: "opc_content_types".to_string(),
    root: "Types".to_string(),
    xmlns_uri: xsd.target_namespace,
    xml_header: PackageXmlHeader::Plain,
    types: vec![
      PackageType {
        name: "Types".to_string(),
        tag: "Types".to_string(),
        prefix: "w".to_string(),
        kind: PackageTypeKind::Composite,
        has_xmlns_fields: true,
        child_fields: vec![PackageChildField {
          field: "children".to_string(),
          kind: PackageChildFieldKind::ChoiceVec,
          enum_name: "TypesChildChoice".to_string(),
          variants: types
            .children
            .iter()
            .map(|name| PackageChildVariant {
              name: name.clone(),
              r#type: name.clone(),
            })
            .collect(),
          ..PackageChildField::default()
        }],
        ..PackageType::default()
      },
      PackageType {
        name: "Default".to_string(),
        tag: "Default".to_string(),
        prefix: "w".to_string(),
        kind: PackageTypeKind::Leaf,
        attributes: default_type.attributes.clone(),
        ..PackageType::default()
      },
      PackageType {
        name: "Override".to_string(),
        tag: "Override".to_string(),
        prefix: "w".to_string(),
        kind: PackageTypeKind::Leaf,
        attributes: override_type.attributes.clone(),
        ..PackageType::default()
      },
    ],
    enums: vec![],
  })
}

fn parse_opc_core_properties_xsd(source: &str) -> Result<PackageSchema> {
  let xsd = parse_xsd(source)?;
  let core_properties = xsd
    .complex_types
    .get("CT_CoreProperties")
    .ok_or("missing CT_CoreProperties")?;

  Ok(PackageSchema {
    module_name: "opc_core_properties".to_string(),
    root: "CoreProperties".to_string(),
    xmlns_uri: xsd.target_namespace,
    xml_header: PackageXmlHeader::Standalone,
    types: vec![PackageType {
      name: "CoreProperties".to_string(),
      tag: "coreProperties".to_string(),
      prefix: "cp".to_string(),
      kind: PackageTypeKind::Composite,
      has_xmlns_fields: true,
      text_children: core_properties
        .children
        .iter()
        .map(|child| match child.as_str() {
          "dcterms:created" | "dcterms:modified" => PackageTextChild {
            field: core_property_field_name(child),
            q_name: child.clone(),
            fixed_attributes: vec![PackageFixedAttribute {
              q_name: "xsi:type".to_string(),
              value: "dcterms:W3CDTF".to_string(),
            }],
          },
          _ => PackageTextChild {
            field: core_property_field_name(child),
            q_name: child.clone(),
            ..PackageTextChild::default()
          },
        })
        .collect(),
      ..PackageType::default()
    }],
    enums: vec![],
  })
}

fn core_property_field_name(q_name: &str) -> String {
  match q_name {
    "cp:category" => "category",
    "cp:contentStatus" => "content_status",
    "dcterms:created" => "created",
    "dc:creator" => "creator",
    "dc:description" => "description",
    "dc:identifier" => "identifier",
    "cp:keywords" => "keywords",
    "dc:language" => "language",
    "cp:lastModifiedBy" => "last_modified_by",
    "cp:lastPrinted" => "last_printed",
    "dcterms:modified" => "modified",
    "cp:revision" => "revision",
    "dc:subject" => "subject",
    "dc:title" => "title",
    "cp:version" => "version",
    _ => q_name,
  }
  .to_string()
}

#[derive(Default)]
struct ParsedXsd {
  target_namespace: String,
  complex_types: BTreeMap<String, ParsedComplexType>,
  simple_types: BTreeMap<String, Vec<String>>,
}

#[derive(Default)]
struct ParsedComplexType {
  children: Vec<String>,
  attributes: Vec<PackageAttribute>,
}

fn parse_xsd(source: &str) -> Result<ParsedXsd> {
  let mut reader = Reader::from_str(source);
  reader.config_mut().trim_text(true);

  let mut parsed = ParsedXsd::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"schema" => {
          parsed.target_namespace = required_attr(&reader, &e, b"targetNamespace")?;
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
  let mut complex_type = ParsedComplexType::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"sequence" | b"choice" | b"all" | b"simpleContent" | b"extension" => {}
        b"element" => {
          complex_type.children.push(parse_child_element(reader, &e)?);
          skip_element(reader, e.name().as_ref())?;
        }
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
          skip_element(reader, e.name().as_ref())?;
        }
        _ => skip_element(reader, e.name().as_ref())?,
      },
      Event::Empty(e) => match local_name(e.name().as_ref()) {
        b"element" => complex_type.children.push(parse_child_element(reader, &e)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
        }
        _ => {}
      },
      Event::End(e) if local_name(e.name().as_ref()) == b"complexType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err("unexpected EOF in complexType".into()),
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
      Event::Eof => return Err("unexpected EOF in simpleType".into()),
      _ => {}
    }
  }

  Ok((name, values))
}

fn parse_child_element(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<String> {
  if let Some(reference) = optional_attr(reader, element, b"ref")? {
    return Ok(reference);
  }

  let name = required_attr(reader, element, b"name")?;
  let prefix = element_prefix(reader, element)?;

  if prefix.is_empty() {
    Ok(name)
  } else {
    Ok(format!("{prefix}:{name}"))
  }
}

fn parse_attribute(
  reader: &Reader<&[u8]>,
  element: &BytesStart<'_>,
) -> Result<Option<PackageAttribute>> {
  let Some(name) = optional_attr(reader, element, b"name")? else {
    return Ok(None);
  };

  Ok(Some(PackageAttribute {
    field: attribute_field_name(&name),
    q_name: name,
    r#type: map_xsd_type_to_package_type(
      optional_attr(reader, element, b"type")?
        .unwrap_or_else(|| "String".to_string())
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
    "Id" => "id",
    "Extension" => "extension",
    "ContentType" => "content_type",
    "PartName" => "part_name",
    _ => name,
  }
  .to_string()
}

fn map_xsd_type_to_package_type(value: &str) -> String {
  match strip_prefix(value) {
    "string" | "anyURI" | "ID" | "dateTime" | "ST_ContentType" | "ST_Extension" => {
      "String".to_string()
    }
    other if other.starts_with("ST_") => other.trim_start_matches("ST_").to_string(),
    other => other.to_string(),
  }
}

fn element_prefix(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<String> {
  let Some(type_name) = optional_attr(reader, element, b"type")? else {
    return Ok("".to_string());
  };

  Ok(
    match strip_prefix(type_name.as_str()) {
      "CT_Keywords" => "cp",
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
