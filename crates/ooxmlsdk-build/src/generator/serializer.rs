use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Arm, Ident, ItemFn, ItemImpl, Stmt, Type, parse_str, parse2};

use crate::GenContext;
use crate::models::{OpenXmlSchema, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild};
use crate::utils::{escape_snake_case, escape_upper_camel_case, get_or_panic};

pub fn gen_serializer(schema: &OpenXmlSchema, gen_context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  let schema_namespace = gen_context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  for e in &schema.enums {
    let enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    let mut variants: Vec<Arm> = vec![];

    for facet in &e.facets {
      let variant_value = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(
        parse2(quote! {
          Self::#variant_ident => #variant_value.to_string(),
        })
        .unwrap(),
      );
    }

    token_stream_list.push(
      parse2(quote! {
        impl #enum_type {
          pub fn to_xml(&self) -> String {
            match self {
              #( #variants )*
            }
          }
        }
      })
      .unwrap(),
    );

    token_stream_list.push(
      parse2(quote! {
        impl std::fmt::Display for #enum_type {
          fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_xml())
          }
        }
      })
      .unwrap(),
    );
  }

  for t in &schema.types {
    if t.is_abstract {
      continue;
    }

    let struct_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      &schema.module_name,
      t.class_name.to_upper_camel_case()
    ))
    .unwrap();

    let child_choice_enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}ChildChoice",
      &schema.module_name,
      t.class_name.to_upper_camel_case()
    ))
    .unwrap();

    let last_name = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];
    let last_name_prefix = &last_name[0..last_name.find(':').unwrap()];
    let last_name_suffix = &last_name[last_name.find(':').unwrap() + 1..last_name.len()];

    let last_name_start_tag = format!("<{last_name}");
    let last_name_suffix_start_tag = format!("<{last_name_suffix}");

    let last_name_end_tag = format!("</{last_name}>");
    let last_name_suffix_end_tag = format!("</{last_name_suffix}>");

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
        if let Some(xml_content) = &self.xml_content {
          writer.write_str(&quick_xml::escape::escape(xml_content.to_string()))?;
        }
      };

      end_tag_writer = quote! {
        writer.write_char('>')?;
      };

      end_writer = quote! {
        if xmlns_prefix == #last_name_prefix {
          writer.write_str(#last_name_suffix_end_tag)?;
        } else {
          writer.write_str(#last_name_end_tag)?;
        }
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
                self.#child_name_ident.write_xml(writer, xmlns_prefix)?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            child_stmt_list.push(
              parse2(quote! {
                if let Some(#child_name_ident) = &self.#child_name_ident {
                  #child_name_ident.write_xml(writer, xmlns_prefix)?;
                }
              })
              .unwrap(),
            );
          } else {
            child_stmt_list.push(
              parse2(quote! {
                for child in &self.#child_name_ident {
                  child.write_xml(writer, xmlns_prefix)?;
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
          if xmlns_prefix == #last_name_prefix {
            writer.write_str(#last_name_suffix_end_tag)?;
          } else {
            writer.write_str(#last_name_end_tag)?;
          }
        };
      } else {
        for child in &t.children {
          child_arms.push(gen_child_arm(child, &child_choice_enum_type));
        }

        children_writer = quote! {
          for child in &self.children {
            match child {
              #( #child_arms )*
            };
          }
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          if xmlns_prefix == #last_name_prefix {
            writer.write_str(#last_name_suffix_end_tag)?;
          } else {
            writer.write_str(#last_name_end_tag)?;
          }
        };
      }
    } else if t.is_derived {
      let base_class_type = get_or_panic!(
        gen_context.type_name_type_map,
        &t.name[0..t.name.find('/').unwrap() + 1]
      );

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
            if let Some(xml_content) = &self.xml_content {
              writer.write_str(&quick_xml::escape::escape(xml_content.to_string()))?;
            }
          };

          end_tag_writer = quote! {
            writer.write_char('>')?;
          };

          end_writer = quote! {
            if xmlns_prefix == #last_name_prefix {
              writer.write_str(#last_name_suffix_end_tag)?;
            } else {
              writer.write_str(#last_name_end_tag)?;
            }
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
                self.#child_name_ident.write_xml(writer, xmlns_prefix)?;
              })
              .unwrap(),
            );
          } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            child_stmt_list.push(
              parse2(quote! {
                if let Some(#child_name_ident) = &self.#child_name_ident {
                  #child_name_ident.write_xml(writer, xmlns_prefix)?;
                }
              })
              .unwrap(),
            );
          } else {
            child_stmt_list.push(
              parse2(quote! {
                for child in &self.#child_name_ident {
                  child.write_xml(writer, xmlns_prefix)?;
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
          if xmlns_prefix == #last_name_prefix {
            writer.write_str(#last_name_suffix_end_tag)?;
          } else {
            writer.write_str(#last_name_end_tag)?;
          }
        };
      } else {
        children_writer = quote! {
          for child in &self.children {
            match child {
              #( #child_arms )*
            };
          }
        };

        end_tag_writer = quote! {
          writer.write_char('>')?;
        };

        end_writer = quote! {
          if xmlns_prefix == #last_name_prefix {
            writer.write_str(#last_name_suffix_end_tag)?;
          } else {
            writer.write_str(#last_name_end_tag)?;
          }
        };
      }
    } else {
      panic!("{t:?}");
    };

    let attr_writer = quote! {
      #( #variants )*
    };

    let mut xmlns_attr_writer_list: Vec<Stmt> = vec![];

    let mut xml_header_writer: Option<Stmt> = None;

    if !t.part.is_empty() || t.base_class == "OpenXmlPartRootElement" {
      xml_header_writer = Some(
        parse2(quote! {
          writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
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

    let to_xml_fn: ItemFn = if !t.part.is_empty()
      || t.base_class == "OpenXmlPartRootElement"
      || ((t.base_class == "OpenXmlCompositeElement"
        || t.base_class == "CustomXmlElement"
        || t.base_class == "OpenXmlPartRootElement"
        || t.base_class == "SdtElement")
        && (schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
          || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"))
    {
      parse2(quote! {
        pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
          let mut writer = String::with_capacity(32);

          self.write_xml(
            &mut writer,
            if let Some(xmlns) = &self.xmlns {
              if xmlns == #xmlns_uri_str {
                #xmlns_prefix_str
              } else {
                ""
              }
            } else {
              ""
            },
          )?;

          Ok(writer)
        }
      })
      .unwrap()
    } else {
      parse2(quote! {
        pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
          let mut writer = String::with_capacity(32);

          self.write_xml(&mut writer, "")?;

          Ok(writer)
        }
      })
      .unwrap()
    };

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #to_xml_fn

          pub(crate) fn write_xml<W: std::fmt::Write>(
            &self,
            writer: &mut W,
            xmlns_prefix: &str,
          ) -> Result<(), std::fmt::Error> {
            #xml_header_writer

            if xmlns_prefix == #last_name_prefix {
              writer.write_str(#last_name_suffix_start_tag)?;
            } else {
              writer.write_str(#last_name_start_tag)?;
            }

            #( #xmlns_attr_writer_list )*

            #attr_writer

            #end_tag_writer

            #children_writer

            #end_writer

            Ok(())
          }
        }
      })
      .unwrap(),
    );

    token_stream_list.push(
      parse2(quote! {
        impl std::fmt::Display for #struct_type {
          fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_xml()?)
          }
        }
      })
      .unwrap(),
    );
  }

  quote! {
    #( #token_stream_list )*
  }
}

fn gen_attr(attr: &OpenXmlSchemaTypeAttribute) -> TokenStream {
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

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  let attr_name_fmt_str = format!(" {attr_name_str}=\"");

  if required {
    quote! {
      writer.write_str(#attr_name_fmt_str)?;
      writer.write_str(&quick_xml::escape::escape(self.#attr_name_ident.to_string()))?;
      writer.write_char('"')?;
    }
  } else {
    quote! {
      if let Some(#attr_name_ident) = &self.#attr_name_ident {
        writer.write_str(#attr_name_fmt_str)?;
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
    #child_choice_enum_type::#child_variant_name_ident(child) => child.write_xml(writer, xmlns_prefix)?,
  })
  .unwrap()
}
