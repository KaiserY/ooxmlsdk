use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::{Reader, escape::unescape};
use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;

use crate::Result;
use crate::simple_type::simple_type_mapping;

#[derive(Debug, Default)]
pub(crate) struct ParsedXsd {
  pub target_namespace: String,
  pub root_elements: BTreeMap<String, ParsedComplexType>,
  pub complex_types: BTreeMap<String, ParsedComplexType>,
  pub groups: BTreeMap<String, ParsedParticleNode>,
  pub simple_types: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedComplexType {
  pub _mixed: bool,
  pub top_level_particle: Option<ParsedParticle>,
  pub particle: Option<Box<ParsedParticleNode>>,
  pub children: Vec<ParsedChildElement>,
  pub attributes: Vec<ParsedAttribute>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParsedParticleKind {
  Sequence,
  Choice,
  All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ParsedParticle {
  pub kind: ParsedParticleKind,
  pub min_occurs: u64,
  pub max_occurs: u64,
}

#[derive(Clone, Debug)]
pub(crate) enum ParsedParticleNode {
  Group {
    particle: ParsedParticle,
    children: Vec<ParsedParticleNode>,
  },
  Element(ParsedChildElement),
  GroupRef {
    _reference: String,
    _min_occurs: u64,
    _max_occurs: u64,
  },
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedChildElement {
  pub q_name: String,
  pub r#type: String,
  pub min_occurs: u64,
  pub max_occurs: u64,
  pub complex_type: Option<ParsedComplexType>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedAttribute {
  pub field: String,
  pub q_name: String,
  pub r#type: String,
  pub xsd_type: String,
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
        b"group" => {
          let (name, particle) = parse_group(&mut reader, e)?;
          parsed.groups.insert(name, particle);
        }
        b"simpleType" => {
          if optional_attr(&reader, &e, b"name")?.is_some() {
            let (name, values) = parse_simple_type(&mut reader, e)?;
            parsed.simple_types.insert(name, values);
          } else {
            skip_element(&mut reader, e.name().as_ref())?;
          }
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

#[cfg(test)]
pub(crate) fn repeatable_choice_element_names(
  xsd: &ParsedXsd,
  complex_type_name: &str,
) -> BTreeSet<String> {
  let Some(complex_type) = xsd.complex_types.get(complex_type_name) else {
    return BTreeSet::new();
  };
  let Some(particle) = &complex_type.particle else {
    return BTreeSet::new();
  };

  let mut names = BTreeSet::new();
  let mut group_stack = BTreeSet::new();
  collect_repeatable_choice_element_names(
    xsd,
    particle.as_ref(),
    false,
    false,
    &mut group_stack,
    &mut names,
  );
  names
}

#[cfg(test)]
fn collect_repeatable_choice_element_names(
  xsd: &ParsedXsd,
  node: &ParsedParticleNode,
  inherited_repeated: bool,
  inherited_choice: bool,
  group_stack: &mut BTreeSet<String>,
  names: &mut BTreeSet<String>,
) {
  match node {
    ParsedParticleNode::Group { particle, children } => {
      let repeated = inherited_repeated || particle.max_occurs > 1;
      let choice = inherited_choice || particle.kind == ParsedParticleKind::Choice;
      for child in children {
        collect_repeatable_choice_element_names(xsd, child, repeated, choice, group_stack, names);
      }
    }
    ParsedParticleNode::Element(element) => {
      if inherited_choice && (inherited_repeated || element.max_occurs > 1) {
        names.insert(xsd_local_name(element.q_name.as_str()).to_string());
      }
    }
    ParsedParticleNode::GroupRef {
      _reference,
      _max_occurs,
      ..
    } => {
      let group_name = xsd_local_name(_reference);
      if !group_stack.insert(group_name.to_string()) {
        return;
      }
      if let Some(group) = xsd.groups.get(group_name) {
        collect_repeatable_choice_element_names(
          xsd,
          group,
          inherited_repeated || *_max_occurs > 1,
          inherited_choice,
          group_stack,
          names,
        );
      }
      group_stack.remove(group_name);
    }
  }
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
        b"sequence" | b"choice" | b"all" => {
          let node = parse_particle_node(reader, e, false)?;
          if complex_type.particle.is_none() {
            if let ParsedParticleNode::Group { particle, .. } = &node {
              complex_type.top_level_particle = Some(*particle);
            }
            collect_particle_elements(&node, &mut complex_type.children);
            complex_type.particle = Some(Box::new(node));
          }
        }
        b"simpleContent" | b"extension" => {}
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
        b"sequence" | b"choice" | b"all" => {
          let node = parse_particle_node(reader, e, true)?;
          if complex_type.particle.is_none() {
            if let ParsedParticleNode::Group { particle, .. } = &node {
              complex_type.top_level_particle = Some(*particle);
            }
            complex_type.particle = Some(Box::new(node));
          }
        }
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

fn parse_group(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
) -> Result<(String, ParsedParticleNode)> {
  let name = required_attr(reader, &start, b"name")?;

  loop {
    match reader.read_event()? {
      Event::Start(e) if is_particle(e.name().as_ref()) => {
        let particle = parse_particle_node(reader, e, false)?;
        skip_to_end(reader, start.name().as_ref())?;
        return Ok((name, particle));
      }
      Event::Empty(e) if is_particle(e.name().as_ref()) => {
        let particle = parse_particle_node(reader, e, true)?;
        return Ok((name, particle));
      }
      Event::End(e) if e.name().as_ref() == start.name().as_ref() => {
        return Err(format!("group {} does not contain a particle", name).into());
      }
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in group {}", name).into()),
      _ => {}
    }
  }
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
    r#type: optional_attr(reader, element, b"type")?.unwrap_or_default(),
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

fn parse_particle_node(
  reader: &mut Reader<&[u8]>,
  element: BytesStart<'_>,
  empty: bool,
) -> Result<ParsedParticleNode> {
  let particle = parse_particle(reader, &element)?;
  let mut children = Vec::new();

  if !empty {
    loop {
      match reader.read_event()? {
        Event::Start(e) if is_particle(e.name().as_ref()) => {
          children.push(parse_particle_node(reader, e, false)?);
        }
        Event::Empty(e) if is_particle(e.name().as_ref()) => {
          children.push(parse_particle_node(reader, e, true)?);
        }
        Event::Start(e) if local_name(e.name().as_ref()) == b"element" => {
          children.push(ParsedParticleNode::Element(parse_child_element(
            reader, &e, false,
          )?));
        }
        Event::Empty(e) if local_name(e.name().as_ref()) == b"element" => {
          children.push(ParsedParticleNode::Element(parse_child_element(
            reader, &e, true,
          )?));
        }
        Event::Start(e) if local_name(e.name().as_ref()) == b"group" => {
          children.push(parse_group_ref(reader, &e)?);
          skip_element(reader, e.name().as_ref())?;
        }
        Event::Empty(e) if local_name(e.name().as_ref()) == b"group" => {
          children.push(parse_group_ref(reader, &e)?);
        }
        Event::End(e) if e.name().as_ref() == element.name().as_ref() => break,
        Event::Text(_) | Event::Comment(_) => {}
        Event::Eof => {
          return Err(
            format!(
              "unexpected EOF in particle {}",
              String::from_utf8_lossy(element.name().as_ref())
            )
            .into(),
          );
        }
        _ => {}
      }
    }
  }

  Ok(ParsedParticleNode::Group { particle, children })
}

fn parse_group_ref(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<ParsedParticleNode> {
  Ok(ParsedParticleNode::GroupRef {
    _reference: required_attr(reader, element, b"ref")?,
    _min_occurs: parse_min_occurs(reader, element)?,
    _max_occurs: parse_max_occurs(reader, element)?,
  })
}

fn parse_particle(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<ParsedParticle> {
  let kind = match local_name(element.name().as_ref()) {
    b"sequence" => ParsedParticleKind::Sequence,
    b"choice" => ParsedParticleKind::Choice,
    b"all" => ParsedParticleKind::All,
    other => {
      return Err(format!("unsupported particle {}", String::from_utf8_lossy(other)).into());
    }
  };

  Ok(ParsedParticle {
    kind,
    min_occurs: parse_min_occurs(reader, element)?,
    max_occurs: parse_max_occurs(reader, element)?,
  })
}

fn collect_particle_elements(node: &ParsedParticleNode, children: &mut Vec<ParsedChildElement>) {
  match node {
    ParsedParticleNode::Group {
      children: particle_children,
      ..
    } => {
      for child in particle_children {
        collect_particle_elements(child, children);
      }
    }
    ParsedParticleNode::Element(element) => children.push(element.clone()),
    ParsedParticleNode::GroupRef { .. } => {}
  }
}

fn parse_min_occurs(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<u64> {
  Ok(
    optional_attr(reader, element, b"minOccurs")?
      .as_deref()
      .unwrap_or("1")
      .parse()
      .unwrap_or(1),
  )
}

fn parse_max_occurs(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<u64> {
  Ok(
    optional_attr(reader, element, b"maxOccurs")?
      .as_deref()
      .map(|value| {
        if value == "unbounded" {
          u64::MAX
        } else {
          value.parse().unwrap_or(1)
        }
      })
      .unwrap_or(1),
  )
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
      xsd_type: String::new(),
      required: optional_attr(reader, element, b"use")?.as_deref() == Some("required"),
    }));
  }

  let Some(name) = optional_attr(reader, element, b"name")? else {
    return Ok(None);
  };

  let xsd_type =
    optional_attr(reader, element, b"type")?.unwrap_or_else(|| "xsd:string".to_string());

  Ok(Some(ParsedAttribute {
    field: attribute_field_name(&name),
    q_name: name,
    r#type: map_xsd_type_to_schema_type(xsd_type.as_str()),
    xsd_type,
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
        unescape(&reader.decoder().decode(attr.value.as_ref())?)?.into_owned(),
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

fn skip_to_end(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<()> {
  loop {
    match reader.read_event()? {
      Event::Start(e) if e.name().as_ref() == tag => skip_element(reader, e.name().as_ref())?,
      Event::End(e) if e.name().as_ref() == tag => return Ok(()),
      Event::Eof => return Err("unexpected EOF while skipping to end".into()),
      _ => {}
    }
  }
}

fn is_particle(name: &[u8]) -> bool {
  matches!(local_name(name), b"sequence" | b"choice" | b"all")
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

#[cfg(test)]
fn xsd_local_name(value: &str) -> &str {
  strip_prefix(value)
}

#[cfg(test)]
mod tests {
  use super::{ParsedParticleKind, ParsedParticleNode, parse_xsd, repeatable_choice_element_names};

  #[test]
  fn captures_top_level_choice_particle() {
    let xsd = parse_xsd(
      r#"
      <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
        <xsd:complexType name="CT_Font">
          <xsd:choice maxOccurs="unbounded">
            <xsd:element name="name" type="xsd:string"/>
            <xsd:element name="sz" type="xsd:string"/>
          </xsd:choice>
        </xsd:complexType>
      </xsd:schema>
      "#,
    )
    .expect("parse xsd");

    let font = xsd.complex_types.get("CT_Font").expect("CT_Font");
    let particle = font.top_level_particle.expect("top level particle");

    assert_eq!(particle.kind, ParsedParticleKind::Choice);
    assert_eq!(particle.min_occurs, 1);
    assert_eq!(particle.max_occurs, u64::MAX);
  }

  #[test]
  fn resolves_repeated_elements_through_group_refs() {
    let xsd = parse_xsd(
      r#"
      <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
        <xsd:group name="EG_RPrBase">
          <xsd:choice>
            <xsd:element name="b" type="CT_OnOff"/>
            <xsd:element name="sz" type="CT_HpsMeasure"/>
          </xsd:choice>
        </xsd:group>
        <xsd:group name="EG_RPrContent">
          <xsd:sequence>
            <xsd:group ref="EG_RPrBase" minOccurs="0" maxOccurs="unbounded"/>
            <xsd:element name="rPrChange" type="CT_RPrChange" minOccurs="0"/>
          </xsd:sequence>
        </xsd:group>
        <xsd:complexType name="CT_RPr">
          <xsd:sequence>
            <xsd:group ref="EG_RPrContent" minOccurs="0"/>
          </xsd:sequence>
        </xsd:complexType>
      </xsd:schema>
      "#,
    )
    .expect("parse xsd");

    assert!(matches!(
      xsd.groups.get("EG_RPrBase"),
      Some(ParsedParticleNode::Group { .. })
    ));
    assert_eq!(
      repeatable_choice_element_names(&xsd, "CT_RPr")
        .into_iter()
        .collect::<Vec<_>>(),
      vec!["b".to_string(), "sz".to_string()],
    );
  }
}
