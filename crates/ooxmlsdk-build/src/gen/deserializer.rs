use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse2, parse_str, Arm, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type};

use crate::gen::context::{GenContext, GenContextNeo};
use crate::gen::simple_type::simple_type_mapping;
use crate::models::{
  OpenXmlSchema, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild, OpenXmlSchemaTypeParticle,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case, get_or_panic};

pub fn gen_deserializers_neo(schema: &OpenXmlSchema, gen_context: &GenContextNeo) -> TokenStream {
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

    for facet in &e.facets {
      let variant_rename = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(
        parse2(quote! {
          #variant_rename => Ok(Self::#variant_ident),
        })
        .unwrap(),
      )
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
    )
  }

  let from_reader_fn = gen_from_reader_fn_neo();

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

    let from_str_impl = gen_from_str_impl_neo(&struct_type);

    token_stream_list.push(from_str_impl);

    let first_name = &t.name[0..t.name.find('/').unwrap()];

    let prefix_type_name_str = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];

    let type_name_str = &prefix_type_name_str
      [prefix_type_name_str.find(':').unwrap() + 1..prefix_type_name_str.len()];

    let prefix_type_name_literal: LitByteStr =
      parse_str(&format!("b\"{}\"", prefix_type_name_str)).unwrap();
    let type_name_literal: LitByteStr = parse_str(&format!("b\"{}\"", type_name_str)).unwrap();

    let xmlns_literal: LitByteStr =
      parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut attr_match_list: Vec<Arm> = vec![];
    let mut field_unwrap_list: Vec<Stmt> = vec![];
    let mut field_ident_list: Vec<Ident> = vec![];
    let mut loop_declaration_list: Vec<Stmt> = vec![];
    let mut loop_children_stmt_opt: Option<Stmt> = None;
    let mut loop_match_arm_list: Vec<Arm> = vec![];

    let mut loop_children_match_list: Vec<Arm> = vec![];
    let mut loop_children_suffix_match_map: HashMap<String, Arm> = HashMap::new();

    let mut attributes: Vec<&OpenXmlSchemaTypeAttribute> = vec![];

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

      loop_match_arm_list.push(gen_simple_child_match_arm_neo(first_name, gen_context));
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
            let mut with_xmlns = with_xmlns;
          })
          .unwrap(),
        );

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

      let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

      for child in &t.children {
        child_map.insert(&child.name, child);
      }

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

          let (suffix_match_name, suffix_match_arm, _, match_arm) =
            gen_one_sequence_match_arm_neo(p, child, gen_context);

          loop_children_suffix_match_map.insert(suffix_match_name, suffix_match_arm);
          loop_children_match_list.push(match_arm);
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
          let (suffix_match_name, suffix_match_arm, _, match_arm) =
            gen_child_match_arm_neo(child, &child_choice_enum_type, gen_context);

          loop_children_suffix_match_map.insert(suffix_match_name, suffix_match_arm);
          loop_children_match_list.push(match_arm);
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

      let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

      for child in &t.children {
        child_map.insert(&child.name, child);
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

          let (suffix_match_name, suffix_match_arm, _, match_arm) =
            gen_one_sequence_match_arm_neo(p, child, gen_context);

          loop_children_suffix_match_map.insert(suffix_match_name, suffix_match_arm);
          loop_children_match_list.push(match_arm);
        }
      } else {
        for child in &t.children {
          let (suffix_match_name, suffix_match_arm, _, match_arm) =
            gen_child_match_arm_neo(child, &child_choice_enum_type, gen_context);

          loop_children_suffix_match_map.insert(suffix_match_name, suffix_match_arm);
          loop_children_match_list.push(match_arm);
        }
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let base_first_name = &base_class_type.name[0..base_class_type.name.find('/').unwrap()];

        loop_match_arm_list.push(gen_simple_child_match_arm_neo(base_first_name, gen_context));
      }
    } else {
      panic!("{:?}", t);
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

      attr_match_list.push(gen_field_match_arm_neo(attr, gen_context));

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

    let mut e_ident: Ident = parse_str("e").unwrap();

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
                xmlns = Some(attr.unescape_value()?.into_owned());
              }
              b"mc:Ignorable" => {
                mc_ignorable = Some(attr.unescape_value()?.into_owned());
              }
              key => {
                if key.starts_with(b"xmlns:") {
                  xmlns_map.insert(
                    String::from_utf8_lossy(&key[6..]).to_string(),
                    attr.unescape_value()?.into_owned(),
                  );

                  if key == #xmlns_literal {
                    with_xmlns = true;
                  }
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
      e_ident = parse_str("_e").unwrap();

      None
    };

    if !loop_children_match_list.is_empty() || !loop_children_suffix_match_map.is_empty() {
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

      let loop_children_suffix_match_list: Vec<Arm> =
        loop_children_suffix_match_map.into_values().collect();

      loop_children_stmt_opt = Some(
        parse2(quote! {
          if let Some(e) = e_opt {
            if with_xmlns {
              match e.name().as_ref() {
                #( #loop_children_match_list )*
                _ => Err(super::super::common::SdkError::CommonError(
                  #class_name_str.to_string(),
                ))?,
              }
            } else {
              match e.name().as_ref() {
                #( #loop_children_suffix_match_list )*
                _ => Err(super::super::common::SdkError::CommonError(
                  #class_name_str.to_string(),
                ))?,
              }
            }
          }
        })
        .unwrap(),
      )
    }

    let deserialize_inner_fn: ItemFn = parse2(quote! {
      #[inline(always)]
      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        with_xmlns: bool,
        mut empty_tag: bool,
        xml_event: Option<quick_xml::events::BytesStart<'de>>,
      ) -> Result<Self, crate::common::SdkError> {
        #( #field_declaration_list )*

        let #e_ident = if let Some(e) = xml_event {
          e
        } else {
          let e = match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => e,
            quick_xml::events::Event::Empty(e) => {
              empty_tag = true;

              e
            }
            _ => Err(crate::common::SdkError::CommonError(
              #class_name_str.to_string(),
            ))?,
          };

          if with_xmlns && e.name().as_ref() != #prefix_type_name_literal {
            Err(crate::common::SdkError::MismatchError {
              expected: #prefix_type_name_str.to_string(),
              found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
            })?;
          } else if e.name().as_ref() != #type_name_literal {
            Err(crate::common::SdkError::MismatchError {
              expected: #type_name_str.to_string(),
              found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
            })?;
          }

          e
        };

        #attr_match_stmt_opt

        if !empty_tag {
          loop {
            #( #loop_declaration_list )*

            match xml_reader.next()? {
              #( #loop_match_arm_list )*
              quick_xml::events::Event::End(e) => {
                if with_xmlns {
                  if e.name().as_ref() == b"w:Override" {
                    break;
                  }
                } else if e.name().as_ref() == b"Override" {
                  break;
                }
              }
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

fn gen_from_str_impl_neo(struct_type: &Type) -> ItemImpl {
  let token_stream = quote! {
    impl std::str::FromStr for #struct_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;

        Self::deserialize_inner(&mut xml_reader, false, false, None)
      }
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_from_reader_fn_neo() -> ItemFn {
  let token_stream = quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;

      Self::deserialize_inner(&mut xml_reader, false, false, None)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_one_sequence_match_arm_neo(
  p: &OpenXmlSchemaTypeParticle,
  child: &OpenXmlSchemaTypeChild,
  gen_context: &GenContextNeo,
) -> (String, Arm, String, Arm) {
  let child_type = get_or_panic!(gen_context.type_name_type_map, child.name.as_str());

  let child_namespace = get_or_panic!(gen_context.type_name_namespace_map, child.name.as_str());

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..child.name.len()];
  let child_suffix_last_name =
    &child_last_name[child_last_name.find(':').unwrap() + 1..child_last_name.len()];

  let child_name_ident: Ident = if child.property_name.is_empty() {
    parse_str(&child_last_name.to_snake_case()).unwrap()
  } else {
    parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap()
  };

  let child_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_last_name)).unwrap();

  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_suffix_last_name)).unwrap();

  let child_schema = get_or_panic!(
    gen_context.prefix_schema_map,
    child_namespace.prefix.as_str()
  );

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    &child_schema.module_name,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  let suffix_match_arm: Arm =
    if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
      parse2(quote! {
        #child_suffix_last_name_literal => {
          #child_name_ident = Some(std::boxed::Box::new(
            #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
          ));
        }
      })
      .unwrap()
    } else {
      parse2(quote! {
        #child_suffix_last_name_literal => {
          #child_name_ident.push(
            #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
          );
        }
      })
      .unwrap()
    };

  let match_arm: Arm = if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
    parse2(quote! {
      #child_last_name_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
        ));
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_last_name_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
        );
      }
    })
    .unwrap()
  };

  (
    child_suffix_last_name.to_string(),
    suffix_match_arm,
    child_last_name.to_string(),
    match_arm,
  )
}

fn gen_child_match_arm_neo(
  child: &OpenXmlSchemaTypeChild,
  child_choice_enum_ident: &Type,
  gen_context: &GenContextNeo,
) -> (String, Arm, String, Arm) {
  let child_type = get_or_panic!(gen_context.type_name_type_map, child.name.as_str());

  let child_namespace = get_or_panic!(gen_context.type_name_namespace_map, child.name.as_str());

  let child_last_name = &child.name[child.name.find('/').unwrap() + 1..child.name.len()];
  let child_suffix_last_name =
    &child_last_name[child_last_name.find(':').unwrap() + 1..child_last_name.len()];

  let child_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_last_name)).unwrap();

  let child_suffix_last_name_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_suffix_last_name)).unwrap();

  let child_variant_name_ident: Ident = parse_str(&child_last_name.to_upper_camel_case()).unwrap();

  let child_schema = get_or_panic!(
    gen_context.prefix_schema_map,
    child_namespace.prefix.as_str()
  );

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    &child_schema.module_name,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  let suffix_match_arm: Arm = parse2(quote! {
    #child_suffix_last_name_literal => {
      children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
        #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
      )));
    }
  })
  .unwrap();

  let match_arm: Arm = parse2(quote! {
    #child_last_name_literal => {
      children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
        #child_variant_type::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
      )));
    }
  })
  .unwrap();

  (
    child_suffix_last_name.to_string(),
    suffix_match_arm,
    child_last_name.to_string(),
    match_arm,
  )
}

fn gen_simple_child_match_arm_neo(first_name: &str, gen_context: &GenContextNeo) -> Arm {
  if let Some(e) = gen_context.enum_type_enum_map.get(first_name) {
    let enum_namespace = get_or_panic!(gen_context.enum_type_namespace_map, e.r#type.as_str());

    let enum_schema = get_or_panic!(
      gen_context.prefix_schema_map,
      enum_namespace.prefix.as_str()
    );

    let simple_type_name: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &enum_schema.module_name,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    parse2(quote! {
      quick_xml::events::Event::Text(t) => {
        use std::str::FromStr;

        xml_content = Some(#simple_type_name::from_str(&t.unescape()?)?);
      }
    })
    .unwrap()
  } else {
    let simple_type_str = simple_type_mapping(first_name);

    let enum_type: Type =
      parse_str(&format!("crate::schemas::simple_type::{}", simple_type_str)).unwrap();

    parse2(match simple_type_str {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(t.unescape()?.to_string());
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(
            match t.unescape()?.as_ref()
            {
              "true" | "1" | "True" | "TRUE" | "t" | "Yes" | "YES" | "yes" | "y" => true,
              "false" | "0" | "False" | "FALSE" | "f" | "No" | "NO" | "no" | "n" | "" => false,
              _ => Err(crate::common::SdkError::CommonError(t.unescape()?.to_string()))?,
            }
          );
        }
      },
      "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
      | "UInt64Value" | "DoubleValue" | "SingleValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          xml_content = Some(t.unescape()?.parse::<#enum_type>()?);
        }
      },
      _ => panic!("{}", simple_type_str),
    })
    .unwrap()
  }
}

fn gen_field_match_arm_neo(attr: &OpenXmlSchemaTypeAttribute, gen_context: &GenContextNeo) -> Arm {
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

  let attr_name_literal: LitByteStr = parse_str(&format!("b\"{}\"", attr_name_str)).unwrap();

  parse2(if attr.r#type.starts_with("ListValue<") {
    quote! {
      #attr_name_literal => {
        #attr_name_ident = Some(attr.unescape_value()?.into_owned());
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
        use std::str::FromStr;

        #attr_name_ident = Some(#e_type::from_str(&attr.unescape_value()?)?);
      }
    }
  } else {
    match attr.r#type.as_str() {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        #attr_name_literal => {
          #attr_name_ident = Some(attr.unescape_value()?.into_owned());
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        #attr_name_literal => {
          #attr_name_ident = Some(
            match attr
              .unescape_value()?
              .as_ref()
            {
              "true" | "1" | "True" | "TRUE" | "t" | "Yes" | "YES" | "yes" | "y" => true,
              "false" | "0" | "False" | "FALSE" | "f" | "No" | "NO" | "no" | "n" | "" => false,
              _ => Err(crate::common::SdkError::CommonError(attr.unescape_value()?.into_owned()))?,
            }
          );
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
                .unescape_value()?
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

pub fn gen_deserializer(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  let schema_namespace = context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  let scheme_mod = context
    .prefix_schema_mod_map
    .get(schema_namespace.prefix.as_str())
    .ok_or(format!("{:?}", schema_namespace.prefix))
    .unwrap();

  for e in &schema.enums {
    let enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    let mut variants: Vec<Arm> = vec![];

    for facet in &e.facets {
      let variant_rename = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(
        parse2(quote! {
          #variant_rename => Ok(Self::#variant_ident),
        })
        .unwrap(),
      )
    }

    token_stream_list.push(
      parse2(quote! {
        impl #enum_type {
          #[allow(clippy::should_implement_trait)]
          pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
            match s {
              #( #variants )*
              _ => Err(crate::common::SdkError::CommonError(s.to_string())),
            }
          }
        }
      })
      .unwrap(),
    )
  }

  let from_str_fn = gen_from_str_fn();

  let from_reader_fn = gen_from_reader_fn();

  for t in &schema.types {
    if t.is_abstract {
      continue;
    }

    let struct_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      t.class_name.to_upper_camel_case()
    ))
    .unwrap();

    let t_name_str = t.class_name.to_upper_camel_case();

    let name_list: Vec<&str> = t.name.split('/').collect();

    let rename_ser_str = name_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_list: Vec<&str> = rename_ser_str.split(':').collect();

    let rename_de_str = rename_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_ser_literal: LitByteStr = parse_str(&format!("b\"{}\"", rename_ser_str)).unwrap();

    let rename_de_literal: LitByteStr = parse_str(&format!("b\"{}\"", rename_de_str)).unwrap();

    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut attr_match_list: Vec<Arm> = vec![];
    let mut field_unwrap_list: Vec<Stmt> = vec![];
    let mut field_init_list: Vec<Ident> = vec![];
    let mut child_ser_match_list: Vec<Arm> = vec![];
    let mut child_de_match_map: HashMap<&str, Arm> = HashMap::new();
    let mut child_match_arm: Option<Arm> = None;
    let mut attributes: Vec<&OpenXmlSchemaTypeAttribute> = vec![];

    let xmlns_literal: LitByteStr =
      parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

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

      field_init_list.push(
        parse2(quote! {
          xml_content
        })
        .unwrap(),
      );

      let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

      child_match_arm = Some(gen_simple_child_match_arm(first_name, context));
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
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/picture"
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

        field_init_list.push(parse_str("xmlns").unwrap());
        field_init_list.push(parse_str("xmlns_map").unwrap());
        field_init_list.push(parse_str("mc_ignorable").unwrap());
      }

      for attr in &t.attributes {
        attributes.push(attr);
      }

      let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

      for child in &t.children {
        child_map.insert(&child.name, child);
      }

      if t.is_one_sequence_flatten() {
        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
            .unwrap();

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_name_str = if child.property_name.is_empty() {
            child_rename_ser_str
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

          field_init_list.push(child_name_ident);
        }
      } else if !t.children.is_empty() {
        field_declaration_list.push(
          parse2(quote! {
            let mut children = vec![];
          })
          .unwrap(),
        );

        field_init_list.push(
          parse2(quote! {
            children
          })
          .unwrap(),
        );
      }

      let scheme_mod = context
        .prefix_schema_mod_map
        .get(schema_namespace.prefix.as_str())
        .ok_or(format!("{:?}", schema_namespace.prefix))
        .unwrap();

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        scheme_mod,
        t.class_name.to_upper_camel_case()
      ))
      .unwrap();

      if t.is_one_sequence_flatten() {
        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
            .unwrap();

          let (ser_arm, de_arm) = gen_one_sequence_match_arm(p, child, context);

          child_ser_match_list.push(ser_arm);

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

          let child_rename_de_str = child_rename_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          child_de_match_map.insert(child_rename_de_str, de_arm);
        }
      } else {
        for child in &t.children {
          let (ser_arm, de_arm) = gen_child_match_arm(child, &child_choice_enum_type, context);

          child_ser_match_list.push(ser_arm);

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

          let child_rename_de_str = child_rename_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          child_de_match_map.insert(child_rename_de_str, de_arm);
        }
      }
    } else if t.is_derived {
      let base_class_type = context
        .type_name_type_map
        .get(&t.name[0..t.name.find('/').unwrap() + 1])
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      for attr in &t.attributes {
        attributes.push(attr);
      }

      for attr in &base_class_type.attributes {
        attributes.push(attr);
      }

      let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

      for child in &t.children {
        child_map.insert(&child.name, child);
      }

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
            .unwrap();

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_name_str = if child.property_name.is_empty() {
            child_rename_ser_str
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

          field_init_list.push(child_name_ident);
        }
      } else if !t.children.is_empty() {
        field_declaration_list.push(
          parse2(quote! {
            let mut children = vec![];
          })
          .unwrap(),
        );

        field_init_list.push(
          parse2(quote! {
            children
          })
          .unwrap(),
        );
      } else if base_class_type.base_class == "OpenXmlLeafTextElement" {
        field_declaration_list.push(
          parse2(quote! {
            let mut child = None;
          })
          .unwrap(),
        );

        field_init_list.push(
          parse2(quote! {
            child
          })
          .unwrap(),
        );
      }

      let scheme_mod = context
        .prefix_schema_mod_map
        .get(schema_namespace.prefix.as_str())
        .ok_or(format!("{:?}", schema_namespace.prefix))
        .unwrap();

      let child_choice_enum_type: Type = parse_str(&format!(
        "crate::schemas::{}::{}ChildChoice",
        scheme_mod,
        t.class_name.to_upper_camel_case()
      ))
      .unwrap();

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
            .unwrap();

          let (ser_arm, de_arm) = gen_one_sequence_match_arm(p, child, context);

          child_ser_match_list.push(ser_arm);

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

          let child_rename_de_str = child_rename_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          child_de_match_map.insert(child_rename_de_str, de_arm);
        }
      } else {
        for child in &t.children {
          let (ser_arm, de_arm) = gen_child_match_arm(child, &child_choice_enum_type, context);

          child_ser_match_list.push(ser_arm);

          let child_name_list: Vec<&str> = child.name.split('/').collect();

          let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

          let child_rename_de_str = child_rename_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

          child_de_match_map.insert(child_rename_de_str, de_arm);
        }
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let base_name_list: Vec<&str> = base_class_type.name.split('/').collect();

        let base_first_name = base_name_list
          .first()
          .ok_or(format!("{:?}", base_class_type.name))
          .unwrap();

        child_match_arm = Some(gen_simple_child_match_arm(base_first_name, context));
      }
    } else {
      panic!("{:?}", t);
    };

    let child_de_match_list: Vec<Arm> = child_de_match_map.into_values().collect();

    if !t.children.is_empty() {
      child_match_arm = Some(
        parse2(quote! {
          quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
            if with_xmlns {
              match e.name().as_ref() {
                #( #child_ser_match_list )*
                _ => Err(crate::common::SdkError::CommonError(#t_name_str.to_string()))?,
              }
            } else {
              match e.name().local_name().as_ref() {
                #( #child_de_match_list )*
                _ => Err(crate::common::SdkError::CommonError(#t_name_str.to_string()))?,
              }
            }
          }
        })
        .unwrap(),
      );
    }

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

      attr_match_list.push(gen_field_match_arm(attr, context));

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

      field_init_list.push(attr_name_ident);
    }

    let attr_match_stmt: Stmt = if (t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement")
      && (!t.part.is_empty()
        || t.base_class == "OpenXmlPartRootElement"
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/main"
        || schema_namespace.uri == "http://schemas.openxmlformats.org/drawingml/2006/picture")
    {
      parse2(quote! {
        for attr in e.attributes() {
          let attr = attr?;

          match attr.key.as_ref() {
            #( #attr_match_list )*
            b"xmlns" => xmlns = Some(
              attr.decode_and_unescape_value(xml_reader.decoder())?.to_string(),
            ),
            b"mc:Ignorable" => mc_ignorable = Some(
              attr.decode_and_unescape_value(xml_reader.decoder())?.to_string(),
            ),
            key => {
              if key.starts_with(b"xmlns:") {
                xmlns_map.insert(
                  String::from_utf8_lossy(&key[6..]).to_string(),
                  attr.decode_and_unescape_value(xml_reader.decoder())?.to_string(),
                );

                if key == #xmlns_literal {
                  with_xmlns = true;
                }
              }
            },
          }
        }
      })
      .unwrap()
    } else if attr_match_list.is_empty() {
      parse2(quote! {
        for attr in e.attributes() {
          let attr = attr?;

          if attr.key.as_ref() == #xmlns_literal {
            with_xmlns = true;
          }
        }
      })
      .unwrap()
    } else {
      parse2(quote! {
        for attr in e.attributes() {
          let attr = attr?;

          match attr.key.as_ref() {
            #( #attr_match_list )*
            #xmlns_literal => with_xmlns = true,
            _ => (),
          }
        }
      })
      .unwrap()
    };

    let deserialize_self_fn: ItemFn = parse2(quote! {
      pub fn deserialize_self<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        with_xmlns: bool,
      ) -> Result<Self, crate::common::SdkError> {
        let mut with_xmlns = with_xmlns;

        let mut empty_tag = false;

        let e = match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => e,
          quick_xml::events::Event::Empty(e) => {
            empty_tag = true;

            e
          }
          _ => Err(crate::common::SdkError::CommonError(#t_name_str.to_string()))?,
        };

        #( #field_declaration_list )*

        #attr_match_stmt

        if with_xmlns {
          if e.name().as_ref() != #rename_ser_literal {
            Err(crate::common::SdkError::MismatchError {
              expected: #rename_ser_str.to_string(),
              found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
            })?;
          }
        } else if e.name().local_name().as_ref() != #rename_de_literal {
          Err(crate::common::SdkError::MismatchError {
            expected: #rename_de_str.to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }

        if !empty_tag {
          loop {
            let peek_event = xml_reader.peek()?;

            match peek_event {
              #child_match_arm
              quick_xml::events::Event::End(e) => {
                if with_xmlns {
                  if e.name().as_ref() == #rename_ser_literal {
                    break;
                  }
                } else if e.name().local_name().as_ref() == #rename_de_literal {
                  break;
                }

                xml_reader.next()?;
              }
              quick_xml::events::Event::Eof => Err(crate::common::SdkError::UnknownError)?,
              _ => {
                xml_reader.next()?;
              },
            }
          }
        }

        #( #field_unwrap_list )*

        Ok(Self {
          #( #field_init_list, )*
        })
      }
    })
    .unwrap();

    token_stream_list.push(
      parse2(quote! {
        impl std::str::FromStr for #struct_type {
          type Err = crate::common::SdkError;

          #from_str_fn
        }
      })
      .unwrap(),
    );

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #from_reader_fn

          #deserialize_self_fn
        }
      })
      .unwrap(),
    );
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_from_str_fn() -> ItemFn {
  let token_stream = quote! {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let mut xml_reader = crate::common::from_str_inner(s)?;

      Self::deserialize_self(&mut xml_reader, false)
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

      Self::deserialize_self(&mut xml_reader, false)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_field_match_arm(attr: &OpenXmlSchemaTypeAttribute, context: &GenContext) -> Arm {
  let attr_rename_ser_str = if attr.q_name.starts_with(':') {
    &attr.q_name[1..attr.q_name.len()]
  } else {
    &attr.q_name
  };

  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let attr_rename_ser_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", attr_rename_ser_str)).unwrap();

  parse2(if attr.r#type.starts_with("ListValue<") {
    quote! {
      #attr_rename_ser_literal => {
        #attr_name_ident = Some(
          attr
            .decode_and_unescape_value(xml_reader.decoder())?
            .to_string(),
        )
      }
    }
  } else if attr.r#type.starts_with("EnumValue<") {
    let e_typed_namespace_str =
      &attr.r#type[attr.r#type.find("<").unwrap() + 1..attr.r#type.rfind(".").unwrap()];

    let enum_name = &attr.r#type[attr.r#type.rfind(".").unwrap() + 1..attr.r#type.len() - 1];

    let mut e_prefix = "";

    for typed_namespace in &context.typed_namespaces {
      if e_typed_namespace_str == typed_namespace.namespace {
        let e_schema = context
          .prefix_schema_map
          .get(typed_namespace.prefix.as_str())
          .ok_or(format!("{:?}", typed_namespace))
          .unwrap();

        for e in &e_schema.enums {
          if e.name == enum_name {
            e_prefix = &typed_namespace.prefix;
          }
        }
      }
    }

    let e_namespace = context
      .prefix_namespace_map
      .get(e_prefix)
      .ok_or(format!("{:?}", e_prefix))
      .unwrap();

    let scheme_mod = context
      .prefix_schema_mod_map
      .get(e_namespace.prefix.as_str())
      .ok_or(format!("{:?}", e_namespace.prefix))
      .unwrap();

    let e_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      enum_name.to_upper_camel_case()
    ))
    .unwrap();

    quote! {
      #attr_rename_ser_literal => {
        #attr_name_ident = Some(#e_type::from_str(
          &attr.decode_and_unescape_value(xml_reader.decoder())?,
        )?)
      }
    }
  } else {
    match attr.r#type.as_str() {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        #attr_rename_ser_literal => {
          #attr_name_ident = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          )
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        #attr_rename_ser_literal => {
          #attr_name_ident = Some(
            match attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .as_ref()
            {
              "true" | "1" | "True" | "TRUE" | "t" | "Yes" | "YES" | "yes" | "y" => true,
              "false" | "0" | "False" | "FALSE" | "f" | "No" | "NO" | "no" | "n" | "" => false,
              _ => Err(crate::common::SdkError::CommonError(attr.decode_and_unescape_value(xml_reader.decoder())?.to_string()))?,
            }
          )
        }
      },
      "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
      | "UInt64Value" | "DoubleValue" | "SingleValue" => {
        let e_type: Type =
          parse_str(&format!("crate::schemas::simple_type::{}", &attr.r#type)).unwrap();

        quote! {
          #attr_rename_ser_literal => {
            #attr_name_ident = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .parse::<#e_type>()?,
            )
          }
        }
      }
      _ => panic!("{}", attr.r#type),
    }
  })
  .unwrap()
}

fn gen_child_match_arm(
  child: &OpenXmlSchemaTypeChild,
  child_choice_enum_ident: &Type,
  context: &GenContext,
) -> (Arm, Arm) {
  let child_type = context
    .type_name_type_map
    .get(child.name.as_str())
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_namespace = context
    .type_name_namespace_map
    .get(child.name.as_str())
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_name_list: Vec<&str> = child.name.split('/').collect();

  let child_rename_ser_str = child_name_list
    .last()
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

  let child_rename_de_str = child_rename_list
    .last()
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_rename_ser_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_rename_ser_str)).unwrap();

  let child_rename_de_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_rename_de_str)).unwrap();

  let child_variant_name_ident: Ident =
    parse_str(&child_rename_ser_str.to_upper_camel_case()).unwrap();

  let scheme_mod = context
    .prefix_schema_mod_map
    .get(child_namespace.prefix.as_str())
    .ok_or(format!("{:?}", child_namespace.prefix))
    .unwrap();

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    scheme_mod,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  let ser_arm: Arm = parse2(quote! {
    #child_rename_ser_literal => {
      children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
        #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
      )));
    }
  })
  .unwrap();

  let de_arm: Arm = parse2(quote! {
    #child_rename_de_literal => {
      children.push(#child_choice_enum_ident::#child_variant_name_ident(std::boxed::Box::new(
        #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
      )));
    }
  })
  .unwrap();

  (ser_arm, de_arm)
}

fn gen_one_sequence_match_arm(
  p: &OpenXmlSchemaTypeParticle,
  child: &OpenXmlSchemaTypeChild,
  context: &GenContext,
) -> (Arm, Arm) {
  let child_type = context
    .type_name_type_map
    .get(child.name.as_str())
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_namespace = context
    .type_name_namespace_map
    .get(child.name.as_str())
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_name_ident: Ident = if child.property_name.is_empty() {
    let child_name_list: Vec<&str> = child.name.split('/').collect();

    let child_rename_ser_str = child_name_list
      .last()
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    parse_str(&child_rename_ser_str.to_snake_case()).unwrap()
  } else {
    parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap()
  };

  let child_name_list: Vec<&str> = child.name.split('/').collect();

  let child_rename_ser_str = child_name_list
    .last()
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

  let child_rename_de_str = child_rename_list
    .last()
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_rename_ser_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_rename_ser_str)).unwrap();

  let child_rename_de_literal: LitByteStr =
    parse_str(&format!("b\"{}\"", child_rename_de_str)).unwrap();

  let scheme_mod = context
    .prefix_schema_mod_map
    .get(child_namespace.prefix.as_str())
    .ok_or(format!("{:?}", child_namespace.prefix))
    .unwrap();

  let child_variant_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    scheme_mod,
    child_type.class_name.to_upper_camel_case()
  ))
  .unwrap();

  let ser_arm: Arm = if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
    parse2(quote! {
      #child_rename_ser_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        ));
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_rename_ser_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        );
      }
    })
    .unwrap()
  };

  let de_arm: Arm = if p.occurs.is_empty() || (p.occurs[0].min == 0 && p.occurs[0].max == 1) {
    parse2(quote! {
      #child_rename_de_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        ));
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_rename_de_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        );
      }
    })
    .unwrap()
  };

  (ser_arm, de_arm)
}

fn gen_simple_child_match_arm(first_name: &str, context: &GenContext) -> Arm {
  if let Some(e) = context.enum_type_enum_map.get(first_name) {
    let e_namespace = context
      .enum_type_namespace_map
      .get(e.r#type.as_str())
      .ok_or(format!("{:?}", e.r#type))
      .unwrap();

    let scheme_mod = context
      .prefix_schema_mod_map
      .get(e_namespace.prefix.as_str())
      .ok_or(format!("{:?}", e_namespace.prefix))
      .unwrap();

    let simple_type_name: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    parse2(quote! {
      quick_xml::events::Event::Text(t) => {
        xml_content = Some(#simple_type_name::from_str(&t.unescape()?)?);

        xml_reader.next()?;
      }
    })
    .unwrap()
  } else {
    let simple_type_str = simple_type_mapping(first_name);

    parse2(match simple_type_str {
      "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue"
      | "IntegerValue" | "SByteValue" | "StringValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          child = Some(t.unescape()?.to_string());

          xml_reader.next()?;
        }
      },
      "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => quote! {
        quick_xml::events::Event::Text(t) => {
          child = Some(
            match t.unescape()?.as_ref()
            {
              "true" | "1" | "True" | "TRUE" | "t" | "Yes" | "YES" | "yes" | "y" => true,
              "false" | "0" | "False" | "FALSE" | "f" | "No" | "NO" | "no" | "n" | "" => false,
              _ => Err(crate::common::SdkError::CommonError(t.unescape()?.to_string()))?,
            }
          );

          xml_reader.next()?;
        }
      },
      "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
      | "UInt64Value" | "DoubleValue" | "SingleValue" => {
        let e_type: Type =
          parse_str(&format!("crate::schemas::simple_type::{}", simple_type_str)).unwrap();

        quote! {
          quick_xml::events::Event::Text(t) => {
            child = Some(t.unescape()?.parse::<#e_type>()?);

            xml_reader.next()?;
          }
        }
      }
      _ => panic!("{}", simple_type_str),
    })
    .unwrap()
  }
}
