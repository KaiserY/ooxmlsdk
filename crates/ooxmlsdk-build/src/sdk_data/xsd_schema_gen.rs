//! General XSD -> `data/schemas` (`OpenXmlSchema`) producer.
//!
//! `gen_schemas` consumes stage-1 `OpenXmlSchema` metadata (the checked-in
//! `data/schemas/*.json`, produced by the .NET Open XML SDK backend). This module
//! produces that same stage-1 model directly from a parsed XSD, so namespaces the
//! backend does not cover (notably Visio) can ride the existing pipeline.
//!
//! The output is deliberately a *projection* of the .NET model: fields the .NET
//! SDK derives with non-XSD heuristics (`ClassName`, `Summary`, enum/facet names,
//! attribute `Type`/`Validators`, `CompositeType`, and root `BaseClass`) are
//! best-effort or left default. The regeneration oracle
//! (`sdk_data::oracle`) compares only XSD-derivable projections.
//!
//! `additionalCharacteristics`, `bibliography`, and `pml` match that projection.
//! For `pml`, the checked-in JSON still contains SDK rewrites the transitional
//! XSD does not state (per-parent extension-list types, Office 2010 attributes
//! and elements, versioned occurs, and renamed or subset enums). The `pml`
//! acceptance test excludes those from the comparison. It does not rename them
//! into the generator.

use std::{
  collections::{BTreeMap, BTreeSet, HashMap},
  path::Path,
};

use heck::ToUpperCamelCase;

use crate::{
  Result,
  sdk_data::{
    open_xml::{
      OpenXmlSchema, OpenXmlSchemaEnum, OpenXmlSchemaEnumFacet, OpenXmlSchemaType,
      OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeAttributeValidator, OpenXmlSchemaTypeChild,
      OpenXmlSchemaTypeParticle, OpenXmlSchemaTypeParticleOccur,
    },
    xsd::{
      ParsedAttribute, ParsedAttributeGroup, ParsedChildElement, ParsedComplexType,
      ParsedParticleKind, ParsedParticleNode, ParsedXsd,
    },
  },
};

/// Configuration supplied by the caller (namespace prefix, part roots, overrides).
#[derive(Clone, Debug, Default)]
pub struct NamespaceGenConfig {
  /// Prefix for the schema's own target namespace (from `data/namespaces.json`).
  pub prefix: String,
  pub version: Option<String>,
  /// Element local name -> part name, for part root elements.
  pub part_root_elements: HashMap<String, String>,
  pub class_name_overrides: HashMap<String, String>,
  pub enum_name_overrides: HashMap<String, String>,
  /// Target namespace URI -> prefix, for cross-namespace type references.
  pub namespace_prefixes: HashMap<String, String>,
  /// XSD type QName as written (`s:ST_String`) -> type QName to emit
  /// (`b:ST_String255`). Used when the checked-in metadata names a simple type
  /// differently from the XSD `type` attribute.
  pub simple_type_aliases: HashMap<String, String>,
  /// Directory of the XSD being generated, so `xsd:import/@schemaLocation`
  /// siblings can be loaded. Unset when the source is not a file.
  pub schema_dir: Option<std::path::PathBuf>,
}

/// Produce an `OpenXmlSchema` (stage 1) from XSD source text.
pub fn gen_open_xml_schema_from_xsd(
  source: &str,
  cfg: &NamespaceGenConfig,
) -> Result<OpenXmlSchema> {
  let xsd = crate::sdk_data::xsd::parse_xsd(source)?;
  let imported = load_imported_schemas(&xsd, cfg)?;
  Ok(build_open_xml_schema(&xsd, &imported, cfg))
}

struct Schemas<'a> {
  cfg: &'a NamespaceGenConfig,
  main: &'a ParsedXsd,
  imported: &'a BTreeMap<String, ParsedXsd>,
}

pub(crate) fn load_imported_schemas(
  main: &ParsedXsd,
  cfg: &NamespaceGenConfig,
) -> Result<BTreeMap<String, ParsedXsd>> {
  let mut imported = BTreeMap::new();
  let Some(dir) = cfg.schema_dir.as_deref() else {
    return Ok(imported);
  };
  let mut stack = BTreeSet::new();
  stack.insert(main.target_namespace.clone());
  load_imports(main, dir, &mut imported, &mut stack)?;
  Ok(imported)
}

fn load_imports(
  xsd: &ParsedXsd,
  dir: &Path,
  imported: &mut BTreeMap<String, ParsedXsd>,
  stack: &mut BTreeSet<String>,
) -> Result<()> {
  for import in &xsd.imports {
    if import.namespace.is_empty()
      || import.schema_location.is_empty()
      || !stack.insert(import.namespace.clone())
    {
      continue;
    }
    let path = dir.join(&import.schema_location);
    let source = std::fs::read_to_string(&path)?;
    let parsed = crate::sdk_data::xsd::parse_xsd(&source)?;
    load_imports(&parsed, dir, imported, stack)?;
    imported.insert(import.namespace.clone(), parsed);
  }
  Ok(())
}

impl<'a> Schemas<'a> {
  fn prefix_targets_main(&self, prefix: &str) -> bool {
    if prefix.is_empty() || prefix == self.cfg.prefix {
      return true;
    }
    self.main.prefixes.get(prefix) == Some(&self.main.target_namespace)
  }

  fn schema_for_prefix(&self, prefix: &str) -> Option<&ParsedXsd> {
    if self.prefix_targets_main(prefix) {
      return Some(self.main);
    }
    let uri = self.main.prefixes.get(prefix)?;
    self.imported.get(uri)
  }

  /// `(complex type, prefix of the schema that declares it)`.
  fn complex_type(
    &self,
    owner_prefix: &str,
    type_qname: &str,
  ) -> Option<(&ParsedComplexType, String)> {
    let (prefix, local) = split_qname(type_qname);
    let prefix = if prefix.is_empty() {
      owner_prefix
    } else {
      prefix
    };
    let schema = self.schema_for_prefix(prefix)?;
    let complex_type = schema.complex_types.get(local)?;
    Some((complex_type, prefix.to_string()))
  }

  fn group(&self, owner_prefix: &str, reference: &str) -> Option<(&ParsedParticleNode, String)> {
    let (prefix, local) = split_qname(reference);
    let prefix = if prefix.is_empty() {
      owner_prefix
    } else {
      prefix
    };
    let schema = self.schema_for_prefix(prefix)?;
    let group = schema.groups.get(local)?;
    Some((group, prefix.to_string()))
  }

  fn attribute_group(
    &self,
    owner_prefix: &str,
    reference: &str,
  ) -> Option<(&ParsedAttributeGroup, String)> {
    let (prefix, local) = split_qname(reference);
    let prefix = if prefix.is_empty() {
      owner_prefix
    } else {
      prefix
    };
    let schema = self.schema_for_prefix(prefix)?;
    let group = schema.attribute_groups.get(local)?;
    Some((group, prefix.to_string()))
  }
}

fn build_open_xml_schema(
  xsd: &ParsedXsd,
  imported: &BTreeMap<String, ParsedXsd>,
  cfg: &NamespaceGenConfig,
) -> OpenXmlSchema {
  let schemas = Schemas {
    cfg,
    main: xsd,
    imported,
  };
  let shared = shared_complex_types(&schemas);
  let mut types = Vec::new();
  let mut seen = BTreeMap::new();
  let owner = cfg.prefix.as_str();

  // Part root elements first (they define the schema's primary types).
  for (element_local, complex_type) in &xsd.root_elements {
    push_type(
      &mut types,
      &mut seen,
      &schemas,
      &shared,
      owner,
      &complex_type.element_type,
      "",
      element_local,
      complex_type,
      true,
    );
  }

  // Then every element that binds a named type, in each complex type
  // (including derivation bodies).
  for complex_type in xsd.complex_types.values() {
    collect_child_bindings(
      &mut types,
      &mut seen,
      &schemas,
      &shared,
      owner,
      complex_type,
    );
  }
  for complex_type in xsd.root_elements.values() {
    collect_child_bindings(
      &mut types,
      &mut seen,
      &schemas,
      &shared,
      owner,
      complex_type,
    );
  }

  // A complex type bound to several element names also has a type-only entry
  // (`b:CT_NameType/`). The element bindings are marked derived; the type-only
  // entry keeps the children and drops the particle, matching the checked-in
  // metadata.
  for type_key in &shared {
    let Some((body, type_owner)) = schemas.complex_type(owner, type_key) else {
      continue;
    };
    let name = format!("{type_key}/");
    if seen.insert(name.clone(), ()).is_some() {
      continue;
    }
    let mut schema_type = build_type(
      &name,
      qname_local(type_key),
      TypeShape::default(),
      &schemas,
      &type_owner,
      body,
    );
    schema_type.particle = OpenXmlSchemaTypeParticle::default();
    types.push(schema_type);
  }

  let enums = build_enums(xsd, cfg);

  OpenXmlSchema {
    target_namespace: xsd.target_namespace.clone(),
    types,
    enums,
    ..Default::default()
  }
}

#[allow(clippy::too_many_arguments)]
fn push_type(
  types: &mut Vec<OpenXmlSchemaType>,
  seen: &mut BTreeMap<String, ()>,
  schemas: &Schemas<'_>,
  shared: &BTreeSet<String>,
  owner_prefix: &str,
  type_qname: &str,
  element_prefix: &str,
  element_local: &str,
  fallback_body: &ParsedComplexType,
  is_root: bool,
) {
  let name = element_type_name(
    schemas,
    owner_prefix,
    type_qname,
    element_prefix,
    element_local,
  );
  if seen.insert(name.clone(), ()).is_some() {
    return;
  }

  let resolved = schemas.complex_type(owner_prefix, type_qname);
  let (body, type_owner) = match resolved {
    Some((body, prefix)) => (body, prefix),
    None => (fallback_body, owner_prefix.to_string()),
  };
  let leaf_text = is_simple_element_type(schemas, owner_prefix, type_qname);
  let shared_binding = !leaf_text && shared.contains(&qualified_type_key(owner_prefix, type_qname));
  let derived = if leaf_text {
    simple_type_is_derived(schemas, owner_prefix, type_qname)
  } else {
    shared_binding
  };
  // Element bindings that live in an imported schema are named from here, but
  // their type entries belong to that schema's own metadata, not this one.
  let element_owner = if element_prefix.is_empty() {
    owner_prefix.to_string()
  } else {
    resolve_prefix(schemas.main, schemas.cfg, element_prefix)
  };
  if !is_root && element_owner != schemas.cfg.prefix {
    return;
  }
  types.push(build_type(
    &name,
    element_local,
    TypeShape {
      is_root,
      leaf_text,
      derived,
      shared_binding,
    },
    schemas,
    &type_owner,
    body,
  ));
}

#[derive(Default)]
struct TypeShape {
  is_root: bool,
  leaf_text: bool,
  derived: bool,
  shared_binding: bool,
}

fn collect_child_bindings(
  types: &mut Vec<OpenXmlSchemaType>,
  seen: &mut BTreeMap<String, ()>,
  schemas: &Schemas<'_>,
  shared: &BTreeSet<String>,
  owner_prefix: &str,
  complex_type: &ParsedComplexType,
) {
  for child in effective_children(schemas, owner_prefix, complex_type) {
    let type_qname = binding_type(schemas, &child);
    if type_qname.is_empty() {
      continue;
    }
    push_type(
      types,
      seen,
      schemas,
      shared,
      owner_prefix,
      &type_qname,
      &child.element_prefix,
      qname_local(&child.q_name),
      &ParsedComplexType::default(),
      false,
    );
  }

  if let Some(derivation) = &complex_type.derivation {
    collect_child_bindings(types, seen, schemas, shared, owner_prefix, &derivation.body);
  }
}

fn build_type(
  name: &str,
  element_local: &str,
  shape: TypeShape,
  schemas: &Schemas<'_>,
  type_owner: &str,
  body: &ParsedComplexType,
) -> OpenXmlSchemaType {
  let children = if shape.leaf_text {
    Vec::new()
  } else {
    effective_children(schemas, type_owner, body)
  };
  let has_simple_content = body.text_value_type.is_some()
    || body
      .derivation
      .as_ref()
      .is_some_and(|derivation| derivation.body.text_value_type.is_some());
  let is_leaf_text = shape.leaf_text || (has_simple_content && body.particle.is_none());
  let is_leaf_element =
    !is_leaf_text && children.is_empty() && body.particle.is_none() && body.derivation.is_none();
  let is_derived = shape.derived || body.derivation.is_some();

  let cfg = schemas.cfg;
  let base_class = if shape.is_root && cfg.part_root_elements.contains_key(element_local) {
    "OpenXmlPartRootElement".to_string()
  } else if is_leaf_text {
    "OpenXmlLeafTextElement".to_string()
  } else if is_leaf_element {
    "OpenXmlLeafElement".to_string()
  } else {
    "OpenXmlCompositeElement".to_string()
  };

  OpenXmlSchemaType {
    name: name.to_string(),
    class_name: cfg
      .class_name_overrides
      .get(name)
      .cloned()
      .unwrap_or_else(|| element_local.to_upper_camel_case()),
    summary: body.documentation.clone(),
    part: if shape.is_root {
      cfg
        .part_root_elements
        .get(element_local)
        .cloned()
        .unwrap_or_default()
    } else {
      String::new()
    },
    base_class,
    is_leaf_element,
    is_leaf_text,
    is_derived,
    is_abstract: body.is_abstract,
    has_xmlns_fields: shape.is_root,
    children: children
      .iter()
      .map(|child| OpenXmlSchemaTypeChild {
        name: child_name(schemas, type_owner, child),
        property_name: qname_local(&child.q_name).to_upper_camel_case(),
        property_comments: child.documentation.clone(),
      })
      .collect(),
    attributes: if shape.leaf_text || shape.shared_binding {
      Vec::new()
    } else {
      build_attributes(schemas, type_owner, body)
    },
    particle: if shape.leaf_text {
      OpenXmlSchemaTypeParticle::default()
    } else {
      build_particle(schemas, type_owner, body)
    },
    ..Default::default()
  }
}

fn attribute_qname(
  xsd: &ParsedXsd,
  cfg: &NamespaceGenConfig,
  attribute: &ParsedAttribute,
) -> String {
  let (prefix, local) = split_qname(&attribute.q_name);
  if prefix.is_empty() {
    format!(":{local}")
  } else {
    format!("{}:{local}", resolve_prefix(xsd, cfg, prefix))
  }
}

fn open_xml_attribute(
  xsd: &ParsedXsd,
  cfg: &NamespaceGenConfig,
  attribute: &ParsedAttribute,
) -> OpenXmlSchemaTypeAttribute {
  let (_, local) = split_qname(&attribute.q_name);
  let validators = if attribute.required {
    vec![OpenXmlSchemaTypeAttributeValidator {
      name: "RequiredValidator".to_string(),
      ..Default::default()
    }]
  } else {
    Vec::new()
  };
  OpenXmlSchemaTypeAttribute {
    q_name: attribute_qname(xsd, cfg, attribute),
    property_name: local.to_upper_camel_case(),
    r#type: attribute.r#type.clone(),
    property_comments: attribute.documentation.clone(),
    validators,
    ..Default::default()
  }
}

fn build_attributes(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  body: &ParsedComplexType,
) -> Vec<OpenXmlSchemaTypeAttribute> {
  let mut attributes: Vec<OpenXmlSchemaTypeAttribute> = body
    .attributes
    .iter()
    .map(|attribute| open_xml_attribute(schemas.main, schemas.cfg, attribute))
    .collect();

  let mut stack = BTreeSet::new();
  for reference in &body.attribute_group_refs {
    attributes.extend(expand_attribute_group(
      schemas,
      owner_prefix,
      reference,
      &mut stack,
    ));
  }

  if let Some(derivation) = &body.derivation {
    attributes.extend(build_attributes(schemas, owner_prefix, &derivation.body));
  }

  attributes
}

fn expand_attribute_group(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  reference: &str,
  stack: &mut BTreeSet<String>,
) -> Vec<OpenXmlSchemaTypeAttribute> {
  let Some((group, group_prefix)) = schemas.attribute_group(owner_prefix, reference) else {
    return Vec::new();
  };
  let key = format!("{group_prefix}:{}", qname_local(reference));
  if !stack.insert(key.clone()) {
    return Vec::new();
  }
  let mut attributes: Vec<_> = group
    .attributes
    .iter()
    .map(|attribute| open_xml_attribute(schemas.main, schemas.cfg, attribute))
    .collect();
  for nested in &group.refs {
    attributes.extend(expand_attribute_group(
      schemas,
      &group_prefix,
      nested,
      stack,
    ));
  }
  stack.remove(&key);
  attributes
}

fn build_particle(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  body: &ParsedComplexType,
) -> OpenXmlSchemaTypeParticle {
  if let Some(particle) = &body.particle {
    return build_particle_node(schemas, owner_prefix, particle, &mut BTreeSet::new());
  }
  if let Some(derivation) = &body.derivation {
    return build_particle(schemas, owner_prefix, &derivation.body);
  }
  OpenXmlSchemaTypeParticle::default()
}

fn build_particle_node(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  node: &ParsedParticleNode,
  stack: &mut BTreeSet<String>,
) -> OpenXmlSchemaTypeParticle {
  match node {
    ParsedParticleNode::Group { particle, children } => OpenXmlSchemaTypeParticle {
      kind: particle_kind_str(particle.kind).to_string(),
      occurs: occur_list(particle.min_occurs, particle.max_occurs),
      items: children
        .iter()
        .map(|child| build_particle_node(schemas, owner_prefix, child, stack))
        .collect(),
      ..Default::default()
    },
    ParsedParticleNode::Element(child) => OpenXmlSchemaTypeParticle {
      name: child_name(schemas, owner_prefix, child),
      occurs: occur_list(child.min_occurs, child.max_occurs),
      ..Default::default()
    },
    ParsedParticleNode::GroupRef {
      _reference,
      _min_occurs,
      _max_occurs,
    } => resolve_group_ref(
      schemas,
      owner_prefix,
      _reference,
      *_min_occurs,
      *_max_occurs,
      stack,
    ),
    ParsedParticleNode::Any {
      min_occurs,
      max_occurs,
    } => OpenXmlSchemaTypeParticle {
      kind: "Any".to_string(),
      occurs: occur_list(*min_occurs, *max_occurs),
      ..Default::default()
    },
  }
}

/// A group reference becomes a `Group` particle whose single item is the
/// referenced group's own particle. Checked-in metadata uses that wrapper
/// (`Sequence` > `Group` > `Choice`) rather than inlining the group.
fn resolve_group_ref(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  reference: &str,
  min_occurs: u64,
  max_occurs: u64,
  stack: &mut BTreeSet<String>,
) -> OpenXmlSchemaTypeParticle {
  let Some((group, group_prefix)) = schemas.group(owner_prefix, reference) else {
    return OpenXmlSchemaTypeParticle::default();
  };
  let key = format!("{group_prefix}:{}", qname_local(reference));
  if !stack.insert(key.clone()) {
    return OpenXmlSchemaTypeParticle::default();
  }
  let particle = OpenXmlSchemaTypeParticle {
    kind: "Group".to_string(),
    occurs: occur_list(min_occurs, max_occurs),
    items: vec![build_particle_node(schemas, &group_prefix, group, stack)],
    ..Default::default()
  };
  stack.remove(&key);
  particle
}

fn build_enums(xsd: &ParsedXsd, cfg: &NamespaceGenConfig) -> Vec<OpenXmlSchemaEnum> {
  xsd
    .simple_types
    .iter()
    .filter(|(_, values)| !values.is_empty())
    .map(|(name, values)| OpenXmlSchemaEnum {
      name: cfg
        .enum_name_overrides
        .get(name)
        .cloned()
        .unwrap_or_else(|| {
          format!(
            "{}Values",
            name.trim_start_matches("ST_").to_upper_camel_case()
          )
        }),
      r#type: format!("{}:{}", cfg.prefix, name),
      facets: values
        .iter()
        .map(|value| OpenXmlSchemaEnumFacet {
          name: value.to_upper_camel_case(),
          value: value.clone(),
          ..Default::default()
        })
        .collect(),
      ..Default::default()
    })
    .collect()
}

/// Direct children, elements reached through group references, and derivation
/// body children. Elements declared in another schema keep that schema's prefix.
fn effective_children(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  complex_type: &ParsedComplexType,
) -> Vec<ParsedChildElement> {
  let mut children = complex_type.children.clone();
  if let Some(particle) = &complex_type.particle {
    append_group_ref_elements(
      schemas,
      owner_prefix,
      particle,
      &mut children,
      &mut BTreeSet::new(),
    );
  }
  if let Some(derivation) = &complex_type.derivation {
    children.extend(effective_children(schemas, owner_prefix, &derivation.body));
  }
  children
}

fn append_group_ref_elements(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  node: &ParsedParticleNode,
  children: &mut Vec<ParsedChildElement>,
  stack: &mut BTreeSet<String>,
) {
  match node {
    ParsedParticleNode::Group {
      children: particle_children,
      ..
    } => {
      for child in particle_children {
        append_group_ref_elements(schemas, owner_prefix, child, children, stack);
      }
    }
    ParsedParticleNode::Element(_) | ParsedParticleNode::Any { .. } => {}
    ParsedParticleNode::GroupRef { _reference, .. } => {
      let Some((group, group_prefix)) = schemas.group(owner_prefix, _reference) else {
        return;
      };
      let key = format!("{group_prefix}:{}", qname_local(_reference));
      if !stack.insert(key.clone()) {
        return;
      }
      collect_resolved_elements(schemas, &group_prefix, group, children, stack);
      stack.remove(&key);
    }
  }
}

fn collect_resolved_elements(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  node: &ParsedParticleNode,
  children: &mut Vec<ParsedChildElement>,
  stack: &mut BTreeSet<String>,
) {
  match node {
    ParsedParticleNode::Group {
      children: particle_children,
      ..
    } => {
      for child in particle_children {
        collect_resolved_elements(schemas, owner_prefix, child, children, stack);
      }
    }
    ParsedParticleNode::Element(element) => {
      children.push(adopt_element(element, owner_prefix));
    }
    ParsedParticleNode::GroupRef { .. } => {
      append_group_ref_elements(schemas, owner_prefix, node, children, stack);
    }
    ParsedParticleNode::Any { .. } => {}
  }
}

/// An element declared in `owner_prefix`'s schema records that prefix on its
/// name and on an unprefixed type, so later naming does not use the parent schema.
fn adopt_element(element: &ParsedChildElement, owner_prefix: &str) -> ParsedChildElement {
  let mut element = element.clone();
  if element.element_prefix.is_empty() {
    element.element_prefix = owner_prefix.to_string();
  }
  if !element.r#type.is_empty() && !element.r#type.contains(':') {
    element.r#type = format!("{owner_prefix}:{}", element.r#type);
  }
  element
}

fn qualified_type_key(owner_prefix: &str, type_qname: &str) -> String {
  let (prefix, local) = split_qname(type_qname);
  let prefix = if prefix.is_empty() {
    owner_prefix
  } else {
    prefix
  };
  format!("{prefix}:{local}")
}

/// Complex types used as the type of more than one distinct element. Those
/// get a type-only entry, and each element binding is derived from it.
fn shared_complex_types(schemas: &Schemas<'_>) -> BTreeSet<String> {
  let mut usage: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
  let owner = schemas.cfg.prefix.as_str();
  let mut note = |type_qname: &str, element_local: &str| {
    if type_qname.is_empty() || element_local.is_empty() {
      return;
    }
    if schemas.complex_type(owner, type_qname).is_none() {
      return;
    }
    usage
      .entry(qualified_type_key(owner, type_qname))
      .or_default()
      .insert(element_local.to_string());
  };

  for (element_local, root) in &schemas.main.root_elements {
    note(&root.element_type, element_local);
    for child in effective_children(schemas, owner, root) {
      note(&binding_type(schemas, &child), qname_local(&child.q_name));
    }
  }
  for complex_type in schemas.main.complex_types.values() {
    for child in effective_children(schemas, owner, complex_type) {
      note(&binding_type(schemas, &child), qname_local(&child.q_name));
    }
  }

  usage
    .into_iter()
    .filter(|(_, element_names)| element_names.len() >= 2)
    .map(|(type_key, _)| type_key)
    .collect()
}

fn is_simple_element_type(schemas: &Schemas<'_>, owner_prefix: &str, type_qname: &str) -> bool {
  if type_qname.is_empty() {
    return false;
  }
  if schemas.cfg.simple_type_aliases.contains_key(type_qname) {
    return true;
  }
  if schemas.complex_type(owner_prefix, type_qname).is_some() {
    return false;
  }
  let (prefix, local) = split_qname(type_qname);
  let prefix = if prefix.is_empty() {
    owner_prefix
  } else {
    prefix
  };
  if schemas
    .schema_for_prefix(prefix)
    .is_some_and(|schema| schema.simple_types.contains_key(local))
  {
    return true;
  }
  is_builtin_simple(type_qname) || is_builtin_simple(local)
}

fn simple_type_is_derived(schemas: &Schemas<'_>, owner_prefix: &str, type_qname: &str) -> bool {
  let (prefix, local) = split_qname(type_qname);
  let prefix = if prefix.is_empty() {
    owner_prefix
  } else {
    prefix
  };
  if schemas
    .schema_for_prefix(prefix)
    .and_then(|schema| schema.simple_types.get(local))
    .is_some_and(|values| !values.is_empty())
  {
    return false;
  }
  true
}

fn is_builtin_simple(name: &str) -> bool {
  matches!(
    name,
    "xsd:string"
      | "xsd:boolean"
      | "xsd:decimal"
      | "xsd:float"
      | "xsd:double"
      | "xsd:duration"
      | "xsd:dateTime"
      | "xsd:time"
      | "xsd:date"
      | "xsd:hexBinary"
      | "xsd:base64Binary"
      | "xsd:anyURI"
      | "xsd:token"
      | "xsd:integer"
      | "xsd:int"
      | "xsd:long"
      | "xsd:short"
      | "xsd:byte"
      | "xsd:nonNegativeInteger"
      | "xsd:positiveInteger"
      | "xsd:unsignedLong"
      | "xsd:unsignedInt"
      | "xsd:unsignedShort"
      | "xsd:unsignedByte"
  )
}

fn child_name(schemas: &Schemas<'_>, owner_prefix: &str, child: &ParsedChildElement) -> String {
  let element_local = qname_local(&child.q_name);
  element_type_name(
    schemas,
    owner_prefix,
    &binding_type(schemas, child),
    &child.element_prefix,
    element_local,
  )
}

/// Type QName of a child element. An `xsd:element/@ref` has no `type` of its
/// own; the type is the referenced global element's `type`.
///
/// The parser records `ref` independently from its prefix: a reference using
/// the schema's default namespace has no lexical prefix.
fn binding_type(schemas: &Schemas<'_>, child: &ParsedChildElement) -> String {
  if !child.r#type.is_empty() {
    return child.r#type.clone();
  }
  if !child.is_reference {
    return String::new();
  }

  let prefix = if child.element_prefix.is_empty() {
    schemas.cfg.prefix.as_str()
  } else {
    child.element_prefix.as_str()
  };
  let local = qname_local(&child.q_name);
  let Some(schema) = schemas.schema_for_prefix(prefix) else {
    return String::new();
  };
  let Some(declaration) = schema.root_elements.get(local) else {
    return String::new();
  };
  let element_type = declaration.element_type.as_str();
  if element_type.is_empty() {
    return String::new();
  }
  if element_type.contains(':') {
    element_type.to_string()
  } else {
    format!("{prefix}:{element_type}")
  }
}

fn element_type_name(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  type_qname: &str,
  element_prefix: &str,
  element_local: &str,
) -> String {
  let element_prefix = if element_prefix.is_empty() {
    owner_prefix.to_string()
  } else {
    resolve_prefix(schemas.main, schemas.cfg, element_prefix)
  };
  if let Some(alias) = schemas.cfg.simple_type_aliases.get(type_qname) {
    let (type_prefix, type_local) = split_qname(alias);
    return format!("{type_prefix}:{type_local}/{element_prefix}:{element_local}");
  }
  let (type_prefix, type_local) = split_qname(type_qname);
  let type_local = if type_local.is_empty() {
    element_local
  } else {
    type_local
  };
  let type_prefix = if type_prefix.is_empty() {
    owner_prefix.to_string()
  } else {
    resolve_prefix(schemas.main, schemas.cfg, type_prefix)
  };
  format!("{type_prefix}:{type_local}/{element_prefix}:{element_local}")
}

fn resolve_prefix(xsd: &ParsedXsd, cfg: &NamespaceGenConfig, prefix: &str) -> String {
  if prefix.is_empty() || prefix == cfg.prefix {
    return cfg.prefix.clone();
  }
  if let Some(uri) = xsd.prefixes.get(prefix)
    && let Some(mapped) = cfg.namespace_prefixes.get(uri)
  {
    return mapped.clone();
  }
  prefix.to_string()
}

fn occur_list(min: u64, max: u64) -> Vec<OpenXmlSchemaTypeParticleOccur> {
  if min == 1 && max == 1 {
    return Vec::new();
  }
  vec![OpenXmlSchemaTypeParticleOccur {
    min: if min == 0 { None } else { Some(min) },
    max: if max == u64::MAX { None } else { Some(max) },
    ..Default::default()
  }]
}

fn particle_kind_str(kind: ParsedParticleKind) -> &'static str {
  match kind {
    ParsedParticleKind::Sequence => "Sequence",
    ParsedParticleKind::Choice => "Choice",
    ParsedParticleKind::All => "All",
  }
}

fn split_qname(q_name: &str) -> (&str, &str) {
  match q_name.split_once(':') {
    Some((prefix, local)) => (prefix, local),
    None => ("", q_name),
  }
}

fn qname_local(q_name: &str) -> &str {
  split_qname(q_name).1
}

#[cfg(test)]
mod tests {
  use super::*;

  fn generate(body: &str) -> OpenXmlSchema {
    let source = format!(
      r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
          xmlns="urn:test" xmlns:t="urn:test" targetNamespace="urn:test">
          {body}</xs:schema>"#
    );
    gen_open_xml_schema_from_xsd(
      &source,
      &NamespaceGenConfig {
        prefix: "t".into(),
        ..Default::default()
      },
    )
    .unwrap()
  }

  #[test]
  fn simple_content_preserves_text_and_attributes() {
    for derivation in ["extension", "restriction"] {
      let schema = generate(&format!(
        r#"
        <xs:complexType name="CT_Base"><xs:simpleContent>
          <xs:extension base="xs:string"><xs:attribute name="id" type="xs:string"/>
          </xs:extension></xs:simpleContent></xs:complexType>
        <xs:complexType name="CT_Text"><xs:simpleContent>
          <xs:{derivation} base="CT_Base"><xs:attribute name="id" type="xs:string"/>
          </xs:{derivation}></xs:simpleContent></xs:complexType>
        <xs:element name="text" type="CT_Text"/>"#
      ));
      let text = schema
        .types
        .iter()
        .find(|ty| ty.name == "t:CT_Text/t:text")
        .unwrap();
      assert!(text.is_leaf_text, "{derivation}");
      assert!(!text.is_leaf_element);
      assert_eq!(text.base_class, "OpenXmlLeafTextElement");
      assert_eq!(text.attributes.len(), 1);
      assert_eq!(text.attributes[0].q_name, ":id");
    }
  }

  #[test]
  fn global_element_references_resolve_with_and_without_prefix() {
    for reference in ["child", "t:child"] {
      let schema = generate(&format!(
        r#"
        <xs:element name="root" type="CT_Root"/>
        <xs:element name="child" type="CT_Child"/>
        <xs:complexType name="CT_Child"><xs:attribute name="id" type="xs:string"/></xs:complexType>
        <xs:complexType name="CT_Root"><xs:sequence>
          <xs:element ref="{reference}"/>
        </xs:sequence></xs:complexType>"#
      ));
      let root = schema
        .types
        .iter()
        .find(|ty| ty.name == "t:CT_Root/t:root")
        .unwrap();
      assert_eq!(root.children[0].name, "t:CT_Child/t:child", "{reference}");
      assert_eq!(root.particle.items[0].name, "t:CT_Child/t:child");
    }
  }

  #[test]
  fn wildcard_occurrences_preserve_optional_repeated_and_default_bounds() {
    for (attributes, bounds) in [
      (r#"minOccurs="0" maxOccurs="unbounded""#, Some((None, None))),
      (r#"minOccurs="2" maxOccurs="3""#, Some((Some(2), Some(3)))),
      ("", None),
    ] {
      for content in [
        format!("<xs:any {attributes}/>"),
        format!("<xs:any {attributes}><xs:annotation/></xs:any>"),
      ] {
        let schema = generate(&format!(
          r#"
          <xs:element name="root" type="CT_Root"/>
          <xs:complexType name="CT_Root"><xs:sequence>{content}</xs:sequence></xs:complexType>"#
        ));
        let root = schema
          .types
          .iter()
          .find(|ty| ty.name == "t:CT_Root/t:root")
          .unwrap();
        let wildcard = &root.particle.items[0];
        assert_eq!(wildcard.kind, "Any");
        assert_eq!(wildcard.occurs.len(), usize::from(bounds.is_some()));
        assert_eq!(
          wildcard.occurs.first().map(|occur| (occur.min, occur.max)),
          bounds
        );
      }
    }
  }
}
