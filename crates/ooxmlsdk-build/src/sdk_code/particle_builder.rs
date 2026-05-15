use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, ElementKind, FieldDecl, FieldWireDecl, MemberDecl, SystemSupportDecl, TypeDecl,
  TypeKind, TypeRefDecl, VariantDecl, VariantWireDecl, XmlHeaderMode,
};
use crate::sdk_code::codegen_ir_builder::{
  build_attr_member_decl, build_child_type_ref_from_name, disambiguate_choice_variant_names,
  effective_child_kind_from_name,
};
use crate::sdk_code::schemas::CodegenContext;
use crate::sdk_code::versioning::effective_version;
use crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaType, SchemaTypeApiKind, SchemaTypeChild, SchemaTypeChildKind,
  SchemaTypeCompositeKind, SchemaTypeKind, SchemaTypeXmlHeader,
};
use heck::ToUpperCamelCase;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleNodeKind {
  Element,
  Any,
  Choice,
  Sequence,
  All,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParticleLeafMetadata {
  pub qname: String,
  pub property_name: String,
  pub property_comments: String,
  pub additional_elements: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticleNode {
  pub kind: ParticleNodeKind,
  pub qname: Option<String>,
  pub version: String,
  pub require_filter: bool,
  pub min_occurs: u64,
  pub max_occurs: Option<u64>,
  pub metadata: Option<ParticleLeafMetadata>,
  pub children: Vec<ParticleNode>,
}

#[derive(Clone)]
struct ParticleBuildContext<'a> {
  schema_type: &'a SchemaType,
  root_type_name: String,
  naming: Rc<RefCell<ParticleNamingState>>,
}

#[derive(Debug, Default)]
struct ParticleNamingState {
  choice_type_total: usize,
  choice_type_index: usize,
  sequence_type_total: usize,
  sequence_type_index: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParticleTypeOutput {
  pub primary_type: TypeDecl,
  pub extra_types: Vec<TypeDecl>,
  pub particle_members: Vec<MemberDecl>,
}

pub fn is_particle_rollout_type(schema_type: &SchemaType) -> bool {
  matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::OneSequence | SchemaTypeCompositeKind::XsdRepeatableChoice
  ) && !schema_type.particle.kind.is_empty()
}

pub fn build_particle_type_output(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<ParticleTypeOutput> {
  let particle_model = lower_particle_model(schema_type)?;
  let mut particle_members = schema_type
    .attributes
    .iter()
    .map(|attr| build_attr_member_decl(attr, schema, context))
    .collect::<Result<Vec<_>>>()?;
  let mut extra_types = Vec::new();
  let metadata = ParticleMetadataResolver::new(schema_type);

  if let Some(root) = particle_model.as_ref() {
    let naming_totals = count_particle_naming_totals(root);
    let build_context = ParticleBuildContext {
      schema_type,
      root_type_name: schema_type.class_name.clone(),
      naming: Rc::new(RefCell::new(ParticleNamingState {
        choice_type_total: naming_totals.choice_types,
        sequence_type_total: naming_totals.sequence_types,
        ..Default::default()
      })),
    };
    build_particle_members_for_root(
      build_context,
      root,
      &metadata,
      schema,
      context,
      &mut particle_members,
      &mut extra_types,
    )?;
  }

  let parent_choice_any_fields = context
    .parent_choice_any_targets(schema_type.class_name.as_str())
    .into_iter()
    .filter_map(|(_, field_name)| field_name.map(str::to_string))
    .collect::<Vec<_>>();
  if !parent_choice_any_fields.is_empty() {
    let _ = apply_particle_parent_choice_has_any(
      &parent_choice_any_fields,
      &mut particle_members,
      &mut extra_types,
    );
  }

  let mut primary_type = build_particle_type_shell(schema_type);
  primary_type.particle_members = particle_members.clone();
  Ok(ParticleTypeOutput {
    primary_type,
    extra_types,
    particle_members,
  })
}

fn apply_particle_parent_choice_has_any(
  target_field_names: &[String],
  members: &mut Vec<MemberDecl>,
  extra_types: &mut Vec<TypeDecl>,
) -> bool {
  add_xml_other_variant_to_particle_direct_choice(target_field_names, members, extra_types)
}

fn add_xml_other_variant_to_particle_direct_choice(
  target_field_names: &[String],
  members: &[MemberDecl],
  extra_types: &mut [TypeDecl],
) -> bool {
  let choice_fields = members
    .iter()
    .filter_map(|member| match member {
      MemberDecl::Field(field)
        if matches!(field.wire, FieldWireDecl::Choice)
          && target_field_names
            .iter()
            .any(|name| name == &field.rust_name) =>
      {
        Some(field)
      }
      _ => None,
    })
    .collect::<Vec<_>>();

  let [choice_field] = choice_fields.as_slice() else {
    return false;
  };

  let Some(choice_type) = extra_types
    .iter_mut()
    .find(|ty| ty.rust_name == choice_field.type_ref.rust_type && ty.kind == TypeKind::ChoiceEnum)
  else {
    return false;
  };

  let already_exists = choice_type
    .effective_members()
    .iter()
    .any(|member| matches!(member, MemberDecl::Variant(variant) if variant.rust_name == "XmlAny"));
  if already_exists {
    return true;
  }

  let xml_any_variant = MemberDecl::Variant(VariantDecl {
    rust_name: "XmlAny".to_string(),
    docs: "Unknown XML child.".to_string(),
    version: String::new(),
    wire: VariantWireDecl::Any,
    payload: TypeRefDecl {
      rust_type: "std::boxed::Box<str>".to_string(),
      module_path: None,
    },
  });
  choice_type.particle_members.push(xml_any_variant);
  true
}

pub fn lower_particle_model(schema_type: &SchemaType) -> Result<Option<ParticleNode>> {
  if schema_type.particle.kind.is_empty() {
    return Ok(None);
  }

  Ok(Some(lower_particle_node(
    &schema_type.particle,
    &schema_type.additional_elements,
  )?))
}

fn lower_particle_node(
  particle: &OpenXmlSchemaTypeParticle,
  additional_elements: &[String],
) -> Result<ParticleNode> {
  let kind = match particle.kind.as_str() {
    "" if !particle.name.is_empty() => ParticleNodeKind::Element,
    "Element" => ParticleNodeKind::Element,
    "Any" => ParticleNodeKind::Any,
    "Choice" => ParticleNodeKind::Choice,
    "Sequence" => ParticleNodeKind::Sequence,
    "All" => ParticleNodeKind::All,
    "Group" => {
      let Some(child) = particle.items.first() else {
        return Err("group particle missing child".into());
      };
      let mut lowered = lower_particle_node(child, additional_elements)?;
      lowered.version =
        effective_version(particle.initial_version.as_str(), lowered.version.as_str()).to_string();
      lowered.require_filter |= particle.require_filter;
      lowered.min_occurs =
        combine_particle_min_occurs(lowered.min_occurs, particle_min_occurs(particle));
      lowered.max_occurs =
        combine_particle_max_occurs(lowered.max_occurs, particle_max_occurs(particle));
      return Ok(lowered);
    }
    other => {
      return Err(format!("unsupported particle kind for phase 1: {other}").into());
    }
  };

  Ok(ParticleNode {
    kind,
    qname: (!particle.name.is_empty()).then(|| particle.name.clone()),
    version: particle.initial_version.clone(),
    require_filter: particle.require_filter,
    min_occurs: particle_min_occurs(particle),
    max_occurs: particle_max_occurs(particle),
    metadata: (!particle.name.is_empty()).then(|| ParticleLeafMetadata {
      qname: particle.name.clone(),
      property_name: String::new(),
      property_comments: String::new(),
      additional_elements: additional_elements.to_vec(),
    }),
    children: particle
      .items
      .iter()
      .map(|item| lower_particle_node(item, additional_elements))
      .collect::<Result<Vec<_>>>()?,
  })
}

fn particle_min_occurs(particle: &OpenXmlSchemaTypeParticle) -> u64 {
  match particle.occurs.first() {
    Some(occur) => occur.min.unwrap_or(0),
    None => 1,
  }
}

fn particle_max_occurs(particle: &OpenXmlSchemaTypeParticle) -> Option<u64> {
  match particle.occurs.first() {
    Some(occur) => occur.max,
    None => Some(1),
  }
}

fn combine_particle_max_occurs(lhs: Option<u64>, rhs: Option<u64>) -> Option<u64> {
  match (lhs, rhs) {
    (Some(1), other) => other,
    (other, Some(1)) => other,
    (None, _) | (_, None) => None,
    (Some(lhs), Some(rhs)) => Some(lhs.saturating_mul(rhs)),
  }
}

fn combine_particle_min_occurs(lhs: u64, rhs: u64) -> u64 {
  lhs.saturating_mul(rhs)
}

fn build_particle_type_shell(schema_type: &SchemaType) -> TypeDecl {
  TypeDecl {
    rust_name: schema_type.class_name.clone(),
    xml_qname: (!schema_type.name.is_empty()).then(|| schema_type.name.clone()),
    docs: schema_type.summary.clone(),
    version: schema_type.version.clone(),
    is_abstract: schema_type.is_abstract,
    kind: if schema_type.api_kind == SchemaTypeApiKind::LeafTextWrapper {
      TypeKind::LeafTextAlias
    } else {
      TypeKind::ElementStruct
    },
    element_kind: if schema_type.api_kind == SchemaTypeApiKind::LeafTextWrapper {
      None
    } else {
      Some(match schema_type.kind {
        SchemaTypeKind::LeafText => ElementKind::LeafText,
        SchemaTypeKind::Leaf => ElementKind::Leaf,
        SchemaTypeKind::Composite => ElementKind::Composite,
        SchemaTypeKind::Derived => ElementKind::Derived,
        SchemaTypeKind::Struct => ElementKind::Composite,
      })
    },
    content_model: None,
    base_rust_name: (!schema_type.base_class.is_empty()
      && schema_type.base_class != "OpenXmlPartRootElement")
      .then(|| schema_type.base_class.clone()),
    xml_content: None,
    support: SystemSupportDecl {
      have_xmlns_fields: schema_type.have_xmlns_fields,
      xml_header: match schema_type.xml_header {
        SchemaTypeXmlHeader::None => XmlHeaderMode::None,
        SchemaTypeXmlHeader::Plain => XmlHeaderMode::Plain,
        SchemaTypeXmlHeader::Standalone => XmlHeaderMode::Standalone,
      },
      have_xml_other_attrs: schema_type.have_xml_other_attrs,
      have_xml_other_children: schema_type.have_direct_xml_other_children,
    },
    content_structure: None,
    members: Vec::new(),
    particle_members: Vec::new(),
  }
}

#[derive(Clone, Default)]
struct ParticleMetadataResolver {
  by_qname: HashMap<String, ParticleLeafMetadata>,
  by_element_qname: HashMap<String, ParticleLeafMetadata>,
}

impl ParticleMetadataResolver {
  fn new(schema_type: &SchemaType) -> Self {
    let mut by_qname = HashMap::new();
    let mut by_element_qname = HashMap::new();
    for child in flatten_schema_children(&schema_type.children) {
      if child.name.is_empty() {
        continue;
      }
      let metadata = ParticleLeafMetadata {
        qname: child.name.clone(),
        property_name: child.property_name.clone(),
        property_comments: child.property_comments.clone(),
        additional_elements: schema_type.additional_elements.clone(),
      };
      by_qname
        .entry(child.name.clone())
        .or_insert_with(|| metadata.clone());
      if let Some(element_qname) = child.name.split('/').nth(1) {
        by_element_qname
          .entry(element_qname.to_string())
          .or_insert(metadata);
      }
    }
    Self {
      by_qname,
      by_element_qname,
    }
  }

  fn resolve(&self, qname: &str) -> ParticleLeafMetadata {
    if let Some(metadata) = self.by_qname.get(qname) {
      return metadata.clone();
    }
    if let Some(element_qname) = qname.split('/').nth(1)
      && let Some(metadata) = self.by_element_qname.get(element_qname)
    {
      return metadata.clone();
    }
    ParticleLeafMetadata {
      qname: qname.to_string(),
      property_name: String::new(),
      property_comments: String::new(),
      additional_elements: Vec::new(),
    }
  }
}

fn flatten_schema_children(children: &[SchemaTypeChild]) -> Vec<&SchemaTypeChild> {
  let mut out = Vec::new();
  for child in children {
    out.push(child);
    out.extend(flatten_schema_children(&child.children));
  }
  out
}

fn build_particle_members_for_root(
  build_context: ParticleBuildContext<'_>,
  root: &ParticleNode,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
  out: &mut Vec<MemberDecl>,
  extra_types: &mut Vec<TypeDecl>,
) -> Result<()> {
  match root.kind {
    ParticleNodeKind::Sequence | ParticleNodeKind::All => build_sequence_members(
      build_context,
      &root.children,
      metadata,
      schema,
      context,
      out,
      extra_types,
    ),
    ParticleNodeKind::Choice => {
      let (field, mut nested_types) =
        build_choice_field(build_context, root, 1, 1, metadata, schema, context)?;
      out.push(MemberDecl::Field(field));
      extra_types.append(&mut nested_types);
      Ok(())
    }
    ParticleNodeKind::Element | ParticleNodeKind::Any => {
      out.push(MemberDecl::Field(build_leaf_field(
        root, metadata, schema, context,
      )?));
      Ok(())
    }
  }
}

fn build_sequence_members(
  build_context: ParticleBuildContext<'_>,
  nodes: &[ParticleNode],
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
  out: &mut Vec<MemberDecl>,
  extra_types: &mut Vec<TypeDecl>,
) -> Result<()> {
  let local_choice_field_total = nodes
    .iter()
    .filter(|node| matches!(node.kind, ParticleNodeKind::Choice))
    .count();
  let local_sequence_field_total = nodes
    .iter()
    .filter(|node| {
      matches!(
        node.kind,
        ParticleNodeKind::Sequence | ParticleNodeKind::All
      )
    })
    .count();
  let mut local_choice_field_index = 0usize;
  let mut local_sequence_field_index = 0usize;
  for node in nodes {
    match node.kind {
      ParticleNodeKind::Element | ParticleNodeKind::Any => {
        out.push(MemberDecl::Field(build_leaf_field(
          node, metadata, schema, context,
        )?));
      }
      ParticleNodeKind::Choice => {
        local_choice_field_index += 1;
        let (field, mut nested_types) = build_choice_field(
          build_context.clone(),
          node,
          local_choice_field_index,
          local_choice_field_total,
          metadata,
          schema,
          context,
        )?;
        out.push(MemberDecl::Field(field));
        extra_types.append(&mut nested_types);
      }
      ParticleNodeKind::Sequence | ParticleNodeKind::All => {
        local_sequence_field_index += 1;
        let (field, mut nested_types) = build_sequence_field(
          build_context.clone(),
          node,
          local_sequence_field_index,
          local_sequence_field_total,
          metadata,
          schema,
          context,
        )?;
        out.push(MemberDecl::Field(field));
        extra_types.append(&mut nested_types);
      }
    }
  }

  Ok(())
}

fn build_choice_field(
  build_context: ParticleBuildContext<'_>,
  choice: &ParticleNode,
  local_choice_field_index: usize,
  local_choice_field_total: usize,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(FieldDecl, Vec<TypeDecl>)> {
  let field_name = choice_field_name(local_choice_field_index, local_choice_field_total);
  let enum_name = choice_enum_name(&build_context);

  let mut enum_members = Vec::new();
  let mut extra_types = Vec::new();
  for (variant_index, branch) in choice.children.iter().enumerate() {
    match branch.kind {
      ParticleNodeKind::Element | ParticleNodeKind::Any => {
        enum_members.push(MemberDecl::Variant(build_leaf_variant(
          branch, metadata, schema, context,
        )?));
      }
      ParticleNodeKind::Choice => {
        let helper_context = ParticleBuildContext {
          schema_type: build_context.schema_type,
          root_type_name: build_context.root_type_name.clone(),
          naming: Rc::clone(&build_context.naming),
        };
        let (nested_choice_field, mut nested_types) =
          build_choice_field(helper_context, branch, 1, 1, metadata, schema, context)?;
        let payload = nested_choice_field.type_ref.clone();
        enum_members.push(MemberDecl::Variant(VariantDecl {
          rust_name: format!("Choice{}", variant_index + 1),
          docs: String::new(),
          version: branch.version.clone(),
          wire: VariantWireDecl::Choice {
            qnames: collect_branch_qnames(branch),
          },
          payload,
        }));
        extra_types.append(&mut nested_types);
      }
      ParticleNodeKind::Sequence | ParticleNodeKind::All => {
        let helper_name = sequence_helper_name(&build_context);
        let helper_context = ParticleBuildContext {
          schema_type: build_context.schema_type,
          root_type_name: build_context.root_type_name.clone(),
          naming: Rc::clone(&build_context.naming),
        };
        let helper_type = build_sequence_helper_type(
          helper_context,
          &helper_name,
          branch,
          metadata,
          schema,
          context,
          &mut extra_types,
        )?;
        enum_members.push(MemberDecl::Variant(VariantDecl {
          rust_name: format!("Sequence{}", variant_index + 1),
          docs: String::new(),
          version: branch.version.clone(),
          wire: VariantWireDecl::Sequence {
            qnames: collect_branch_qnames(branch),
          },
          payload: TypeRefDecl {
            rust_type: helper_type.rust_name.clone(),
            module_path: None,
          },
        }));
        extra_types.push(helper_type);
      }
    }
  }
  disambiguate_choice_variant_names(&mut enum_members);

  let field = FieldDecl {
    rust_name: field_name,
    docs: String::new(),
    version: choice.version.clone(),
    wire: FieldWireDecl::Choice,
    cardinality: particle_node_cardinality(choice),
    type_ref: TypeRefDecl {
      rust_type: enum_name.clone(),
      module_path: None,
    },
    validators: Vec::new(),
  };

  extra_types.push(TypeDecl {
    rust_name: enum_name,
    xml_qname: None,
    docs: String::new(),
    version: (!choice.version.is_empty()).then_some(choice.version.clone()),
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    content_structure: None,
    members: Vec::new(),
    particle_members: enum_members,
  });

  Ok((field, extra_types))
}

fn choice_field_name(local_choice_field_index: usize, local_choice_field_total: usize) -> String {
  if local_choice_field_total <= 1 {
    "choice".to_string()
  } else {
    format!("choice{}", local_choice_field_index)
  }
}

fn choice_enum_name(build_context: &ParticleBuildContext<'_>) -> String {
  let mut naming = build_context.naming.borrow_mut();
  naming.choice_type_index += 1;
  let root = build_context.root_type_name.to_upper_camel_case();
  if naming.choice_type_total <= 1 {
    format!("{root}Choice")
  } else {
    format!("{root}Choice{}", naming.choice_type_index)
  }
}

fn sequence_helper_name(build_context: &ParticleBuildContext<'_>) -> String {
  let mut naming = build_context.naming.borrow_mut();
  naming.sequence_type_index += 1;
  let root = build_context.root_type_name.to_upper_camel_case();
  if naming.sequence_type_total <= 1 {
    format!("{root}Sequence")
  } else {
    format!("{root}Sequence{}", naming.sequence_type_index)
  }
}

fn sequence_field_name(
  local_sequence_field_index: usize,
  local_sequence_field_total: usize,
) -> String {
  if local_sequence_field_total <= 1 {
    "sequence".to_string()
  } else {
    format!("sequence{}", local_sequence_field_index)
  }
}

fn build_sequence_field(
  build_context: ParticleBuildContext<'_>,
  sequence: &ParticleNode,
  local_sequence_field_index: usize,
  local_sequence_field_total: usize,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(FieldDecl, Vec<TypeDecl>)> {
  let helper_name = sequence_helper_name(&build_context);
  let mut extra_types = Vec::new();
  let helper_type = build_sequence_helper_type(
    build_context,
    &helper_name,
    sequence,
    metadata,
    schema,
    context,
    &mut extra_types,
  )?;
  let field = FieldDecl {
    rust_name: sequence_field_name(local_sequence_field_index, local_sequence_field_total),
    docs: String::new(),
    version: sequence.version.clone(),
    wire: FieldWireDecl::Child {
      qname: String::new(),
      qnames: collect_first_visible_qnames(sequence),
    },
    cardinality: particle_sequence_field_cardinality(sequence),
    type_ref: TypeRefDecl {
      rust_type: helper_type.rust_name.clone(),
      module_path: None,
    },
    validators: Vec::new(),
  };
  extra_types.push(helper_type);
  Ok((field, extra_types))
}

fn build_sequence_helper_type(
  build_context: ParticleBuildContext<'_>,
  helper_name: &str,
  sequence: &ParticleNode,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
  extra_types: &mut Vec<TypeDecl>,
) -> Result<TypeDecl> {
  let mut members = Vec::new();
  build_sequence_members(
    build_context,
    &sequence.children,
    metadata,
    schema,
    context,
    &mut members,
    extra_types,
  )?;
  Ok(TypeDecl {
    rust_name: helper_name.to_string(),
    xml_qname: None,
    docs: String::new(),
    version: (!sequence.version.is_empty()).then_some(sequence.version.clone()),
    is_abstract: false,
    kind: TypeKind::HelperStruct,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    content_structure: None,
    members: Vec::new(),
    particle_members: members,
  })
}

fn build_leaf_field(
  node: &ParticleNode,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<FieldDecl> {
  let raw_qname = node.qname.as_deref().unwrap_or_default();
  let leaf = if node.kind == ParticleNodeKind::Any {
    ParticleLeafMetadata::default()
  } else {
    metadata.resolve(raw_qname)
  };
  let qname = if leaf.qname.is_empty() {
    raw_qname
  } else {
    leaf.qname.as_str()
  };
  let child_kind = if node.kind == ParticleNodeKind::Any {
    SchemaTypeChildKind::Any
  } else {
    effective_child_kind_from_name(qname, SchemaTypeChildKind::Child, context)
  };

  Ok(FieldDecl {
    rust_name: if child_kind == SchemaTypeChildKind::Any {
      "xml_children".to_string()
    } else if leaf.property_name.is_empty() {
      crate::sdk_code::schemas::schema_child_field_rust_name(
        qname.split('/').nth(1).unwrap_or(qname),
      )
    } else {
      crate::sdk_code::schemas::schema_child_field_rust_name(leaf.property_name.as_str())
    },
    docs: if leaf.property_comments.is_empty() {
      " _".to_string()
    } else {
      leaf.property_comments.clone()
    },
    version: node.version.clone(),
    wire: match child_kind {
      SchemaTypeChildKind::TextChild => FieldWireDecl::TextChild {
        qname: qname.to_string(),
      },
      SchemaTypeChildKind::Any => FieldWireDecl::Any,
      _ => FieldWireDecl::Child {
        qname: qname.to_string(),
        qnames: Vec::new(),
      },
    },
    cardinality: particle_node_cardinality(node),
    type_ref: if child_kind == SchemaTypeChildKind::Any {
      TypeRefDecl {
        rust_type: "std::boxed::Box<str>".to_string(),
        module_path: None,
      }
    } else {
      build_child_type_ref_from_name(qname, child_kind, schema, context)?
    },
    validators: Vec::new(),
  })
}

fn build_leaf_variant(
  node: &ParticleNode,
  metadata: &ParticleMetadataResolver,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<VariantDecl> {
  let raw_qname = node.qname.as_deref().unwrap_or_default();
  let leaf = if node.kind == ParticleNodeKind::Any {
    ParticleLeafMetadata::default()
  } else {
    metadata.resolve(raw_qname)
  };
  let qname = if leaf.qname.is_empty() {
    raw_qname
  } else {
    leaf.qname.as_str()
  };
  let child_kind = if node.kind == ParticleNodeKind::Any {
    SchemaTypeChildKind::Any
  } else {
    effective_child_kind_from_name(qname, SchemaTypeChildKind::Child, context)
  };

  Ok(VariantDecl {
    rust_name: if child_kind == SchemaTypeChildKind::Any {
      "XmlAny".to_string()
    } else {
      qname
        .split('/')
        .nth(1)
        .unwrap_or(qname)
        .split(':')
        .next_back()
        .unwrap_or(qname)
        .to_upper_camel_case()
    },
    docs: leaf.property_comments,
    version: node.version.clone(),
    wire: match child_kind {
      SchemaTypeChildKind::TextChild => VariantWireDecl::TextChild {
        qnames: vec![qname.to_string()],
      },
      SchemaTypeChildKind::Any => VariantWireDecl::Any,
      _ => VariantWireDecl::Child {
        qnames: vec![qname.to_string()],
      },
    },
    payload: if child_kind == SchemaTypeChildKind::Any {
      TypeRefDecl {
        rust_type: "std::boxed::Box<str>".to_string(),
        module_path: None,
      }
    } else {
      build_child_type_ref_from_name(qname, child_kind, schema, context)?
    },
  })
}

fn particle_node_cardinality(node: &ParticleNode) -> Cardinality {
  match (node.max_occurs, node.min_occurs) {
    (Some(max), _) if max > 1 => Cardinality::Many,
    (None, _) => Cardinality::Many,
    (_, 0) => Cardinality::Optional,
    _ => Cardinality::One,
  }
}

fn particle_sequence_field_cardinality(node: &ParticleNode) -> Cardinality {
  match node.max_occurs {
    Some(max) if max > 1 => Cardinality::Many,
    None => Cardinality::Many,
    Some(_) => {
      if particle_node_can_be_empty(node) {
        Cardinality::Optional
      } else {
        Cardinality::One
      }
    }
  }
}

fn particle_node_can_be_empty(node: &ParticleNode) -> bool {
  if node.min_occurs == 0 {
    return true;
  }

  match node.kind {
    ParticleNodeKind::Element | ParticleNodeKind::Any => false,
    ParticleNodeKind::Choice => node.children.iter().any(particle_node_can_be_empty),
    ParticleNodeKind::Sequence | ParticleNodeKind::All => {
      node.children.iter().all(particle_node_can_be_empty)
    }
  }
}

fn collect_branch_qnames(node: &ParticleNode) -> Vec<String> {
  let mut qnames = Vec::new();
  collect_branch_qnames_inner(node, &mut qnames);
  qnames
}

fn collect_first_visible_qnames(node: &ParticleNode) -> Vec<String> {
  let mut qnames = Vec::new();
  collect_first_visible_qnames_inner(node, &mut qnames);
  qnames
}

fn collect_first_visible_qnames_inner(node: &ParticleNode, out: &mut Vec<String>) -> bool {
  match node.kind {
    ParticleNodeKind::Element => {
      if let Some(qname) = &node.qname
        && !out.contains(qname)
      {
        out.push(qname.clone());
      }
      particle_node_can_be_empty(node)
    }
    ParticleNodeKind::Any => particle_node_can_be_empty(node),
    ParticleNodeKind::Choice => {
      let mut can_be_empty = false;
      for branch in &node.children {
        can_be_empty |= collect_first_visible_qnames_inner(branch, out);
      }
      can_be_empty
    }
    ParticleNodeKind::Sequence => {
      for child in &node.children {
        let child_can_be_empty = collect_first_visible_qnames_inner(child, out);
        if !child_can_be_empty {
          return false;
        }
      }
      true
    }
    ParticleNodeKind::All => {
      let mut can_be_empty = true;
      for child in &node.children {
        can_be_empty &= collect_first_visible_qnames_inner(child, out);
      }
      can_be_empty
    }
  }
}

fn collect_branch_qnames_inner(node: &ParticleNode, out: &mut Vec<String>) {
  if let Some(qname) = &node.qname {
    out.push(qname.clone());
  }
  for child in &node.children {
    collect_branch_qnames_inner(child, out);
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ParticleNamingTotals {
  choice_types: usize,
  sequence_types: usize,
}

fn count_particle_naming_totals(root: &ParticleNode) -> ParticleNamingTotals {
  let mut totals = ParticleNamingTotals::default();
  count_particle_naming_totals_inner(root, &mut totals, true);
  totals
}

fn count_particle_naming_totals_inner(
  node: &ParticleNode,
  totals: &mut ParticleNamingTotals,
  is_root: bool,
) {
  match node.kind {
    ParticleNodeKind::Choice => {
      totals.choice_types += 1;
      for branch in &node.children {
        if branch.kind == ParticleNodeKind::Choice {
          totals.sequence_types += 1;
        }
        count_particle_naming_totals_inner(branch, totals, false);
      }
    }
    ParticleNodeKind::Sequence | ParticleNodeKind::All => {
      if !is_root {
        totals.sequence_types += 1;
      }
      for child in &node.children {
        count_particle_naming_totals_inner(child, totals, false);
      }
    }
    ParticleNodeKind::Element | ParticleNodeKind::Any => {}
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_code::schemas::CodegenContext;
  use crate::sdk_data::open_xml::{OpenXmlSchemaTypeParticle, OpenXmlSchemaTypeParticleOccur};
  use crate::sdk_data::sdk_data_model::Schema;

  fn occur(min: Option<u64>, max: Option<u64>) -> OpenXmlSchemaTypeParticleOccur {
    OpenXmlSchemaTypeParticleOccur {
      min,
      max,
      include_version: false,
      version: String::new(),
    }
  }

  #[test]
  fn lowers_group_by_propagating_constraints() {
    let leaf = OpenXmlSchemaTypeParticle {
      kind: "Element".to_string(),
      name: "a:CT_Test/a:test".to_string(),
      occurs: vec![occur(Some(0), Some(2))],
      initial_version: "Office2010".to_string(),
      ..Default::default()
    };
    let group = OpenXmlSchemaTypeParticle {
      kind: "Group".to_string(),
      occurs: vec![occur(Some(0), Some(3))],
      initial_version: "Office2013".to_string(),
      require_filter: true,
      items: vec![leaf],
      ..Default::default()
    };

    let lowered = lower_particle_node(&group, &[]).expect("lower group");
    assert_eq!(lowered.kind, ParticleNodeKind::Element);
    assert_eq!(lowered.min_occurs, 0);
    assert_eq!(lowered.max_occurs, Some(6));
    assert_eq!(lowered.version, "Office2013");
    assert!(lowered.require_filter);
  }

  #[test]
  fn lowers_blank_kind_named_leaf_as_element() {
    let leaf = OpenXmlSchemaTypeParticle {
      name: "a:CT_Test/a:test".to_string(),
      occurs: vec![occur(Some(0), Some(1))],
      ..Default::default()
    };

    let lowered = lower_particle_node(&leaf, &[]).expect("lower blank-kind leaf");
    assert_eq!(lowered.kind, ParticleNodeKind::Element);
    assert_eq!(lowered.qname.as_deref(), Some("a:CT_Test/a:test"));
    assert_eq!(lowered.min_occurs, 0);
    assert_eq!(lowered.max_occurs, Some(1));
  }

  #[test]
  fn missing_min_in_present_occurs_is_optional() {
    let leaf = OpenXmlSchemaTypeParticle {
      name: "mso:CT_Commands/mso:commands".to_string(),
      occurs: vec![occur(None, Some(1))],
      ..Default::default()
    };

    let lowered = lower_particle_node(&leaf, &[]).expect("lower optional leaf");
    assert_eq!(lowered.min_occurs, 0);
    assert_eq!(particle_node_cardinality(&lowered), Cardinality::Optional);
  }

  #[test]
  fn missing_occurs_defaults_to_single_required() {
    let leaf = OpenXmlSchemaTypeParticle {
      name: "mso:CT_ButtonRegular/mso:button".to_string(),
      ..Default::default()
    };

    let lowered = lower_particle_node(&leaf, &[]).expect("lower required leaf");
    assert_eq!(lowered.min_occurs, 1);
    assert_eq!(lowered.max_occurs, Some(1));
    assert_eq!(particle_node_cardinality(&lowered), Cardinality::One);
  }

  #[test]
  fn optional_contents_make_sequence_helper_field_optional() {
    let sequence = ParticleNode {
      kind: ParticleNodeKind::Sequence,
      qname: None,
      version: String::new(),
      require_filter: false,
      min_occurs: 1,
      max_occurs: Some(1),
      metadata: None,
      children: vec![ParticleNode {
        kind: ParticleNodeKind::Element,
        qname: Some("w:CT_Background/w:background".to_string()),
        version: String::new(),
        require_filter: false,
        min_occurs: 0,
        max_occurs: Some(1),
        metadata: None,
        children: Vec::new(),
      }],
    };

    assert_eq!(
      particle_sequence_field_cardinality(&sequence),
      Cardinality::Optional
    );
  }

  #[test]
  fn required_contents_keep_sequence_helper_field_required() {
    let sequence = ParticleNode {
      kind: ParticleNodeKind::Sequence,
      qname: None,
      version: String::new(),
      require_filter: false,
      min_occurs: 1,
      max_occurs: Some(1),
      metadata: None,
      children: vec![ParticleNode {
        kind: ParticleNodeKind::Element,
        qname: Some("w:CT_Body/w:body".to_string()),
        version: String::new(),
        require_filter: false,
        min_occurs: 1,
        max_occurs: Some(1),
        metadata: None,
        children: Vec::new(),
      }],
    };

    assert_eq!(
      particle_sequence_field_cardinality(&sequence),
      Cardinality::One
    );
  }

  #[test]
  fn resolves_metadata_by_element_qname_suffix() {
    let schema_type = SchemaType {
      additional_elements: vec!["x:test".to_string()],
      children: vec![SchemaTypeChild {
        name: "w:CT_TblWidth/w:left".to_string(),
        property_name: "TableCellLeftMargin".to_string(),
        property_comments: "Table Cell Left Margin Default".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    };

    let resolver = ParticleMetadataResolver::new(&schema_type);
    let metadata = resolver.resolve("w:CT_TblWidthDxaNil/w:left");
    assert_eq!(metadata.qname, "w:CT_TblWidth/w:left");
    assert_eq!(metadata.property_name, "TableCellLeftMargin");
    assert_eq!(metadata.property_comments, "Table Cell Left Margin Default");
  }

  #[test]
  fn allocates_top_level_numbered_choice_and_sequence_names() {
    let context = ParticleBuildContext {
      schema_type: &SchemaType::default(),
      root_type_name: "SimpleField".to_string(),
      naming: Rc::new(RefCell::new(ParticleNamingState {
        choice_type_total: 2,
        sequence_type_total: 2,
        ..Default::default()
      })),
    };

    assert_eq!(choice_field_name(1, 2), "choice1");
    assert_eq!(choice_enum_name(&context), "SimpleFieldChoice1");
    assert_eq!(sequence_helper_name(&context), "SimpleFieldSequence1");
    assert_eq!(choice_field_name(2, 2), "choice2");
    assert_eq!(choice_enum_name(&context), "SimpleFieldChoice2");
    assert_eq!(sequence_helper_name(&context), "SimpleFieldSequence2");
    assert_eq!(sequence_field_name(1, 2), "sequence1");
    assert_eq!(sequence_field_name(2, 2), "sequence2");
  }

  #[test]
  fn omits_numeric_suffix_for_single_choice_and_sequence_name() {
    let context = ParticleBuildContext {
      schema_type: &SchemaType::default(),
      root_type_name: "SimpleField".to_string(),
      naming: Rc::new(RefCell::new(ParticleNamingState {
        choice_type_total: 1,
        sequence_type_total: 1,
        ..Default::default()
      })),
    };

    assert_eq!(choice_field_name(1, 1), "choice");
    assert_eq!(choice_enum_name(&context), "SimpleFieldChoice");
    assert_eq!(sequence_helper_name(&context), "SimpleFieldSequence");
    assert_eq!(sequence_field_name(1, 1), "sequence");
  }

  #[test]
  fn particle_cardinality_respects_required_choice() {
    let required_choice = ParticleNode {
      kind: ParticleNodeKind::Choice,
      qname: None,
      version: String::new(),
      require_filter: false,
      min_occurs: 1,
      max_occurs: Some(1),
      metadata: None,
      children: Vec::new(),
    };
    let optional_choice = ParticleNode {
      min_occurs: 0,
      ..required_choice.clone()
    };
    let repeated_choice = ParticleNode {
      max_occurs: None,
      ..required_choice.clone()
    };

    assert_eq!(
      particle_node_cardinality(&required_choice),
      Cardinality::One
    );
    assert_eq!(
      particle_node_cardinality(&optional_choice),
      Cardinality::Optional
    );
    assert_eq!(
      particle_node_cardinality(&repeated_choice),
      Cardinality::Many
    );
  }

  #[test]
  fn combines_nested_sequence_occurs_without_error() {
    let nested = OpenXmlSchemaTypeParticle {
      kind: "Sequence".to_string(),
      occurs: vec![occur(Some(0), Some(2))],
      items: vec![OpenXmlSchemaTypeParticle {
        kind: "Element".to_string(),
        name: "a:CT_Test/a:test".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    };

    let root = OpenXmlSchemaTypeParticle {
      kind: "Sequence".to_string(),
      items: vec![nested],
      ..Default::default()
    };

    let schema_type = SchemaType {
      class_name: "TestType".to_string(),
      particle: root,
      composite_kind: SchemaTypeCompositeKind::OneSequence,
      kind: SchemaTypeKind::Composite,
      ..Default::default()
    };

    let lowered = lower_particle_model(&schema_type).expect("lower root");
    let root = lowered.expect("root");
    assert_eq!(root.kind, ParticleNodeKind::Sequence);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].kind, ParticleNodeKind::Sequence);
  }

  #[test]
  fn rollout_includes_xsd_repeatable_choice_with_particle() {
    let schema_type = SchemaType {
      composite_kind: SchemaTypeCompositeKind::XsdRepeatableChoice,
      particle: OpenXmlSchemaTypeParticle {
        kind: "Sequence".to_string(),
        ..Default::default()
      },
      ..Default::default()
    };

    assert!(is_particle_rollout_type(&schema_type));
  }

  #[test]
  fn parent_choice_has_any_applies_to_direct_choice_with_leading_sequence_field() {
    let child_schema = Schema {
      module_name: "test".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Drawing/t:drawing".to_string(),
          class_name: "Drawing".to_string(),
          parent_choice_has_any_in: vec!["Run.choice".to_string()],
          kind: SchemaTypeKind::Composite,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_RPr/t:rPr".to_string(),
          class_name: "RunProperties".to_string(),
          kind: SchemaTypeKind::Composite,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Text/t:t".to_string(),
          class_name: "Text".to_string(),
          kind: SchemaTypeKind::LeafText,
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let run_schema_type = SchemaType {
      name: "t:CT_R/t:r".to_string(),
      class_name: "Run".to_string(),
      kind: SchemaTypeKind::Composite,
      composite_kind: SchemaTypeCompositeKind::OneSequence,
      particle: OpenXmlSchemaTypeParticle {
        kind: "Sequence".to_string(),
        items: vec![
          OpenXmlSchemaTypeParticle {
            kind: "Group".to_string(),
            occurs: vec![occur(None, Some(1))],
            items: vec![OpenXmlSchemaTypeParticle {
              kind: "Sequence".to_string(),
              items: vec![OpenXmlSchemaTypeParticle {
                kind: "Element".to_string(),
                name: "t:CT_RPr/t:rPr".to_string(),
                occurs: vec![occur(None, Some(1))],
                ..Default::default()
              }],
              ..Default::default()
            }],
            ..Default::default()
          },
          OpenXmlSchemaTypeParticle {
            kind: "Group".to_string(),
            occurs: vec![occur(None, None)],
            items: vec![OpenXmlSchemaTypeParticle {
              kind: "Choice".to_string(),
              items: vec![
                OpenXmlSchemaTypeParticle {
                  kind: "Element".to_string(),
                  name: "t:CT_Text/t:t".to_string(),
                  ..Default::default()
                },
                OpenXmlSchemaTypeParticle {
                  kind: "Element".to_string(),
                  name: "t:CT_Drawing/t:drawing".to_string(),
                  ..Default::default()
                },
              ],
              ..Default::default()
            }],
            ..Default::default()
          },
        ],
        ..Default::default()
      },
      children: vec![
        SchemaTypeChild {
          kind: SchemaTypeChildKind::Sequence,
          optional: true,
          children: vec![SchemaTypeChild {
            name: "t:CT_RPr/t:rPr".to_string(),
            property_name: "RunProperties".to_string(),
            kind: SchemaTypeChildKind::Child,
            optional: true,
            ..Default::default()
          }],
          ..Default::default()
        },
        SchemaTypeChild {
          kind: SchemaTypeChildKind::Choice,
          repeated: true,
          children: vec![
            SchemaTypeChild {
              name: "t:CT_Text/t:t".to_string(),
              property_name: "Text".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "t:CT_Drawing/t:drawing".to_string(),
              property_name: "Drawing".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let schema = Schema {
      module_name: "test".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        run_schema_type.clone(),
        child_schema.types[0].clone(),
        child_schema.types[1].clone(),
        child_schema.types[2].clone(),
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let output = build_particle_type_output(&run_schema_type, &schema, &context).unwrap();
    let choice_field = output
      .particle_members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Choice) => Some(field),
        _ => None,
      })
      .collect::<Vec<_>>();
    let [choice_field] = choice_field.as_slice() else {
      panic!("expected one direct choice field");
    };
    let choice_type = output
      .extra_types
      .iter()
      .find(|ty| ty.rust_name == choice_field.type_ref.rust_type)
      .unwrap();
    assert!(choice_type.effective_members().iter().any(|member| {
      matches!(member, MemberDecl::Variant(variant) if variant.rust_name == "XmlAny")
    }));
  }

  #[test]
  fn parent_choice_has_any_applies_to_real_wordprocessing_run() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../..")
      .canonicalize()
      .expect("repo root");
    let gen_context =
      crate::sdk_data::context::Context::new(&repo_root.join("data")).expect("context");
    let mut schemas = crate::sdk_data::schemas::gen_schemas(&gen_context);
    let schema_extensions = crate::sdk_data::schema_extensions::read_schema_extensions(
      &repo_root.join("sdk_data/schema_extensions"),
    )
    .expect("schema extensions");
    crate::sdk_data::schema_extensions::apply_schema_extensions(&mut schemas, &schema_extensions)
      .expect("apply schema extensions");
    let context = CodegenContext::new(&schemas);
    let schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("word schema");
    let run_schema_type = schema
      .types
      .iter()
      .find(|ty| ty.class_name == "Run" && ty.name == "w:CT_R/w:r")
      .expect("run type");

    let output =
      build_particle_type_output(run_schema_type, schema, &context).expect("particle output");
    let choice_field = output
      .particle_members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Choice) => Some(field),
        _ => None,
      })
      .collect::<Vec<_>>();
    let [choice_field] = choice_field.as_slice() else {
      panic!("expected one direct choice field");
    };
    let choice_type = output
      .extra_types
      .iter()
      .find(|ty| ty.rust_name == choice_field.type_ref.rust_type)
      .expect("run choice type");
    assert!(choice_type.effective_members().iter().any(|member| {
      matches!(member, MemberDecl::Variant(variant) if variant.rust_name == "XmlAny")
    }));
  }
}
