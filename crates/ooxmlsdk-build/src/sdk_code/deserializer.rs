use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Arm, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type, parse_str, parse2};

use crate::sdk_code::helpers::{
  AttrTypeKind, SimpleValueKind, classify_attr_type, classify_simple_type,
  flatten_one_sequence_particles, is_composite_type, is_derived_type, is_leaf_element_type,
  is_leaf_text_type, is_leaf_text_wrapper, is_one_sequence_flatten, supports_xmlns_fields,
};
use crate::sdk_code::schemas::{CodegenContext, ResolvedCompositeChild, ResolvedOneSequenceChild};
use crate::sdk_code::versioning::{effective_version, version_cfg_attrs, versioned_tokens};
use crate::sdk_data::sdk_data_model::{Schema, SchemaTypeAttribute};
use crate::simple_type::simple_type_mapping;
use crate::utils::{escape_snake_case, escape_upper_camel_case};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

struct MatchBranch {
  body: Stmt,
  arm: Arm,
  literal: LitByteStr,
}

pub fn gen_schema_deserializer(
  schema: &Schema,
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
    let mut loop_declaration_list: Vec<Stmt> = vec![];
    let mut loop_children_stmt_opt: Option<Stmt> = None;
    let mut loop_match_arm_list: Vec<Arm> = vec![];

    let mut loop_children_match_list: Vec<Arm> = vec![];
    let mut loop_children_visit_match_list: Vec<Arm> = vec![];
    let mut loop_children_suffix_match_set: HashSet<String> = HashSet::new();
    let mut loop_children_visit_suffix_match_set: HashSet<String> = HashSet::new();

    let mut attributes: Vec<&SchemaTypeAttribute> = vec![];
    if is_leaf_text_wrapper(schema_type) {
      field_declaration_list.push(parse2(quote! {
        let mut xml_content = None;
      })?);

      loop_match_arm_list.extend(gen_simple_child_match_arms(
        first_name,
        class_name_literal,
        context,
      )?);
    } else if is_leaf_text_type(schema_type) {
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
        context,
      )?);
    } else if is_leaf_element_type(schema_type) {
      for attr in &schema_type.attributes {
        attributes.push(attr);
      }
    } else if is_composite_type(schema_type) {
      if supports_xmlns_fields(schema_type, schema) {
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

        for flat_particle in flatten_one_sequence_particles(schema_type) {
          let particle = flat_particle.particle;
          let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
          let child_name_ident: Ident = parse_str(&child.field_name)?;

          if !field_name_set.insert(child_name_ident.to_string()) {
            continue;
          }

          if flat_particle.repeated {
            field_declaration_list.push(parse2(versioned_tokens(
              effective_version(child.version, particle.initial_version.as_str()),
              quote! {
              let mut #child_name_ident = vec![];
              },
            ))?);
          } else {
            field_declaration_list.push(parse2(versioned_tokens(
              effective_version(child.version, particle.initial_version.as_str()),
              quote! {
              let mut #child_name_ident = None;
              },
            ))?);

            if !flat_particle.optional {
              let child_name_str = child.field_name.as_ref();
              field_unwrap_list.push(parse2(versioned_tokens(
                effective_version(child.version, particle.initial_version.as_str()),
                quote! {
                let #child_name_ident = #child_name_ident
                  .ok_or_else(|| crate::common::missing_field(#class_name_literal, #child_name_str))?;
                },
              ))?);
            }
          }

          field_ident_list.push(versioned_tokens(
            effective_version(child.version, particle.initial_version.as_str()),
            quote! { #child_name_ident },
          ));

          loop_children_match_list.push(gen_one_sequence_match_arm(
            &child,
            effective_version(child.version, particle.initial_version.as_str()),
            schema,
            context,
            flat_particle.repeated,
            &mut loop_children_suffix_match_set,
          )?);
          loop_children_visit_match_list.push(gen_one_sequence_visitor_arm(
            &child,
            effective_version(child.version, particle.initial_version.as_str()),
            schema,
            context,
            flat_particle.repeated,
            &mut loop_children_visit_suffix_match_set,
          )?);
        }
      } else {
        let resolved_children = context.resolve_children(schema_type)?;

        if !resolved_children.is_empty() {
          field_declaration_list.push(parse2(quote! {
            let mut children = vec![];
          })?);

          field_ident_list.push(quote! { children });
        }

        for child in &resolved_children {
          loop_children_match_list.push(gen_child_match_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_suffix_match_set,
          )?);
          loop_children_visit_match_list.push(gen_child_visitor_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_visit_suffix_match_set,
          )?);
        }
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
        let mut field_name_set = HashSet::new();

        for flat_particle in flatten_one_sequence_particles(schema_type) {
          let particle = flat_particle.particle;
          let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
          let child_name_ident: Ident = parse_str(&child.field_name)?;

          if !field_name_set.insert(child_name_ident.to_string()) {
            continue;
          }

          if flat_particle.repeated {
            field_declaration_list.push(parse2(versioned_tokens(
              effective_version(child.version, particle.initial_version.as_str()),
              quote! {
              let mut #child_name_ident = vec![];
              },
            ))?);
          } else {
            field_declaration_list.push(parse2(versioned_tokens(
              effective_version(child.version, particle.initial_version.as_str()),
              quote! {
              let mut #child_name_ident = None;
              },
            ))?);

            if !flat_particle.optional {
              let child_name_str = child.field_name.as_ref();
              field_unwrap_list.push(parse2(versioned_tokens(
                effective_version(child.version, particle.initial_version.as_str()),
                quote! {
                let #child_name_ident = #child_name_ident
                  .ok_or_else(|| crate::common::missing_field(#class_name_literal, #child_name_str))?;
                },
              ))?);
            }
          }

          field_ident_list.push(versioned_tokens(
            effective_version(child.version, particle.initial_version.as_str()),
            quote! { #child_name_ident },
          ));
        }
      } else if !resolved_children.is_empty() {
        field_declaration_list.push(parse2(quote! {
          let mut children = vec![];
        })?);

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
        for flat_particle in flatten_one_sequence_particles(schema_type) {
          let particle = flat_particle.particle;
          let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;

          loop_children_match_list.push(gen_one_sequence_match_arm(
            &child,
            effective_version(child.version, particle.initial_version.as_str()),
            schema,
            context,
            flat_particle.repeated,
            &mut loop_children_suffix_match_set,
          )?);
          loop_children_visit_match_list.push(gen_one_sequence_visitor_arm(
            &child,
            effective_version(child.version, particle.initial_version.as_str()),
            schema,
            context,
            flat_particle.repeated,
            &mut loop_children_visit_suffix_match_set,
          )?);
        }
      } else {
        for child in &resolved_children {
          loop_children_match_list.push(gen_child_match_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_suffix_match_set,
          )?);
          loop_children_visit_match_list.push(gen_child_visitor_arm(
            child,
            &child_choice_enum_type,
            schema,
            context,
            &mut loop_children_visit_suffix_match_set,
          )?);
        }
      }

      if resolved_children.is_empty() && is_leaf_text_type(base_class_type) {
        let base_first_name = &base_class_type.name[0..base_class_type.name.find('/').unwrap()];

        loop_match_arm_list.extend(gen_simple_child_match_arms(
          base_first_name,
          class_name_literal,
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

      attr_match_list.push(gen_field_match_arm(attr, class_name_literal, context)?);

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
      .map(|branch| branch.arm.clone())
      .collect();

    let attr_match_stmt_opt: Option<Stmt> =
      if is_composite_type(schema_type) && supports_xmlns_fields(schema_type, schema) {
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

    if !loop_children_match_list.is_empty() {
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

      loop_children_stmt_opt = Some(parse2(quote! {{
        let mut visit_foreign_child =
          |xml_reader: &mut R, e: quick_xml::events::BytesStart<'de>, e_empty: bool| -> Result<bool, crate::common::SdkError> {
            match e.name().as_ref() {
              #( #loop_children_visit_match_list )*
              _ => Ok(false),
            }
          };

        if let Some(e) = e_opt {
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
          }
        }
      }})?);
    }

    let result_expr: TokenStream = if is_leaf_text_wrapper(schema_type) {
      quote! {
        Ok(Self(xml_content))
      }
    } else {
      quote! {
        Ok(Self {
          #( #field_ident_list, )*
        })
      }
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
) -> Result<Arm> {
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

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    if repeated {
      Ok(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #child_name_ident.push(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            );
          }
        },
      ))?)
    } else {
      Ok(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            #child_name_ident = Some(std::boxed::Box::new(
              #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
            ));
          }
        },
      ))?)
    }
  } else if !repeated {
    Ok(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          #child_name_ident = Some(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          ));
        }
      },
    ))?)
  } else {
    Ok(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          #child_name_ident.push(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          );
        }
      },
    ))?)
  }
}

fn gen_one_sequence_visitor_arm(
  child: &ResolvedOneSequenceChild<'_>,
  version: &str,
  schema: &Schema,
  context: &CodegenContext<'_>,
  repeated: bool,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Arm> {
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

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    if repeated {
      Ok(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
              Ok(parsed_child) => {
                #child_name_ident.push(parsed_child);
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?)
    } else {
      Ok(parse2(versioned_tokens(
        version,
        quote! {
          #child_last_name_literal | #child_suffix_last_name_literal => {
            match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
              Ok(parsed_child) => {
                #child_name_ident = Some(std::boxed::Box::new(parsed_child));
                Ok(true)
              }
              Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
              Err(err) => Err(err),
            }
          }
        },
      ))?)
    }
  } else if !repeated {
    Ok(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
            Ok(parsed_child) => {
              #child_name_ident = Some(std::boxed::Box::new(parsed_child));
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?)
  } else {
    Ok(parse2(versioned_tokens(
      version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
            Ok(parsed_child) => {
              #child_name_ident.push(parsed_child);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          }
        }
      },
    ))?)
  }
}

fn gen_child_match_arm(
  child: &ResolvedCompositeChild<'_>,
  child_choice_enum_ident: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Arm> {
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

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    Ok(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?)
  } else {
    Ok(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          )));
        }
      },
    ))?)
  }
}

fn gen_child_visitor_arm(
  child: &ResolvedCompositeChild<'_>,
  child_choice_enum_ident: &Type,
  schema: &Schema,
  context: &CodegenContext<'_>,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Result<Arm> {
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

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    Ok(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
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
    ))?)
  } else {
    Ok(parse2(versioned_tokens(
      child.version,
      quote! {
        #child_last_name_literal => {
          match #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty))) {
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
    ))?)
  }
}

fn gen_simple_child_match_arms(
  first_name: &str,
  owner_type: &str,
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
    let simple_type_str = simple_type_mapping(first_name);
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
  if let [branch] = attr_match_list {
    let literal = &branch.literal;
    let body = &branch.body;

    return Ok(parse2(quote! {
      for attr in e.attributes().with_checks(false) {
        let attr = attr?;

        if attr.key.as_ref() == #literal {
          #body
        }
      }
    })?);
  }

  let arms: Vec<Arm> = attr_match_list
    .iter()
    .map(|branch| branch.arm.clone())
    .collect();

  Ok(parse2(quote! {
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;

      match attr.key.as_ref() {
        #( #arms )*
        _ => {}
      }
    }
  })?)
}

fn gen_field_match_arm(
  attr: &SchemaTypeAttribute,
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

  let body: Stmt = match classify_attr_type(&attr.r#type).ok_or_else(|| attr.r#type.clone())? {
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

      parse2(quote! {
        #attr_name_ident = Some(crate::common::parse_enum_attr::<#enum_type>(
          &attr,
          xml_reader.decoder(),
          stringify!(#enum_type),
        )?);
      })?
    }
    AttrTypeKind::Simple {
      simple_type,
      value_kind,
    } => match value_kind {
      SimpleValueKind::StringLike => parse2(quote! {
        #attr_name_ident = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
      })?,
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
  let arm: Arm = parse2(versioned_tokens(
    &attr.version,
    quote! {
      #attr_name_literal => {
        #body
      }
    },
  ))?;

  Ok(MatchBranch {
    body,
    arm,
    literal: attr_name_literal,
  })
}
