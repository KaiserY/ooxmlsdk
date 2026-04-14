use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, ContentModelDecl, ElementKind, EnumDecl, EnumValueType, EnumVariantDecl, FieldDecl,
  FieldWireDecl, MemberDecl, SchemaModuleDecl, SystemSupportDecl, TypeDecl, TypeKind, TypeRefDecl,
  ValidatorDecl, ValidatorKind, VariantDecl, VariantWireDecl, XmlHeaderMode, XmlnsMode,
};
use crate::sdk_code::helpers::{
  AttrTypeKind, FlatParticleKind, StructuredParticleKind, classify_attr_type,
  flatten_one_sequence_particles, is_one_sequence_flatten, is_one_sequence_structurable,
  structure_one_sequence_particles,
};
use crate::sdk_code::schemas::{
  CodegenContext, ResolvedCompositeChild, ResolvedOneSequenceChild,
  ResolvedOneSequenceChoiceVariant, ResolvedOneSequenceSequenceVariant,
  one_sequence_choice_enum_name, one_sequence_choice_field_name,
  one_sequence_choice_sequence_struct_name,
};
use crate::sdk_code::versioning::effective_version;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaType, SchemaTypeApiKind, SchemaTypeAttribute, SchemaTypeAttributeValidator,
  SchemaTypeXmlHeader,
};
use crate::simple_type::simple_type_mapping;
use crate::utils::escape_snake_case;
use heck::{ToSnakeCase, ToUpperCamelCase};

fn disambiguate_choice_variant_names(members: &mut [MemberDecl]) {
  let mut counts = std::collections::HashMap::<String, usize>::new();
  for member in members.iter() {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    *counts.entry(variant.rust_name.clone()).or_insert(0) += 1;
  }

  let mut used = std::collections::HashSet::<String>::new();
  for member in members.iter_mut() {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    if counts.get(&variant.rust_name).copied().unwrap_or_default() <= 1 {
      used.insert(variant.rust_name.clone());
      continue;
    }

    let base_name = variant.rust_name.clone();
    let prefix_name = variant_qname_prefix(variant)
      .map(|prefix| format!("{}{}", prefix.to_upper_camel_case(), base_name))
      .filter(|candidate| !candidate.is_empty());

    if let Some(candidate) = prefix_name
      && used.insert(candidate.clone())
    {
      variant.rust_name = candidate;
      continue;
    }

    let mut index = 2usize;
    loop {
      let candidate = format!("{base_name}{index}");
      if used.insert(candidate.clone()) {
        variant.rust_name = candidate;
        break;
      }
      index += 1;
    }
  }
}

fn variant_qname_prefix(variant: &VariantDecl) -> Option<&str> {
  let qname = match &variant.wire {
    VariantWireDecl::Child { qnames } | VariantWireDecl::TextChild { qnames } => qnames.first()?,
    VariantWireDecl::Any | VariantWireDecl::Text => return None,
  };
  let element_qname = qname.split('/').nth(1).unwrap_or(qname.as_str());
  element_qname.split(':').next()
}

fn child_variant_rust_name(qname: &str) -> String {
  let element_qname = qname.split('/').nth(1).unwrap_or(qname);
  let mut parts = element_qname.split(':');
  match (parts.next(), parts.next()) {
    (Some(prefix), Some(local)) => {
      format!(
        "{}{}",
        prefix.to_upper_camel_case(),
        local.to_upper_camel_case()
      )
    }
    (Some(local), None) => local.to_upper_camel_case(),
    _ => element_qname.to_upper_camel_case(),
  }
}

pub fn build_codegen_ir(schema: &Schema, context: &CodegenContext<'_>) -> Result<SchemaModuleDecl> {
  let mut types = Vec::new();
  for schema_type in &schema.types {
    let (primary_type, extra_types) =
      build_type_decl(schema_type, schema, context).map_err(|err| {
        format!(
          "failed to build IR type {} ({}) in {}: {err}",
          schema_type.class_name, schema_type.name, schema.module_name
        )
      })?;
    types.push(primary_type);
    types.extend(extra_types);
  }

  Ok(SchemaModuleDecl {
    module_name: schema.module_name.clone(),
    target_namespace: schema.target_namespace.clone(),
    prefix: schema.prefix.clone(),
    typed_namespace: schema.typed_namespace.clone(),
    enums: schema
      .enums
      .iter()
      .map(|schema_enum| EnumDecl {
        rust_name: schema_enum.name.clone(),
        docs: String::new(),
        version: schema_enum.version.clone(),
        value_type: match schema_enum.r#type.as_str() {
          "xsd:byte" | "xsd:int" | "xsd:integer" | "xsd:long" | "xsd:short"
          | "xsd:unsignedByte" | "xsd:unsignedInt" | "xsd:unsignedLong" | "xsd:unsignedShort" => {
            EnumValueType::NumericLike
          }
          _ => EnumValueType::StringLike,
        },
        variants: schema_enum
          .facets
          .iter()
          .map(|facet| EnumVariantDecl {
            rust_name: if facet.name.is_empty() {
              facet.value.clone()
            } else {
              facet.name.clone()
            },
            xml_value: facet.value.clone(),
            aliases: facet.aliases.clone(),
            version: facet.version.clone(),
          })
          .collect(),
      })
      .collect(),
    types,
  })
}

fn build_type_decl(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(TypeDecl, Vec<TypeDecl>)> {
  let mut members = schema_type
    .attributes
    .iter()
    .map(|attr| build_attr_member_decl(attr, schema, context))
    .collect::<Result<Vec<_>>>()?;
  let extra_types = if has_mixed_choice_children_pattern(schema_type) {
    build_mixed_choice_children_members(schema_type, schema, context, &mut members)
      .map_err(|err| format!("mixed choice children: {err}"))?
  } else if is_one_sequence_flatten(schema_type) {
    build_flatten_one_sequence_members(schema_type, schema, context, &mut members)
      .map_err(|err| format!("flatten one-sequence: {err}"))?
  } else if is_one_sequence_structurable(schema_type) {
    build_structured_one_sequence_members(schema_type, schema, context, &mut members)
      .map_err(|err| format!("structured one-sequence: {err}"))?
  } else {
    let direct_child_members = build_direct_child_member_decls(schema_type, schema, context)?;
    let content_model = build_content_model_decl(schema_type);
    if content_model != Some(ContentModelDecl::GenericChildrenFallback) {
      members.extend(direct_child_members.clone());
    }
    let extra_types = build_simple_one_choice_members(schema_type, schema, context, &mut members)
      .map_err(|err| format!("simple one-choice: {err}"))?;
    if content_model == Some(ContentModelDecl::GenericChildrenFallback) && extra_types.is_empty() {
      build_generic_children_members(schema_type, schema, context, &mut members)
        .map_err(|err| format!("generic children fallback: {err}"))?
    } else if extra_types.is_empty() && direct_child_members.is_empty() {
      build_generic_children_members(schema_type, schema, context, &mut members)
        .map_err(|err| format!("generic children fallback: {err}"))?
    } else {
      extra_types
    }
  };

  Ok((
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
          crate::sdk_data::sdk_data_model::SchemaTypeKind::LeafText => ElementKind::LeafText,
          crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf => ElementKind::Leaf,
          crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite => ElementKind::Composite,
          crate::sdk_data::sdk_data_model::SchemaTypeKind::Derived => ElementKind::Derived,
          crate::sdk_data::sdk_data_model::SchemaTypeKind::Struct => ElementKind::Composite,
        })
      },
      content_model: build_content_model_decl(schema_type),
      base_rust_name: (!schema_type.base_class.is_empty()
        && schema_type.base_class != "OpenXmlPartRootElement")
        .then(|| schema_type.base_class.clone()),
      xml_content: build_xml_content_type_ref(schema_type, schema, context)?,
      support: SystemSupportDecl {
        xmlns_mode: if schema_type.has_xmlns_fields {
          XmlnsMode::MapOnly
        } else {
          XmlnsMode::None
        },
        xml_header: match schema_type.xml_header {
          SchemaTypeXmlHeader::None => XmlHeaderMode::None,
          SchemaTypeXmlHeader::Plain => XmlHeaderMode::Plain,
          SchemaTypeXmlHeader::Standalone => XmlHeaderMode::Standalone,
        },
        has_mce: schema_type.has_mc_ignorable_field,
      },
      members,
    },
    extra_types,
  ))
}

fn build_attr_member_decl(
  attr: &SchemaTypeAttribute,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<MemberDecl> {
  Ok(MemberDecl::Field(FieldDecl {
    rust_name: if attr.property_name.is_empty() {
      escape_snake_case(attr.q_name.to_snake_case())
    } else {
      escape_snake_case(attr.property_name.to_snake_case())
    },
    docs: attr.property_comments.clone(),
    version: attr.version.clone(),
    wire: FieldWireDecl::Attribute {
      qname: attr.q_name.clone(),
      bit: attr.bit,
    },
    cardinality: if attr.required {
      Cardinality::One
    } else {
      Cardinality::Optional
    },
    type_ref: build_attr_type_ref(attr, schema, context)?,
    validators: build_attr_validator_decls(attr),
  }))
}

fn build_attr_type_ref(
  attr: &SchemaTypeAttribute,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<TypeRefDecl> {
  Ok(
    match classify_attr_type(attr.r#type.as_str()).ok_or_else(|| attr.r#type.clone())? {
      AttrTypeKind::List => TypeRefDecl {
        rust_type: "String".to_string(),
        module_path: None,
      },
      AttrTypeKind::Enum { .. } => {
        let (enum_module_name, enum_name) = context.resolve_attr_enum_module(&attr.r#type)?;

        TypeRefDecl {
          rust_type: enum_name.to_upper_camel_case(),
          module_path: if enum_module_name == schema.module_name {
            None
          } else {
            Some(format!("crate::schemas::{enum_module_name}"))
          },
        }
      }
      AttrTypeKind::Simple { simple_type, .. } => TypeRefDecl {
        rust_type: simple_type.to_string(),
        module_path: Some("crate::simple_type".to_string()),
      },
    },
  )
}

fn build_xml_content_type_ref(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Option<TypeRefDecl>> {
  if !schema_type.text_value_type.is_empty() {
    return Ok(Some(TypeRefDecl {
      rust_type: schema_type.text_value_type.clone(),
      module_path: Some("crate::simple_type".to_string()),
    }));
  }

  if schema_type.kind == crate::sdk_data::sdk_data_model::SchemaTypeKind::LeafText {
    // continue below
  } else if schema_type.kind == crate::sdk_data::sdk_data_model::SchemaTypeKind::Derived {
    let Some(base_type) = context.type_by_class_name(schema_type.base_class.as_str()) else {
      return Ok(None);
    };
    if base_type.text_value_type.is_empty()
      && base_type.kind != crate::sdk_data::sdk_data_model::SchemaTypeKind::LeafText
      && base_type.api_kind != SchemaTypeApiKind::LeafTextWrapper
    {
      return Ok(None);
    }
  } else {
    return Ok(None);
  }

  if schema_type.name.is_empty() || !schema_type.name.contains('/') {
    return Ok(None);
  }

  let first_name = &schema_type.name[..schema_type.name.find('/').unwrap()];

  if let Some(schema_enum) = context.enum_by_type(first_name) {
    let enum_module = context
      .enum_module_by_type(first_name)
      .ok_or_else(|| format!("{first_name:?}"))?;

    return Ok(Some(TypeRefDecl {
      rust_type: schema_enum.name.to_upper_camel_case(),
      module_path: if enum_module == schema.module_name {
        None
      } else {
        Some(format!("crate::schemas::{enum_module}"))
      },
    }));
  }

  if let Some(type_ref) = build_simple_type_ref_from_name(first_name) {
    return Ok(Some(type_ref));
  }

  let kind = classify_attr_type(first_name).ok_or_else(|| first_name.to_string())?;
  let AttrTypeKind::Simple { simple_type, .. } = kind else {
    return Ok(None);
  };

  Ok(Some(TypeRefDecl {
    rust_type: simple_type.to_string(),
    module_path: Some("crate::simple_type".to_string()),
  }))
}

fn build_simple_type_ref_from_name(name: &str) -> Option<TypeRefDecl> {
  let simple_type = simple_type_mapping(name);
  if simple_type == name {
    None
  } else {
    Some(TypeRefDecl {
      rust_type: simple_type.to_string(),
      module_path: Some("crate::simple_type".to_string()),
    })
  }
}

fn build_child_type_ref_from_name(
  child_name: &str,
  child_kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<TypeRefDecl> {
  if let Some(child_type) = context.type_by_name(child_name) {
    build_child_type_ref(child_kind, child_type, schema, context)
  } else if let Some(type_ref) = build_simple_type_ref_from_name(child_name) {
    Ok(type_ref)
  } else {
    Err(child_name.to_string().into())
  }
}

fn effective_child_kind_from_name(
  child_name: &str,
  child_kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind,
  context: &CodegenContext<'_>,
) -> crate::sdk_data::sdk_data_model::SchemaTypeChildKind {
  use crate::sdk_data::sdk_data_model::{
    SchemaTypeApiKind, SchemaTypeChildKind, SchemaTypeXmlHeader,
  };

  if !matches!(
    child_kind,
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
  ) {
    return child_kind;
  }

  let Some(child_type) = context.type_by_name(child_name) else {
    return child_kind;
  };

  if child_type.api_kind == SchemaTypeApiKind::LeafTextWrapper
    && child_type.attributes.is_empty()
    && !child_type.has_xmlns_fields
    && !child_type.has_mc_ignorable_field
    && child_type.xml_header == SchemaTypeXmlHeader::None
  {
    SchemaTypeChildKind::TextChild
  } else {
    SchemaTypeChildKind::Child
  }
}

fn build_direct_child_member_decls(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Vec<MemberDecl>> {
  let mut members = Vec::new();
  let mut field_name_set = std::collections::HashSet::new();

  for child in &schema_type.children {
    if !matches!(
      child.kind,
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
        | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    ) {
      continue;
    }

    let rust_name = if child.property_name.is_empty() {
      escape_snake_case(
        child
          .name
          .split('/')
          .nth(1)
          .unwrap_or(child.name.as_str())
          .to_snake_case(),
      )
    } else {
      escape_snake_case(child.property_name.to_snake_case())
    };
    if !field_name_set.insert(rust_name.clone()) {
      continue;
    }

    let version = if child.initial_version.is_empty() {
      context
        .type_by_name(child.name.as_str())
        .and_then(|child_type| child_type.version.clone())
        .unwrap_or_default()
    } else {
      child.initial_version.clone()
    };

    let effective_kind = effective_child_kind_from_name(child.name.as_str(), child.kind, context);

    members.push(MemberDecl::Field(FieldDecl {
      rust_name,
      docs: if child.property_comments.is_empty() {
        " _".to_string()
      } else {
        child.property_comments.clone()
      },
      version,
      wire: match effective_kind {
        crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child => FieldWireDecl::Child {
          qname: child.name.clone(),
        },
        crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => {
          FieldWireDecl::TextChild {
            qname: child.name.clone(),
          }
        }
        _ => unreachable!(),
      },
      cardinality: if child.repeated {
        Cardinality::Many
      } else if child.optional {
        Cardinality::Optional
      } else {
        Cardinality::One
      },
      type_ref: build_child_type_ref_from_name(
        child.name.as_str(),
        effective_kind,
        schema,
        context,
      )?,
      validators: Vec::new(),
    }));
  }

  Ok(members)
}

fn build_child_type_ref(
  child_kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind,
  child_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<TypeRefDecl> {
  if child_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    && child_type.api_kind == SchemaTypeApiKind::LeafTextWrapper
    && child_type.attributes.is_empty()
    && !child_type.has_xmlns_fields
    && !child_type.has_mc_ignorable_field
    && child_type.xml_header == SchemaTypeXmlHeader::None
  {
    return build_xml_content_type_ref(child_type, schema, context)?
      .ok_or_else(|| child_type.name.clone().into());
  }

  let child_prefix = context
    .type_prefix(child_type.name.as_str())
    .ok_or_else(|| child_type.name.clone())?;

  Ok(TypeRefDecl {
    rust_type: child_type.class_name.to_upper_camel_case(),
    module_path: if child_prefix == schema.prefix {
      None
    } else {
      Some(format!(
        "crate::schemas::{}",
        context
          .type_module(child_type.name.as_str())
          .ok_or_else(|| child_type.name.clone())?
      ))
    },
  })
}

fn build_simple_one_choice_members(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  members: &mut Vec<MemberDecl>,
) -> Result<Vec<TypeDecl>> {
  let resolved_choice = match context.resolve_one_choice(schema_type) {
    Ok(resolved_choice) => resolved_choice,
    Err(_) => return Ok(Vec::new()),
  };

  if resolved_choice.variants.is_empty() {
    return Ok(Vec::new());
  }

  let choice_version = schema_type
    .children
    .first()
    .map(|child| child.initial_version.clone())
    .unwrap_or_default();

  members.push(MemberDecl::Field(FieldDecl {
    rust_name: resolved_choice.field_name.clone(),
    docs: " Choice of child elements.".to_string(),
    version: choice_version.clone(),
    wire: FieldWireDecl::Choice,
    cardinality: Cardinality::Many,
    type_ref: TypeRefDecl {
      rust_type: resolved_choice.enum_name.clone(),
      module_path: None,
    },
    validators: Vec::new(),
  }));

  let mut enum_members = resolved_choice
    .variants
    .iter()
    .map(|variant| build_simple_one_choice_variant_decl(variant, schema, context))
    .collect::<Result<Vec<_>>>()?;
  disambiguate_choice_variant_names(&mut enum_members);

  Ok(vec![TypeDecl {
    rust_name: resolved_choice.enum_name.clone(),
    xml_qname: None,
    docs: format!(" Choice variants for {}.", schema_type.class_name),
    version: schema_type.version.clone(),
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    members: enum_members,
  }])
}

fn build_simple_one_choice_variant_decl(
  variant: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<MemberDecl> {
  let effective_kind = effective_child_kind_from_name(variant.name, variant.kind, context);

  let wire = match effective_kind {
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => VariantWireDecl::Any,
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild if variant.name.is_empty() => {
      VariantWireDecl::Text
    }
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => VariantWireDecl::TextChild {
      qnames: vec![variant.name.to_string()],
    },
    _ => VariantWireDecl::Child {
      qnames: vec![variant.name.to_string()],
    },
  };

  let payload = if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any {
    TypeRefDecl {
      rust_type: "String".to_string(),
      module_path: None,
    }
  } else if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    && variant.name.is_empty()
  {
    TypeRefDecl {
      rust_type: "StringValue".to_string(),
      module_path: Some("crate::simple_type".to_string()),
    }
  } else {
    build_child_type_ref_from_name(variant.name, effective_kind, schema, context)?
  };

  let rust_name = if variant.name.is_empty() {
    variant.field_name.to_upper_camel_case()
  } else {
    child_variant_rust_name(variant.name)
  };

  Ok(MemberDecl::Variant(VariantDecl {
    rust_name,
    docs: variant.property_comments.to_string(),
    version: variant.version.to_string(),
    wire,
    payload,
  }))
}

fn build_flatten_one_sequence_members(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  members: &mut Vec<MemberDecl>,
) -> Result<Vec<TypeDecl>> {
  let mut extra_types = Vec::new();
  let mut field_name_set = std::collections::HashSet::new();
  let mut paragraph_prefix_members = Vec::new();
  let flat_particles = flatten_one_sequence_particles(schema_type);
  let choice_slot_count = flat_particles
    .iter()
    .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
    .count();
  let mut choice_slot_index = 0usize;

  for flat_particle in flat_particles {
    match flat_particle.kind {
      FlatParticleKind::Leaf(child_particle) => {
        let child =
          context.resolve_one_sequence_child(schema_type, child_particle.name.as_str())?;
        if !field_name_set.insert(child.field_name.to_string()) {
          continue;
        }

        let effective_kind = effective_child_kind_from_name(child.name, child.kind, context);

        let field = FieldDecl {
          rust_name: child.field_name.to_string(),
          docs: child.property_comments.to_string(),
          version: effective_version(child.version, flat_particle.initial_version).to_string(),
          wire: match effective_kind {
            crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => {
              FieldWireDecl::TextChild {
                qname: child.name.to_string(),
              }
            }
            crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => FieldWireDecl::Any,
            _ => FieldWireDecl::Child {
              qname: child.name.to_string(),
            },
          },
          cardinality: if flat_particle.repeated {
            Cardinality::Many
          } else if flat_particle.optional {
            Cardinality::Optional
          } else {
            Cardinality::One
          },
          type_ref: if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any {
            TypeRefDecl {
              rust_type: "String".to_string(),
              module_path: None,
            }
          } else {
            build_child_type_ref_from_name(child.name, effective_kind, schema, context)?
          },
          validators: Vec::new(),
        };
        if schema_type.class_name == "Paragraph" && field.rust_name == "paragraph_properties" {
          paragraph_prefix_members.push(MemberDecl::Field(field));
        } else {
          members.push(MemberDecl::Field(field));
        }
      }
      FlatParticleKind::Choice(choice_particle) => {
        let choice = context.resolve_one_sequence_choice(
          schema_type,
          choice_particle,
          choice_slot_count,
          choice_slot_index,
        )?;
        choice_slot_index += 1;

        if !field_name_set.insert(choice.field_name.clone()) {
          continue;
        }

        let choice_version = common_choice_version_ir(
          flat_particle.initial_version,
          &choice
            .variants
            .iter()
            .map(|variant| variant.version)
            .collect::<Vec<_>>(),
        )
        .to_string();

        members.push(MemberDecl::Field(FieldDecl {
          rust_name: choice.field_name.clone(),
          docs: choice.property_comments.clone(),
          version: choice_version,
          wire: FieldWireDecl::Choice,
          cardinality: if flat_particle.repeated {
            Cardinality::Many
          } else {
            Cardinality::Optional
          },
          type_ref: TypeRefDecl {
            rust_type: choice.enum_name.clone(),
            module_path: None,
          },
          validators: Vec::new(),
        }));

        let mut enum_members = choice
          .variants
          .iter()
          .map(|variant| build_one_sequence_choice_variant_decl(variant, schema, context))
          .collect::<Result<Vec<_>>>()?;
        disambiguate_choice_variant_names(&mut enum_members);

        extra_types.push(TypeDecl {
          rust_name: choice.enum_name,
          xml_qname: None,
          docs: choice.property_comments,
          version: schema_type.version.clone(),
          is_abstract: false,
          kind: TypeKind::ChoiceEnum,
          element_kind: None,
          content_model: None,
          base_rust_name: None,
          xml_content: None,
          support: SystemSupportDecl::default(),
          members: enum_members,
        });
      }
    }
  }

  if !paragraph_prefix_members.is_empty() {
    paragraph_prefix_members.append(members);
    *members = paragraph_prefix_members;
  }

  Ok(extra_types)
}

fn build_one_sequence_choice_variant_decl(
  variant: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<MemberDecl> {
  let effective_kind = effective_child_kind_from_name(variant.name, variant.kind, context);

  let wire = match effective_kind {
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => VariantWireDecl::Any,
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild if variant.name.is_empty() => {
      VariantWireDecl::Text
    }
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => VariantWireDecl::TextChild {
      qnames: vec![variant.name.to_string()],
    },
    _ => VariantWireDecl::Child {
      qnames: vec![variant.name.to_string()],
    },
  };

  let payload = if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any {
    TypeRefDecl {
      rust_type: "String".to_string(),
      module_path: None,
    }
  } else if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    && variant.name.is_empty()
  {
    TypeRefDecl {
      rust_type: "StringValue".to_string(),
      module_path: Some("crate::simple_type".to_string()),
    }
  } else {
    build_child_type_ref_from_name(variant.name, effective_kind, schema, context)?
  };

  Ok(MemberDecl::Variant(VariantDecl {
    rust_name: if variant.name.is_empty() {
      variant.field_name.to_upper_camel_case()
    } else {
      child_variant_rust_name(variant.name)
    },
    docs: variant.property_comments.to_string(),
    version: variant.version.to_string(),
    wire,
    payload,
  }))
}

fn build_structured_one_sequence_members(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  members: &mut Vec<MemberDecl>,
) -> Result<Vec<TypeDecl>> {
  let mut extra_types = Vec::new();
  let mut field_name_set = std::collections::HashSet::new();
  let mut paragraph_prefix_members = Vec::new();
  let structured_particles = structure_one_sequence_particles(schema_type);
  let choice_slot_count = structured_particles
    .iter()
    .filter(|particle| matches!(particle.kind, StructuredParticleKind::Choice(_)))
    .count();
  let mut choice_slot_index = 0usize;

  for particle in structured_particles {
    match particle.kind {
      StructuredParticleKind::Leaf(leaf) => {
        let child = context.resolve_one_sequence_child(schema_type, leaf.name.as_str())?;
        if !field_name_set.insert(child.field_name.to_string()) {
          continue;
        }

        let field = build_one_sequence_leaf_field_decl(
          &child,
          effective_version(child.version, particle.initial_version).to_string(),
          particle.optional,
          particle.repeated,
          schema,
          context,
        )?;
        if schema_type.class_name == "Paragraph" && field.rust_name == "paragraph_properties" {
          paragraph_prefix_members.push(MemberDecl::Field(field));
        } else {
          members.push(MemberDecl::Field(field));
        }
      }
      StructuredParticleKind::Choice(choice) => {
        let choice = context.resolve_one_sequence_structured_choice(
          schema_type,
          &choice,
          choice_slot_count,
          choice_slot_index,
        )?;
        choice_slot_index += 1;

        if !field_name_set.insert(choice.field_name.clone()) {
          continue;
        }

        let choice_version = common_choice_version_ir(
          particle.initial_version,
          &choice
            .variants
            .iter()
            .map(|variant| match variant {
              ResolvedOneSequenceChoiceVariant::Leaf(child) => child.version,
              ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
                common_choice_version_ir(
                  "",
                  &sequence_variant
                    .fields
                    .iter()
                    .map(|field| effective_version(field.child.version, field.initial_version))
                    .collect::<Vec<_>>(),
                )
              }
            })
            .collect::<Vec<_>>(),
        )
        .to_string();

        members.push(MemberDecl::Field(FieldDecl {
          rust_name: choice.field_name.clone(),
          docs: choice.property_comments.clone(),
          version: choice_version,
          wire: FieldWireDecl::Choice,
          cardinality: if particle.repeated {
            Cardinality::Many
          } else {
            Cardinality::Optional
          },
          type_ref: TypeRefDecl {
            rust_type: choice.enum_name.clone(),
            module_path: None,
          },
          validators: Vec::new(),
        }));

        let mut enum_members = Vec::new();
        for variant in &choice.variants {
          match variant {
            ResolvedOneSequenceChoiceVariant::Leaf(child) => {
              enum_members.push(build_one_sequence_choice_variant_decl(
                child, schema, context,
              )?);
            }
            ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
              extra_types.push(build_structured_one_sequence_helper_struct_decl(
                sequence_variant,
                schema,
                context,
              )?);
              enum_members.push(build_structured_one_sequence_sequence_variant_decl(
                sequence_variant,
              )?);
            }
          }
        }

        disambiguate_choice_variant_names(&mut enum_members);

        extra_types.push(TypeDecl {
          rust_name: choice.enum_name,
          xml_qname: None,
          docs: choice.property_comments,
          version: schema_type.version.clone(),
          is_abstract: false,
          kind: TypeKind::ChoiceEnum,
          element_kind: None,
          content_model: None,
          base_rust_name: None,
          xml_content: None,
          support: SystemSupportDecl::default(),
          members: enum_members,
        });
      }
    }
  }

  if !paragraph_prefix_members.is_empty() {
    paragraph_prefix_members.append(members);
    *members = paragraph_prefix_members;
  }

  Ok(extra_types)
}

fn build_one_sequence_leaf_field_decl(
  child: &ResolvedOneSequenceChild<'_>,
  version: String,
  optional: bool,
  repeated: bool,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<FieldDecl> {
  let effective_kind = effective_child_kind_from_name(child.name, child.kind, context);

  Ok(FieldDecl {
    rust_name: child.field_name.to_string(),
    docs: child.property_comments.to_string(),
    version,
    wire: match effective_kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => FieldWireDecl::TextChild {
        qname: child.name.to_string(),
      },
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => FieldWireDecl::Any,
      _ => FieldWireDecl::Child {
        qname: child.name.to_string(),
      },
    },
    cardinality: if repeated {
      Cardinality::Many
    } else if optional {
      Cardinality::Optional
    } else {
      Cardinality::One
    },
    type_ref: if effective_kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any {
      TypeRefDecl {
        rust_type: "String".to_string(),
        module_path: None,
      }
    } else {
      build_child_type_ref_from_name(child.name, effective_kind, schema, context)?
    },
    validators: Vec::new(),
  })
}

fn build_structured_one_sequence_helper_struct_decl(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<TypeDecl> {
  let helper_version = common_choice_version_ir(
    "",
    &sequence_variant
      .fields
      .iter()
      .map(|field| effective_version(field.child.version, field.initial_version))
      .collect::<Vec<_>>(),
  )
  .to_string();

  Ok(TypeDecl {
    rust_name: sequence_variant.struct_name.clone(),
    xml_qname: None,
    docs: sequence_variant.property_comments.clone(),
    version: (!helper_version.is_empty()).then_some(helper_version),
    is_abstract: false,
    kind: TypeKind::HelperStruct,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    members: sequence_variant
      .fields
      .iter()
      .map(|field| {
        build_one_sequence_leaf_field_decl(
          &field.child,
          effective_version(field.child.version, field.initial_version).to_string(),
          field.optional,
          field.repeated,
          schema,
          context,
        )
        .map(MemberDecl::Field)
      })
      .collect::<Result<Vec<_>>>()?,
  })
}

fn build_structured_one_sequence_sequence_variant_decl(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
) -> Result<MemberDecl> {
  Ok(MemberDecl::Variant(VariantDecl {
    rust_name: sequence_variant.variant_name.clone(),
    docs: sequence_variant.property_comments.clone(),
    version: common_choice_version_ir(
      "",
      &sequence_variant
        .fields
        .iter()
        .map(|field| effective_version(field.child.version, field.initial_version))
        .collect::<Vec<_>>(),
    )
    .to_string(),
    wire: VariantWireDecl::Child {
      qnames: sequence_variant
        .fields
        .iter()
        .map(|field| field.child.name.to_string())
        .collect(),
    },
    payload: TypeRefDecl {
      rust_type: sequence_variant.struct_name.clone(),
      module_path: None,
    },
  }))
}

fn build_mixed_choice_children_members(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  members: &mut Vec<MemberDecl>,
) -> Result<Vec<TypeDecl>> {
  let mut extra_types = Vec::new();
  let choice_index = schema_type
    .children
    .iter()
    .position(|child| child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice)
    .ok_or_else(|| schema_type.class_name.clone())?;
  let choice_child = &schema_type.children[choice_index];

  for child in &schema_type.children[..choice_index] {
    if let Some(field) = build_direct_child_member_decl_from_schema_child(child, schema, context)? {
      members.push(MemberDecl::Field(field));
    }
  }

  let choice_variants = collect_mixed_choice_variants(schema_type, context)?;
  let choice_version = common_choice_version_ir(
    choice_child.initial_version.as_str(),
    &choice_variants
      .iter()
      .map(|child| child.version)
      .collect::<Vec<_>>(),
  )
  .to_string();
  let choice_field_name = one_sequence_choice_field_name(schema_type, 1, 0);
  let choice_enum_name = one_sequence_choice_enum_name(schema_type, 1, 0);
  members.push(MemberDecl::Field(FieldDecl {
    rust_name: choice_field_name,
    docs: build_mixed_choice_property_comments(&choice_variants),
    version: choice_version,
    wire: FieldWireDecl::Choice,
    cardinality: if choice_child.repeated {
      Cardinality::Many
    } else {
      Cardinality::Optional
    },
    type_ref: TypeRefDecl {
      rust_type: choice_enum_name.clone(),
      module_path: None,
    },
    validators: Vec::new(),
  }));

  let mut enum_members = Vec::new();
  for (variant_index, child) in choice_variants.iter().enumerate() {
    match child.kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
        if child.children.iter().all(|item| {
          matches!(
            item.kind,
            crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
              | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
              | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any
          )
        }) {
          for sequence_child in &child.children {
            enum_members.push(build_mixed_choice_leaf_variant_decl(
              sequence_child,
              schema,
              context,
            )?);
          }
        } else {
          let sequence_leafs = collect_resolved_sequence_leafs_ir(&child.children);
          let sequence_variant = ResolvedOneSequenceSequenceVariant {
            variant_name: format!("Sequence{}", variant_index + 1),
            struct_name: one_sequence_choice_sequence_struct_name(schema_type, 1, 0, variant_index),
            property_comments: format!(
              " Sequence of {}",
              sequence_leafs
                .iter()
                .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
                .collect::<Vec<_>>()
                .join(", ")
            ),
            fields: sequence_leafs
              .iter()
              .map(|field| {
                let resolved_child = context.resolve_one_sequence_child(schema_type, field.name)?;
                Ok(crate::sdk_code::schemas::ResolvedOneSequenceSequenceField {
                  child: resolved_child,
                  optional: true,
                  repeated: false,
                  initial_version: field.version,
                })
              })
              .collect::<Result<Vec<_>>>()?,
          };
          extra_types.push(build_structured_one_sequence_helper_struct_decl(
            &sequence_variant,
            schema,
            context,
          )?);
          enum_members.push(build_structured_one_sequence_sequence_variant_decl(
            &sequence_variant,
          )?);
        }
      }
      _ => enum_members.push(build_mixed_choice_leaf_variant_decl(
        child, schema, context,
      )?),
    }
  }

  disambiguate_choice_variant_names(&mut enum_members);

  extra_types.push(TypeDecl {
    rust_name: choice_enum_name,
    xml_qname: None,
    docs: build_mixed_choice_property_comments(&choice_variants),
    version: schema_type.version.clone(),
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    members: enum_members,
  });

  for child in &schema_type.children[choice_index + 1..] {
    if let Some(field) = build_direct_child_member_decl_from_schema_child(child, schema, context)? {
      members.push(MemberDecl::Field(field));
    }
  }

  Ok(extra_types)
}

fn build_direct_child_member_decl_from_schema_child(
  child: &crate::sdk_data::sdk_data_model::SchemaTypeChild,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Option<FieldDecl>> {
  if !matches!(
    child.kind,
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
  ) {
    return Ok(None);
  }
  let field_name = if child.property_name.is_empty() {
    escape_snake_case(
      child
        .name
        .split('/')
        .nth(1)
        .unwrap_or(child.name.as_str())
        .to_snake_case(),
    )
  } else {
    escape_snake_case(child.property_name.to_snake_case())
  };
  let version = if child.initial_version.is_empty() {
    context
      .type_by_name(child.name.as_str())
      .and_then(|child_type| child_type.version.clone())
      .unwrap_or_default()
  } else {
    child.initial_version.clone()
  };
  let effective_kind = effective_child_kind_from_name(child.name.as_str(), child.kind, context);

  Ok(Some(FieldDecl {
    rust_name: field_name,
    docs: if child.property_comments.is_empty() {
      " _".to_string()
    } else {
      child.property_comments.clone()
    },
    version,
    wire: match effective_kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child => FieldWireDecl::Child {
        qname: child.name.clone(),
      },
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild => FieldWireDecl::TextChild {
        qname: child.name.clone(),
      },
      _ => unreachable!(),
    },
    cardinality: if child.repeated {
      Cardinality::Many
    } else if child.optional {
      Cardinality::Optional
    } else {
      Cardinality::One
    },
    type_ref: build_child_type_ref_from_name(child.name.as_str(), effective_kind, schema, context)?,
    validators: Vec::new(),
  }))
}

fn has_mixed_choice_children_pattern(schema_type: &SchemaType) -> bool {
  matches!(
    schema_type.composite_kind,
    crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind::SdkSequence
      | crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind::OneSequence
  ) && schema_type
    .children
    .iter()
    .filter(|child| child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice)
    .count()
    == 1
    && count_uncovered_direct_children_ir(schema_type) > 0
}

fn count_uncovered_direct_children_ir(schema_type: &SchemaType) -> usize {
  let Some(choice_child) = schema_type
    .children
    .iter()
    .find(|child| child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice)
  else {
    return 0;
  };
  let mut choice_leaf_names = std::collections::HashSet::new();
  collect_choice_child_leaf_names_ir(choice_child, &mut choice_leaf_names);
  schema_type
    .children
    .iter()
    .filter(|child| {
      matches!(
        child.kind,
        crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
          | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
      ) && !choice_leaf_names.contains(child.name.as_str())
    })
    .count()
}

fn collect_choice_child_leaf_names_ir(
  child: &crate::sdk_data::sdk_data_model::SchemaTypeChild,
  out: &mut std::collections::HashSet<String>,
) {
  match child.kind {
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
    | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => {
      if !child.name.is_empty() {
        out.insert(child.name.clone());
      }
    }
    crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice
    | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
      for item in &child.children {
        collect_choice_child_leaf_names_ir(item, out);
      }
    }
  }
}

fn collect_mixed_choice_variants<'a>(
  schema_type: &'a SchemaType,
  context: &'a CodegenContext<'a>,
) -> Result<Vec<ResolvedCompositeChild<'a>>> {
  let choice_child = schema_type
    .children
    .iter()
    .find(|child| child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice)
    .ok_or_else(|| schema_type.class_name.clone())?;
  let mut resolved = Vec::new();
  let mut resolved_names = std::collections::HashSet::new();
  collect_mixed_choice_variants_inner(
    &choice_child.children,
    &mut resolved,
    &mut resolved_names,
    true,
    context,
  )?;
  Ok(resolved)
}

fn collect_mixed_choice_variants_inner<'a>(
  children: &'a [crate::sdk_data::sdk_data_model::SchemaTypeChild],
  resolved: &mut Vec<ResolvedCompositeChild<'a>>,
  resolved_names: &mut std::collections::HashSet<String>,
  preserve_sequences: bool,
  context: &CodegenContext<'a>,
) -> Result<()> {
  for child in children {
    match child.kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => {
        push_mixed_resolved_child(child, resolved, resolved_names, context)?;
      }
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice => {
        collect_mixed_choice_variants_inner(
          &child.children,
          resolved,
          resolved_names,
          true,
          context,
        )?;
      }
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
        if preserve_sequences {
          let mut sequence_children = Vec::new();
          collect_mixed_choice_variants_inner(
            &child.children,
            &mut sequence_children,
            resolved_names,
            true,
            context,
          )?;
          let leaf_versions = collect_resolved_sequence_leafs_ir(&sequence_children);
          let sequence_version = common_choice_version_ir(
            child.initial_version.as_str(),
            &leaf_versions
              .iter()
              .map(|field| field.version)
              .collect::<Vec<_>>(),
          );
          resolved.push(ResolvedCompositeChild {
            name: "",
            variant_name: std::borrow::Cow::Borrowed("Sequence"),
            version: sequence_version,
            is_any: false,
            kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence,
            repeated: child.repeated,
            children: sequence_children,
          });
        } else {
          collect_mixed_choice_variants_inner(
            &child.children,
            resolved,
            resolved_names,
            preserve_sequences,
            context,
          )?;
        }
      }
    }
  }
  Ok(())
}

fn push_mixed_resolved_child<'a>(
  child: &'a crate::sdk_data::sdk_data_model::SchemaTypeChild,
  resolved: &mut Vec<ResolvedCompositeChild<'a>>,
  resolved_names: &mut std::collections::HashSet<String>,
  context: &CodegenContext<'a>,
) -> Result<()> {
  if child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
    && child.name.is_empty()
  {
    resolved.push(ResolvedCompositeChild {
      name: "",
      variant_name: std::borrow::Cow::Borrowed(if child.property_name.is_empty() {
        "Text"
      } else {
        child.property_name.as_str()
      }),
      version: child.initial_version.as_str(),
      is_any: false,
      kind: child.kind,
      repeated: child.repeated,
      children: Vec::new(),
    });
    return Ok(());
  }
  if !child.name.is_empty() && !resolved_names.insert(child.name.clone()) {
    return Ok(());
  }
  let (variant_name, is_any) =
    if child.kind == crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any {
      (
        std::borrow::Cow::Borrowed(if child.property_name.is_empty() {
          "UnknownXml"
        } else {
          child.property_name.as_str()
        }),
        true,
      )
    } else {
      (
        std::borrow::Cow::Borrowed(child.name.split('/').nth(1).unwrap_or(child.name.as_str())),
        false,
      )
    };
  resolved.push(ResolvedCompositeChild {
    name: child.name.as_str(),
    variant_name,
    version: context
      .type_by_name(child.name.as_str())
      .map(|ty| ty.version.as_deref().unwrap_or_default())
      .unwrap_or_default(),
    is_any,
    kind: child.kind,
    repeated: child.repeated,
    children: Vec::new(),
  });
  Ok(())
}

fn collect_resolved_sequence_leafs_ir<'a>(
  children: &'a [ResolvedCompositeChild<'a>],
) -> Vec<&'a ResolvedCompositeChild<'a>> {
  let mut out = Vec::new();
  collect_resolved_sequence_leafs_ir_inner(children, &mut out);
  out
}

fn collect_resolved_sequence_leafs_ir_inner<'a>(
  children: &'a [ResolvedCompositeChild<'a>],
  out: &mut Vec<&'a ResolvedCompositeChild<'a>>,
) {
  for child in children {
    match child.kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::TextChild
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Any => out.push(child),
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Choice
      | crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
        collect_resolved_sequence_leafs_ir_inner(&child.children, out);
      }
    }
  }
}

fn build_mixed_choice_property_comments(children: &[ResolvedCompositeChild<'_>]) -> String {
  let names = children
    .iter()
    .map(|child| match child.kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
        let sequence_leafs = collect_resolved_sequence_leafs_ir(&child.children);
        format!(
          "sequence of {}",
          sequence_leafs
            .iter()
            .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      _ => child.variant_name.to_string(),
    })
    .collect::<Vec<_>>()
    .join(", ");
  format!("Choice of {names}")
}

fn build_mixed_choice_leaf_variant_decl(
  child: &ResolvedCompositeChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<MemberDecl> {
  let synthetic_child = ResolvedOneSequenceChild {
    name: child.name,
    field_name: std::borrow::Cow::Owned(escape_snake_case(
      child.variant_name.to_string().to_snake_case(),
    )),
    property_comments: std::borrow::Cow::Borrowed(" _"),
    version: child.version,
    kind: child.kind,
  };
  build_one_sequence_choice_variant_decl(&synthetic_child, schema, context)
}

fn build_generic_children_members(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  members: &mut Vec<MemberDecl>,
) -> Result<Vec<TypeDecl>> {
  let resolved_children = context.resolve_children(schema_type)?;
  if resolved_children.is_empty() {
    return Ok(Vec::new());
  }

  let choice_version = common_choice_version_ir(
    "",
    &resolved_children
      .iter()
      .map(|child| child.version)
      .collect::<Vec<_>>(),
  )
  .to_string();
  let choice_field_name = one_sequence_choice_field_name(schema_type, 1, 0);
  let choice_enum_name = one_sequence_choice_enum_name(schema_type, 1, 0);
  members.push(MemberDecl::Field(FieldDecl {
    rust_name: choice_field_name,
    docs: "Choice of child elements.".to_string(),
    version: choice_version,
    wire: FieldWireDecl::Choice,
    cardinality: Cardinality::Many,
    type_ref: TypeRefDecl {
      rust_type: choice_enum_name.clone(),
      module_path: None,
    },
    validators: Vec::new(),
  }));

  let mut extra_types = Vec::new();
  let mut enum_members = Vec::new();
  for (variant_index, child) in resolved_children.iter().enumerate() {
    match child.kind {
      crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Sequence => {
        let sequence_leafs = collect_resolved_sequence_leafs_ir(&child.children);
        let sequence_variant = ResolvedOneSequenceSequenceVariant {
          variant_name: format!("Sequence{}", variant_index + 1),
          struct_name: one_sequence_choice_sequence_struct_name(schema_type, 1, 0, variant_index),
          property_comments: format!(
            " Sequence of {}",
            sequence_leafs
              .iter()
              .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
              .collect::<Vec<_>>()
              .join(", ")
          ),
          fields: sequence_leafs
            .iter()
            .map(|field| {
              let resolved_child = context.resolve_one_sequence_child(schema_type, field.name)?;
              Ok(crate::sdk_code::schemas::ResolvedOneSequenceSequenceField {
                child: resolved_child,
                optional: true,
                repeated: field.repeated,
                initial_version: field.version,
              })
            })
            .collect::<Result<Vec<_>>>()?,
        };
        extra_types.push(build_structured_one_sequence_helper_struct_decl(
          &sequence_variant,
          schema,
          context,
        )?);
        enum_members.push(build_structured_one_sequence_sequence_variant_decl(
          &sequence_variant,
        )?);
      }
      _ => enum_members.push(build_mixed_choice_leaf_variant_decl(
        child, schema, context,
      )?),
    }
  }

  disambiguate_choice_variant_names(&mut enum_members);

  extra_types.push(TypeDecl {
    rust_name: choice_enum_name,
    xml_qname: None,
    docs: "Choice of child elements.".to_string(),
    version: schema_type.version.clone(),
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    xml_content: None,
    support: SystemSupportDecl::default(),
    members: enum_members,
  });

  Ok(extra_types)
}

fn build_content_model_decl(schema_type: &SchemaType) -> Option<ContentModelDecl> {
  use crate::sdk_data::sdk_data_model::{SchemaTypeChildKind, SchemaTypeCompositeKind};

  if !matches!(
    schema_type.kind,
    crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite
      | crate::sdk_data::sdk_data_model::SchemaTypeKind::Derived
  ) {
    return None;
  }

  if schema_type.children.is_empty() {
    return None;
  }

  let value = if schema_type.composite_kind == SchemaTypeCompositeKind::OneAll
    && !schema_type.children.is_empty()
    && schema_type.children.iter().all(|child| {
      matches!(
        child.kind,
        SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
      )
    }) {
    ContentModelDecl::OneAllDirectChildren
  } else if schema_type.composite_kind == SchemaTypeCompositeKind::OneChoice
    && schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Choice
  {
    ContentModelDecl::OneChoiceSingle
  } else if has_mixed_choice_children_pattern(schema_type) {
    ContentModelDecl::MixedChoiceChildren
  } else if matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
  ) && schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Any
  {
    ContentModelDecl::SequenceAnyOnly
  } else if matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
  ) && schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Choice
  {
    ContentModelDecl::SequenceSingleChoice
  } else if matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
  ) && schema_type.children.iter().all(|child| {
    matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    )
  }) {
    ContentModelDecl::SequenceDirectChildren
  } else if is_one_sequence_flatten(schema_type) {
    ContentModelDecl::OneSequenceFlatten
  } else if is_one_sequence_structurable(schema_type) {
    ContentModelDecl::OneSequenceStructured
  } else {
    ContentModelDecl::GenericChildrenFallback
  };

  Some(value)
}

fn common_choice_version_ir<'a>(container_version: &'a str, variant_versions: &[&str]) -> &'a str {
  if crate::sdk_code::versioning::is_microsoft365_version(container_version)
    || (!variant_versions.is_empty()
      && variant_versions
        .iter()
        .all(|version| crate::sdk_code::versioning::is_microsoft365_version(version)))
  {
    "Microsoft365"
  } else {
    ""
  }
}

#[allow(dead_code)]
fn build_attr_validator_decls(attr: &SchemaTypeAttribute) -> Vec<ValidatorDecl> {
  attr
    .validators
    .iter()
    .filter_map(build_validator_decl)
    .collect()
}

#[allow(dead_code)]
fn build_validator_decl(validator: &SchemaTypeAttributeValidator) -> Option<ValidatorDecl> {
  let version = if validator.is_initial_version {
    "Office2007".to_string()
  } else {
    String::new()
  };

  match validator.name.as_str() {
    "RequiredValidator" => None,
    "StringValidator" => {
      build_string_validator_kind(validator).map(|kind| ValidatorDecl { version, kind })
    }
    "NumberValidator" => {
      build_number_validator_kind(validator).map(|kind| ValidatorDecl { version, kind })
    }
    _ => None,
  }
}

#[allow(dead_code)]
fn build_string_validator_kind(validator: &SchemaTypeAttributeValidator) -> Option<ValidatorKind> {
  let pattern = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "Pattern")
    .map(|argument| argument.value.clone());
  if let Some(regex) = pattern {
    return Some(ValidatorKind::Pattern { regex });
  }

  let min = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MinLength")
    .and_then(|argument| argument.value.parse::<u32>().ok());
  let max = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MaxLength")
    .and_then(|argument| argument.value.parse::<u32>().ok());

  if min.is_some() || max.is_some() {
    return Some(ValidatorKind::StringLength { min, max });
  }

  None
}

#[allow(dead_code)]
fn build_number_validator_kind(validator: &SchemaTypeAttributeValidator) -> Option<ValidatorKind> {
  let min_inclusive = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MinInclusive")
    .map(|argument| argument.value.clone());
  let min_exclusive = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MinExclusive")
    .map(|argument| argument.value.clone());
  let max_inclusive = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MaxInclusive")
    .map(|argument| argument.value.clone());
  let max_exclusive = validator
    .arguments
    .iter()
    .find(|argument| argument.name == "MaxExclusive")
    .map(|argument| argument.value.clone());

  if min_inclusive.is_none()
    && min_exclusive.is_none()
    && max_inclusive.is_none()
    && max_exclusive.is_none()
  {
    return None;
  }

  let min_inclusive_flag = min_inclusive.is_some();
  let max_inclusive_flag = max_inclusive.is_some();

  Some(ValidatorKind::NumberRange {
    min: min_inclusive.or(min_exclusive),
    max: max_inclusive.or(max_exclusive),
    min_inclusive: min_inclusive_flag,
    max_inclusive: max_inclusive_flag,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{
    SchemaTypeAttributeValidator, SchemaTypeAttributeValidatorArgument, SchemaTypeChild,
    SchemaTypeChildKind,
  };

  #[test]
  fn carries_schema_enums_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      enums: vec![crate::sdk_data::sdk_data_model::SchemaEnum {
        name: "TestEnum".to_string(),
        r#type: "xsd:string".to_string(),
        facets: vec![crate::sdk_data::sdk_data_model::SchemaEnumFacet {
          name: "FirstValue".to_string(),
          value: "first".to_string(),
          version: String::new(),
          aliases: vec!["uno".to_string()],
        }],
        version: Some("Office2010".to_string()),
      }],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    assert_eq!(ir.module_name, "test_module");
    assert_eq!(ir.enums.len(), 1);
    assert_eq!(ir.enums[0].rust_name, "TestEnum");
    assert_eq!(ir.enums[0].version.as_deref(), Some("Office2010"));
    assert_eq!(ir.enums[0].value_type, EnumValueType::StringLike);
    assert_eq!(ir.enums[0].variants.len(), 1);
    assert_eq!(ir.enums[0].variants[0].rust_name, "FirstValue");
    assert_eq!(ir.enums[0].variants[0].xml_value, "first");
    assert_eq!(ir.enums[0].variants[0].aliases, vec!["uno"]);
  }

  #[test]
  fn carries_schema_types_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          class_name: "Paragraph".to_string(),
          name: "t:CT_P/t:p".to_string(),
          summary: "Paragraph.".to_string(),
          version: Some("Office2007".to_string()),
          has_xmlns_fields: true,
          has_mc_ignorable_field: true,
          xml_header: SchemaTypeXmlHeader::Standalone,
          ..Default::default()
        },
        SchemaType {
          class_name: "TextValue".to_string(),
          name: "t:ST_Text/t:val".to_string(),
          summary: "Text value.".to_string(),
          api_kind: SchemaTypeApiKind::LeafTextWrapper,
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    assert_eq!(ir.types.len(), 2);
    assert_eq!(ir.types[0].rust_name, "Paragraph");
    assert_eq!(ir.types[0].kind, TypeKind::ElementStruct);
    assert_eq!(ir.types[0].xml_qname.as_deref(), Some("t:CT_P/t:p"));
    assert_eq!(ir.types[0].support.xmlns_mode, XmlnsMode::MapOnly);
    assert_eq!(ir.types[0].support.xml_header, XmlHeaderMode::Standalone);
    assert!(ir.types[0].support.has_mce);

    assert_eq!(ir.types[1].rust_name, "TextValue");
    assert_eq!(ir.types[1].kind, TypeKind::LeafTextAlias);
  }

  #[test]
  fn carries_attribute_members_and_validators_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![SchemaType {
        class_name: "Paragraph".to_string(),
        attributes: vec![SchemaTypeAttribute {
          q_name: ":val".to_string(),
          property_name: "Val".to_string(),
          r#type: "StringValue".to_string(),
          property_comments: "Value".to_string(),
          required: false,
          bit: Some(7),
          validators: vec![
            SchemaTypeAttributeValidator {
              name: "RequiredValidator".to_string(),
              ..Default::default()
            },
            SchemaTypeAttributeValidator {
              name: "StringValidator".to_string(),
              arguments: vec![SchemaTypeAttributeValidatorArgument {
                name: "Pattern".to_string(),
                r#type: "String".to_string(),
                value: "[a-z]+".to_string(),
              }],
              ..Default::default()
            },
          ],
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    assert_eq!(ir.types.len(), 1);
    assert_eq!(ir.types[0].members.len(), 1);
    let MemberDecl::Field(field) = &ir.types[0].members[0] else {
      panic!("expected field member");
    };
    assert_eq!(field.rust_name, "val");
    assert_eq!(field.docs, "Value");
    assert_eq!(field.version, "");
    assert_eq!(
      field.wire,
      FieldWireDecl::Attribute {
        qname: ":val".to_string(),
        bit: Some(7),
      }
    );
    assert_eq!(field.cardinality, Cardinality::Optional);
    assert_eq!(
      field.type_ref,
      TypeRefDecl {
        rust_type: "StringValue".to_string(),
        module_path: Some("crate::simple_type".to_string()),
      }
    );
    assert_eq!(
      field.validators,
      vec![ValidatorDecl {
        version: String::new(),
        kind: ValidatorKind::Pattern {
          regex: "[a-z]+".to_string(),
        },
      }]
    );
  }

  #[test]
  fn carries_direct_child_members_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Leaf/t:leaf".to_string(),
          class_name: "Leaf".to_string(),
          version: Some("Office2010".to_string()),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Text/t:text".to_string(),
          class_name: "TextLeaf".to_string(),
          base_class: "OpenXmlLeafTextElement".to_string(),
          api_kind: SchemaTypeApiKind::LeafTextWrapper,
          text_value_type: "StringValue".to_string(),
          ..Default::default()
        },
        SchemaType {
          class_name: "Paragraph".to_string(),
          children: vec![
            SchemaTypeChild {
              name: "t:CT_Leaf/t:leaf".to_string(),
              property_name: "LeafChild".to_string(),
              property_comments: "Leaf child".to_string(),
              kind: SchemaTypeChildKind::Child,
              optional: true,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "t:CT_Text/t:text".to_string(),
              property_name: "TextChild".to_string(),
              property_comments: "Text child".to_string(),
              kind: SchemaTypeChildKind::TextChild,
              repeated: true,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let paragraph = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "Paragraph")
      .unwrap();
    let fields: Vec<_> = paragraph
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field)
          if matches!(
            field.wire,
            FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. }
          ) =>
        {
          Some(field)
        }
        _ => None,
      })
      .collect();

    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].rust_name, "leaf_child");
    assert_eq!(fields[0].cardinality, Cardinality::Optional);
    assert_eq!(
      fields[0].type_ref,
      TypeRefDecl {
        rust_type: "Leaf".to_string(),
        module_path: None,
      }
    );
    assert_eq!(fields[1].rust_name, "text_child");
    assert_eq!(fields[1].cardinality, Cardinality::Many);
    assert_eq!(
      fields[1].type_ref,
      TypeRefDecl {
        rust_type: "StringValue".to_string(),
        module_path: Some("crate::simple_type".to_string()),
      }
    );
  }

  #[test]
  fn carries_simple_one_choice_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Leaf/t:leaf".to_string(),
          class_name: "Leaf".to_string(),
          version: Some("Office2010".to_string()),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_ChoiceHolder/t:holder".to_string(),
          class_name: "ChoiceHolder".to_string(),
          children: vec![SchemaTypeChild {
            kind: SchemaTypeChildKind::Choice,
            initial_version: "Office2010".to_string(),
            children: vec![SchemaTypeChild {
              name: "t:CT_Leaf/t:leaf".to_string(),
              property_name: "LeafChild".to_string(),
              property_comments: "Leaf child".to_string(),
              kind: SchemaTypeChildKind::Child,
              initial_version: "Office2010".to_string(),
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let holder = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "ChoiceHolder")
      .unwrap();
    let choice_field = holder
      .members
      .iter()
      .find_map(|member| match member {
        MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Choice) => Some(field),
        _ => None,
      })
      .unwrap();
    assert_eq!(choice_field.rust_name, "choice_holder_choice");
    assert_eq!(choice_field.cardinality, Cardinality::Many);
    assert_eq!(choice_field.type_ref.rust_type, "ChoiceHolderChoice");

    let choice_enum = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "ChoiceHolderChoice")
      .unwrap();
    assert_eq!(choice_enum.kind, TypeKind::ChoiceEnum);
    let MemberDecl::Variant(variant) = &choice_enum.members[0] else {
      panic!("expected variant member");
    };
    assert_eq!(variant.rust_name, "TLeaf");
    assert_eq!(
      variant.wire,
      VariantWireDecl::Child {
        qnames: vec!["t:CT_Leaf/t:leaf".to_string()],
      }
    );
  }

  #[test]
  fn carries_flatten_one_sequence_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Leaf/t:leaf".to_string(),
          class_name: "Leaf".to_string(),
          version: Some("Office2010".to_string()),
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Text/t:text".to_string(),
          class_name: "TextLeaf".to_string(),
          api_kind: SchemaTypeApiKind::LeafTextWrapper,
          text_value_type: "StringValue".to_string(),
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Seq/t:seq".to_string(),
          class_name: "SequenceHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind::OneSequence,
          children: vec![
            SchemaTypeChild {
              name: "t:CT_Leaf/t:leaf".to_string(),
              property_name: "LeafChild".to_string(),
              property_comments: "Leaf child".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "wrapper".to_string(),
              kind: SchemaTypeChildKind::Sequence,
              optional: true,
              children: vec![SchemaTypeChild {
                kind: SchemaTypeChildKind::Choice,
                children: vec![SchemaTypeChild {
                  name: "t:CT_Text/t:text".to_string(),
                  property_name: "TextChild".to_string(),
                  property_comments: "Text child".to_string(),
                  kind: SchemaTypeChildKind::TextChild,
                  initial_version: "Office2010".to_string(),
                  ..Default::default()
                }],
                ..Default::default()
              }],
              ..Default::default()
            },
            SchemaTypeChild {
              kind: SchemaTypeChildKind::Any,
              property_name: "UnknownXml".to_string(),
              property_comments: "Unknown xml".to_string(),
              repeated: true,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let holder = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "SequenceHolder")
      .unwrap();
    let fields: Vec<_> = holder
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field),
        _ => None,
      })
      .collect();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].rust_name, "leaf_child");
    assert!(matches!(fields[0].wire, FieldWireDecl::Child { .. }));
    assert_eq!(fields[0].cardinality, Cardinality::One);
    assert_eq!(fields[1].rust_name, "sequence_holder_choice");
    assert!(matches!(fields[1].wire, FieldWireDecl::Choice));
    assert_eq!(fields[1].cardinality, Cardinality::Optional);
    assert_eq!(fields[2].rust_name, "unknown_xml");
    assert_eq!(fields[2].wire, FieldWireDecl::Any);
    assert_eq!(fields[2].cardinality, Cardinality::Many);

    let choice_enum = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "SequenceHolderChoice")
      .unwrap();
    assert_eq!(choice_enum.kind, TypeKind::ChoiceEnum);
    let variants: Vec<_> = choice_enum
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Variant(variant) => Some(variant),
        _ => None,
      })
      .collect();
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].rust_name, "TText");
    assert_eq!(
      variants[0].wire,
      VariantWireDecl::TextChild {
        qnames: vec!["t:CT_Text/t:text".to_string()],
      }
    );
  }

  #[test]
  fn carries_structured_one_sequence_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_C/t:c".to_string(),
          class_name: "LeafC".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_D/t:d".to_string(),
          class_name: "LeafD".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Seq/t:seq".to_string(),
          class_name: "StructuredHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind::OneSequence,
          children: vec![SchemaTypeChild {
            name: "wrapper".to_string(),
            kind: SchemaTypeChildKind::Sequence,
            children: vec![
              SchemaTypeChild {
                name: "t:CT_A/t:a".to_string(),
                property_name: "LeafA".to_string(),
                property_comments: "Leaf A".to_string(),
                kind: SchemaTypeChildKind::Child,
                ..Default::default()
              },
              SchemaTypeChild {
                name: "choice_wrapper".to_string(),
                kind: SchemaTypeChildKind::Choice,
                children: vec![
                  SchemaTypeChild {
                    name: "t:CT_B/t:b".to_string(),
                    property_name: "LeafB".to_string(),
                    property_comments: "Leaf B".to_string(),
                    kind: SchemaTypeChildKind::Child,
                    ..Default::default()
                  },
                  SchemaTypeChild {
                    name: "seq_variant".to_string(),
                    kind: SchemaTypeChildKind::Sequence,
                    children: vec![
                      SchemaTypeChild {
                        name: "t:CT_C/t:c".to_string(),
                        property_name: "LeafC".to_string(),
                        property_comments: "Leaf C".to_string(),
                        kind: SchemaTypeChildKind::Child,
                        ..Default::default()
                      },
                      SchemaTypeChild {
                        name: "t:CT_D/t:d".to_string(),
                        property_name: "LeafD".to_string(),
                        property_comments: "Leaf D".to_string(),
                        kind: SchemaTypeChildKind::Child,
                        ..Default::default()
                      },
                    ],
                    ..Default::default()
                  },
                ],
                ..Default::default()
              },
            ],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let holder = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "StructuredHolder")
      .unwrap();
    let fields: Vec<_> = holder
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field),
        _ => None,
      })
      .collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].rust_name, "leaf_a");
    assert_eq!(fields[1].rust_name, "structured_holder_choice");
    assert!(matches!(fields[1].wire, FieldWireDecl::Choice));

    let choice_enum = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "StructuredHolderChoice")
      .unwrap();
    assert_eq!(choice_enum.kind, TypeKind::ChoiceEnum);
    assert_eq!(choice_enum.members.len(), 2);

    let helper_struct = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "StructuredHolderChoiceSequence2")
      .unwrap();
    assert_eq!(helper_struct.kind, TypeKind::HelperStruct);
    let helper_fields: Vec<_> = helper_struct
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field),
        _ => None,
      })
      .collect();
    assert_eq!(helper_fields.len(), 2);
    assert_eq!(helper_fields[0].rust_name, "leaf_c");
    assert_eq!(helper_fields[1].rust_name, "leaf_d");
  }

  #[test]
  fn carries_mixed_choice_children_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_C/t:c".to_string(),
          class_name: "LeafC".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_D/t:d".to_string(),
          class_name: "LeafD".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Mixed/t:mixed".to_string(),
          class_name: "MixedHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind::OneSequence,
          children: vec![
            SchemaTypeChild {
              name: "t:CT_A/t:a".to_string(),
              property_name: "LeafA".to_string(),
              property_comments: "Leaf A".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "choice_wrapper".to_string(),
              kind: SchemaTypeChildKind::Choice,
              children: vec![
                SchemaTypeChild {
                  name: "t:CT_B/t:b".to_string(),
                  property_name: "LeafB".to_string(),
                  property_comments: "Leaf B".to_string(),
                  kind: SchemaTypeChildKind::Child,
                  ..Default::default()
                },
                SchemaTypeChild {
                  name: "seq_variant".to_string(),
                  kind: SchemaTypeChildKind::Sequence,
                  children: vec![
                    SchemaTypeChild {
                      name: "t:CT_C/t:c".to_string(),
                      property_name: "LeafC".to_string(),
                      property_comments: "Leaf C".to_string(),
                      kind: SchemaTypeChildKind::Child,
                      ..Default::default()
                    },
                    SchemaTypeChild {
                      name: "t:CT_D/t:d".to_string(),
                      property_name: "LeafD".to_string(),
                      property_comments: "Leaf D".to_string(),
                      kind: SchemaTypeChildKind::Child,
                      ..Default::default()
                    },
                  ],
                  ..Default::default()
                },
              ],
              ..Default::default()
            },
            SchemaTypeChild {
              name: "t:CT_D/t:d".to_string(),
              property_name: "TrailingLeaf".to_string(),
              property_comments: "Trailing leaf".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let holder = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "MixedHolder")
      .unwrap();
    let fields: Vec<_> = holder
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field),
        _ => None,
      })
      .collect();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].rust_name, "leaf_a");
    assert_eq!(fields[1].rust_name, "mixed_holder_choice");
    assert_eq!(fields[2].rust_name, "trailing_leaf");

    let choice_enum = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "MixedHolderChoice")
      .unwrap();
    let variants: Vec<_> = choice_enum
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Variant(variant) => Some(variant),
        _ => None,
      })
      .collect();
    assert_eq!(variants.len(), 3);
    assert_eq!(variants[0].rust_name, "TB");
    assert_eq!(variants[1].rust_name, "TC");
    assert_eq!(variants[2].rust_name, "TD");
  }

  #[test]
  fn carries_generic_children_fallback_into_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Fallback/t:fb".to_string(),
          class_name: "FallbackHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          children: vec![SchemaTypeChild {
            name: "wrapper".to_string(),
            kind: SchemaTypeChildKind::Sequence,
            children: vec![SchemaTypeChild {
              name: "choice".to_string(),
              kind: SchemaTypeChildKind::Choice,
              children: vec![
                SchemaTypeChild {
                  name: "t:CT_A/t:a".to_string(),
                  property_name: "LeafA".to_string(),
                  property_comments: "Leaf A".to_string(),
                  kind: SchemaTypeChildKind::Child,
                  ..Default::default()
                },
                SchemaTypeChild {
                  name: "inner".to_string(),
                  kind: SchemaTypeChildKind::Sequence,
                  children: vec![SchemaTypeChild {
                    name: "t:CT_B/t:b".to_string(),
                    property_name: "LeafB".to_string(),
                    property_comments: "Leaf B".to_string(),
                    kind: SchemaTypeChildKind::Child,
                    ..Default::default()
                  }],
                  ..Default::default()
                },
              ],
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let ir = build_codegen_ir(&schema, &context).unwrap();

    let holder = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "FallbackHolder")
      .unwrap();
    let fields: Vec<_> = holder
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field),
        _ => None,
      })
      .collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].rust_name, "fallback_holder_choice");
    assert!(matches!(fields[0].wire, FieldWireDecl::Choice));
    assert_eq!(fields[0].cardinality, Cardinality::Many);

    let choice_enum = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "FallbackHolderChoice")
      .unwrap();
    assert_eq!(choice_enum.kind, TypeKind::ChoiceEnum);
    let variants: Vec<_> = choice_enum
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Variant(variant) => Some(variant),
        _ => None,
      })
      .collect();
    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0].rust_name, "TA");
    assert_eq!(variants[1].rust_name, "Sequence2");

    let helper_struct = ir
      .types
      .iter()
      .find(|ty| ty.rust_name == "FallbackHolderChoiceSequence2")
      .unwrap();
    assert_eq!(helper_struct.kind, TypeKind::HelperStruct);
  }

  #[test]
  fn drops_required_and_maps_pattern_and_range_validators() {
    let attr = SchemaTypeAttribute {
      validators: vec![
        SchemaTypeAttributeValidator {
          name: "RequiredValidator".to_string(),
          ..Default::default()
        },
        SchemaTypeAttributeValidator {
          name: "StringValidator".to_string(),
          is_initial_version: true,
          arguments: vec![SchemaTypeAttributeValidatorArgument {
            name: "Pattern".to_string(),
            r#type: "String".to_string(),
            value: "[A-Z]+".to_string(),
          }],
          ..Default::default()
        },
        SchemaTypeAttributeValidator {
          name: "NumberValidator".to_string(),
          arguments: vec![
            SchemaTypeAttributeValidatorArgument {
              name: "MinInclusive".to_string(),
              r#type: "Long".to_string(),
              value: "1".to_string(),
            },
            SchemaTypeAttributeValidatorArgument {
              name: "MaxExclusive".to_string(),
              r#type: "Long".to_string(),
              value: "10".to_string(),
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let validators = build_attr_validator_decls(&attr);

    assert_eq!(validators.len(), 2);
    assert_eq!(
      validators[0],
      ValidatorDecl {
        version: "Office2007".to_string(),
        kind: ValidatorKind::Pattern {
          regex: "[A-Z]+".to_string(),
        },
      }
    );
    assert_eq!(
      validators[1],
      ValidatorDecl {
        version: String::new(),
        kind: ValidatorKind::NumberRange {
          min: Some("1".to_string()),
          max: Some("10".to_string()),
          min_inclusive: true,
          max_inclusive: false,
        },
      }
    );
  }
}
