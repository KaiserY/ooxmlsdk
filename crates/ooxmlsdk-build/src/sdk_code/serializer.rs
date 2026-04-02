use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Arm, Ident, ItemFn, ItemImpl, Stmt, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::helpers::{
  FlatParticleKind, StructuredParticleKind, flatten_one_sequence_particles, is_composite_type,
  is_derived_type, is_leaf_element_type, is_leaf_text_type, is_leaf_text_wrapper,
  is_one_sequence_flatten, is_one_sequence_structurable, needs_xml_header,
  structure_one_sequence_particles, supports_compat_xmlns_fields,
};
use crate::sdk_code::schemas::{
  CodegenContext, ResolvedCompositeChild, ResolvedOneSequenceChild,
  ResolvedOneSequenceChoiceVariant, ResolvedOneSequenceSequenceVariant,
};
use crate::sdk_code::versioning::{
  effective_version, is_microsoft365_version, version_cfg_attrs, versioned_tokens,
};
use crate::sdk_data::sdk_data_model::CompatibilityRule;
use crate::sdk_data::sdk_data_model::{Schema, SchemaEnum, SchemaType, SchemaTypeAttribute};
use crate::utils::{escape_snake_case, escape_upper_camel_case};

pub fn gen_schema_serializer(
  schema: &Schema,
  compatibility_rules: &[CompatibilityRule],
  context: &CodegenContext<'_>,
) -> Result<TokenStream> {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  for schema_enum in &schema.enums {
    let (impl_block, display_impl) = gen_enum_serializer(schema, schema_enum)?;
    token_stream_list.push(impl_block);
    token_stream_list.push(display_impl);
  }

  for schema_type in &schema.types {
    if schema_type.is_abstract {
      continue;
    }

    let (impl_block, display_impl) =
      gen_struct_serializer(schema, schema_type, compatibility_rules, context)?;
    token_stream_list.push(impl_block);
    token_stream_list.push(display_impl);
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn structured_sequence_version(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
) -> &'static str {
  if !sequence_variant.fields.is_empty()
    && sequence_variant.fields.iter().all(|field| {
      is_microsoft365_version(effective_version(
        field.child.version,
        field.initial_version,
      ))
    })
  {
    "Microsoft365"
  } else {
    ""
  }
}

fn structured_choice_version(
  choice: &[ResolvedOneSequenceChoiceVariant<'_>],
  particle_version: &str,
) -> &'static str {
  if is_microsoft365_version(particle_version)
    || (!choice.is_empty()
      && choice.iter().all(|variant| match variant {
        ResolvedOneSequenceChoiceVariant::Leaf(child) => is_microsoft365_version(child.version),
        ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
          is_microsoft365_version(structured_sequence_version(sequence_variant))
        }
      }))
  {
    "Microsoft365"
  } else {
    ""
  }
}

fn flat_choice_version(
  choice: &crate::sdk_code::schemas::ResolvedOneSequenceChoice<'_>,
  particle_version: &str,
) -> &'static str {
  if is_microsoft365_version(particle_version)
    || (!choice.variants.is_empty()
      && choice
        .variants
        .iter()
        .all(|variant| is_microsoft365_version(variant.version)))
  {
    "Microsoft365"
  } else {
    ""
  }
}

fn gen_enum_serializer(schema: &Schema, schema_enum: &SchemaEnum) -> Result<(ItemImpl, ItemImpl)> {
  let enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    schema.module_name,
    schema_enum.name.to_upper_camel_case()
  ))?;
  let mut variants: Vec<Arm> = vec![];
  let enum_attrs = version_cfg_attrs(&schema_enum.version);

  for facet in &schema_enum.facets {
    let variant_ident: Ident = if facet.name.is_empty() {
      parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
    } else {
      parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
    };
    let variant_value = facet.value.as_str();

    variants.push(parse2(versioned_tokens(
      &facet.version,
      quote! {
        Self::#variant_ident => #variant_value,
      },
    ))?);
  }

  let impl_block = parse2(quote! {
    #( #enum_attrs )*
    impl #enum_type {
      pub fn as_xml_str(&self) -> &'static str {
        match self {
          #( #variants )*
        }
      }

      pub fn to_xml(&self) -> String {
        self.as_xml_str().to_string()
      }
    }
  })?;

  let display_impl = parse2(quote! {
    #( #enum_attrs )*
    impl std::fmt::Display for #enum_type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_xml_str())
      }
    }
  })?;

  Ok((impl_block, display_impl))
}

fn gen_struct_serializer(
  schema: &Schema,
  schema_type: &SchemaType,
  compatibility_rules: &[CompatibilityRule],
  context: &CodegenContext<'_>,
) -> Result<(ItemImpl, ItemImpl)> {
  let struct_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    schema.module_name,
    schema_type.class_name.to_upper_camel_case()
  ))?;
  let child_choice_enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}ChildChoice",
    schema.module_name,
    schema_type.class_name.to_upper_camel_case()
  ))?;

  let (_, _, last_name_prefix, last_name_suffix) = split_type_name(&schema_type.name)?;

  let mut attr_stmts: Vec<TokenStream> = vec![];
  let mut children_writer = quote! {};
  let end_tag_writer: TokenStream;
  let end_writer: TokenStream;

  let mut attributes: Vec<&SchemaTypeAttribute> = vec![];

  if is_leaf_text_wrapper(schema_type) {
    children_writer = quote! {
      if let Some(xml_content) = &self.0 {
        crate::common::write_escaped_text(writer, xml_content)?;
      }
    };

    end_tag_writer = quote! {
      writer.write_char('>')?;
    };

    end_writer = quote! {
      crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
    };
  } else if is_leaf_text_type(schema_type) {
    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

    children_writer = quote! {
      if let Some(xml_content) = &self.xml_content {
        crate::common::write_escaped_text(writer, xml_content)?;
      }
    };

    end_tag_writer = quote! {
      writer.write_char('>')?;
    };

    end_writer = quote! {
      crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
    };
  } else if is_leaf_element_type(schema_type) {
    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

    end_tag_writer = quote! {};
    end_writer = quote! {
      writer.write_str("/>")?;
    };
  } else if is_composite_type(schema_type) {
    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

    if schema_type.children.is_empty() {
      end_tag_writer = quote! {};
      end_writer = quote! {
        writer.write_str("/>")?;
      };
    } else if is_one_sequence_flatten(schema_type) {
      let mut field_name_set = std::collections::HashSet::new();

      let mut child_stmt_list: Vec<Stmt> = vec![];

      let flat_particles = flatten_one_sequence_particles(schema_type);
      let choice_slot_count = flat_particles
        .iter()
        .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
        .count();

      for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
        match flat_particle.kind {
          FlatParticleKind::Leaf(particle) => {
            let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;

            let child_name_ident = one_sequence_child_field_ident(&child)?;
            if !field_name_set.insert(child_name_ident.to_string()) {
              continue;
            }

            let child_stmt =
              gen_one_sequence_child_stmt(&child, flat_particle.optional, flat_particle.repeated)?;
            child_stmt_list.push(parse2(versioned_tokens(
              effective_version(child.version, flat_particle.initial_version),
              quote! { #child_stmt },
            ))?);
          }
          FlatParticleKind::Choice(choice_particle) => {
            let choice = context.resolve_one_sequence_choice(
              schema_type,
              choice_particle,
              choice_slot_count,
              slot_index,
            )?;
            let choice_name_ident: Ident = parse_str(&choice.field_name)?;
            let choice_version = flat_choice_version(&choice, flat_particle.initial_version);

            if !field_name_set.insert(choice_name_ident.to_string()) {
              continue;
            }

            let choice_enum_type: Type = parse_str(&format!(
              "crate::schemas::{}::{}",
              &schema.module_name, choice.enum_name
            ))?;

            let choice_stmt = gen_one_sequence_choice_stmt(
              &choice_name_ident,
              &choice_enum_type,
              &choice.variants,
              flat_particle.optional,
              flat_particle.repeated,
              schema,
              context,
            )?;
            child_stmt_list.push(parse2(versioned_tokens(
              choice_version,
              quote! { #choice_stmt },
            ))?);
          }
        }
      }

      children_writer = quote! {
        #( #child_stmt_list )*
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    } else if is_one_sequence_structurable(schema_type) {
      let mut field_name_set = std::collections::HashSet::new();
      let mut child_stmt_list: Vec<Stmt> = vec![];

      let structured_particles = structure_one_sequence_particles(schema_type);
      let choice_slot_count = structured_particles
        .iter()
        .filter(|particle| matches!(particle.kind, StructuredParticleKind::Choice(_)))
        .count();

      for (slot_index, particle) in structured_particles.into_iter().enumerate() {
        match particle.kind {
          StructuredParticleKind::Leaf(leaf) => {
            let child = context.resolve_one_sequence_child(schema_type, leaf.name.as_str())?;
            let child_name_ident = one_sequence_child_field_ident(&child)?;
            if !field_name_set.insert(child_name_ident.to_string()) {
              continue;
            }

            let child_stmt =
              gen_one_sequence_child_stmt(&child, particle.optional, particle.repeated)?;
            child_stmt_list.push(parse2(versioned_tokens(
              effective_version(child.version, particle.initial_version),
              quote! { #child_stmt },
            ))?);
          }
          StructuredParticleKind::Choice(choice) => {
            let choice = context.resolve_one_sequence_structured_choice(
              schema_type,
              &choice,
              choice_slot_count,
              slot_index,
            )?;
            let choice_name_ident: Ident = parse_str(&choice.field_name)?;
            let choice_version =
              structured_choice_version(&choice.variants, particle.initial_version);
            if !field_name_set.insert(choice_name_ident.to_string()) {
              continue;
            }

            let choice_enum_type: Type = parse_str(&format!(
              "crate::schemas::{}::{}",
              &schema.module_name, choice.enum_name
            ))?;

            let choice_stmt = gen_structured_one_sequence_choice_stmt(
              &choice_name_ident,
              &choice_enum_type,
              &choice.variants,
              particle.optional,
              particle.repeated,
              schema,
              context,
            )?;
            child_stmt_list.push(parse2(versioned_tokens(
              choice_version,
              quote! { #choice_stmt },
            ))?);
          }
        }
      }

      children_writer = quote! {
        #( #child_stmt_list )*
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    } else {
      let mut child_arms: Vec<Arm> = vec![];
      let resolved_children = context.resolve_children(schema_type)?;

      for child in &resolved_children {
        child_arms.push(gen_child_arm(child, &child_choice_enum_type)?);
      }
      let fallback_arm = child_match_fallback(&resolved_children);
      children_writer = quote! {
        for child in &self.children {
          match child {
            #( #child_arms )*
            #fallback_arm
          };
        }
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    }
  } else if is_derived_type(schema_type) {
    let base_class_type = context
      .derived_base_type(schema_type)
      .ok_or_else(|| format!("{:?}", schema_type.name))?;
    let resolved_children = context.resolve_children(schema_type)?;

    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

    for attr in &base_class_type.attributes {
      attributes.push(attr);
    }

    if is_one_sequence_flatten(schema_type) && is_one_sequence_flatten(base_class_type) {
      let mut field_name_set = std::collections::HashSet::new();

      let mut child_stmt_list: Vec<Stmt> = vec![];

      let flat_particles = flatten_one_sequence_particles(schema_type);
      let choice_slot_count = flat_particles
        .iter()
        .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
        .count();

      for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
        match flat_particle.kind {
          FlatParticleKind::Leaf(particle) => {
            let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;

            let child_name_ident = one_sequence_child_field_ident(&child)?;
            if !field_name_set.insert(child_name_ident.to_string()) {
              continue;
            }

            let child_stmt =
              gen_one_sequence_child_stmt(&child, flat_particle.optional, flat_particle.repeated)?;
            child_stmt_list.push(parse2(versioned_tokens(
              effective_version(child.version, flat_particle.initial_version),
              quote! { #child_stmt },
            ))?);
          }
          FlatParticleKind::Choice(choice_particle) => {
            let choice = context.resolve_one_sequence_choice(
              schema_type,
              choice_particle,
              choice_slot_count,
              slot_index,
            )?;
            let choice_name_ident: Ident = parse_str(&choice.field_name)?;
            let choice_version = flat_choice_version(&choice, flat_particle.initial_version);

            if !field_name_set.insert(choice_name_ident.to_string()) {
              continue;
            }

            let choice_enum_type: Type = parse_str(&format!(
              "crate::schemas::{}::{}",
              &schema.module_name, choice.enum_name
            ))?;

            let choice_stmt = gen_one_sequence_choice_stmt(
              &choice_name_ident,
              &choice_enum_type,
              &choice.variants,
              flat_particle.optional,
              flat_particle.repeated,
              schema,
              context,
            )?;
            child_stmt_list.push(parse2(versioned_tokens(
              choice_version,
              quote! { #choice_stmt },
            ))?);
          }
        }
      }

      children_writer = quote! {
        #( #child_stmt_list )*
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    } else if !resolved_children.is_empty() {
      let mut child_arms: Vec<Arm> = vec![];

      for child in &resolved_children {
        child_arms.push(gen_child_arm(child, &child_choice_enum_type)?);
      }
      let fallback_arm = child_match_fallback(&resolved_children);
      children_writer = quote! {
        for child in &self.children {
          match child {
            #( #child_arms )*
            #fallback_arm
          };
        }
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    } else if is_leaf_text_type(base_class_type) {
      children_writer = quote! {
        if let Some(xml_content) = &self.xml_content {
          crate::common::write_escaped_text(writer, xml_content)?;
        }
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        crate::common::write_end_tag(writer, xmlns_prefix, #last_name_prefix, #last_name_suffix)?;
      };
    } else {
      end_tag_writer = quote! {};
      end_writer = quote! {
        writer.write_str("/>")?;
      };
    }
  } else {
    return Err(format!("{schema_type:?}").into());
  }

  for attr in attributes {
    attr_stmts.push(versioned_tokens(&attr.version, gen_attr_stmt(attr)?));
  }

  let mut xmlns_attr_writer_list: Vec<Stmt> = vec![];
  let mut xml_header_writer: Option<Stmt> = None;

  if needs_xml_header(schema_type) {
    xml_header_writer = Some(parse2(quote! {
      writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
    })?);
  }

  if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
    xmlns_attr_writer_list.push(parse2(quote! {
      if let Some(xmlns) = &self.xmlns {
        crate::common::write_xmlns_attr(writer, None, xmlns)?;
      }
    })?);

    xmlns_attr_writer_list.push(parse2(quote! {
      {
        let mut xmlns_entries: Vec<_> = self.xmlns_map.iter().collect();
        xmlns_entries.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
        for (k, v) in xmlns_entries {
          crate::common::write_xmlns_attr(writer, Some(k), v)?;
        }
      }
    })?);

    xmlns_attr_writer_list.push(parse2(quote! {
      if let Some(mc_ignorable) = &self.mc_ignorable {
        crate::common::write_attr_value(writer, "mc:Ignorable", mc_ignorable)?;
      }
    })?);
  }

  let xmlns_uri = schema.target_namespace.as_str();
  let xmlns_prefix = schema.prefix.as_str();

  let to_xml_fn: ItemFn = if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules)
  {
    parse2(quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);

        self.write_xml(
          &mut writer,
          if let Some(xmlns) = &self.xmlns {
            if xmlns == #xmlns_uri {
              #xmlns_prefix
            } else {
              ""
            }
          } else {
            ""
          },
        )?;

        Ok(writer)
      }
    })?
  } else {
    parse2(quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, "")?;
        Ok(writer)
      }
    })?
  };

  let type_attrs = version_cfg_attrs(&schema_type.version);
  let impl_block = parse2(quote! {
    #( #type_attrs )*
    impl #struct_type {
      #to_xml_fn

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
        #xml_header_writer

        crate::common::write_start_tag_open(
          writer,
          xmlns_prefix,
          #last_name_prefix,
          #last_name_suffix,
        )?;

        #( #xmlns_attr_writer_list )*
        #( #attr_stmts )*

        #end_tag_writer
        #children_writer
        #end_writer

        Ok(())
      }
    }
  })?;

  let display_impl = parse2(quote! {
    #( #type_attrs )*
    impl std::fmt::Display for #struct_type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_xml()?)
      }
    }
  })?;

  Ok((impl_block, display_impl))
}

fn gen_attr_stmt(attr: &SchemaTypeAttribute) -> Result<TokenStream> {
  let attr_name_str = if attr.q_name.starts_with(':') {
    &attr.q_name[1..]
  } else {
    &attr.q_name
  };
  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case()))?
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case()))?
  };
  let required = attr
    .validators
    .iter()
    .any(|validator| validator.name == "RequiredValidator");

  Ok(if required {
    quote! {
      crate::common::write_attr_value(writer, #attr_name_str, &self.#attr_name_ident)?;
    }
  } else {
    quote! {
      if let Some(#attr_name_ident) = &self.#attr_name_ident {
        crate::common::write_attr_value(writer, #attr_name_str, #attr_name_ident)?;
      }
    }
  })
}

fn gen_one_sequence_child_stmt(
  child: &ResolvedOneSequenceChild<'_>,
  optional: bool,
  repeated: bool,
) -> Result<Stmt> {
  let child_name_ident = one_sequence_child_field_ident(child)?;

  if repeated {
    Ok(parse2(quote! {
      for child in &self.#child_name_ident {
        child.write_xml(writer, xmlns_prefix)?;
      }
    })?)
  } else if optional {
    Ok(parse2(quote! {
      if let Some(#child_name_ident) = &self.#child_name_ident {
        #child_name_ident.write_xml(writer, xmlns_prefix)?;
      }
    })?)
  } else {
    Ok(parse2(quote! {
      self.#child_name_ident.write_xml(writer, xmlns_prefix)?;
    })?)
  }
}

fn gen_one_sequence_choice_stmt(
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  variants: &[ResolvedOneSequenceChild<'_>],
  optional: bool,
  repeated: bool,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Stmt> {
  let mut arms: Vec<Arm> = vec![];

  for variant in variants {
    let variant_ident: Ident = parse_str(&variant.field_name.to_upper_camel_case())?;
    let variant_type = {
      let child_type = context
        .type_by_name(variant.name)
        .ok_or_else(|| format!("{:?}", variant.name))?;
      let child_module_name = context
        .type_module(variant.name)
        .unwrap_or(&schema.module_name);

      parse_str::<Type>(&format!(
        "crate::schemas::{}::{}",
        child_module_name,
        child_type.class_name.to_upper_camel_case()
      ))?
    };

    arms.push(parse2(versioned_tokens(
      variant.version,
      quote! {
        #choice_enum_type::#variant_ident(child) => {
          let child: &#variant_type = child.as_ref();
          child.write_xml(writer, xmlns_prefix)?;
        }
      },
    ))?);
  }

  if repeated {
    Ok(parse2(quote! {
      for choice in &self.#choice_name_ident {
        match choice {
          #( #arms )*
        }
      }
    })?)
  } else if optional {
    Ok(parse2(quote! {
      if let Some(choice) = &self.#choice_name_ident {
        match choice {
          #( #arms )*
        }
      }
    })?)
  } else {
    Ok(parse2(quote! {
      match &self.#choice_name_ident {
        #( #arms )*
      }
    })?)
  }
}

fn gen_structured_one_sequence_choice_stmt(
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  variants: &[ResolvedOneSequenceChoiceVariant<'_>],
  optional: bool,
  repeated: bool,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Stmt> {
  let mut arms: Vec<Arm> = vec![];

  for variant in variants {
    match variant {
      ResolvedOneSequenceChoiceVariant::Leaf(child) => {
        let variant_ident: Ident = parse_str(&child.field_name.to_upper_camel_case())?;
        let variant_type = {
          let child_type = context
            .type_by_name(child.name)
            .ok_or_else(|| format!("{:?}", child.name))?;
          let child_module_name = context
            .type_module(child.name)
            .unwrap_or(&schema.module_name);

          parse_str::<Type>(&format!(
            "crate::schemas::{}::{}",
            child_module_name,
            child_type.class_name.to_upper_camel_case()
          ))?
        };

        arms.push(parse2(versioned_tokens(
          child.version,
          quote! {
            #choice_enum_type::#variant_ident(child) => {
              let child: &#variant_type = child.as_ref();
              child.write_xml(writer, xmlns_prefix)?;
            }
          },
        ))?);
      }
      ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
        let variant_ident: Ident = parse_str(&sequence_variant.variant_name)?;
        let sequence_stmt = gen_sequence_variant_write_stmt(sequence_variant)?;
        let sequence_version = structured_sequence_version(sequence_variant);

        arms.push(parse2(versioned_tokens(
          sequence_version,
          quote! {
            #choice_enum_type::#variant_ident(sequence) => {
              #sequence_stmt
            }
          },
        ))?);
      }
    }
  }

  if repeated {
    Ok(parse2(quote! {
      for choice in &self.#choice_name_ident {
        match choice {
          #( #arms )*
        }
      }
    })?)
  } else if optional {
    Ok(parse2(quote! {
      if let Some(choice) = &self.#choice_name_ident {
        match choice {
          #( #arms )*
        }
      }
    })?)
  } else {
    Ok(parse2(quote! {
      match &self.#choice_name_ident {
        #( #arms )*
      }
    })?)
  }
}

fn gen_sequence_variant_write_stmt(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
) -> Result<Stmt> {
  let mut field_stmts: Vec<Stmt> = vec![];

  for field in &sequence_variant.fields {
    let child_name_ident = one_sequence_child_field_ident(&field.child)?;

    if field.optional {
      field_stmts.push(parse2(quote! {
        if let Some(child) = &sequence.#child_name_ident {
          child.write_xml(writer, xmlns_prefix)?;
        }
      })?);
    } else {
      field_stmts.push(parse2(quote! {
        sequence.#child_name_ident.write_xml(writer, xmlns_prefix)?;
      })?);
    }
  }

  Ok(parse2(quote! {{
    #( #field_stmts )*
  }})?)
}

fn one_sequence_child_field_ident(child: &ResolvedOneSequenceChild<'_>) -> Result<Ident> {
  Ok(parse_str(child.field_name.as_ref())?)
}

fn gen_child_arm(child: &ResolvedCompositeChild<'_>, child_choice_enum_type: &Type) -> Result<Arm> {
  let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

  if child.is_any {
    return Ok(parse2(versioned_tokens(
      child.version,
      quote! {
      #child_choice_enum_type::#child_variant_name_ident(child) => writer.write_str(child.as_ref())?,
      },
    ))?);
  }

  Ok(parse2(versioned_tokens(
    child.version,
    quote! {
    #child_choice_enum_type::#child_variant_name_ident(child) => child.write_xml(writer, xmlns_prefix)?,
    },
  ))?)
}

fn child_match_fallback(children: &[ResolvedCompositeChild<'_>]) -> TokenStream {
  if !children.is_empty()
    && children
      .iter()
      .all(|child| is_microsoft365_version(child.version))
  {
    quote! {
      #[cfg(not(feature = "microsoft365"))]
      _ => {}
    }
  } else {
    quote! {}
  }
}

fn split_type_name(name: &str) -> Result<(&str, &str, &str, &str)> {
  let last_name = &name[name.find('/').ok_or_else(|| name.to_string())? + 1..];
  let colon_index = last_name.find(':').ok_or_else(|| name.to_string())?;
  Ok((
    &name[..name.find('/').ok_or_else(|| name.to_string())?],
    last_name,
    &last_name[..colon_index],
    &last_name[colon_index + 1..],
  ))
}
