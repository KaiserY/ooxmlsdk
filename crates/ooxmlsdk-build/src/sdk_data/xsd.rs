use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::{Reader, escape::unescape};
use std::collections::{BTreeMap, BTreeSet};

use crate::Result;
use crate::simple_type::simple_type_mapping;

#[derive(Debug, Default)]
pub(crate) struct ParsedXsd {
  pub target_namespace: String,
  /// `xmlns:*` declarations from the schema element: prefix -> URI.
  pub prefixes: BTreeMap<String, String>,
  pub root_elements: BTreeMap<String, ParsedComplexType>,
  pub complex_types: BTreeMap<String, ParsedComplexType>,
  pub groups: BTreeMap<String, ParsedParticleNode>,
  pub simple_types: BTreeMap<String, Vec<String>>,
  /// Named simple types: name -> `xsd:restriction`/`xsd:extension` base QName.
  pub simple_type_bases: BTreeMap<String, String>,
  pub imports: Vec<ParsedImport>,
  pub attribute_groups: BTreeMap<String, ParsedAttributeGroup>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedImport {
  pub namespace: String,
  pub schema_location: String,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedAttributeGroup {
  pub attributes: Vec<ParsedAttribute>,
  pub refs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedComplexType {
  pub _mixed: bool,
  pub top_level_particle: Option<ParsedParticle>,
  pub particle: Option<Box<ParsedParticleNode>>,
  pub children: Vec<ParsedChildElement>,
  pub attributes: Vec<ParsedAttribute>,
  /// For top-level `xs:element` declarations: the referenced type QName.
  pub element_type: String,
  pub is_abstract: bool,
  pub documentation: String,
  pub has_any_attribute: bool,
  pub attribute_group_refs: Vec<String>,
  /// For `xs:simpleContent` extension/restriction: the base QName.
  pub text_value_type: Option<String>,
  /// For `xs:complexContent`/`xs:simpleContent`: the derivation (base + body),
  /// kept separate from the direct `particle`/`children`/`attributes`.
  /// simpleContent attributes stay on `derivation.body`. `opc_schemas` appends
  /// those when it builds Relationship and Keyword.
  pub derivation: Option<Box<ParsedDerivation>>,
}

/// `xs:complexContent` / `xs:simpleContent` derivation: base QName plus the
/// `xs:extension`/`xs:restriction` body.
#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedDerivation {
  pub base: String,
  pub is_extension: bool,
  pub is_restriction: bool,
  pub body: ParsedComplexType,
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
  Element(Box<ParsedChildElement>),
  GroupRef {
    _reference: String,
    _min_occurs: u64,
    _max_occurs: u64,
  },
  /// `xs:any` wildcard, including its occurrence bounds.
  Any {
    min_occurs: u64,
    max_occurs: u64,
  },
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedChildElement {
  pub q_name: String,
  /// Distinguishes `ref` from `name`, including unprefixed global references.
  pub is_reference: bool,
  /// Namespace prefix of the element: the `ref` prefix, or empty for a
  /// name-based (own-namespace) child. Unlike `q_name`, this is not rewritten
  /// for OPC consumers.
  pub element_prefix: String,
  pub r#type: String,
  pub min_occurs: u64,
  pub max_occurs: u64,
  pub complex_type: Option<ParsedComplexType>,
  pub documentation: String,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ParsedAttribute {
  pub field: String,
  pub q_name: String,
  pub r#type: String,
  pub xsd_type: String,
  pub required: bool,
  pub documentation: String,
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
          parsed.prefixes = collect_namespaces(&reader, &e)?;
        }
        b"element" => {
          let (name, complex_type) = parse_element(&mut reader, e, false)?;
          parsed.root_elements.insert(name, complex_type);
        }
        b"import" => {
          parsed.imports.push(parse_import(&reader, &e));
          skip_element(&mut reader, e.name().as_ref())?;
        }
        b"attributeGroup" => {
          let (name, group) = parse_attribute_group(&mut reader, e, false)?;
          if !name.is_empty() {
            parsed.attribute_groups.insert(name, group);
          }
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
            let (name, values, base) = parse_simple_type(&mut reader, e)?;
            if let Some(base) = base {
              parsed.simple_type_bases.insert(name.clone(), base);
            }
            parsed.simple_types.insert(name, values);
          } else {
            skip_element(&mut reader, e.name().as_ref())?;
          }
        }
        _ => skip_element(&mut reader, e.name().as_ref())?,
      },
      Event::Empty(e) => match local_name(e.name().as_ref()) {
        // Self-closing top-level declarations (e.g. `<xsd:element ... />`).
        b"element" => {
          let (name, complex_type) = parse_element(&mut reader, e, true)?;
          parsed.root_elements.insert(name, complex_type);
        }
        b"import" => parsed.imports.push(parse_import(&reader, &e)),
        b"attributeGroup" => {
          let (name, group) = parse_attribute_group(&mut reader, e, true)?;
          if !name.is_empty() {
            parsed.attribute_groups.insert(name, group);
          }
        }
        b"complexType" => {
          if let Some(name) = optional_attr(&reader, &e, b"name")? {
            parsed
              .complex_types
              .insert(name, ParsedComplexType::default());
          }
        }
        b"simpleType" => {
          if let Some(name) = optional_attr(&reader, &e, b"name")? {
            parsed.simple_types.insert(name, Vec::new());
          }
        }
        _ => {}
      },
      Event::Text(_) | Event::Comment(_) | Event::Decl(_) => {}
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

pub(crate) fn repeatable_group_choice_element_names(
  xsd: &ParsedXsd,
  complex_type_name: &str,
  group_name: &str,
) -> BTreeSet<String> {
  let Some(complex_type) = xsd.complex_types.get(complex_type_name) else {
    return BTreeSet::new();
  };
  let Some(particle) = &complex_type.particle else {
    return BTreeSet::new();
  };

  let mut names = BTreeSet::new();
  let mut group_stack = BTreeSet::new();
  collect_repeatable_group_choice_element_names(
    xsd,
    particle.as_ref(),
    group_name,
    &mut group_stack,
    &mut names,
  );
  names
}

fn collect_repeatable_group_choice_element_names(
  xsd: &ParsedXsd,
  node: &ParsedParticleNode,
  target_group_name: &str,
  group_stack: &mut BTreeSet<String>,
  names: &mut BTreeSet<String>,
) {
  match node {
    ParsedParticleNode::Group { children, .. } => {
      for child in children {
        collect_repeatable_group_choice_element_names(
          xsd,
          child,
          target_group_name,
          group_stack,
          names,
        );
      }
    }
    ParsedParticleNode::Element(_) => {}
    ParsedParticleNode::Any { .. } => {}
    ParsedParticleNode::GroupRef {
      _reference,
      _max_occurs,
      ..
    } => {
      let group_name = xsd_local_name(_reference);
      if group_name == target_group_name && *_max_occurs > 1 {
        collect_choice_group_element_names(xsd, group_name, group_stack, names);
        return;
      }

      if !group_stack.insert(group_name.to_string()) {
        return;
      }
      if let Some(group) = xsd.groups.get(group_name) {
        collect_repeatable_group_choice_element_names(
          xsd,
          group,
          target_group_name,
          group_stack,
          names,
        );
      }
      group_stack.remove(group_name);
    }
  }
}

fn collect_choice_group_element_names(
  xsd: &ParsedXsd,
  group_name: &str,
  group_stack: &mut BTreeSet<String>,
  names: &mut BTreeSet<String>,
) {
  if !group_stack.insert(group_name.to_string()) {
    return;
  }
  if let Some(ParsedParticleNode::Group { particle, children }) = xsd.groups.get(group_name)
    && particle.kind == ParsedParticleKind::Choice
  {
    for child in children {
      collect_group_element_names(xsd, child, group_stack, names);
    }
  }
  group_stack.remove(group_name);
}

fn collect_group_element_names(
  xsd: &ParsedXsd,
  node: &ParsedParticleNode,
  group_stack: &mut BTreeSet<String>,
  names: &mut BTreeSet<String>,
) {
  match node {
    ParsedParticleNode::Element(element) => {
      names.insert(xsd_local_name(element.q_name.as_str()).to_string());
    }
    ParsedParticleNode::Group { children, .. } => {
      for child in children {
        collect_group_element_names(xsd, child, group_stack, names);
      }
    }
    ParsedParticleNode::GroupRef { _reference, .. } => {
      let group_name = xsd_local_name(_reference);
      collect_choice_group_element_names(xsd, group_name, group_stack, names);
    }
    ParsedParticleNode::Any { .. } => {}
  }
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
    ParsedParticleNode::Any { .. } => {}
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
    is_abstract: optional_attr(reader, &start, b"abstract")?.as_deref() == Some("true"),
    ..ParsedComplexType::default()
  };
  parse_type_content(reader, &mut complex_type, b"complexType", true)?;
  Ok(complex_type)
}

/// Parse the content of a type body until `end_tag`. When `allow_derivation` is
/// set, `xs:complexContent`/`xs:simpleContent` are captured into
/// `complex_type.derivation` and kept off the direct particle, children, and
/// attributes. simpleContent attributes stay on the derivation body.
fn parse_type_content(
  reader: &mut Reader<&[u8]>,
  complex_type: &mut ParsedComplexType,
  end_tag: &[u8],
  allow_derivation: bool,
) -> Result<()> {
  loop {
    match reader.read_event()? {
      Event::Start(e) => match local_name(e.name().as_ref()) {
        b"annotation" => complex_type.documentation = parse_annotation(reader)?,
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
        b"complexContent" if allow_derivation => {
          complex_type.derivation = Some(Box::new(parse_complex_content(reader, false)?));
        }
        b"simpleContent" if allow_derivation => {
          complex_type.derivation = Some(Box::new(parse_complex_content(reader, true)?));
        }
        b"extension" | b"restriction" => {
          // Direct (unwrapped) derivation: preserve the pre-existing behaviour of
          // parsing its body into this type's direct fields.
          parse_type_content(reader, complex_type, local_name(e.name().as_ref()), false)?;
        }
        b"anyAttribute" => {
          complex_type.has_any_attribute = true;
          skip_element(reader, e.name().as_ref())?;
        }
        b"element" => complex_type
          .children
          .push(parse_child_element(reader, &e, false)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
          skip_element(reader, e.name().as_ref())?;
        }
        b"attributeGroup" => {
          if let Some(reference) = optional_attr(reader, &e, b"ref")? {
            complex_type.attribute_group_refs.push(reference);
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
        b"anyAttribute" => complex_type.has_any_attribute = true,
        b"element" => complex_type
          .children
          .push(parse_child_element(reader, &e, true)?),
        b"attribute" => {
          if let Some(attribute) = parse_attribute(reader, &e)? {
            complex_type.attributes.push(attribute);
          }
        }
        b"attributeGroup" => {
          if let Some(reference) = optional_attr(reader, &e, b"ref")? {
            complex_type.attribute_group_refs.push(reference);
          }
        }
        _ => {}
      },
      Event::End(e) if local_name(e.name().as_ref()) == end_tag => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err("unexpected EOF while parsing type content".into()),
      _ => {}
    }
  }

  Ok(())
}

/// Parse `xs:complexContent`/`xs:simpleContent` into a [`ParsedDerivation`].
fn parse_complex_content(reader: &mut Reader<&[u8]>, simple: bool) -> Result<ParsedDerivation> {
  let mut derivation = ParsedDerivation::default();

  loop {
    match reader.read_event()? {
      Event::Start(e) if matches!(local_name(e.name().as_ref()), b"extension" | b"restriction") => {
        derivation.base = optional_attr(reader, &e, b"base")?.unwrap_or_default();
        derivation.is_extension = local_name(e.name().as_ref()) == b"extension";
        derivation.is_restriction = local_name(e.name().as_ref()) == b"restriction";
        parse_type_content(
          reader,
          &mut derivation.body,
          local_name(e.name().as_ref()),
          false,
        )?;
      }
      Event::Empty(e) if matches!(local_name(e.name().as_ref()), b"extension" | b"restriction") => {
        derivation.base = optional_attr(reader, &e, b"base")?.unwrap_or_default();
        derivation.is_extension = local_name(e.name().as_ref()) == b"extension";
        derivation.is_restriction = local_name(e.name().as_ref()) == b"restriction";
      }
      Event::End(e)
        if matches!(
          local_name(e.name().as_ref()),
          b"complexContent" | b"simpleContent"
        ) =>
      {
        break;
      }
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => break,
      _ => {}
    }
  }

  if simple {
    derivation.body.text_value_type = Some(derivation.base.clone());
  }

  Ok(derivation)
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

fn parse_import(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> ParsedImport {
  ParsedImport {
    namespace: optional_attr(reader, element, b"namespace")
      .ok()
      .flatten()
      .unwrap_or_default(),
    schema_location: optional_attr(reader, element, b"schemaLocation")
      .ok()
      .flatten()
      .unwrap_or_default(),
  }
}

fn parse_attribute_group(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
  empty: bool,
) -> Result<(String, ParsedAttributeGroup)> {
  let name = optional_attr(reader, &start, b"name")?.unwrap_or_default();
  let mut group = ParsedAttributeGroup::default();
  if let Some(reference) = optional_attr(reader, &start, b"ref")? {
    group.refs.push(reference);
  }
  if empty {
    return Ok((name, group));
  }

  loop {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"attribute" => {
        if let Some(attribute) = parse_attribute(reader, &e)? {
          group.attributes.push(attribute);
        }
        skip_element(reader, e.name().as_ref())?;
      }
      Event::Empty(e) if local_name(e.name().as_ref()) == b"attribute" => {
        if let Some(attribute) = parse_attribute(reader, &e)? {
          group.attributes.push(attribute);
        }
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"attributeGroup" => {
        if let Some(reference) = optional_attr(reader, &e, b"ref")? {
          group.refs.push(reference);
        }
        skip_element(reader, e.name().as_ref())?;
      }
      Event::Empty(e) if local_name(e.name().as_ref()) == b"attributeGroup" => {
        if let Some(reference) = optional_attr(reader, &e, b"ref")? {
          group.refs.push(reference);
        }
      }
      Event::Start(e) => skip_element(reader, e.name().as_ref())?,
      Event::End(e) if e.name().as_ref() == start.name().as_ref() => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in attributeGroup {name}").into()),
      _ => {}
    }
  }

  Ok((name, group))
}

fn parse_element(
  reader: &mut Reader<&[u8]>,
  start: BytesStart<'_>,
  empty: bool,
) -> Result<(String, ParsedComplexType)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut complex_type = ParsedComplexType {
    element_type: optional_attr(reader, &start, b"type")?.unwrap_or_default(),
    is_abstract: optional_attr(reader, &start, b"abstract")?.as_deref() == Some("true"),
    ..ParsedComplexType::default()
  };

  if empty {
    return Ok((name, complex_type));
  }

  loop {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"complexType" => {
        let body = parse_complex_type_body(reader, e)?;
        let element_type = std::mem::take(&mut complex_type.element_type);
        let is_abstract = complex_type.is_abstract;
        let documentation = std::mem::take(&mut complex_type.documentation);
        complex_type = body;
        complex_type.element_type = element_type;
        complex_type.is_abstract = is_abstract;
        complex_type.documentation = documentation;
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"annotation" => {
        complex_type.documentation = parse_annotation(reader)?;
      }
      Event::Empty(e) if local_name(e.name().as_ref()) == b"complexType" => {}
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
) -> Result<(String, Vec<String>, Option<String>)> {
  let name = required_attr(reader, &start, b"name")?;
  let mut values = vec![];
  let mut base = None;

  loop {
    match reader.read_event()? {
      Event::Empty(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"enumeration" => {
        values.push(required_attr(reader, &e, b"value")?);
        skip_element(reader, e.name().as_ref())?;
      }
      Event::Empty(e) if matches!(local_name(e.name().as_ref()), b"restriction" | b"extension") => {
        if let Some(value) = optional_attr(reader, &e, b"base")? {
          base = Some(value);
        }
      }
      Event::Start(e) if matches!(local_name(e.name().as_ref()), b"restriction" | b"extension") => {
        if let Some(value) = optional_attr(reader, &e, b"base")? {
          base = Some(value);
        }
      }
      Event::End(e) if local_name(e.name().as_ref()) == b"simpleType" => break,
      Event::Text(_) | Event::Comment(_) => {}
      Event::Eof => return Err(format!("unexpected EOF in simpleType {}", name).into()),
      _ => {}
    }
  }

  Ok((name, values, base))
}

fn parse_child_element(
  reader: &mut Reader<&[u8]>,
  element: &BytesStart<'_>,
  empty: bool,
) -> Result<ParsedChildElement> {
  let reference = optional_attr(reader, element, b"ref")?;
  let q_name = if let Some(reference) = reference.clone() {
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
  let element_prefix = reference
    .as_deref()
    .and_then(|reference| reference.split_once(':'))
    .map(|(prefix, _)| prefix.to_string())
    .unwrap_or_default();

  let mut complex_type = None;
  let mut documentation = String::new();
  let mut depth = 1usize;
  let saw_body = !empty;

  while saw_body && depth > 0 {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"complexType" => {
        complex_type = Some(parse_complex_type_body(reader, e)?);
      }
      Event::Start(e) if local_name(e.name().as_ref()) == b"annotation" => {
        documentation = parse_annotation(reader)?;
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
    is_reference: reference.is_some(),
    element_prefix,
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
    documentation,
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
          children.push(ParsedParticleNode::Element(Box::new(parse_child_element(
            reader, &e, false,
          )?)));
        }
        Event::Empty(e) if local_name(e.name().as_ref()) == b"element" => {
          children.push(ParsedParticleNode::Element(Box::new(parse_child_element(
            reader, &e, true,
          )?)));
        }
        Event::Start(e) if local_name(e.name().as_ref()) == b"group" => {
          children.push(parse_group_ref(reader, &e)?);
          skip_element(reader, e.name().as_ref())?;
        }
        Event::Empty(e) if local_name(e.name().as_ref()) == b"group" => {
          children.push(parse_group_ref(reader, &e)?);
        }
        Event::Empty(e) if local_name(e.name().as_ref()) == b"any" => {
          children.push(ParsedParticleNode::Any {
            min_occurs: parse_min_occurs(reader, &e)?,
            max_occurs: parse_max_occurs(reader, &e)?,
          });
        }
        Event::Start(e) if local_name(e.name().as_ref()) == b"any" => {
          children.push(ParsedParticleNode::Any {
            min_occurs: parse_min_occurs(reader, &e)?,
            max_occurs: parse_max_occurs(reader, &e)?,
          });
          skip_element(reader, e.name().as_ref())?;
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
    ParsedParticleNode::Element(element) => children.push((**element).clone()),
    ParsedParticleNode::GroupRef { .. } => {}
    ParsedParticleNode::Any { .. } => {}
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
      documentation: String::new(),
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
    documentation: String::new(),
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

/// Collect `xmlns` / `xmlns:*` declarations from a schema element: prefix -> URI.
fn collect_namespaces(
  reader: &Reader<&[u8]>,
  start: &BytesStart<'_>,
) -> Result<BTreeMap<String, String>> {
  let mut namespaces = BTreeMap::new();

  for attr in start.attributes().with_checks(false) {
    let attr = attr?;
    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
    let value = unescape(&reader.decoder().decode(attr.value.as_ref())?)?.into_owned();
    if key == "xmlns" {
      namespaces.insert(String::new(), value);
    } else if let Some(prefix) = key.strip_prefix("xmlns:") {
      namespaces.insert(prefix.to_string(), value);
    }
  }

  Ok(namespaces)
}

/// Consume an `xs:annotation` element and return its concatenated
/// `xs:documentation` text.
fn parse_annotation(reader: &mut Reader<&[u8]>) -> Result<String> {
  let mut documentation = String::new();

  loop {
    match reader.read_event()? {
      Event::Start(e) if local_name(e.name().as_ref()) == b"documentation" => loop {
        match reader.read_event()? {
          Event::Text(text) => {
            let decoded = reader.decoder().decode(text.as_ref())?;
            documentation.push_str(&unescape(&decoded)?);
          }
          Event::CData(data) => documentation.push_str(&String::from_utf8_lossy(data.as_ref())),
          Event::End(e) if local_name(e.name().as_ref()) == b"documentation" => break,
          Event::Eof => return Ok(documentation.trim().to_string()),
          _ => {}
        }
      },
      Event::End(e) if local_name(e.name().as_ref()) == b"annotation" => break,
      Event::Eof => break,
      _ => {}
    }
  }

  Ok(documentation.trim().to_string())
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

fn xsd_local_name(value: &str) -> &str {
  strip_prefix(value)
}

#[cfg(test)]
mod tests {
  use super::repeatable_group_choice_element_names;
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

  #[test]
  fn resolves_repeated_named_choice_group_elements() {
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

    assert_eq!(
      repeatable_group_choice_element_names(&xsd, "CT_RPr", "EG_RPrBase")
        .into_iter()
        .collect::<Vec<_>>(),
      vec!["b".to_string(), "sz".to_string()],
    );
  }
}
