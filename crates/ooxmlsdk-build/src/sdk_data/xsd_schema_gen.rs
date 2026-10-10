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
//! `additionalCharacteristics`, `bibliography`, `pml`, `sml`, and `wml` match
//! that projection. For `pml`, `sml`, and `wml`, the checked-in JSON still
//! contains SDK rewrites the transitional XSD does not state (per-parent
//! extension-list types, Office 2010 attributes and elements, versioned
//! occurs, and renamed or subset enums). Those acceptance tests exclude the
//! rewrites from the comparison. They do not rename them into the generator.
//! `bibliography`'s `b:ST_String255` alias stays on that test.

use std::{
  collections::{BTreeMap, BTreeSet, HashMap},
  path::Path,
};

use heck::ToUpperCamelCase;

use crate::{
  Result,
  sdk_data::{
    open_xml::{
      OpenXmlPart, OpenXmlSchema, OpenXmlSchemaEnum, OpenXmlSchemaEnumFacet, OpenXmlSchemaType,
      OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeAttributeValidator, OpenXmlSchemaTypeChild,
      OpenXmlSchemaTypeParticle, OpenXmlSchemaTypeParticleOccur,
    },
    xsd::{
      ParsedAttribute, ParsedAttributeGroup, ParsedChildElement, ParsedComplexType, ParsedParticle,
      ParsedParticleKind, ParsedParticleNode, ParsedXsd,
    },
  },
};

pub(crate) const VISIO_MODULE_NAME: &str = "schemas_microsoft_com_office_visio_2012_main";
const VISIO_PREFIX: &str = "visio";
const VISIO_EMITTED_TARGET_NAMESPACE: &str = "http://schemas.microsoft.com/office/visio/2012/main";
const VISIO_RELATIONSHIP_PREFIX: &str = "http://schemas.microsoft.com/visio/2010/relationships/";

/// Load Visio stage-1 metadata from `schemas/VisioXMLSchema/visio-main.xsd`
/// when that file is next to `data_dir`. Returns `None` when the XSD is absent
/// so OOXML generation still runs. The result stays in memory: this function
/// does not write `data/schemas`.
///
/// `simple_type_aliases` and `class_name_overrides` stay empty. `xsd:alternative`
/// and `xsd:assert` are left unread, so `Cell` and `Section` stay the generic
/// types declared on the element. The XSD file keeps
/// `http://schemas.microsoft.com/office/visio/2011/1/core`. The emitted schema
/// URI is `http://schemas.microsoft.com/office/visio/2012/main`.
pub(crate) fn gen_visio_schema(
  data_dir: &Path,
  parts: &[OpenXmlPart],
  namespace_prefixes: &HashMap<String, String>,
) -> Result<Option<OpenXmlSchema>> {
  let Some(xsd_path) = data_dir
    .parent()
    .map(|root| root.join("schemas/VisioXMLSchema/visio-main.xsd"))
  else {
    return Ok(None);
  };
  if !xsd_path.is_file() {
    return Ok(None);
  }

  let source = std::fs::read_to_string(&xsd_path)?;
  let mut part_root_elements = HashMap::new();
  for part in parts {
    if part
      .relationship_type
      .starts_with(VISIO_RELATIONSHIP_PREFIX)
      && !part.root_element.is_empty()
    {
      part_root_elements.insert(part.root_element.clone(), part.name.clone());
    }
  }
  let cfg = NamespaceGenConfig {
    prefix: VISIO_PREFIX.to_string(),
    emitted_target_namespace: Some(VISIO_EMITTED_TARGET_NAMESPACE.to_string()),
    namespace_prefixes: namespace_prefixes.clone(),
    part_root_elements,
    schema_dir: xsd_path.parent().map(Path::to_path_buf),
    ..Default::default()
  };
  let mut schema = gen_open_xml_schema_from_xsd(&source, &cfg)?;
  schema.module_name = VISIO_MODULE_NAME.to_string();
  for schema_type in &mut schema.types {
    schema_type.module_name = schema.module_name.clone();
  }
  for schema_enum in &mut schema.enums {
    schema_enum.module_name = schema.module_name.clone();
  }
  Ok(Some(schema))
}

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
  /// URI written on the generated `OpenXmlSchema`. When unset, the parsed XSD
  /// `targetNamespace` is emitted. Prefix resolution and imports keep the
  /// parsed URI.
  pub emitted_target_namespace: Option<String>,
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
  disambiguate_shared_element_class_names(&mut types);

  OpenXmlSchema {
    target_namespace: cfg
      .emitted_target_namespace
      .clone()
      .unwrap_or_else(|| xsd.target_namespace.clone()),
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

/// Inline `xs:complexContent` extension and `xs:simpleContent` so the projected
/// type carries the base attributes, children, and particle. `is_derived` stays
/// the shared-element flag and is not set from the derivation itself.
fn flatten_type(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  body: &ParsedComplexType,
) -> ParsedComplexType {
  flatten_type_rec(schemas, owner_prefix, body, &mut BTreeSet::new())
}

fn flatten_type_rec(
  schemas: &Schemas<'_>,
  owner_prefix: &str,
  body: &ParsedComplexType,
  stack: &mut BTreeSet<String>,
) -> ParsedComplexType {
  let Some(derivation) = &body.derivation else {
    return body.clone();
  };

  if derivation.body.text_value_type.is_some() {
    let mut flat = derivation.body.clone();
    flat.derivation = None;
    flat.is_abstract = body.is_abstract;
    if flat.documentation.is_empty() {
      flat.documentation = body.documentation.clone();
    }
    return flat;
  }

  let mut flat = if derivation.is_extension {
    let base_key = qualified_type_key(owner_prefix, &derivation.base);
    if stack.insert(base_key.clone()) {
      let base = schemas
        .complex_type(owner_prefix, &derivation.base)
        .map(|(base, base_prefix)| flatten_type_rec(schemas, &base_prefix, base, stack))
        .unwrap_or_default();
      stack.remove(&base_key);
      base
    } else {
      ParsedComplexType::default()
    }
  } else {
    ParsedComplexType::default()
  };

  let extension = &derivation.body;
  flat.attributes.extend(extension.attributes.clone());
  flat
    .attribute_group_refs
    .extend(extension.attribute_group_refs.clone());
  flat.children.extend(extension.children.clone());
  flat.particle = merge_particles(flat.particle, extension.particle.clone());
  if extension.text_value_type.is_some() {
    flat.text_value_type = extension.text_value_type.clone();
  }
  flat.has_any_attribute = flat.has_any_attribute || extension.has_any_attribute;
  flat.is_abstract = body.is_abstract;
  if !body.documentation.is_empty() {
    flat.documentation = body.documentation.clone();
  }
  flat.derivation = None;
  flat
}

fn merge_particles(
  base: Option<Box<ParsedParticleNode>>,
  extension: Option<Box<ParsedParticleNode>>,
) -> Option<Box<ParsedParticleNode>> {
  match (base, extension) {
    (None, None) => None,
    (Some(particle), None) | (None, Some(particle)) => Some(particle),
    (Some(base), Some(extension)) => Some(Box::new(ParsedParticleNode::Group {
      particle: ParsedParticle {
        kind: ParsedParticleKind::Sequence,
        min_occurs: 1,
        max_occurs: 1,
      },
      children: vec![*base, *extension],
    })),
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
  let flat = flatten_type(schemas, type_owner, body);
  let children = if shape.leaf_text {
    Vec::new()
  } else {
    effective_children(schemas, type_owner, &flat)
  };
  let is_leaf_text = shape.leaf_text || (flat.text_value_type.is_some() && flat.particle.is_none());
  let is_leaf_element = !is_leaf_text && children.is_empty() && flat.particle.is_none();
  // `is_derived` means the element binding shares its complex type with other
  // elements. An `xs:extension` base is inlined instead, matching the
  // checked-in metadata.
  let is_derived = shape.derived;

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
      build_attributes(schemas, type_owner, &flat)
    },
    particle: if shape.leaf_text || is_leaf_text {
      OpenXmlSchemaTypeParticle::default()
    } else {
      build_particle(schemas, type_owner, &flat)
    },
    text_value_type: flat.text_value_type.clone().unwrap_or_default(),
    ..Default::default()
  }
}

fn attribute_qname(
  xsd: &ParsedXsd,
  cfg: &NamespaceGenConfig,
  owner_prefix: &str,
  attribute: &ParsedAttribute,
) -> String {
  let (prefix, local) = split_qname(&attribute.q_name);
  if prefix.is_empty() {
    if xsd.attribute_form_qualified {
      format!("{owner_prefix}:{local}")
    } else {
      format!(":{local}")
    }
  } else {
    format!("{}:{local}", resolve_prefix(xsd, cfg, prefix))
  }
}

fn open_xml_attribute(
  xsd: &ParsedXsd,
  cfg: &NamespaceGenConfig,
  owner_prefix: &str,
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
    q_name: attribute_qname(xsd, cfg, owner_prefix, attribute),
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
  let schema = schemas
    .schema_for_prefix(owner_prefix)
    .unwrap_or(schemas.main);
  let mut attributes: Vec<OpenXmlSchemaTypeAttribute> = body
    .attributes
    .iter()
    .map(|attribute| open_xml_attribute(schema, schemas.cfg, owner_prefix, attribute))
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
  let schema = schemas
    .schema_for_prefix(&group_prefix)
    .unwrap_or(schemas.main);
  let mut attributes: Vec<_> = group
    .attributes
    .iter()
    .map(|attribute| open_xml_attribute(schema, schemas.cfg, &group_prefix, attribute))
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
    ParsedParticleNode::Any => OpenXmlSchemaTypeParticle {
      kind: "Any".to_string(),
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
    ParsedParticleNode::Element(_) | ParsedParticleNode::Any => {}
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
    ParsedParticleNode::Any => {}
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
/// Name-based children in this parser store an OPC prefix on `q_name` and leave
/// `element_prefix` empty, so a ref is recognized only when `element_prefix`
/// is already the prefix of `q_name`.
fn binding_type(schemas: &Schemas<'_>, child: &ParsedChildElement) -> String {
  if !child.r#type.is_empty() {
    return child.r#type.clone();
  }
  if child.element_prefix.is_empty()
    || !child
      .q_name
      .starts_with(&format!("{}:", child.element_prefix))
  {
    return String::new();
  }

  let local = qname_local(&child.q_name);
  let Some(schema) = schemas.schema_for_prefix(&child.element_prefix) else {
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
    format!("{}:{element_type}", child.element_prefix)
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

/// Element bindings that share a local name (`visio:Row` on nineteen complex
/// types) cannot share one Rust class name. Use the complex type's local name,
/// without a trailing `_Type`, for those bindings only.
fn disambiguate_shared_element_class_names(types: &mut [OpenXmlSchemaType]) {
  let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
  for schema_type in types.iter() {
    *counts.entry(schema_type.class_name.as_str()).or_insert(0) += 1;
  }
  let shared: BTreeSet<String> = counts
    .into_iter()
    .filter(|(_, count)| *count > 1)
    .map(|(name, _)| name.to_string())
    .collect();
  if shared.is_empty() {
    return;
  }
  for schema_type in types.iter_mut() {
    if !shared.contains(schema_type.class_name.as_str()) {
      continue;
    }
    let Some((type_name, _)) = schema_type.name.split_once('/') else {
      continue;
    };
    if type_name.is_empty() {
      continue;
    }
    let local = qname_local(type_name);
    let local = local.strip_suffix("_Type").unwrap_or(local);
    schema_type.class_name = local.to_upper_camel_case();
  }
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
  use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
  };

  use super::{
    NamespaceGenConfig, VISIO_MODULE_NAME, VISIO_RELATIONSHIP_PREFIX, gen_open_xml_schema_from_xsd,
    gen_visio_schema,
  };
  use crate::sdk_data::open_xml::{OpenXmlPart, OpenXmlSchema};

  fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .and_then(|path| path.parent())
      .expect("workspace root")
      .to_path_buf()
  }

  fn schema_fingerprint(dir: &Path) -> BTreeMap<String, (u64, SystemTime)> {
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(dir).expect("read data/schemas") {
      let entry = entry.expect("schema dir entry");
      let meta = entry.metadata().expect("schema metadata");
      if meta.is_file() {
        files.insert(
          entry.file_name().to_string_lossy().into_owned(),
          (meta.len(), meta.modified().expect("schema mtime")),
        );
      }
    }
    files
  }

  fn class_name<'a>(schema: &'a OpenXmlSchema, name: &str) -> &'a str {
    schema
      .types
      .iter()
      .find(|schema_type| schema_type.name == name)
      .unwrap_or_else(|| panic!("missing {name}"))
      .class_name
      .as_str()
  }

  fn assert_unique_class_names(schema: &OpenXmlSchema) {
    let mut seen = BTreeMap::new();
    for schema_type in &schema.types {
      if schema_type.class_name.is_empty() {
        continue;
      }
      let count = seen.entry(schema_type.class_name.clone()).or_insert(0);
      *count += 1;
    }
    let duplicates: Vec<_> = seen
      .into_iter()
      .filter(|(_, count)| *count > 1)
      .map(|(name, _)| name)
      .collect();
    assert!(
      duplicates.is_empty(),
      "duplicate class names: {duplicates:?}"
    );
  }

  fn text_value_type<'a>(schema: &'a OpenXmlSchema, name: &str) -> &'a str {
    schema
      .types
      .iter()
      .find(|schema_type| schema_type.name == name)
      .unwrap_or_else(|| panic!("missing {name}"))
      .text_value_type
      .as_str()
  }

  fn assert_generic_cell_and_section(schema: &OpenXmlSchema) {
    assert!(
      schema
        .types
        .iter()
        .any(|schema_type| schema_type.name == "visio:Cell_Type/visio:Cell")
    );
    assert!(
      schema
        .types
        .iter()
        .any(|schema_type| schema_type.name == "visio:Section_Type/visio:Section")
    );
    assert!(
      schema
        .types
        .iter()
        .all(|schema_type| !schema_type.name.contains("PinX_Type"))
    );
  }

  #[test]
  fn visio_cell_and_section_stay_generic() {
    let root = workspace_root();
    let schemas_dir = root.join("data/schemas");
    let before = schema_fingerprint(&schemas_dir);
    assert!(!before.is_empty());

    let xsd_path = root.join("schemas/VisioXMLSchema/visio-main.xsd");
    let source = fs::read_to_string(&xsd_path).expect("read visio-main.xsd");
    let cfg = NamespaceGenConfig {
      prefix: "visio".to_string(),
      schema_dir: xsd_path.parent().map(Path::to_path_buf),
      ..Default::default()
    };
    assert!(cfg.simple_type_aliases.is_empty());
    assert!(cfg.class_name_overrides.is_empty());
    assert!(cfg.emitted_target_namespace.is_none());
    let parsed = crate::sdk_data::xsd::parse_xsd(&source).expect("parse visio-main.xsd");
    let direct = gen_open_xml_schema_from_xsd(&source, &cfg).expect("generate visio schema");
    let wired = gen_visio_schema(&root.join("data"), &[], &HashMap::new())
      .expect("load visio schema")
      .expect("visio-main.xsd is present");

    assert_eq!(
      parsed.target_namespace,
      "http://schemas.microsoft.com/office/visio/2011/1/core"
    );
    assert_eq!(direct.target_namespace, parsed.target_namespace);
    assert_eq!(wired.module_name, VISIO_MODULE_NAME);
    assert_eq!(
      wired.target_namespace,
      "http://schemas.microsoft.com/office/visio/2012/main"
    );
    assert_generic_cell_and_section(&direct);
    assert_generic_cell_and_section(&wired);
    assert_eq!(
      text_value_type(&wired, "visio:Title_Type/visio:Title"),
      "xsd:string"
    );
    assert_eq!(
      text_value_type(&wired, "visio:PreviewPicture_Type/visio:PreviewPicture"),
      "xsd:base64Binary"
    );
    assert_eq!(
      text_value_type(&wired, "visio:GlueSettings_Type/visio:GlueSettings"),
      "xsd:int"
    );
    assert_unique_class_names(&wired);
    assert_eq!(
      class_name(&wired, "visio:UserRow_Type/visio:Row"),
      "UserRow"
    );
    assert_eq!(class_name(&wired, "visio:Cell_Type/visio:Cell"), "Cell");
    assert_eq!(before, schema_fingerprint(&schemas_dir));
  }

  #[test]
  fn visio_part_roots_follow_relationship_parts() {
    let root = workspace_root();
    let mut parts = Vec::new();
    for entry in fs::read_dir(root.join("data/parts")).expect("read data/parts") {
      let path = entry.expect("part dir entry").path();
      let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
      if !file_name.starts_with("Visio")
        || path.extension().and_then(|ext| ext.to_str()) != Some("json")
      {
        continue;
      }
      let file = fs::File::open(&path).expect("open visio part");
      parts.push(serde_json::from_reader::<_, OpenXmlPart>(file).expect("parse visio part"));
    }
    assert!(!parts.is_empty());

    let schema = gen_visio_schema(&root.join("data"), &parts, &HashMap::new())
      .expect("load visio schema")
      .expect("visio-main.xsd is present");

    let mut marked = 0usize;
    for part in &parts {
      if !part
        .relationship_type
        .starts_with(VISIO_RELATIONSHIP_PREFIX)
        || part.root_element.is_empty()
      {
        continue;
      }
      let binding = format!(
        "visio:{}_Type/visio:{}",
        part.root_element, part.root_element
      );
      let schema_type = schema
        .types
        .iter()
        .find(|schema_type| schema_type.name == binding)
        .unwrap_or_else(|| panic!("missing {binding}"));
      if part.root_element == "Comments" {
        assert!(schema_type.part.is_empty());
        continue;
      }
      assert_eq!(schema_type.part, part.name);
      assert_eq!(schema_type.base_class, "OpenXmlPartRootElement");
      assert_eq!(schema_type.class_name, part.root_element);
      marked += 1;
    }
    assert_eq!(marked, 8);
  }
}
