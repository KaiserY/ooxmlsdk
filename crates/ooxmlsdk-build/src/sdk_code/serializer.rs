use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Arm, Ident, ItemFn, ItemImpl, Stmt, Type, parse_str, parse2};

use crate::sdk_code::helpers::{
  is_composite_type, is_one_sequence_flatten, needs_xml_header, supports_xmlns_fields,
};
use crate::sdk_code::schemas::CodegenContext;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaType, SchemaTypeAttribute, SchemaTypeChild, SchemaTypeParticle,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

pub fn gen_schema_serializer(schema: &Schema, context: &CodegenContext<'_>) -> Result<TokenStream> {
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

    let (impl_block, display_impl) = gen_struct_serializer(schema, schema_type, context)?;
    token_stream_list.push(impl_block);
    token_stream_list.push(display_impl);
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn gen_enum_serializer(schema: &Schema, schema_enum: &SchemaEnum) -> Result<(ItemImpl, ItemImpl)> {
  let enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    schema.module_name,
    schema_enum.name.to_upper_camel_case()
  ))?;
  let mut variants: Vec<Arm> = vec![];

  for facet in &schema_enum.facets {
    let variant_ident: Ident = if facet.name.is_empty() {
      parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
    } else {
      parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
    };
    let variant_value = facet.value.as_str();

    variants.push(parse2(quote! {
      Self::#variant_ident => #variant_value,
    })?);
  }

  let impl_block = parse2(quote! {
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

  let (_, last_name, last_name_prefix, last_name_suffix) = split_type_name(&schema_type.name)?;
  let last_name_start_tag = format!("<{last_name}");
  let last_name_suffix_start_tag = format!("<{last_name_suffix}");
  let last_name_end_tag = format!("</{last_name}>");
  let last_name_suffix_end_tag = format!("</{last_name_suffix}>");

  let mut attr_stmts: Vec<TokenStream> = vec![];
  let mut children_writer = quote! {};
  let end_tag_writer: TokenStream;
  let end_writer: TokenStream;

  let mut attributes: Vec<&SchemaTypeAttribute> = vec![];

  if schema_type.base_class == "OpenXmlLeafTextElement" {
    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

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
  } else if schema_type.base_class == "OpenXmlLeafElement" {
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
      let mut child_map: HashMap<&str, &SchemaTypeChild> = HashMap::new();

      for child in &schema_type.children {
        child_map.insert(&child.name, child);
      }

      let mut child_stmt_list: Vec<Stmt> = vec![];

      for particle in &schema_type.particle.items {
        let child = child_map
          .get(particle.name.as_str())
          .ok_or_else(|| format!("{:?}", particle.name))?;
        child_stmt_list.push(gen_one_sequence_child_stmt(particle, child)?);
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
      let mut child_arms: Vec<Arm> = vec![];

      for child in &schema_type.children {
        child_arms.push(gen_child_arm(child, &child_choice_enum_type)?);
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
  } else if schema_type.is_derived {
    let base_class_type_name = &schema_type.name[0..schema_type.name.find('/').unwrap() + 1];
    let base_class_type = context
      .type_by_name(base_class_type_name)
      .ok_or_else(|| format!("{base_class_type_name:?}"))?;

    for attr in &schema_type.attributes {
      attributes.push(attr);
    }

    for attr in &base_class_type.attributes {
      attributes.push(attr);
    }

    if is_one_sequence_flatten(schema_type) && base_class_type.composite_type == "OneSequence" {
      let mut child_map: HashMap<&str, &SchemaTypeChild> = HashMap::new();

      for child in &schema_type.children {
        child_map.insert(&child.name, child);
      }

      let mut child_stmt_list: Vec<Stmt> = vec![];

      for particle in &schema_type.particle.items {
        let child = child_map
          .get(particle.name.as_str())
          .ok_or_else(|| format!("{:?}", particle.name))?;
        child_stmt_list.push(gen_one_sequence_child_stmt(particle, child)?);
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
    } else if !schema_type.children.is_empty() {
      let mut child_arms: Vec<Arm> = vec![];

      for child in &schema_type.children {
        child_arms.push(gen_child_arm(child, &child_choice_enum_type)?);
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
    } else if base_class_type.base_class == "OpenXmlLeafTextElement" {
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
  } else {
    return Err(format!("{schema_type:?}").into());
  }

  for attr in attributes {
    attr_stmts.push(gen_attr_stmt(attr)?);
  }

  let mut xmlns_attr_writer_list: Vec<Stmt> = vec![];
  let mut xml_header_writer: Option<Stmt> = None;

  if needs_xml_header(schema_type) {
    xml_header_writer = Some(parse2(quote! {
      writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
    })?);
  }

  if supports_xmlns_fields(schema_type, schema) {
    xmlns_attr_writer_list.push(parse2(quote! {
      if let Some(xmlns) = &self.xmlns {
        writer.write_str(r#" xmlns=""#)?;
        writer.write_str(xmlns)?;
        writer.write_char('"')?;
      }
    })?);

    xmlns_attr_writer_list.push(parse2(quote! {
      {
        let mut xmlns_entries: Vec<_> = self.xmlns_map.iter().collect();
        xmlns_entries.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
        for (k, v) in xmlns_entries {
          writer.write_str(" xmlns:")?;
          writer.write_str(k)?;
          writer.write_str("=\"")?;
          writer.write_str(v)?;
          writer.write_char('"')?;
        }
      }
    })?);

    xmlns_attr_writer_list.push(parse2(quote! {
      if let Some(mc_ignorable) = &self.mc_ignorable {
        writer.write_str(r#" mc:Ignorable=""#)?;
        writer.write_str(mc_ignorable)?;
        writer.write_char('"')?;
      }
    })?);
  }

  let xmlns_uri = schema.target_namespace.as_str();
  let xmlns_prefix = schema.prefix.as_str();

  let to_xml_fn: ItemFn = if supports_xmlns_fields(schema_type, schema) {
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

  let impl_block = parse2(quote! {
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
        #( #attr_stmts )*

        #end_tag_writer
        #children_writer
        #end_writer

        Ok(())
      }
    }
  })?;

  let display_impl = parse2(quote! {
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
  let attr_name_fmt_str = format!(" {attr_name_str}=\"");
  let required = attr
    .validators
    .iter()
    .any(|validator| validator.name == "RequiredValidator");

  Ok(if required {
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
  })
}

fn gen_one_sequence_child_stmt(
  particle: &SchemaTypeParticle,
  child: &SchemaTypeChild,
) -> Result<Stmt> {
  let child_name_ident = child_field_ident(child)?;

  if particle.occurs.is_empty() {
    Ok(parse2(quote! {
      self.#child_name_ident.write_xml(writer, xmlns_prefix)?;
    })?)
  } else if particle.occurs[0].min == 0 && particle.occurs[0].max == 1 {
    Ok(parse2(quote! {
      if let Some(#child_name_ident) = &self.#child_name_ident {
        #child_name_ident.write_xml(writer, xmlns_prefix)?;
      }
    })?)
  } else {
    Ok(parse2(quote! {
      for child in &self.#child_name_ident {
        child.write_xml(writer, xmlns_prefix)?;
      }
    })?)
  }
}

fn gen_child_arm(child: &SchemaTypeChild, child_choice_enum_type: &Type) -> Result<Arm> {
  let (_, child_last_name, _, _) = split_type_name(&child.name)?;
  let child_variant_name_ident: Ident = parse_str(&child_last_name.to_upper_camel_case())?;

  Ok(parse2(quote! {
    #child_choice_enum_type::#child_variant_name_ident(child) => child.write_xml(writer, xmlns_prefix)?,
  })?)
}

fn child_field_ident(child: &SchemaTypeChild) -> Result<Ident> {
  if child.property_name.is_empty() {
    let (_, child_last_name, _, _) = split_type_name(&child.name)?;
    Ok(parse_str(&child_last_name.to_snake_case())?)
  } else {
    Ok(parse_str(&escape_snake_case(
      child.property_name.to_snake_case(),
    ))?)
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
