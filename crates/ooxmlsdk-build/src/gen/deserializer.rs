use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{parse2, parse_str, Arm, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type};

use crate::gen::simple_type::simple_type_mapping;
use crate::models::{
  OpenXmlNamespace, OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeAttribute,
  OpenXmlSchemaTypeChild, OpenXmlSchemaTypeParticle,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::GenContext;

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

    let deserialize_self_fn = if t.base_class == "OpenXmlLeafTextElement" {
      gen_open_xml_leaf_text_element_fn(t, schema_namespace, context)
    } else if t.base_class == "OpenXmlLeafElement" {
      gen_open_xml_leaf_element_fn(t, schema_namespace, context)
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      gen_open_xml_composite_element_fn(t, schema_namespace, context)
    } else if t.is_derived {
      gen_derived_fn(t, schema_namespace, context)
    } else {
      panic!("{:?}", t);
    };

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #from_str_fn

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
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
      use crate::common::XmlReader;

      let mut xml_reader = quick_xml::Reader::from_str(s);

      xml_reader.config_mut().trim_text(true);

      let mut xml_reader = crate::common::SliceReader::new(xml_reader);

      if let quick_xml::events::Event::Decl(_) = xml_reader.peek()? {
        xml_reader.next()?;
      }

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
      use crate::common::XmlReader;

      let mut xml_reader = quick_xml::Reader::from_reader(reader);

      xml_reader.config_mut().trim_text(true);

      let mut xml_reader = crate::common::IoReader::new(xml_reader);

      if let quick_xml::events::Event::Decl(_) = xml_reader.peek()? {
        xml_reader.next()?;
      }

      Self::deserialize_self(&mut xml_reader, false)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

fn gen_open_xml_leaf_element_fn(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> ItemFn {
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

  for attr in &t.attributes {
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

  let xmlns_literal: LitByteStr =
    parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

  let attr_match_stmt: Stmt = if attr_match_list.is_empty() {
    parse2(quote! {
      for attr in e.attributes() {
        let attr = attr?;

        if attr.key.as_ref() == #xmlns_literal {
          with_xmlns = true
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

  parse2(quote! {
    pub fn deserialize_self<'de, R: crate::common::XmlReader<'de>>(
      xml_reader: &mut R,
      with_xmlns: bool,
    ) -> Result<Self, crate::common::SdkError> {
      let mut with_xmlns = with_xmlns;

      if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
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

        #( #field_unwrap_list )*

        Ok(Self {
          #( #field_init_list, )*
        })
      } else {
        Err(crate::common::SdkError::CommonError(#t_name_str.to_string()))?
      }
    }
  })
  .unwrap()
}

fn gen_open_xml_leaf_text_element_fn(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> ItemFn {
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

  for attr in &t.attributes {
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

    field_init_list.push(attr_name_ident)
  }

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

  let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

  let child_match_arm = gen_simple_child_match_arm(first_name, context);

  let xmlns_literal: LitByteStr =
    parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

  let attr_match_stmt: Stmt = if attr_match_list.is_empty() {
    parse2(quote! {
      for attr in e.attributes() {
        let attr = attr?;

        if attr.key.as_ref() == #xmlns_literal {
          with_xmlns = true
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

  parse2(quote! {
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
                  xml_reader.next()?;

                  break;
                }
              } else if e.name().local_name().as_ref() == #rename_de_literal {
                xml_reader.next()?;

                break;
              }
            }
            quick_xml::events::Event::Eof => Err(crate::common::SdkError::CommonError(#t_name_str.to_string()))?,
            _ => (),
          }
        }
      }

      #( #field_unwrap_list )*

      Ok(Self {
        #( #field_init_list, )*
      })
    }
  })
  .unwrap()
}

fn gen_open_xml_composite_element_fn(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> ItemFn {
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

  if !t.part.is_empty() || t.base_class == "OpenXmlPartRootElement" {
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
      );
    }

    field_init_list.push(attr_name_ident);
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

      if !p.occurs.is_empty() && p.occurs[0].max > 1 {
        field_declaration_list.push(
          parse2(quote! {
            let mut #child_name_ident = vec![];
          })
          .unwrap(),
        );
      } else {
        field_declaration_list.push(
          parse2(quote! {
            let mut #child_name_ident = None;
          })
          .unwrap(),
        );
      }

      if !p.occurs.is_empty() && p.occurs[0].min == 1 && p.occurs[0].max == 1 {
        field_unwrap_list.push(
          parse2(quote! {
            let #child_name_ident = #child_name_ident
              .ok_or_else(|| crate::common::SdkError::CommonError(#child_name_str.to_string()))?;
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

  let mut child_variant_name_set: HashSet<String> = HashSet::new();

  let mut child_de_match_map: HashMap<&str, Arm> = HashMap::new();

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
      if !child_variant_name_set.contains(child.name.as_str()) {
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

        child_variant_name_set.insert(child.name.clone());
      }
    }
  }

  let child_de_match_list: Vec<Arm> = child_de_match_map.into_values().collect();

  let child_match_arm: TokenStream = if t.children.is_empty() {
    quote! {}
  } else {
    quote! {
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
    }
  };

  let xmlns_literal: LitByteStr =
    parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

  let attr_match_stmt: Stmt = if !t.part.is_empty() || t.base_class == "OpenXmlPartRootElement" {
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

  parse2(quote! {
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
                  xml_reader.next()?;

                  break;
                }
              } else if e.name().local_name().as_ref() == #rename_de_literal {
                xml_reader.next()?;

                break;
              }
            }
            quick_xml::events::Event::Eof => Err(crate::common::SdkError::UnknownError)?,
            _ => (),
          }
        }
      }

      #( #field_unwrap_list )*

      Ok(Self {
        #( #field_init_list, )*
      })
    }
  })
  .unwrap()
}

fn gen_derived_fn(
  t: &OpenXmlSchemaType,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> ItemFn {
  let t_name_str = t.class_name.to_upper_camel_case();

  let base_class_type = context
    .type_name_type_map
    .get(&t.name[0..t.name.find('/').unwrap() + 1])
    .ok_or(format!("{:?}", t.base_class))
    .unwrap();

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

  let mut attributes: Vec<&OpenXmlSchemaTypeAttribute> = vec![];

  for attr in &t.attributes {
    attributes.push(attr);
  }

  for attr in &base_class_type.attributes {
    attributes.push(attr);
  }

  for attr in attributes {
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
      );
    }

    field_init_list.push(attr_name_ident);
  }

  let mut children: Vec<&OpenXmlSchemaTypeChild> = vec![];

  for child in &t.children {
    children.push(child);
  }

  for child in &base_class_type.children {
    children.push(child);
  }

  if !children.is_empty() {
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

  let mut child_variant_name_set: HashSet<&str> = HashSet::new();

  let mut child_de_match_map: HashMap<&str, Arm> = HashMap::new();

  for child in children.iter() {
    if !child_variant_name_set.contains(child.name.as_str()) {
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

      child_variant_name_set.insert(&child.name);
    }
  }

  let child_de_match_list: Vec<Arm> = child_de_match_map.into_values().collect();

  let child_match_arm: TokenStream = if children.is_empty() {
    if base_class_type.base_class == "OpenXmlLeafTextElement" {
      let base_name_list: Vec<&str> = base_class_type.name.split('/').collect();

      let base_first_name = base_name_list
        .first()
        .ok_or(format!("{:?}", base_class_type.name))
        .unwrap();

      let child_match_arm = gen_simple_child_match_arm(base_first_name, context);

      quote! {#child_match_arm}
    } else {
      quote! {}
    }
  } else {
    quote! {
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
    }
  };

  let xmlns_literal: LitByteStr =
    parse_str(&format!("b\"xmlns:{}\"", schema_namespace.prefix)).unwrap();

  let attr_match_stmt: Stmt = if attr_match_list.is_empty() {
    parse2(quote! {
      for attr in e.attributes() {
        let attr = attr?;

        if attr.key.as_ref() == #xmlns_literal {
          with_xmlns = true
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

  parse2(quote! {
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
                  xml_reader.next()?;

                  break;
                }
              } else if e.name().local_name().as_ref() == #rename_de_literal {
                xml_reader.next()?;

                break;
              }
            }
            quick_xml::events::Event::Eof => Err(crate::common::SdkError::UnknownError)?,
            _ => (),
          }
        }
      }

      #( #field_unwrap_list )*

      Ok(Self {
        #( #field_init_list, )*
      })
    }
  })
  .unwrap()
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

  let ser_arm: Arm = if !p.occurs.is_empty() && p.occurs[0].max > 1 {
    parse2(quote! {
      #child_rename_ser_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        );
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_rename_ser_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        ));
      }
    })
    .unwrap()
  };

  let de_arm: Arm = if !p.occurs.is_empty() && p.occurs[0].max > 1 {
    parse2(quote! {
      #child_rename_de_literal => {
        #child_name_ident.push(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        );
      }
    })
    .unwrap()
  } else {
    parse2(quote! {
      #child_rename_de_literal => {
        #child_name_ident = Some(std::boxed::Box::new(
          #child_variant_type::deserialize_self(xml_reader, with_xmlns)?,
        ));
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
        child = Some(#simple_type_name::from_str(&t.unescape()?)?);

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
