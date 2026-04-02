use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Arm, Expr, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::helpers::{
  AttrTypeKind, FlatParticleKind, SimpleValueKind, StructuredParticleKind, classify_attr_type,
  classify_simple_type, flatten_one_sequence_particles, is_composite_type, is_derived_type,
  is_leaf_element_type, is_leaf_text_type, is_leaf_text_wrapper, is_one_sequence_flatten,
  is_one_sequence_structurable, structure_one_sequence_particles, supports_compat_xmlns_fields,
};
use crate::sdk_code::schemas::{
  CodegenContext, ResolvedCompositeChild, ResolvedOneSequenceChild,
  ResolvedOneSequenceChoiceVariant, ResolvedOneSequenceSequenceVariant,
};
use crate::sdk_code::versioning::{
  effective_version, is_microsoft365_version, version_cfg_attrs, versioned_tokens,
};
use crate::sdk_data::compatibility::{
  map_attribute_value_rule_for_field, strict_bitmask_rule_for_field, treat_as_string_rule_for_field,
};
use crate::sdk_data::sdk_data_model::{
  CompatibilityAction, CompatibilityRule, Schema, SchemaTypeAttribute,
};
use crate::simple_type::simple_type_mapping;
use crate::utils::{escape_snake_case, escape_upper_camel_case};

struct MatchBranch {
  version: String,
  key: LitByteStr,
  body: Stmt,
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

fn composite_children_all_microsoft365(children: &[ResolvedCompositeChild<'_>]) -> bool {
  !children.is_empty()
    && children
      .iter()
      .all(|child| is_microsoft365_version(child.version))
}

pub fn gen_schema_deserializer(
  schema: &Schema,
  compatibility_rules: &[CompatibilityRule],
  context: &CodegenContext<'_>,
) -> Result<TokenStream> {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  for schema_enum in &schema.enums {
    let enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name,
      schema_enum.name.to_upper_camel_case()
    ))?;

    let mut variants: Vec<Arm> = vec![];
    let mut byte_variants: Vec<Arm> = vec![];

    for facet in &schema_enum.facets {
      let variant_value = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
      };

      let variant_value_literal: LitByteStr = parse_str(&format!("b\"{variant_value}\""))?;

      variants.push(parse2(versioned_tokens(
        &facet.version,
        quote! {
          #variant_value => Ok(Self::#variant_ident),
        },
      ))?);

      byte_variants.push(parse2(versioned_tokens(
        &facet.version,
        quote! {
          #variant_value_literal => Ok(Self::#variant_ident),
        },
      ))?);
    }

    let enum_attrs = version_cfg_attrs(&schema_enum.version);
    token_stream_list.push(parse2(quote! {
      #( #enum_attrs )*
      impl std::str::FromStr for #enum_type {
        type Err = crate::common::SdkError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
          match s {
            #( #variants )*
            _ => Err(crate::common::invalid_enum_value(
              stringify!(#enum_type),
              s,
            )),
          }
        }
      }
    })?);

    token_stream_list.push(parse2(quote! {
      #( #enum_attrs )*
      impl #enum_type {
        pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
          match b {
            #( #byte_variants )*
            other => Err(crate::common::invalid_enum_value(
              stringify!(#enum_type),
              String::from_utf8_lossy(other).into_owned(),
            )),
          }
        }
      }
    })?);
  }

  let from_reader_fn = gen_from_reader_fn()?;

  for schema_type in &schema.types {
    if schema_type.is_abstract {
      continue;
    }

    let class_name_str = schema_type.class_name.to_upper_camel_case();
    let class_name_literal = class_name_str.as_str();

    let struct_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name, &class_name_str
    ))?;

    let from_str_impl = gen_from_str_impl(&struct_type)?;
    let type_attrs = version_cfg_attrs(&schema_type.version);
    token_stream_list.push(parse2(quote! {
      #( #type_attrs )*
      #from_str_impl
    })?);

    let first_name = &schema_type.name[0..schema_type.name.find('/').unwrap()];
    let prefix_type_name_str = &schema_type.name[schema_type.name.find('/').unwrap() + 1..];
    let type_name_str = &prefix_type_name_str[prefix_type_name_str.find(':').unwrap() + 1..];

    let prefix_type_name_literal: LitByteStr = parse_str(&format!("b\"{prefix_type_name_str}\""))?;
    let type_name_literal: LitByteStr = parse_str(&format!("b\"{type_name_str}\""))?;
    let default_prefix_str = prefix_type_name_str
      .split(':')
      .next()
      .unwrap_or_default()
      .to_string();

    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut attr_match_list: Vec<MatchBranch> = vec![];
    let mut field_unwrap_list: Vec<Stmt> = vec![];
    let mut field_ident_list: Vec<TokenStream> = vec![];
    let mut helper_fn_list: Vec<ItemFn> = vec![];
    let mut loop_declaration_list: Vec<Stmt> = vec![];
    let mut loop_children_stmt_opt: Option<Stmt> = None;
    let mut loop_match_arm_list: Vec<Arm> = vec![];

    let mut loop_children_match_list: Vec<Arm> = vec![];
    let mut loop_children_visit_match_list: Vec<Arm> = vec![];
    let mut loop_children_suffix_match_set: HashSet<String> = HashSet::new();
    let mut loop_children_visit_suffix_match_set: HashSet<String> = HashSet::new();
    let mut any_child_variant_ident_opt: Option<Ident> = None;

    let mut attributes: Vec<&SchemaTypeAttribute> = vec![];
    if is_leaf_text_wrapper(schema_type) {
      field_declaration_list.push(parse2(quote! {
        let mut xml_content = None;
      })?);

      loop_match_arm_list.extend(gen_simple_child_match_arms(
        first_name,
        class_name_literal,
        treat_as_string_rule_for_field(
          compatibility_rules,
          &schema.module_name,
          &schema_type.class_name,
          "xml_content",
        )
        .is_some()
          || treat_as_string_rule_for_field(
            compatibility_rules,
            &schema.module_name,
            &schema_type.name,
            "xml_content",
          )
          .is_some(),
        context,
      )?);
    } else if is_leaf_text_type(schema_type) {
      if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
        field_declaration_list.push(parse2(quote! {
          let mut xmlns = None;
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut mc_ignorable = None;
        })?);

        field_ident_list.push(quote! { xmlns });
        field_ident_list.push(quote! { xmlns_map });
        field_ident_list.push(quote! { mc_ignorable });
      }

      for attr in &schema_type.attributes {
        attributes.push(attr);
      }

      field_declaration_list.push(parse2(quote! {
        let mut xml_content = None;
      })?);

      field_ident_list.push(quote! { xml_content });

      loop_match_arm_list.extend(gen_simple_child_match_arms(
        first_name,
        class_name_literal,
        treat_as_string_rule_for_field(
          compatibility_rules,
          &schema.module_name,
          &schema_type.class_name,
          "xml_content",
        )
        .is_some()
          || treat_as_string_rule_for_field(
            compatibility_rules,
            &schema.module_name,
            &schema_type.name,
            "xml_content",
          )
          .is_some(),
        context,
      )?);
    } else if is_leaf_element_type(schema_type) {
      if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
        field_declaration_list.push(parse2(quote! {
          let mut xmlns = None;
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut mc_ignorable = None;
        })?);

        field_ident_list.push(quote! { xmlns });
        field_ident_list.push(quote! { xmlns_map });
        field_ident_list.push(quote! { mc_ignorable });
      }

      for attr in &schema_type.attributes {
        attributes.push(attr);
      }
    } else if is_composite_type(schema_type) {
      if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
        field_declaration_list.push(parse2(quote! {
          let mut xmlns = None;
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut mc_ignorable = None;
        })?);

        field_ident_list.push(quote! { xmlns });
        field_ident_list.push(quote! { xmlns_map });
        field_ident_list.push(quote! { mc_ignorable });
      }

      for attr in &schema_type.attributes {
        attributes.push(attr);
      }

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        &schema.module_name,
        schema_type.class_name.to_upper_camel_case()
      ))?;

      if is_one_sequence_flatten(schema_type) {
        let mut field_name_set = HashSet::new();

        let flat_particles = flatten_one_sequence_particles(schema_type);
        let choice_slot_count = flat_particles
          .iter()
          .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
          .count();

        for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
          match flat_particle.kind {
            FlatParticleKind::Leaf(particle) => {
              let child =
                context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
              let child_name_ident: Ident = parse_str(&child.field_name)?;

              if !field_name_set.insert(child_name_ident.to_string()) {
                continue;
              }

              if flat_particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, flat_particle.initial_version),
                  quote! {
                  let mut #child_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, flat_particle.initial_version),
                  quote! {
                  let mut #child_name_ident = None;
                  },
                ))?);

                if !flat_particle.optional {
                  let child_name_str = child.field_name.as_ref();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    effective_version(child.version, flat_particle.initial_version),
                    quote! {
                    let #child_name_ident = #child_name_ident
                      .ok_or_else(|| crate::common::missing_field(#class_name_literal, #child_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                effective_version(child.version, flat_particle.initial_version),
                quote! { #child_name_ident },
              ));

              if let Some(arm) = gen_one_sequence_match_arm(
                &child,
                effective_version(child.version, flat_particle.initial_version),
                schema,
                context,
                flat_particle.repeated,
                &mut loop_children_suffix_match_set,
              )? {
                loop_children_match_list.push(arm);
              }
              if let Some(arm) = gen_one_sequence_visitor_arm(
                &child,
                effective_version(child.version, flat_particle.initial_version),
                schema,
                context,
                flat_particle.repeated,
                &mut loop_children_visit_suffix_match_set,
              )? {
                loop_children_visit_match_list.push(arm);
              }
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

              if flat_particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = None;
                  },
                ))?);

                if !flat_particle.optional {
                  let choice_name_str = choice.field_name.as_str();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    choice_version,
                    quote! {
                      let #choice_name_ident = #choice_name_ident
                        .ok_or_else(|| crate::common::missing_field(#class_name_literal, #choice_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                choice_version,
                quote! { #choice_name_ident },
              ));

              let choice_enum_type: Type = parse_str(&format!(
                "crate::schemas::{}::{}",
                &schema.module_name, choice.enum_name
              ))?;

              for variant in &choice.variants {
                if let Some(arm) = gen_one_sequence_choice_match_arm(
                  variant,
                  &choice_name_ident,
                  &choice_enum_type,
                  schema,
                  context,
                  flat_particle.repeated,
                  &mut loop_children_suffix_match_set,
                )? {
                  loop_children_match_list.push(arm);
                }
                if let Some(arm) = gen_one_sequence_choice_visitor_arm(
                  variant,
                  &choice_name_ident,
                  &choice_enum_type,
                  schema,
                  context,
                  flat_particle.repeated,
                  &mut loop_children_visit_suffix_match_set,
                )? {
                  loop_children_visit_match_list.push(arm);
                }
              }
            }
          }
        }
      } else if is_one_sequence_structurable(schema_type) {
        let mut field_name_set = HashSet::new();

        let structured_particles = structure_one_sequence_particles(schema_type);
        let choice_slot_count = structured_particles
          .iter()
          .filter(|particle| matches!(particle.kind, StructuredParticleKind::Choice(_)))
          .count();

        for (slot_index, particle) in structured_particles.into_iter().enumerate() {
          match particle.kind {
            StructuredParticleKind::Leaf(leaf) => {
              let child = context.resolve_one_sequence_child(schema_type, leaf.name.as_str())?;
              let child_name_ident: Ident = parse_str(&child.field_name)?;

              if !field_name_set.insert(child_name_ident.to_string()) {
                continue;
              }

              if particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, particle.initial_version),
                  quote! {
                    let mut #child_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, particle.initial_version),
                  quote! {
                    let mut #child_name_ident = None;
                  },
                ))?);

                if !particle.optional {
                  let child_name_str = child.field_name.as_ref();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    effective_version(child.version, particle.initial_version),
                    quote! {
                      let #child_name_ident = #child_name_ident
                        .ok_or_else(|| crate::common::missing_field(#class_name_literal, #child_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                effective_version(child.version, particle.initial_version),
                quote! { #child_name_ident },
              ));

              if let Some(arm) = gen_one_sequence_match_arm(
                &child,
                effective_version(child.version, particle.initial_version),
                schema,
                context,
                particle.repeated,
                &mut loop_children_suffix_match_set,
              )? {
                loop_children_match_list.push(arm);
              }
              if let Some(arm) = gen_one_sequence_visitor_arm(
                &child,
                effective_version(child.version, particle.initial_version),
                schema,
                context,
                particle.repeated,
                &mut loop_children_visit_suffix_match_set,
              )? {
                loop_children_visit_match_list.push(arm);
              }
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

              if particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = None;
                  },
                ))?);

                if !particle.optional {
                  let choice_name_str = choice.field_name.as_str();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    choice_version,
                    quote! {
                      let #choice_name_ident = #choice_name_ident
                        .ok_or_else(|| crate::common::missing_field(#class_name_literal, #choice_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                choice_version,
                quote! { #choice_name_ident },
              ));

              let choice_enum_type: Type = parse_str(&format!(
                "crate::schemas::{}::{}",
                &schema.module_name, choice.enum_name
              ))?;

              for variant in &choice.variants {
                match variant {
                  ResolvedOneSequenceChoiceVariant::Leaf(child) => {
                    if let Some(arm) = gen_one_sequence_choice_match_arm(
                      child,
                      &choice_name_ident,
                      &choice_enum_type,
                      schema,
                      context,
                      particle.repeated,
                      &mut loop_children_suffix_match_set,
                    )? {
                      loop_children_match_list.push(arm);
                    }
                    if let Some(arm) = gen_one_sequence_choice_visitor_arm(
                      child,
                      &choice_name_ident,
                      &choice_enum_type,
                      schema,
                      context,
                      particle.repeated,
                      &mut loop_children_visit_suffix_match_set,
                    )? {
                      loop_children_visit_match_list.push(arm);
                    }
                  }
                  ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
                    helper_fn_list.push(gen_structured_sequence_variant_deserialize_fn(
                      sequence_variant,
                      schema,
                      context,
                    )?);

                    loop_children_match_list.extend(gen_structured_sequence_variant_match_arms(
                      sequence_variant,
                      &choice_name_ident,
                      &choice_enum_type,
                      particle.repeated,
                      &mut loop_children_suffix_match_set,
                    )?);
                    loop_children_visit_match_list.extend(
                      gen_structured_sequence_variant_visitor_arms(
                        sequence_variant,
                        &choice_name_ident,
                        &choice_enum_type,
                        particle.repeated,
                        &mut loop_children_visit_suffix_match_set,
                      )?,
                    );
                  }
                }
              }
            }
          }
        }
      } else {
        let resolved_children = context.resolve_children(schema_type)?;

        if !resolved_children.is_empty() {
          if composite_children_all_microsoft365(&resolved_children) {
            field_declaration_list.push(parse2(quote! {
              #[cfg(feature = "microsoft365")]
              let mut children = vec![];
            })?);
            field_declaration_list.push(parse2(quote! {
              #[cfg(not(feature = "microsoft365"))]
              let children = vec![];
            })?);
          } else {
            field_declaration_list.push(parse2(quote! {
              let mut children = vec![];
            })?);
          }

          field_ident_list.push(quote! { children });
        }

        for child in &resolved_children {
          if child.is_any {
            any_child_variant_ident_opt =
              Some(parse_str(&child.variant_name.to_upper_camel_case())?);
            continue;
          }

          if let Some(arm) = gen_child_match_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_suffix_match_set,
          )? {
            loop_children_match_list.push(arm);
          }
          if let Some(arm) = gen_child_visitor_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_visit_suffix_match_set,
          )? {
            loop_children_visit_match_list.push(arm);
          }
        }
      }
    } else if is_derived_type(schema_type) {
      let base_class_type = context
        .derived_base_type(schema_type)
        .ok_or_else(|| format!("{:?}", schema_type.name))?;
      let resolved_children = context.resolve_children(schema_type)?;

      if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
        field_declaration_list.push(parse2(quote! {
          let mut xmlns = None;
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
        })?);

        field_declaration_list.push(parse2(quote! {
          let mut mc_ignorable = None;
        })?);

        field_ident_list.push(quote! { xmlns });
        field_ident_list.push(quote! { xmlns_map });
        field_ident_list.push(quote! { mc_ignorable });
      }

      for attr in &schema_type.attributes {
        attributes.push(attr);
      }

      for attr in &base_class_type.attributes {
        attributes.push(attr);
      }

      if is_one_sequence_flatten(schema_type) && is_one_sequence_flatten(base_class_type) {
        let mut field_name_set = HashSet::new();

        let flat_particles = flatten_one_sequence_particles(schema_type);
        let choice_slot_count = flat_particles
          .iter()
          .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
          .count();

        for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
          match flat_particle.kind {
            FlatParticleKind::Leaf(particle) => {
              let child =
                context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
              let child_name_ident: Ident = parse_str(&child.field_name)?;

              if !field_name_set.insert(child_name_ident.to_string()) {
                continue;
              }

              if flat_particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, flat_particle.initial_version),
                  quote! {
                  let mut #child_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  effective_version(child.version, flat_particle.initial_version),
                  quote! {
                  let mut #child_name_ident = None;
                  },
                ))?);

                if !flat_particle.optional {
                  let child_name_str = child.field_name.as_ref();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    effective_version(child.version, flat_particle.initial_version),
                    quote! {
                    let #child_name_ident = #child_name_ident
                      .ok_or_else(|| crate::common::missing_field(#class_name_literal, #child_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                effective_version(child.version, flat_particle.initial_version),
                quote! { #child_name_ident },
              ));
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

              if flat_particle.repeated {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = vec![];
                  },
                ))?);
              } else {
                field_declaration_list.push(parse2(versioned_tokens(
                  choice_version,
                  quote! {
                    let mut #choice_name_ident = None;
                  },
                ))?);

                if !flat_particle.optional {
                  let choice_name_str = choice.field_name.as_str();
                  field_unwrap_list.push(parse2(versioned_tokens(
                    choice_version,
                    quote! {
                      let #choice_name_ident = #choice_name_ident
                        .ok_or_else(|| crate::common::missing_field(#class_name_literal, #choice_name_str))?;
                    },
                  ))?);
                }
              }

              field_ident_list.push(versioned_tokens(
                choice_version,
                quote! { #choice_name_ident },
              ));
            }
          }
        }
      } else if !resolved_children.is_empty() {
        if composite_children_all_microsoft365(&resolved_children) {
          field_declaration_list.push(parse2(quote! {
            #[cfg(feature = "microsoft365")]
            let mut children = vec![];
          })?);
          field_declaration_list.push(parse2(quote! {
            #[cfg(not(feature = "microsoft365"))]
            let children = vec![];
          })?);
        } else {
          field_declaration_list.push(parse2(quote! {
            let mut children = vec![];
          })?);
        }

        field_ident_list.push(quote! { children });
      } else if is_leaf_text_type(base_class_type) {
        field_declaration_list.push(parse2(quote! {
          let mut xml_content = None;
        })?);

        field_ident_list.push(quote! { xml_content });
      }

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        &schema.module_name,
        schema_type.class_name.to_upper_camel_case()
      ))?;

      if is_one_sequence_flatten(schema_type) && is_one_sequence_flatten(base_class_type) {
        let flat_particles = flatten_one_sequence_particles(schema_type);
        let choice_slot_count = flat_particles
          .iter()
          .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
          .count();

        for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
          match flat_particle.kind {
            FlatParticleKind::Leaf(particle) => {
              let child =
                context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;

              if let Some(arm) = gen_one_sequence_match_arm(
                &child,
                effective_version(child.version, flat_particle.initial_version),
                schema,
                context,
                flat_particle.repeated,
                &mut loop_children_suffix_match_set,
              )? {
                loop_children_match_list.push(arm);
              }
              if let Some(arm) = gen_one_sequence_visitor_arm(
                &child,
                effective_version(child.version, flat_particle.initial_version),
                schema,
                context,
                flat_particle.repeated,
                &mut loop_children_visit_suffix_match_set,
              )? {
                loop_children_visit_match_list.push(arm);
              }
            }
            FlatParticleKind::Choice(choice_particle) => {
              let choice = context.resolve_one_sequence_choice(
                schema_type,
                choice_particle,
                choice_slot_count,
                slot_index,
              )?;
              let choice_name_ident: Ident = parse_str(&choice.field_name)?;
              let choice_enum_type: Type = parse_str(&format!(
                "crate::schemas::{}::{}",
                &schema.module_name, choice.enum_name
              ))?;

              for variant in &choice.variants {
                if let Some(arm) = gen_one_sequence_choice_match_arm(
                  variant,
                  &choice_name_ident,
                  &choice_enum_type,
                  schema,
                  context,
                  flat_particle.repeated,
                  &mut loop_children_suffix_match_set,
                )? {
                  loop_children_match_list.push(arm);
                }
                if let Some(arm) = gen_one_sequence_choice_visitor_arm(
                  variant,
                  &choice_name_ident,
                  &choice_enum_type,
                  schema,
                  context,
                  flat_particle.repeated,
                  &mut loop_children_visit_suffix_match_set,
                )? {
                  loop_children_visit_match_list.push(arm);
                }
              }
            }
          }
        }
      } else {
        for child in &resolved_children {
          if child.is_any {
            any_child_variant_ident_opt =
              Some(parse_str(&child.variant_name.to_upper_camel_case())?);
            continue;
          }

          if let Some(arm) = gen_child_match_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_suffix_match_set,
          )? {
            loop_children_match_list.push(arm);
          }
          if let Some(arm) = gen_child_visitor_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_visit_suffix_match_set,
          )? {
            loop_children_visit_match_list.push(arm);
          }
        }
      }

      if resolved_children.is_empty() && is_leaf_text_type(base_class_type) {
        let base_first_name = &base_class_type.name[0..base_class_type.name.find('/').unwrap()];

        loop_match_arm_list.extend(gen_simple_child_match_arms(
          base_first_name,
          class_name_literal,
          treat_as_string_rule_for_field(
            compatibility_rules,
            &schema.module_name,
            &schema_type.class_name,
            "xml_content",
          )
          .is_some()
            || treat_as_string_rule_for_field(
              compatibility_rules,
              &schema.module_name,
              &schema_type.name,
              "xml_content",
            )
            .is_some(),
          context,
        )?);
      }
    } else {
      return Err(format!("{schema_type:?}").into());
    }

    for attr in &attributes {
      let attr_name_str = if attr.property_name.is_empty() {
        escape_snake_case(attr.q_name.to_snake_case())
      } else {
        escape_snake_case(attr.property_name.to_snake_case())
      };

      let attr_name_ident: Ident = parse_str(&attr_name_str)?;

      field_declaration_list.push(parse2(versioned_tokens(
        &attr.version,
        quote! {
          let mut #attr_name_ident = None;
        },
      ))?);

      attr_match_list.push(gen_field_match_arm(
        attr,
        &schema.module_name,
        &schema_type.class_name,
        &schema_type.name,
        compatibility_rules,
        class_name_literal,
        context,
      )?);

      if let Some(rule) = strict_bitmask_rule_for_field(
        compatibility_rules,
        &schema.module_name,
        &schema_type.class_name,
        &attr.property_name,
      ) {
        attr_match_list.extend(gen_strict_bitmask_match_branches(
          rule,
          &attr_name_ident,
          class_name_literal,
          &attr_name_str,
        )?);
      }

      let required = attr
        .validators
        .iter()
        .any(|validator| validator.name == "RequiredValidator");

      if required {
        field_unwrap_list.push(parse2(versioned_tokens(
          &attr.version,
          quote! {
            let #attr_name_ident = #attr_name_ident
              .ok_or_else(|| crate::common::missing_field(#class_name_literal, #attr_name_str))?;
          },
        ))?);
      }

      field_ident_list.push(versioned_tokens(&attr.version, quote! { #attr_name_ident }));
    }

    let mut expect_event_start_stmt: Stmt = parse2(quote! {
      let (e, empty_tag) = crate::common::expect_event_start!(
        xml_reader,
        xml_event,
        #class_name_literal,
        #prefix_type_name_str,
        #prefix_type_name_literal,
        #type_name_literal
      );
    })?;

    let attr_match_list_arms: Vec<Arm> = attr_match_list
      .iter()
      .map(gen_attr_match_arm)
      .collect::<Result<_>>()?;

    let attr_match_stmt_opt: Option<Stmt> =
      if supports_compat_xmlns_fields(schema_type, schema, compatibility_rules) {
        Some(parse2(quote! {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;

            match attr.key.as_ref() {
              #( #attr_match_list_arms )*
              b"xmlns" => {
                xmlns = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
              }
              b"mc:Ignorable" => {
                mc_ignorable = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
              }
              key => {
                if key.starts_with(b"xmlns:") {
                  let prefix = String::from_utf8_lossy(&key[6..]).into_owned();
                  let uri = crate::common::decode_attr_value(&attr, xml_reader.decoder())?;

                  if let Some(canonical_prefix) = crate::namespaces::prefix_by_uri(uri.as_str()) {
                    xmlns_map.entry(canonical_prefix.to_string()).or_insert(uri);
                  } else {
                    xmlns_map.insert(prefix, uri);
                  }
                }
              }
            }
          }
        })?)
      } else if !attr_match_list.is_empty() {
        Some(gen_attr_stmt(&attr_match_list)?)
      } else {
        expect_event_start_stmt = parse2(quote! {
          let (_, empty_tag) = crate::common::expect_event_start!(
            xml_reader,
            xml_event,
            #class_name_literal,
            #prefix_type_name_str,
            #prefix_type_name_literal,
            #type_name_literal
          );
        })?;

        None
      };

    if !loop_children_match_list.is_empty() || any_child_variant_ident_opt.is_some() {
      let any_child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        &schema.module_name,
        schema_type.class_name.to_upper_camel_case()
      ))?;
      let has_known_child_match = !loop_children_match_list.is_empty();
      let has_known_visit_match = !loop_children_visit_match_list.is_empty();
      let any_visit_fallback_arm = if let Some(any_child_variant_ident) =
        &any_child_variant_ident_opt
      {
        quote! {
          _ => {
            children.push(#any_child_choice_enum_type::#any_child_variant_ident(std::boxed::Box::new(
              crate::common::read_outer_xml(_xml_reader, e, _e_empty)?,
            )));
            Ok(true)
          }
        }
      } else {
        quote! {
          _ => Ok(false),
        }
      };
      let any_visit_fallback_body = if let Some(any_child_variant_ident) =
        &any_child_variant_ident_opt
      {
        quote! {
          children.push(#any_child_choice_enum_type::#any_child_variant_ident(std::boxed::Box::new(
            crate::common::read_outer_xml(_xml_reader, e, _e_empty)?,
          )));
          Ok(true)
        }
      } else {
        quote! {
          Ok(false)
        }
      };
      let any_main_fallback_body = if let Some(any_child_variant_ident) =
        &any_child_variant_ident_opt
      {
        quote! {
          children.push(#any_child_choice_enum_type::#any_child_variant_ident(std::boxed::Box::new(
            crate::common::read_outer_xml(xml_reader, e, e_empty)?,
          )));
        }
      } else {
        quote! {
          if crate::common::is_foreign_prefixed_child(e.name().as_ref(), #default_prefix_str) {
            crate::common::process_foreign_element_children(
              xml_reader,
              e_empty,
              &mut visit_foreign_child,
            )?;
          } else {
            Err(crate::common::unexpected_tag(
              #class_name_literal,
              "known child",
              e.name().as_ref(),
            ))?;
          }
        }
      };
      let visit_foreign_child_body = if has_known_visit_match {
        quote! {
          match e.name().as_ref() {
            #( #loop_children_visit_match_list )*
            #any_visit_fallback_arm
          }
        }
      } else {
        quote! {
          let _ = e.name().as_ref();
          #any_visit_fallback_body
        }
      };
      let main_child_body = if has_known_child_match {
        quote! {
          match e.name().as_ref() {
            #( #loop_children_match_list )*
            b"mc:AlternateContent" | b"AlternateContent" => {
              crate::common::process_foreign_element_children(
                xml_reader,
                e_empty,
                &mut visit_foreign_child,
              )?;
            }
            _ => {
              #any_main_fallback_body
            }
          }
        }
      } else {
        quote! {
          let _ = e.name().as_ref();
          #any_main_fallback_body
        }
      };

      loop_declaration_list.push(parse2(quote! {
        let mut e_opt = None;
      })?);

      loop_declaration_list.push(parse2(quote! {
        let mut e_empty = false;
      })?);

      loop_match_arm_list.push(parse2(quote! {
        quick_xml::events::Event::Start(e) => {
          e_opt = Some(e);
        }
      })?);

      loop_match_arm_list.push(parse2(quote! {
        quick_xml::events::Event::Empty(e) => {
          e_empty = true;
          e_opt = Some(e);
        }
      })?);

      let visit_foreign_child_decl = if has_known_child_match {
        quote! {
          let mut visit_foreign_child =
            |_xml_reader: &mut R, e: quick_xml::events::BytesStart<'de>, _e_empty: bool| -> Result<bool, crate::common::SdkError> {
              #visit_foreign_child_body
            };
        }
      } else {
        quote! {}
      };

      loop_children_stmt_opt = Some(parse2(quote! {{
        #visit_foreign_child_decl

        if let Some(e) = e_opt {
          #main_child_body
        }
      }})?);
    }

    let result_expr: Expr = if is_leaf_text_wrapper(schema_type) {
      parse2(quote! {
          Ok(Self(xml_content))
      })?
    } else {
      parse2(quote! {
          Ok(Self {
              #( #field_ident_list, )*
          })
      })?
    };

    let deserialize_inner_fn: ItemFn = parse2(quote! {
      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        #expect_event_start_stmt

        #( #field_declaration_list )*

        #attr_match_stmt_opt

        if !empty_tag {
          loop {
            #( #loop_declaration_list )*

            match xml_reader.next()? {
              #( #loop_match_arm_list )*
              quick_xml::events::Event::End(e) => match e.name().as_ref() {
                #prefix_type_name_literal | #type_name_literal => {
                  break;
                }
                _ => (),
              },
              quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(
                #class_name_literal,
              ))?,
              _ => (),
            }

            #loop_children_stmt_opt
          }
        }

        #( #field_unwrap_list )*

        #result_expr
      }
    })?;

    token_stream_list.push(parse2(quote! {
      #( #type_attrs )*
      impl #struct_type {
        #from_reader_fn
        #( #helper_fn_list )*

        #deserialize_inner_fn
      }
    })?);
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn gen_from_str_impl(struct_type: &Type) -> Result<ItemImpl> {
  Ok(parse2(quote! {
    impl std::str::FromStr for #struct_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;

        Self::deserialize_inner(&mut xml_reader, None)
      }
    }
  })?)
}

fn gen_from_reader_fn() -> Result<ItemFn> {
  Ok(parse2(quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;

      Self::deserialize_inner(&mut xml_reader, None)
    }
  })?)
}

fn gen_one_sequence_match_arm(
  child: &ResolvedOneSequenceChild<'_>,
  version: &str,
  schema: &Schema,
  context: &CodegenContext<'_>,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];

  let child_name_ident: Ident = parse_str(&child.field_name)?;

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;

  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    if repeated {
      Ok(Some(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #child_name_ident.push(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            );
          }
        },
      ))?))
    } else {
      Ok(Some(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #child_name_ident = Some(std::boxed::Box::new(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            ));
          }
        },
      ))?))
    }
  } else if insert_full && !repeated {
    Ok(Some(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          #child_name_ident = Some(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          ));
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          #child_name_ident.push(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          );
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_one_sequence_choice_match_arm(
  child: &ResolvedOneSequenceChild<'_>,
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;
  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];
  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;
  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;
  let choice_variant_ident: Ident = parse_str(&child.field_name.to_upper_camel_case())?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    if repeated {
      Ok(Some(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #choice_name_ident.push(#choice_enum_type::#choice_variant_ident(std::boxed::Box::new(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            )));
          }
        },
      ))?))
    } else {
      Ok(Some(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #choice_name_ident = Some(#choice_enum_type::#choice_variant_ident(std::boxed::Box::new(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            )));
          }
        },
      ))?))
    }
  } else if insert_full && repeated {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          #choice_name_ident.push(#choice_enum_type::#choice_variant_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          #choice_name_ident = Some(#choice_enum_type::#choice_variant_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_structured_sequence_variant_deserialize_fn(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<ItemFn> {
  let fn_ident: Ident = parse_str(&format!(
    "deserialize_{}",
    sequence_variant.struct_name.to_snake_case()
  ))?;
  let variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    &schema.module_name, sequence_variant.struct_name
  ))?;
  let variant_type_name = sequence_variant.struct_name.as_str();
  let helper_version = structured_sequence_version(sequence_variant);
  let helper_attrs = version_cfg_attrs(helper_version);

  let mut field_declarations: Vec<Stmt> = vec![];
  let mut field_unwraps: Vec<Stmt> = vec![];
  let mut field_idents: Vec<TokenStream> = vec![];
  let mut field_match_arms: Vec<Arm> = vec![];

  for field in &sequence_variant.fields {
    let child = &field.child;
    let child_name_ident: Ident = parse_str(&child.field_name)?;
    let child_type = context
      .type_by_name(child.name)
      .ok_or_else(|| format!("{:?}", child.name))?;
    let child_module_name = context
      .type_module(child.name)
      .unwrap_or(&schema.module_name);
    let child_variant_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      child_module_name,
      child_type.class_name.to_upper_camel_case()
    ))?;
    let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
    let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];
    let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
    let child_suffix_last_name_literal: LitByteStr =
      parse_str(&format!("b\"{child_suffix_last_name}\""))?;

    field_declarations.push(parse2(versioned_tokens(
      effective_version(child.version, field.initial_version),
      quote! {
        let mut #child_name_ident = None;
      },
    ))?);

    if !field.optional {
      let child_name_str = child.field_name.as_ref();
      field_unwraps.push(parse2(versioned_tokens(
        effective_version(child.version, field.initial_version),
        quote! {
          let #child_name_ident = #child_name_ident
            .ok_or_else(|| crate::common::missing_field(#variant_type_name, #child_name_str))?;
        },
      ))?);
    }

    field_idents.push(versioned_tokens(
      effective_version(child.version, field.initial_version),
      quote! { #child_name_ident },
    ));

    field_match_arms.push(parse2(versioned_tokens(
      effective_version(child.version, field.initial_version),
      quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          #child_name_ident = Some(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          ));
        }
      },
    ))?);
  }

  Ok(parse2(quote! {
    #( #helper_attrs )*
    fn #fn_ident<'de, R: crate::common::XmlReader<'de>>(
      xml_reader: &mut R,
      e: quick_xml::events::BytesStart<'de>,
      e_empty: bool,
    ) -> Result<#variant_type, crate::common::SdkError> {
      #( #field_declarations )*

      let mut pending_event = Some((e, e_empty));

      loop {
        if let Some((e, e_empty)) = pending_event.take() {
          match e.name().as_ref() {
            #( #field_match_arms )*
            _ => {
              xml_reader.unread(if e_empty {
                quick_xml::events::Event::Empty(e)
              } else {
                quick_xml::events::Event::Start(e)
              })?;
              break;
            }
          }
        }

        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => {
            pending_event = Some((e, false));
          }
          quick_xml::events::Event::Empty(e) => {
            pending_event = Some((e, true));
          }
          quick_xml::events::Event::End(e) => {
            xml_reader.unread(quick_xml::events::Event::End(e))?;
            break;
          }
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#variant_type_name))?,
          _ => {}
        }
      }

      #( #field_unwraps )*

      Ok(#variant_type {
        #( #field_idents, )*
      })
    }
  })?)
}

fn gen_structured_sequence_variant_match_arms(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Vec<Arm>> {
  let fn_ident: Ident = parse_str(&format!(
    "deserialize_{}",
    sequence_variant.struct_name.to_snake_case()
  ))?;
  let variant_ident: Ident = parse_str(&sequence_variant.variant_name)?;
  let mut arms = Vec::new();

  for field in &sequence_variant.fields {
    let child = &field.child;
    let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
    let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];
    let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
    let child_suffix_last_name_literal: LitByteStr =
      parse_str(&format!("b\"{child_suffix_last_name}\""))?;

    let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
    let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

    if insert_suffix && child_last_name != child_suffix_last_name {
      let assign_stmt = if repeated {
        quote! {
          #choice_name_ident.push(#choice_enum_type::#variant_ident(
            Self::#fn_ident(xml_reader, e, e_empty)?,
          ));
        }
      } else {
        quote! {
          #choice_name_ident = Some(#choice_enum_type::#variant_ident(
            Self::#fn_ident(xml_reader, e, e_empty)?,
          ));
        }
      };
      arms.push(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #assign_stmt
          }
        },
      ))?);
    } else if insert_full {
      let assign_stmt = if repeated {
        quote! {
          #choice_name_ident.push(#choice_enum_type::#variant_ident(
            Self::#fn_ident(xml_reader, e, e_empty)?,
          ));
        }
      } else {
        quote! {
          #choice_name_ident = Some(#choice_enum_type::#variant_ident(
            Self::#fn_ident(xml_reader, e, e_empty)?,
          ));
        }
      };
      arms.push(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal => {
            #assign_stmt
          }
        },
      ))?);
    }
  }

  Ok(arms)
}

fn gen_structured_sequence_variant_visitor_arms(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Vec<Arm>> {
  let fn_ident: Ident = parse_str(&format!(
    "deserialize_{}",
    sequence_variant.struct_name.to_snake_case()
  ))?;
  let variant_ident: Ident = parse_str(&sequence_variant.variant_name)?;
  let mut arms = Vec::new();

  for field in &sequence_variant.fields {
    let child = &field.child;
    let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
    let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];
    let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
    let child_suffix_last_name_literal: LitByteStr =
      parse_str(&format!("b\"{child_suffix_last_name}\""))?;

    let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
    let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

    if insert_suffix && child_last_name != child_suffix_last_name {
      let assign_stmt = if repeated {
        quote! {
          #choice_name_ident.push(#choice_enum_type::#variant_ident(
            Self::#fn_ident(_xml_reader, e, _e_empty)?,
          ));
        }
      } else {
        quote! {
          #choice_name_ident = Some(#choice_enum_type::#variant_ident(
            Self::#fn_ident(_xml_reader, e, _e_empty)?,
          ));
        }
      };
      arms.push(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #assign_stmt
            Ok(true)
          }
        },
      ))?);
    } else if insert_full {
      let assign_stmt = if repeated {
        quote! {
          #choice_name_ident.push(#choice_enum_type::#variant_ident(
            Self::#fn_ident(_xml_reader, e, _e_empty)?,
          ));
        }
      } else {
        quote! {
          #choice_name_ident = Some(#choice_enum_type::#variant_ident(
            Self::#fn_ident(_xml_reader, e, _e_empty)?,
          ));
        }
      };
      arms.push(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal => {
            #assign_stmt
            Ok(true)
          }
        },
      ))?);
    }
  }

  Ok(arms)
}

fn gen_one_sequence_visitor_arm(
  child: &ResolvedOneSequenceChild<'_>,
  version: &str,
  schema: &Schema,
  context: &CodegenContext<'_>,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];

  let child_name_ident: Ident = parse_str(&child.field_name)?;

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;

  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    if repeated {
      Ok(Some(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
              Ok(parsed_child) => {
                #child_name_ident.push(parsed_child);
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?))
    } else {
      Ok(Some(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
              Ok(parsed_child) => {
                #child_name_ident = Some(std::boxed::Box::new(parsed_child));
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?))
    }
  } else if insert_full && !repeated {
    Ok(Some(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              #child_name_ident = Some(std::boxed::Box::new(parsed_child));
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              #child_name_ident.push(parsed_child);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_one_sequence_choice_visitor_arm(
  child: &ResolvedOneSequenceChild<'_>,
  choice_name_ident: &Ident,
  choice_enum_type: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;
  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];
  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;
  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;
  let choice_variant_ident: Ident = parse_str(&child.field_name.to_upper_camel_case())?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    if repeated {
      Ok(Some(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
              Ok(parsed_child) => {
                #choice_name_ident.push(
                  #choice_enum_type::#choice_variant_ident(std::boxed::Box::new(parsed_child))
                );
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?))
    } else {
      Ok(Some(parse2(versioned_tokens(
        child.version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
              Ok(parsed_child) => {
                #choice_name_ident = Some(
                  #choice_enum_type::#choice_variant_ident(std::boxed::Box::new(parsed_child))
                );
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?))
    }
  } else if insert_full && repeated {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              #choice_name_ident.push(
                #choice_enum_type::#choice_variant_ident(std::boxed::Box::new(parsed_child))
              );
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              #choice_name_ident = Some(
                #choice_enum_type::#choice_variant_ident(std::boxed::Box::new(parsed_child))
              );
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_child_match_arm(
  child: &ResolvedCompositeChild<'_>,
  child_choice_enum_ident: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  if child.is_any {
    let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

    return Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        _ => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            crate::common::read_outer_xml(xml_reader, e, e_empty)?,
          )));
        }
      },
    ))?));
  }

  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;

  let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_child_visitor_arm(
  child: &ResolvedCompositeChild<'_>,
  child_choice_enum_ident: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Option<Arm>> {
  if child.is_any {
    let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

    return Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        _ => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            crate::common::read_outer_xml(_xml_reader, e, _e_empty)?,
          )));
          Ok(true)
        }
      },
    ))?));
  }

  let child_type = context
    .type_by_name(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
  let child_suffix_last_name = &child_last_name[child_last_name.find(':').unwrap() + 1..];

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\""))?;
  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\""))?;

  let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

  let child_module_name = context
    .type_module(child.name)
    .unwrap_or(&schema.module_name);
  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    child_module_name,
    child_type.class_name.to_upper_camel_case()
  ))?;

  let insert_suffix = loop_children_suffix_match_set.insert(child_suffix_last_name.to_string());
  let insert_full = loop_children_suffix_match_set.insert(child_last_name.to_string());

  if insert_suffix && child_last_name != child_suffix_last_name {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
                parsed_child,
              )));
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else if insert_full {
    Ok(Some(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(_xml_reader, Some((e, _e_empty))) {
            Ok(parsed_child) => {
              children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
                parsed_child,
              )));
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?))
  } else {
    Ok(None)
  }
}

fn gen_simple_child_match_arms(
  first_name: &str,
  owner_type: &str,
  treat_as_string: bool,
  context: &CodegenContext<'_>,
) -> Result<Vec<Arm>> {
  if let Some(schema_enum) = context.enum_by_type(first_name) {
    let enum_module = context
      .enum_module_by_type(first_name)
      .ok_or_else(|| format!("{first_name:?}"))?;

    let simple_type_name: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      enum_module,
      schema_enum.name.to_upper_camel_case()
    ))?;

    Ok(vec![parse2(quote! {
      quick_xml::events::Event::Text(t) => {
        xml_content = Some(#simple_type_name::from_bytes(&t.into_inner())?);
      }
    })?])
  } else {
    let simple_type_str = if treat_as_string {
      "StringValue"
    } else {
      simple_type_mapping(first_name)
    };
    let simple_type: Type = parse_str(&format!("crate::simple_type::{simple_type_str}"))?;

    Ok(
      match classify_simple_type(simple_type_str).ok_or_else(|| simple_type_str.to_string())? {
        SimpleValueKind::StringLike => vec![
          parse2(quote! {
            quick_xml::events::Event::Text(t) => {
              crate::common::push_xml_text(&mut xml_content, t)?;
            }
          })?,
          parse2(quote! {
            quick_xml::events::Event::GeneralRef(t) => {
              crate::common::push_xml_general_ref(&mut xml_content, t, #owner_type, "xml_content")?;
            }
          })?,
        ],
        SimpleValueKind::BoolLike => {
          vec![parse2(quote! {
            quick_xml::events::Event::Text(t) => {
              xml_content = Some(crate::common::parse_bool_bytes(&t.into_inner())?);
            }
          })?]
        }
        SimpleValueKind::NumericLike => vec![parse2(quote! {
          quick_xml::events::Event::Text(t) => {
            let value = t.decode()?;
            xml_content = Some(crate::common::parse_value::<#simple_type>(
              value.as_ref(),
              #owner_type,
              "xml_content",
            )?);
          }
        })?],
      },
    )
  }
}

fn gen_attr_stmt(attr_match_list: &[MatchBranch]) -> Result<Stmt> {
  let attr_binding = if !attr_match_list.is_empty()
    && attr_match_list
      .iter()
      .all(|branch| is_microsoft365_version(branch.version.as_str()))
  {
    quote! {
      #[cfg(feature = "microsoft365")]
      let attr = attr?;
      #[cfg(not(feature = "microsoft365"))]
      let _attr = attr?;
    }
  } else {
    quote! {
      let attr = attr?;
    }
  };

  let attr_ifs: Vec<Stmt> = attr_match_list
    .iter()
    .map(|branch| {
      let version = branch.version.as_str();
      let key = &branch.key;
      let body = &branch.body;

      parse2(versioned_tokens(
        version,
        quote! {
          if attr.key.as_ref() == #key {
            #body
          }
        },
      ))
    })
    .collect::<syn::Result<_>>()?;

  Ok(parse2(quote! {
    for attr in e.attributes().with_checks(false) {
      #attr_binding
      #( #attr_ifs )*
    }
  })?)
}

fn gen_attr_match_arm(branch: &MatchBranch) -> Result<Arm> {
  let version = branch.version.as_str();
  let key = &branch.key;
  let body = &branch.body;

  Ok(parse2(versioned_tokens(
    version,
    quote! {
      #key => {
        #body
      }
    },
  ))?)
}

fn gen_field_match_arm(
  attr: &SchemaTypeAttribute,
  schema_name: &str,
  type_name: &str,
  type_qname: &str,
  compatibility_rules: &[CompatibilityRule],
  owner_type: &str,
  context: &CodegenContext<'_>,
) -> Result<MatchBranch> {
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

  let attr_name_literal: LitByteStr = parse_str(&format!("b\"{attr_name_str}\""))?;
  let treat_as_string = treat_as_string_rule_for_field(
    compatibility_rules,
    schema_name,
    type_name,
    &attr.property_name,
  )
  .is_some()
    || treat_as_string_rule_for_field(
      compatibility_rules,
      schema_name,
      type_qname,
      &attr.property_name,
    )
    .is_some()
    || treat_as_string_rule_for_field(compatibility_rules, schema_name, type_name, &attr.q_name)
      .is_some()
    || treat_as_string_rule_for_field(compatibility_rules, schema_name, type_qname, &attr.q_name)
      .is_some();
  let map_value_rule = map_attribute_value_rule_for_field(
    compatibility_rules,
    schema_name,
    type_name,
    &attr.property_name,
  )
  .or_else(|| {
    map_attribute_value_rule_for_field(
      compatibility_rules,
      schema_name,
      type_qname,
      &attr.property_name,
    )
  })
  .or_else(|| {
    map_attribute_value_rule_for_field(compatibility_rules, schema_name, type_name, &attr.q_name)
  })
  .or_else(|| {
    map_attribute_value_rule_for_field(compatibility_rules, schema_name, type_qname, &attr.q_name)
  });

  let mapped_value_stmt: Option<TokenStream> = if let Some(rule) = map_value_rule {
    let CompatibilityAction::MapAttributeValue { mappings } = &rule.action else {
      unreachable!()
    };
    let mut mapping_entries = Vec::<TokenStream>::new();
    for mapping in mappings {
      let from = &mapping.from;
      let to = &mapping.to;
      mapping_entries.push(quote! { (#from, #to) });
    }

    Some(quote! {
      let raw_value = crate::common::decode_attr_value(&attr, xml_reader.decoder())?;
      let mapped_value = crate::common::map_compat_attr_value(
        raw_value,
        &[ #( #mapping_entries ),* ],
      );
    })
  } else {
    None
  };

  let effective_attr_type = if treat_as_string {
    "StringValue"
  } else {
    attr.r#type.as_str()
  };

  let body_core: TokenStream = match classify_attr_type(effective_attr_type)
    .ok_or_else(|| effective_attr_type.to_string())?
  {
    AttrTypeKind::List => parse2(quote! {
      #attr_name_ident = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
    })?,
    AttrTypeKind::Enum { .. } => {
      let (enum_module_name, enum_name) = context.resolve_attr_enum_module(&attr.r#type)?;

      let enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}",
        enum_module_name,
        enum_name.to_upper_camel_case()
      ))?;

      if mapped_value_stmt.is_some() {
        parse2(quote! {
          #attr_name_ident = Some(mapped_value.parse::<#enum_type>().map_err(|_| {
            crate::common::invalid_enum_value(stringify!(#enum_type), mapped_value.clone())
          })?);
        })?
      } else {
        parse2(quote! {
          #attr_name_ident = Some(crate::common::parse_enum_attr::<#enum_type>(
            &attr,
            xml_reader.decoder(),
            stringify!(#enum_type),
          )?);
        })?
      }
    }
    AttrTypeKind::Simple {
      simple_type,
      value_kind,
    } => match value_kind {
      SimpleValueKind::StringLike => {
        if mapped_value_stmt.is_some() {
          parse2(quote! {
            #attr_name_ident = Some(mapped_value);
          })?
        } else {
          parse2(quote! {
            #attr_name_ident = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
          })?
        }
      }
      SimpleValueKind::BoolLike => parse2(quote! {
        #attr_name_ident = Some(crate::common::parse_bool_attr(
          &attr,
          xml_reader.decoder(),
          #owner_type,
          #attr_name_str,
        )?);
      })?,
      SimpleValueKind::NumericLike => {
        let simple_type: Type = parse_str(&format!("crate::simple_type::{simple_type}"))?;

        parse2(quote! {
          #attr_name_ident = Some(crate::common::parse_attr_value::<#simple_type>(
            &attr,
            xml_reader.decoder(),
            #owner_type,
            #attr_name_str,
          )?);
        })?
      }
    },
  };
  let body: Stmt = if let Some(mapped_value_stmt) = mapped_value_stmt {
    parse2(quote! {
      {
        #mapped_value_stmt
        #body_core
      }
    })?
  } else {
    parse2(body_core)?
  };
  Ok(MatchBranch {
    version: attr.version.clone(),
    key: attr_name_literal,
    body,
  })
}

fn gen_strict_bitmask_match_branches(
  rule: &CompatibilityRule,
  target_ident: &Ident,
  owner_type: &str,
  field_name: &str,
) -> Result<Vec<MatchBranch>> {
  let CompatibilityAction::StrictBitmaskAttributes {
    radix,
    width,
    attributes,
  } = &rule.action
  else {
    return Ok(vec![]);
  };

  let mut branches = Vec::with_capacity(attributes.len());

  for attribute in attributes {
    let key: LitByteStr = parse_str(&format!("b\"{}\"", attribute.q_name))?;
    let bit = attribute.bit;
    let radix = *radix;
    let width = *width;

    branches.push(MatchBranch {
      version: String::new(),
      key,
      body: parse2(quote! {
        {
          let raw_value = crate::common::decode_attr_value(&attr, xml_reader.decoder())?;
          #target_ident = Some(crate::common::merge_strict_bitmask_attr(
            #target_ident.as_deref(),
            raw_value.as_ref(),
            #bit,
            #radix,
            #width,
            #owner_type,
            #field_name,
          )?);
        }
      })?,
    });
  }

  Ok(branches)
}
