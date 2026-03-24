use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{Arm, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type, parse_str, parse2};

use crate::generator::context::GenContext;
use crate::generator::simple_type::simple_type_mapping;
use crate::models::{
  OpenXmlSchema, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild, OpenXmlSchemaTypeParticle,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case, get_or_panic};

pub fn gen_deserializers(schema: &OpenXmlSchema, gen_context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  let schema_namespace = get_or_panic!(
    gen_context.uri_namespace_map,
    schema.target_namespace.as_str()
  );

  for e in &schema.enums {
    let enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    let mut variants: Vec<Arm> = vec![];
    let mut byte_variants: Vec<Arm> = vec![];

    for facet in &e.facets {
      let variant_value = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      let variant_value_literal: LitByteStr = parse_str(&format!("b\"{variant_value}\"")).unwrap();

      variants.push(
        parse2(quote! {
          #variant_value => Ok(Self::#variant_ident),
        })
        .unwrap(),
      );

      byte_variants.push(
        parse2(quote! {
          #variant_value_literal => Ok(Self::#variant_ident),
        })
        .unwrap(),
      );
    }

    token_stream_list.push(
      parse2(quote! {
        impl std::str::FromStr for #enum_type {
          type Err = crate::common::SdkError;

          fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
              #( #variants )*
              _ => Err(crate::common::SdkError::CommonError(s.to_string())),
            }
          }
        }
      })
      .unwrap(),
    );

    token_stream_list.push(
      parse2(quote! {
        impl #enum_type {
          pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
            match b {
              #( #byte_variants )*
              other => Err(crate::common::SdkError::CommonError(
                String::from_utf8_lossy(other).into_owned(),
              )),
            }
          }
        }
      })
      .unwrap(),
    );
  }

  let from_reader_fn = gen_from_reader_fn();

  for t in &schema.types {
    if t.is_abstract {
      continue;
    }

    let class_name_str = t.class_name.to_upper_camel_case();

    let struct_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name, &class_name_str
    ))
    .unwrap();

    let from_str_impl = gen_from_str_impl(&struct_type);

    token_stream_list.push(from_str_impl);

    let first_name = &t.name[0..t.name.find('/').unwrap()];

    let prefix_type_name_str = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];

    let type_name_str = &prefix_type_name_str
      [prefix_type_name_str.find(':').unwrap() + 1..prefix_type_name_str.len()];

    let prefix_type_name_literal: LitByteStr =
      parse_str(&format!("b\"{prefix_type_name_str}\"")).unwrap();
    let type_name_literal: LitByteStr = parse_str(&format!("b\"{type_name_str}\"")).unwrap();

    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut attr_match_list: Vec<Arm> = vec![];
    let mut field_unwrap_list: Vec<Stmt> = vec![];
    let mut field_ident_list: Vec<Ident> = vec![];
    let mut loop_declaration_list: Vec<Stmt> = vec![];
    let mut loop_children_stmt_opt: Option<Stmt> = None;
    let mut loop_match_arm_list: Vec<Arm> = vec![];

    let mut loop_children_match_list: Vec<Arm> = vec![];
    let mut loop_children_suffix_match_set: HashSet<String> = HashSet::new();

    let mut attributes: Vec<&OpenXmlSchemaTypeAttribute> = vec![];

    let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

    for child in &t.children {
      child_map.insert(&child.name, child);
    }

    if t.base_class == "OpenXmlLeafTextElement" {
      for attr in &t.attributes {
        attributes.push(attr);
      }

      field_declaration_list.push(
        parse2(quote! {
          let mut xml_content = None;
        })
        .unwrap(),
      );

      field_ident_list.push(
        parse2(quote! {
          xml_content
        })
        .unwrap(),
      );

      loop_match_arm_list.push(gen_simple_child_match_arm(first_name, gen_context));
    } else if t.base_class == "OpenXmlLeafElement" {
      for attr in &t.attributes {
        attributes.push(attr);
      }
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      if !t.part.is_empty()
        || t.base_class == "OpenXmlPartRootElement"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"
      {
        field_declaration_list.push(
          parse2(quote! {
            let mut xmlns = None;
          })
          .unwrap(),
        );

        field_declaration_list.push(
          parse2(quote! {
            let mut xmlns_map = std::collections::HashMap::<String, String>::new();
          })
          .unwrap(),
        );

        field_declaration_list.push(
          parse2(quote! {
            let mut mc_ignorable = None;
          })
          .unwrap(),
        );

        field_ident_list.push(parse_str("xmlns").unwrap());
        field_ident_list.push(parse_str("xmlns_map").unwrap());
        field_ident_list.push(parse_str("mc_ignorable").unwrap());
      }

      for attr in &t.attributes {
        attributes.push(attr);
      }

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        &schema.module_name,
        t.class_name.to_upper_camel_case()
      ))
      .unwrap();

      if t.is_one_sequence_flatten() {
        for p in &t.particle.items {
          let child = get_or_panic!(child_map, p.name.as_str());

          let child_name_str = if child.property_name.is_empty() {
            &child.name[child.name.find('/').unwrap() + 1..child.name.len()]
          } else {
            child.property_name.as_str()
          };

          let child_name_ident: Ident =
            parse_str(&escape_snake_case(child_name_str.to_snake_case())).unwrap();

          if p.occurs.is_empty() {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = None;
              })
              .unwrap(),
            );

            field_unwrap_list.push(
              parse2(quote! {
                let #child_name_ident = #child_name_ident
                  .ok_or_else(|| crate::common::SdkError::CommonError(#child_name_str.to_string()))?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = None;
              })
              .unwrap(),
            );
          } else {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = vec![];
              })
              .unwrap(),
            );
          }

          field_ident_list.push(child_name_ident);

          loop_children_match_list.push(gen_one_sequence_match_arm(
            p,
            child,
            gen_context,
            &mut loop_children_suffix_match_set,
          ));
        }
      } else {
        if !t.children.is_empty() {
          field_declaration_list.push(
            parse2(quote! {
              let mut children = vec![];
            })
            .unwrap(),
          );

          field_ident_list.push(
            parse2(quote! {
              children
            })
            .unwrap(),
          );
        }

        for child in &t.children {
          loop_children_match_list.push(gen_child_match_arm(
            child,
            &child_choice_enum_type,
            gen_context,
            &mut loop_children_suffix_match_set,
          ));
        }
      }
    } else if t.is_derived {
      let base_class_type = get_or_panic!(
        gen_context.type_name_type_map,
        &t.name[0..t.name.find('/').unwrap() + 1]
      );

      for attr in &t.attributes {
        attributes.push(attr);
      }

      for attr in &base_class_type.attributes {
        attributes.push(attr);
      }

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        for p in &t.particle.items {
          let child = get_or_panic!(child_map, p.name.as_str());

          let child_name_str = if child.property_name.is_empty() {
            &child.name[child.name.find('/').unwrap() + 1..child.name.len()]
          } else {
            child.property_name.as_str()
          };

          let child_name_ident: Ident =
            parse_str(&escape_snake_case(child_name_str.to_snake_case())).unwrap();

          if p.occurs.is_empty() {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = None;
              })
              .unwrap(),
            );

            field_unwrap_list.push(
              parse2(quote! {
                let #child_name_ident = #child_name_ident
                  .ok_or_else(|| crate::common::SdkError::CommonError(#child_name_str.to_string()))?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = None;
              })
              .unwrap(),
            );
          } else {
            field_declaration_list.push(
              parse2(quote! {
                let mut #child_name_ident = vec![];
              })
              .unwrap(),
            );
          }

          field_ident_list.push(child_name_ident);
        }
      } else if !t.children.is_empty() {
        field_declaration_list.push(
          parse2(quote! {
            let mut children = vec![];
          })
          .unwrap(),
        );

        field_ident_list.push(
          parse2(quote! {
            children
          })
          .unwrap(),
        );
      } else if base_class_type.base_class == "OpenXmlLeafTextElement" {
        field_declaration_list.push(
          parse2(quote! {
            let mut xml_content = None;
          })
          .unwrap(),
        );

        field_ident_list.push(
          parse2(quote! {
            xml_content
          })
          .unwrap(),
        );
      }

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        &schema.module_name,
        t.class_name.to_upper_camel_case()
      ))
      .unwrap();

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        for p in &t.particle.items {
          let child = get_or_panic!(child_map, p.name.as_str());

          loop_children_match_list.push(gen_one_sequence_match_arm(
            p,
            child,
            gen_context,
            &mut loop_children_suffix_match_set,
          ));
        }
      } else {
        for child in &t.children {
          loop_children_match_list.push(gen_child_match_arm(
            child,
            &child_choice_enum_type,
            gen_context,
            &mut loop_children_suffix_match_set,
          ));
        }
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let base_first_name = &base_class_type.name[0..base_class_type.name.find('/').unwrap()];

        loop_match_arm_list.push(gen_simple_child_match_arm(base_first_name, gen_context));
      }
    } else {
      panic!("{t:?}");
    };

    for attr in &attributes {
      let attr_name_str = if attr.property_name.is_empty() {
        escape_snake_case(attr.q_name.to_snake_case())
      } else {
        escape_snake_case(attr.property_name.to_snake_case())
      };

      let attr_name_ident: Ident = parse_str(&attr_name_str).unwrap();

      field_declaration_list.push(
        parse2(quote! {
          let mut #attr_name_ident = None;
        })
        .unwrap(),
      );

      attr_match_list.push(gen_field_match_arm(attr, gen_context));

      let mut required = false;

      for validator in &attr.validators {
        if validator.name == "RequiredValidator" {
          required = true;
        }
      }

      if required {
        field_unwrap_list.push(
          parse2(quote! {
            let #attr_name_ident = #attr_name_ident
              .ok_or_else(|| crate::common::SdkError::CommonError(#attr_name_str.to_string()))?;
          })
          .unwrap(),
        )
      }

      field_ident_list.push(attr_name_ident);
    }

    let mut expect_event_start_stmt: Stmt = parse2(quote! {
      let (e, empty_tag) =
        crate::common::expect_event_start!(xml_reader, xml_event, #prefix_type_name_literal, #type_name_literal);
    }).unwrap();

    let attr_match_stmt_opt: Option<Stmt> = if (t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement")
      && (!t.part.is_empty()
        || t.base_class == "OpenXmlPartRootElement"
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/picture")
    {
      Some(
        parse2(quote! {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;

            match attr.key.as_ref() {
              #( #attr_match_list )*
              b"xmlns" => {
                xmlns = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
              }
              b"mc:Ignorable" => {
                mc_ignorable = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
              }
              key => {
                if key.starts_with(b"xmlns:") {
                  xmlns_map.insert(
                    String::from_utf8_lossy(&key[6..]).to_string(),
                    attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned(),
                  );
                }
              }
            }
          }
        })
        .unwrap(),
      )
    } else if !attr_match_list.is_empty() {
      Some(
        parse2(quote! {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;

            #[allow(clippy::single_match)]
            match attr.key.as_ref() {
              #( #attr_match_list )*
              _ => {}
            }
          }
        })
        .unwrap(),
      )
    } else {
      expect_event_start_stmt = parse2(quote! {
        let (_, empty_tag) =
          crate::common::expect_event_start!(xml_reader, xml_event, #prefix_type_name_literal, #type_name_literal);
      }).unwrap();

      None
    };

    if !loop_children_match_list.is_empty() {
      loop_declaration_list.push(
        parse2(quote! {
          let mut e_opt = None;
        })
        .unwrap(),
      );

      loop_declaration_list.push(
        parse2(quote! {
          let mut e_empty = false;
        })
        .unwrap(),
      );

      loop_match_arm_list.push(
        parse2(quote! {
          quick_xml::events::Event::Start(e) => {
            e_opt = Some(e);
          }
        })
        .unwrap(),
      );

      loop_match_arm_list.push(
        parse2(quote! {
          quick_xml::events::Event::Empty(e) => {
            e_empty = true;
            e_opt = Some(e);
          }
        })
        .unwrap(),
      );

      loop_children_stmt_opt = Some(
        parse2(quote! {
          if let Some(e) = e_opt {
            match e.name().as_ref() {
              #( #loop_children_match_list )*
              _ => Err(super::super::common::SdkError::CommonError(
                #class_name_str.to_string(),
              ))?,
            }
          }
        })
        .unwrap(),
      )
    }

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
              quick_xml::events::Event::Eof => Err(crate::common::SdkError::UnknownError)?,
              _ => (),
            }

            #loop_children_stmt_opt
          }
        }

        #( #field_unwrap_list )*

        Ok(Self {
          #( #field_ident_list, )*
        })
      }
    })
    .unwrap();

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #from_reader_fn

          #deserialize_inner_fn
        }
      })
      .unwrap(),
    );
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_from_str_impl(struct_type: &Type) -> ItemImpl {
  let token_stream = quote! {
    impl std::str::FromStr for #struct_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;

        Self::deserialize_inner(&mut xml_reader, None)
      }
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_from_reader_fn() -> ItemFn {
  let token_stream = quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;

      Self::deserialize_inner(&mut xml_reader, None)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_one_sequence_match_arm(
  p: &OpenXmlSchemaTypeParticle,
  child: &OpenXmlSchemaTypeChild,
  gen_context: &GenContext,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Arm {
  let child_type = get_or_panic!(gen_context.type_name_type_map, child.name.as_str());

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..child.name.len()];
  let child_suffix_last_name =
    &child_last_name[child_last_name.find(':').unwrap() + 1..child_last_name.len()];

  let child_name_ident: Ident = if child.property_name.is_empty() {
    parse_str(&child_last_name.to_snake_case()).unwrap()
  } else {
    parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap()
  };

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\"")).unwrap();

  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\"")).unwrap();

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    &child_type.module_name,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
      parse2(quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          #child_name_ident = Some(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          ));
        }
      })
      .unwrap()
    } else {
      parse2(quote! {
        #child_last_name_literal | #child_suffix_last_name_literal => {
          #child_name_ident.push(
            #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
          );
        }
      })
      .unwrap()
    }
  } else if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
    parse2(quote! {
      #child_last_name_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
        ));
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_last_name_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
        );
      }
    })
    .unwrap()
  }
}

fn gen_child_match_arm(
  child: &OpenXmlSchemaTypeChild,
  child_choice_enum_ident: &Type,
  gen_context: &GenContext,
  loop_children_suffix_match_set: &mut HashSet<String>,
) -> Arm {
  let child_type = get_or_panic!(gen_context.type_name_type_map, child.name.as_str());

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..child.name.len()];
  let child_suffix_last_name =
    &child_last_name[child_last_name.find(':').unwrap() + 1..child_last_name.len()];

  let child_last_name_literal: LitByteStr = parse_str(&format!("b\"{child_last_name}\"")).unwrap();

  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{child_suffix_last_name}\"")).unwrap();

  let child_variant_name_ident: Ident = parse_str(&child_last_name.to_upper_camel_case()).unwrap();

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    &child_type.module_name,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  if loop_children_suffix_match_set.insert(child_suffix_last_name.to_string()) {
    parse2(quote! {
      #child_last_name_literal | #child_suffix_last_name_literal => {
        children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
          #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
        )));
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_last_name_literal => {
        children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
          #child_variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
        )));
      }
    })
    .unwrap()
  }
}

fn gen_simple_child_match_arm(first_name: &str, gen_context: &GenContext) -> Arm {
  if let Some(e) = gen_context.enum_type_enum_map.get(first_name) {
    let simple_type_name: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &e.module_name,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    parse2(quote! {
      quick_xml::events::Event::Text(t) => {
        xml_content = Some(#simple_type_name::from_bytes(&t.into_inner())?);
      }
    })
    .unwrap()
  } else {
    let simple_type_str = simple_type_mapping(first_name);

    let enum_type: Type =
      parse_str(&format!("crate::schemas::simple_type::{simple_type_str}")).unwrap();

    parse2(match simple_type_str {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(t.decode()?.to_string());
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(crate::common::parse_bool_bytes(&t.into_inner())?);
        }
      },
      "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
      | "UInt64Value" | "DoubleValue" | "SingleValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(t.decode()?.parse::<#enum_type>()?);
        }
      },
      _ => panic!("{}", simple_type_str),
    })
    .unwrap()
  }
}

fn gen_field_match_arm(attr: &OpenXmlSchemaTypeAttribute, gen_context: &GenContext) -> Arm {
  let attr_name_str = if attr.q_name.starts_with(':') {
    &attr.q_name[1..attr.q_name.len()]
  } else {
    &attr.q_name
  };

  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let attr_name_literal: LitByteStr = parse_str(&format!("b\"{attr_name_str}\"")).unwrap();

  parse2(if attr.r#type.starts_with("ListValue<") {
    quote! {
      #attr_name_literal => {
        #attr_name_ident = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
      }
    }
  } else if attr.r#type.starts_with("EnumValue<") {
    let e_typed_namespace_str =
      &attr.r#type[attr.r#type.find("<").unwrap() + 1..attr.r#type.rfind(".").unwrap()];

    let enum_name = &attr.r#type[attr.r#type.rfind(".").unwrap() + 1..attr.r#type.len() - 1];

    let mut e_prefix = "";

    for typed_namespace in &gen_context.typed_namespaces {
      if e_typed_namespace_str == typed_namespace.namespace {
        let e_schema = get_or_panic!(
          gen_context.prefix_schema_map,
          typed_namespace.prefix.as_str()
        );

        for e in &e_schema.enums {
          if e.name == enum_name {
            e_prefix = &typed_namespace.prefix;
          }
        }
      }
    }

    let e_namespace = get_or_panic!(gen_context.prefix_namespace_map, e_prefix);

    let e_schema = get_or_panic!(gen_context.prefix_schema_map, e_namespace.prefix.as_str());

    let e_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      e_schema.module_name,
      enum_name.to_upper_camel_case()
    ))
    .unwrap();

    quote! {
      #attr_name_literal => {
        #attr_name_ident = Some(#e_type::from_bytes(&attr.value)?);
      }
    }
  } else {
    match attr.r#type.as_str() {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        #attr_name_literal => {
          #attr_name_ident = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        #attr_name_literal => {
          #attr_name_ident = Some(crate::common::parse_bool_bytes(&attr.value)?);
        }
      },
      "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
      | "UInt64Value" | "DoubleValue" | "SingleValue" => {
        let e_type: Type =
          parse_str(&format!("crate::schemas::simple_type::{}", &attr.r#type)).unwrap();

        quote! {
          #attr_name_literal => {
            #attr_name_ident = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .parse::<#e_type>()?,
            );
          }
        }
      }
      _ => panic!("{}", attr.r#type),
    }
  })
  .unwrap()
}
