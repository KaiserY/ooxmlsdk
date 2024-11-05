use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse2, parse_str, Arm, Ident, ItemFn, ItemImpl, Stmt, Type};

use crate::models::{OpenXmlSchema, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::GenContext;

pub fn gen_serializer(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
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
           Self::#variant_ident => #variant_rename.to_string(),
        })
        .unwrap(),
      )
    }

    token_stream_list.push(
      parse2(quote! {
        impl #enum_type {
          #[allow(clippy::inherent_to_string)]
          pub fn to_string(&self) -> String {
            match self {
              #( #variants )*
            }
          }
        }
      })
      .unwrap(),
    )
  }

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

    let child_choice_enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}ChildChoice",
      scheme_mod,
      t.class_name.to_upper_camel_case()
    ))
    .unwrap();

    let name_list: Vec<&str> = t.name.split('/').collect();

    let rename_ser_str = name_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_list: Vec<&str> = rename_ser_str.split(':').collect();

    let name_prefix = rename_list.first().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_de_str = rename_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let end_tag_writer;

    let end_writer;

    let mut variants: Vec<TokenStream> = vec![];

    let mut children_writer = quote! {};

    let mut child_arms: Vec<Arm> = vec![];

    for attr in &t.attributes {
      variants.push(gen_attr(attr));
    }

    if t.base_class == "OpenXmlLeafTextElement" {
      children_writer = quote! {
        if let Some(child) = &self.child {
          writer.write_str(&quick_xml::escape::escape(child.to_string()))?;
        }
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        writer.write_str("</")?;

        if xmlns_prefix == #name_prefix {
          writer.write_str(#rename_de_str)?;
        } else {
          writer.write_str(#rename_ser_str)?;
        }

        writer.write_char('>')?;
      };
    } else if t.base_class == "OpenXmlLeafElement" {
      children_writer = quote! {};

      end_tag_writer = quote! {};

      end_writer = quote! {
        writer.write_str("/>")?;
      };
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      if t.children.is_empty() {
        end_tag_writer = quote! {};

        end_writer = quote! {
          writer.write_str("/>")?;
        };
      } else if t.is_one_sequence_flatten() {
        let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

        for child in &t.children {
          child_map.insert(&child.name, child);
        }

        let mut child_stmt_list: Vec<Stmt> = vec![];

        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
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

          if p.occurs.is_empty() {
            child_stmt_list.push(
              parse2(quote! {
                writer.write_str(&self.#child_name_ident.to_string_inner(xmlns_prefix)?)?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            child_stmt_list.push(
              parse2(quote! {
                if let Some(#child_name_ident) = &self.#child_name_ident {
                  writer.write_str(&#child_name_ident.to_string_inner(xmlns_prefix)?)?;
                }
              })
              .unwrap(),
            );
          } else {
            child_stmt_list.push(
              parse2(quote! {
                for child in &self.#child_name_ident {
                  writer.write_str(&child.to_string_inner(xmlns_prefix)?)?;
                }
              })
              .unwrap(),
            );
          }
        }

        children_writer = quote! {
          #( #child_stmt_list )*
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          writer.write_str("</")?;

          if xmlns_prefix == #name_prefix {
            writer.write_str(#rename_de_str)?;
          } else {
            writer.write_str(#rename_ser_str)?;
          }

          writer.write_char('>')?;
        };
      } else {
        for child in &t.children {
          child_arms.push(gen_child_arm(child, &child_choice_enum_type));
        }

        children_writer = quote! {
          for child in &self.children {
            let child_str = match child {
              #( #child_arms )*
            };

            writer.write_str(&child_str)?;
          }
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          writer.write_str("</")?;

          if xmlns_prefix == #name_prefix {
            writer.write_str(#rename_de_str)?;
          } else {
            writer.write_str(#rename_ser_str)?;
          }

          writer.write_char('>')?;
        };
      }
    } else if t.is_derived {
      let base_class_type = context
        .type_name_type_map
        .get(&t.name[0..t.name.find('/').unwrap() + 1])
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      for attr in &base_class_type.attributes {
        variants.push(gen_attr(attr));
      }

      let mut children_map: HashMap<&str, OpenXmlSchemaTypeChild> = HashMap::new();

      for c in &t.children {
        children_map.insert(&c.name, c.clone());
      }

      for c in &base_class_type.children {
        children_map.insert(&c.name, c.clone());
      }

      let children: Vec<OpenXmlSchemaTypeChild> = children_map.into_values().collect();

      for child in &children {
        child_arms.push(gen_child_arm(child, &child_choice_enum_type));
      }

      if children.is_empty() {
        if base_class_type.base_class == "OpenXmlLeafTextElement" {
          children_writer = quote! {
            if let Some(child) = &self.child {
              writer.write_str(&quick_xml::escape::escape(child.to_string()))?;
            }
          };

          end_tag_writer = quote! {
            writer.write_char('>')?;
          };

          end_writer = quote! {
            writer.write_str("</")?;

            if xmlns_prefix == #name_prefix {
              writer.write_str(#rename_de_str)?;
            } else {
              writer.write_str(#rename_ser_str)?;
            }

            writer.write_char('>')?;
          };
        } else {
          end_tag_writer = quote! {};

          end_writer = quote! {
            writer.write_str("/>")?;
          };
        }
      } else if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
        let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

        for child in &t.children {
          child_map.insert(&child.name, child);
        }

        let mut child_stmt_list: Vec<Stmt> = vec![];

        for p in &t.particle.items {
          let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
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

          if p.occurs.is_empty() {
            child_stmt_list.push(
              parse2(quote! {
                writer.write_str(&self.#child_name_ident.to_string_inner(xmlns_prefix)?)?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            child_stmt_list.push(
              parse2(quote! {
                if let Some(#child_name_ident) = &self.#child_name_ident {
                  writer.write_str(&#child_name_ident.to_string_inner(xmlns_prefix)?)?;
                }
              })
              .unwrap(),
            );
          } else {
            child_stmt_list.push(
              parse2(quote! {
                for child in &self.#child_name_ident {
                  writer.write_str(&child.to_string_inner(xmlns_prefix)?)?;
                }
              })
              .unwrap(),
            );
          }
        }

        children_writer = quote! {
          #( #child_stmt_list )*
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          writer.write_str("</")?;

          if xmlns_prefix == #name_prefix {
            writer.write_str(#rename_de_str)?;
          } else {
            writer.write_str(#rename_ser_str)?;
          }

          writer.write_char('>')?;
        };
      } else {
        children_writer = quote! {
          for child in &self.children {
            let child_str = match child {
              #( #child_arms )*
            };

            writer.write_str(&child_str)?;
          }
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          writer.write_str("</")?;

          if xmlns_prefix == #name_prefix {
            writer.write_str(#rename_de_str)?;
          } else {
            writer.write_str(#rename_ser_str)?;
          }

          writer.write_char('>')?;
        };
      }
    } else {
      panic!("{:?}", t);
    };

    let attr_writer = quote! {
      #( #variants )*
    };

    let mut xmlns_attr_writer_list: Vec<Stmt> = vec![];

    let mut xml_header_writer: Option<Stmt> = None;

    if !t.part.is_empty() || t.base_class == "OpenXmlPartRootElement" {
      xml_header_writer = Some(
        parse2(quote! {
          writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;
        })
        .unwrap(),
      );
    }

    if !t.part.is_empty()
      || t.base_class == "OpenXmlPartRootElement"
      || ((t.base_class == "OpenXmlCompositeElement"
        || t.base_class == "CustomXmlElement"
        || t.base_class == "OpenXmlPartRootElement"
        || t.base_class == "SdtElement")
        && (schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
          || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"))
    {
      xmlns_attr_writer_list.push(
        parse2(quote! {
          if let Some(xmlns) = &self.xmlns {
            writer.write_str(r#" xmlns=""#)?;
            writer.write_str(xmlns)?;
            writer.write_str("\"")?;
          }
        })
        .unwrap(),
      );

      xmlns_attr_writer_list.push(
        parse2(quote! {
          for (k, v) in &self.xmlns_map {
            writer.write_str(" xmlns:")?;
            writer.write_str(k)?;
            writer.write_str("=\"")?;
            writer.write_str(v)?;
            writer.write_str("\"")?;
          }
        })
        .unwrap(),
      );

      xmlns_attr_writer_list.push(
        parse2(quote! {
          if let Some(mc_ignorable) = &self.mc_ignorable {
            writer.write_str(r#" mc:Ignorable=""#)?;
            writer.write_str(mc_ignorable)?;
            writer.write_str("\"")?;
          }
        })
        .unwrap(),
      );
    }

    let xmlns_uri_str = &schema_namespace.uri;

    let xmlns_prefix_str = &schema_namespace.prefix;

    let to_string_fn: ItemFn = if !t.part.is_empty()
      || t.base_class == "OpenXmlPartRootElement"
      || ((t.base_class == "OpenXmlCompositeElement"
        || t.base_class == "CustomXmlElement"
        || t.base_class == "OpenXmlPartRootElement"
        || t.base_class == "SdtElement")
        && (schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
          || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"))
    {
      parse2(quote! {
        #[allow(clippy::inherent_to_string)]
        pub fn to_string(&self) -> Result<String, crate::common::SdkError> {
          self.to_string_inner(
            if let Some(xmlns) = &self.xmlns {
              if xmlns == #xmlns_uri_str {
                #xmlns_prefix_str
              } else {
                ""
              }
            } else {
              ""
            }
          )
        }
      })
      .unwrap()
    } else {
      parse2(quote! {
        #[allow(clippy::inherent_to_string)]
        pub fn to_string(&self) -> Result<String, crate::common::SdkError> {
          self.to_string_inner("")
        }
      })
      .unwrap()
    };

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #to_string_fn

          pub fn to_string_inner(&self, xmlns_prefix: &str) -> Result<String, crate::common::SdkError> {
            use std::fmt::Write;

            let mut writer = String::new();

            #xml_header_writer

            writer.write_char('<')?;

            if xmlns_prefix == #name_prefix {
              writer.write_str(#rename_de_str)?;
            } else {
              writer.write_str(#rename_ser_str)?;
            }

            #( #xmlns_attr_writer_list )*

            #attr_writer

            #end_tag_writer

            #children_writer

            #end_writer

            Ok(writer)
          }
        }
      })
      .unwrap(),
    )
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_attr(attr: &OpenXmlSchemaTypeAttribute) -> TokenStream {
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

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  if required {
    quote! {
      writer.write_char(' ')?;
      writer.write_str(#attr_rename_ser_str)?;
      writer.write_str("=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.#attr_name_ident.to_string()))?;
      writer.write_char('"')?;
    }
  } else {
    quote! {
      if let Some(#attr_name_ident) = &self.#attr_name_ident {
        writer.write_char(' ')?;
        writer.write_str(#attr_rename_ser_str)?;
        writer.write_str("=\"")?;
        writer.write_str(&quick_xml::escape::escape(#attr_name_ident.to_string()))?;
        writer.write_char('"')?;
      }
    }
  }
}

fn gen_child_arm(child: &OpenXmlSchemaTypeChild, child_choice_enum_type: &Type) -> Arm {
  let child_name_list: Vec<&str> = child.name.split('/').collect();

  let child_rename_ser_str = child_name_list
    .last()
    .ok_or(format!("{:?}", child.name))
    .unwrap();

  let child_variant_name_ident: Ident =
    parse_str(&child_rename_ser_str.to_upper_camel_case()).unwrap();

  parse2(quote! {
    #child_choice_enum_type::#child_variant_name_ident(child) => child.to_string_inner(xmlns_prefix)?,
  })
  .unwrap()
}
