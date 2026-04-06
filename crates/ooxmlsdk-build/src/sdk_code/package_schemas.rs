use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
  Arm, Expr, Ident, ImplItemFn, ItemImpl, ItemStruct, LitByteStr, LitStr, Stmt, Type, parse_str,
  parse2,
};

use crate::Result;
use crate::sdk_data::sdk_data_model::{
  PackageAttribute, PackageChildFieldKind, PackageEnum, PackageFixedAttribute, PackageSchema,
  PackageTextChild, PackageType, PackageTypeKind, PackageXmlHeader,
};
use crate::utils::escape_snake_case;

pub fn gen_package_schema(schema: &PackageSchema) -> Result<TokenStream> {
  let mut token_stream_list = vec![];

  for package_enum in &schema.enums {
    token_stream_list.push(gen_package_enum(package_enum)?);
  }

  for package_type in &schema.types {
    if let Some(choice_enum) = gen_choice_enum(schema, package_type)? {
      token_stream_list.push(choice_enum);
    }

    token_stream_list.push(gen_package_type(schema, package_type)?);
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn gen_package_enum(package_enum: &PackageEnum) -> Result<TokenStream> {
  let enum_ident: Ident = parse_str(&package_enum.name.to_upper_camel_case())?;
  let mut variants: Vec<TokenStream> = vec![];

  for variant in &package_enum.variants {
    let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
    let variant_value = variant.value.as_str();
    let variant_value_lit = LitStr::new(variant_value, proc_macro2::Span::call_site());

    if variant.is_default {
      variants.push(quote! {
        #[sdk(rename = #variant_value_lit)]
        #[default]
        #variant_ident
      });
    } else {
      variants.push(quote! {
        #[sdk(rename = #variant_value_lit)]
        #variant_ident
      });
    }
  }

  Ok(quote! {
    #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
    pub enum #enum_ident {
      #( #variants, )*
    }
  })
}

fn gen_choice_enum(
  schema: &PackageSchema,
  package_type: &PackageType,
) -> Result<Option<TokenStream>> {
  let Some(choice_field) = package_type
    .child_fields
    .iter()
    .find(|field| field.kind == PackageChildFieldKind::ChoiceVec)
  else {
    return Ok(None);
  };

  let enum_ident: Ident = parse_str(&choice_field.enum_name)?;
  let mut variants = vec![];

  for variant in &choice_field.variants {
    let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
    let variant_type: Type = parse_str(&variant.r#type.to_upper_camel_case())?;
    let variant_schema_type = find_package_type(schema, &variant.r#type)?;
    let q_name = package_prefixed_tag(&variant_schema_type.prefix, &variant_schema_type.tag);
    let q_name_lit = LitStr::new(&q_name, proc_macro2::Span::call_site());
    variants.push(quote! {
      #[sdk(child(qname = #q_name_lit))]
      #variant_ident(std::boxed::Box<#variant_type>),
    });
  }

  Ok(Some(quote! {
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
    pub enum #enum_ident {
      #( #variants )*
    }
  }))
}

fn gen_package_type(schema: &PackageSchema, package_type: &PackageType) -> Result<TokenStream> {
  let struct_ident: Ident = parse_str(&package_type.name.to_upper_camel_case())?;
  let local_tag = package_type.tag.as_str();
  let local_tag_literal: LitByteStr = parse_str(&format!("b\"{local_tag}\""))?;

  let fields = gen_type_fields(package_type)?;
  let struct_item: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, Default)]
    pub struct #struct_ident {
      #( #fields )*
    }
  })?;

  let from_str_impl: ItemImpl = parse2(quote! {
    impl std::str::FromStr for #struct_ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }
    }
  })?;

  let from_reader_fn: ImplItemFn = parse2(quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;
      Self::deserialize_inner(&mut xml_reader, None)
    }
  })?;

  let deserialize_inner_fn = gen_deserialize_inner_fn(schema, package_type, &local_tag_literal)?;
  let serializer_impl = gen_package_type_serializer(schema, package_type)?;

  let impl_block: ItemImpl = parse2(quote! {
    impl #struct_ident {
      #from_reader_fn
      #deserialize_inner_fn
    }
  })?;

  Ok(quote! {
    #struct_item
    #from_str_impl
    #impl_block
    #serializer_impl
  })
}

fn gen_type_fields(package_type: &PackageType) -> Result<Vec<TokenStream>> {
  let mut fields = vec![];

  if package_type.has_xmlns_fields {
    fields.push(quote! { pub xmlns: Option<String>, });
    fields.push(quote! { pub xmlns_map: std::collections::HashMap<String, String>, });
    fields.push(quote! { pub mc_ignorable: Option<String>, });
  }

  for attr in &package_type.attributes {
    let field_ident: Ident = parse_str(&escape_snake_case(attr.field.clone()))?;
    let field_ty = attr_type(attr)?;
    fields.push(quote! {
      pub #field_ident: #field_ty,
    });
  }

  for child in &package_type.text_children {
    let field_ident: Ident = parse_str(&escape_snake_case(child.field.clone()))?;
    fields.push(quote! {
      pub #field_ident: Option<String>,
    });
  }

  for child_field in &package_type.child_fields {
    let field_ident: Ident = parse_str(&escape_snake_case(child_field.field.clone()))?;
    match child_field.kind {
      PackageChildFieldKind::Vec => {
        let item_type: Type = parse_str(&child_field.item_type.to_upper_camel_case())?;
        fields.push(quote! {
          pub #field_ident: Vec<#item_type>,
        });
      }
      PackageChildFieldKind::ChoiceVec => {
        let enum_type: Type = parse_str(&child_field.enum_name)?;
        fields.push(quote! {
          pub #field_ident: Vec<#enum_type>,
        });
      }
    }
  }

  Ok(fields)
}

fn gen_deserialize_inner_fn(
  schema: &PackageSchema,
  package_type: &PackageType,
  local_tag_literal: &LitByteStr,
) -> Result<ImplItemFn> {
  let mut declarations = vec![];
  let mut attr_arms: Vec<Arm> = vec![];
  let mut build_fields = vec![];
  let mut child_arms: Vec<Arm> = vec![];
  let prefixed_tag = package_prefixed_tag(&package_type.prefix, &package_type.tag);
  let prefixed_tag_literal: LitByteStr = parse_str(&format!("b\"{prefixed_tag}\""))?;

  if package_type.has_xmlns_fields {
    declarations.push(quote! { let mut xmlns = None; });
    declarations
      .push(quote! { let mut xmlns_map = std::collections::HashMap::<String, String>::new(); });
    declarations.push(quote! { let mut mc_ignorable = None; });
    build_fields.push(quote! { xmlns });
    build_fields.push(quote! { xmlns_map });
    build_fields.push(quote! { mc_ignorable });
  }

  for attr in &package_type.attributes {
    let field_ident: Ident = parse_str(&escape_snake_case(attr.field.clone()))?;
    declarations.push(quote! { let mut #field_ident = None; });
    let q_name = attr.q_name.as_str();
    let q_name_literal: LitByteStr = parse_str(&format!("b\"{q_name}\""))?;
    let assign_expr = attr_parse_expr(package_type, attr)?;
    attr_arms.push(parse2(quote! {
      #q_name_literal => {
        #field_ident = Some(#assign_expr);
      }
    })?);
    build_fields.push(attr_build_expr(attr)?);
  }

  for child in &package_type.text_children {
    let field_ident: Ident = parse_str(&escape_snake_case(child.field.clone()))?;
    declarations.push(quote! { let mut #field_ident = None; });
    build_fields.push(quote! { #field_ident });
    child_arms.push(gen_text_child_arm(child)?);
  }

  for child_field in &package_type.child_fields {
    let field_ident: Ident = parse_str(&escape_snake_case(child_field.field.clone()))?;
    declarations.push(quote! { let mut #field_ident = vec![]; });
    build_fields.push(quote! { #field_ident });

    match child_field.kind {
      PackageChildFieldKind::Vec => {
        let child_type: Type = parse_str(&child_field.item_type.to_upper_camel_case())?;
        let child_schema_type = find_package_type(schema, &child_field.item_type)?;
        let child_type_name = child_schema_type.tag.as_str();
        let child_prefixed = package_prefixed_tag(&child_schema_type.prefix, child_type_name);
        let child_prefixed_literal: LitByteStr = parse_str(&format!("b\"{child_prefixed}\""))?;
        let child_local_literal: LitByteStr = parse_str(&format!("b\"{child_type_name}\""))?;

        child_arms.push(parse2(quote! {
          #child_prefixed_literal | #child_local_literal => {
            #field_ident.push(#child_type::deserialize_inner(xml_reader, Some((e, e_empty)))?);
          }
        })?);
      }
      PackageChildFieldKind::ChoiceVec => {
        let enum_ident: Ident = parse_str(&child_field.enum_name)?;
        for variant in &child_field.variants {
          let variant_schema_type = find_package_type(schema, &variant.r#type)?;
          let tag = variant_schema_type.tag.as_str();
          let prefixed_tag = package_prefixed_tag(&variant_schema_type.prefix, tag);
          let prefixed_tag_literal: LitByteStr = parse_str(&format!("b\"{prefixed_tag}\""))?;
          let local_tag_literal: LitByteStr = parse_str(&format!("b\"{tag}\""))?;
          child_arms.push(parse2(quote! {
            #prefixed_tag_literal | #local_tag_literal => {
              #field_ident.push(#enum_ident::deserialize_inner(xml_reader, Some((e, e_empty)))?);
            }
          })?);
        }
      }
    }
  }

  let local_tag = package_type.tag.as_str();
  let child_match = if package_type.kind == PackageTypeKind::Composite
    && (!package_type.text_children.is_empty() || !package_type.child_fields.is_empty())
  {
    quote! {
      if !_empty_tag {
        loop {
          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              let e_empty = false;
              match e.name().as_ref() {
                #( #child_arms )*
                _ => Err(crate::common::SdkError::CommonError(#local_tag.to_string()))?,
              }
            }
            quick_xml::events::Event::Empty(e) => {
              let e_empty = true;
              match e.name().as_ref() {
                #( #child_arms )*
                _ => Err(crate::common::SdkError::CommonError(#local_tag.to_string()))?,
              }
            }
            quick_xml::events::Event::End(e) => match e.name().as_ref() {
              #prefixed_tag_literal | #local_tag_literal => break,
              _ => (),
            },
            quick_xml::events::Event::Eof => Err(crate::common::SdkError::UnknownError)?,
            _ => (),
          }
        }
      }
    }
  } else {
    quote! {}
  };

  let attr_match = if package_type.has_xmlns_fields && attr_arms.is_empty() {
    quote! {
      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
        }
        key if key.starts_with(b"xmlns:") => {
          xmlns_map.insert(
            String::from_utf8_lossy(&key[6..]).to_string(),
            attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned(),
          );
        }
        _ => {}
      }
    }
  } else if package_type.has_xmlns_fields {
    quote! {
      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned());
        }
        key if key.starts_with(b"xmlns:") => {
          xmlns_map.insert(
            String::from_utf8_lossy(&key[6..]).to_string(),
            attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned(),
          );
        }
        #( #attr_arms )*
        _ => {}
      }
    }
  } else if attr_arms.is_empty() {
    quote! {
      let _ = attr;
    }
  } else {
    quote! {
      match attr.key.as_ref() {
        #( #attr_arms )*
        _ => {}
      }
    }
  };

  parse2(quote! {
    pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
      xml_reader: &mut R,
      xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    ) -> Result<Self, crate::common::SdkError> {
      let (e, _empty_tag) =
        crate::common::expect_event_start!(xml_reader, xml_event, #prefixed_tag_literal, #local_tag_literal);

      #( #declarations )*

      for attr in e.attributes().with_checks(false) {
        let attr = attr?;

        #attr_match
      }

      #child_match

      Ok(Self {
        #( #build_fields, )*
      })
    }
  })
  .map_err(Into::into)
}

fn gen_package_type_serializer(
  schema: &PackageSchema,
  package_type: &PackageType,
) -> Result<TokenStream> {
  let struct_ident: Ident = parse_str(&package_type.name.to_upper_camel_case())?;
  let root_default_namespace_prefix = gen_root_default_namespace_prefix(schema, package_type);
  let header = gen_root_xml_header(schema, package_type);
  let tag_prefix = package_type.prefix.as_str();
  let local_tag = package_type.tag.as_str();
  let is_leaf = package_type.kind == PackageTypeKind::Leaf;

  let attr_writers = gen_attr_writers(package_type)?;
  let child_writers = gen_child_writers(package_type)?;
  let write_body = if is_leaf {
    quote! {
      writer.write_str("/>")?;
      Ok(())
    }
  } else {
    quote! {
      writer.write_char('>')?;

      #( #child_writers )*

      crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_tag)?;
      Ok(())
    }
  };
  let to_xml_fn = if package_type.name == schema.root {
    quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, #root_default_namespace_prefix)?;
        Ok(writer)
      }
    }
  } else {
    quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, #tag_prefix)?;
        Ok(writer)
      }
    }
  };

  let serializer_impl: ItemImpl = parse2(quote! {
    impl #struct_ident {
      #to_xml_fn

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
        #header

        crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_tag)?;

        #( #attr_writers )*

        #write_body
      }
    }
  })?;

  Ok(quote! {
    #serializer_impl
  })
}

fn gen_root_default_namespace_prefix(
  schema: &PackageSchema,
  package_type: &PackageType,
) -> TokenStream {
  let prefix_lit = LitStr::new(&package_type.prefix, proc_macro2::Span::call_site());
  if package_type.name == schema.root {
    let xmlns_uri = schema.xmlns_uri.as_str();
    let xmlns_uri_lit = LitStr::new(xmlns_uri, proc_macro2::Span::call_site());
    quote! {
      if let Some(xmlns) = &self.xmlns {
        if xmlns != #xmlns_uri_lit {
          ""
        } else {
          #prefix_lit
        }
      } else {
        ""
      }
    }
  } else {
    quote! { #prefix_lit }
  }
}

fn gen_root_xml_header(schema: &PackageSchema, package_type: &PackageType) -> TokenStream {
  if package_type.name == schema.root {
    match schema.xml_header {
      PackageXmlHeader::None => quote! {},
      PackageXmlHeader::Plain => quote! {
        writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;
      },
      PackageXmlHeader::Standalone => quote! {
        writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
      },
    }
  } else {
    quote! {}
  }
}

fn attr_type(attr: &PackageAttribute) -> Result<Type> {
  if attr.required {
    parse_str(&attr_type_name(attr))
  } else {
    parse_str(&format!("Option<{}>", attr_type_name(attr)))
  }
  .map_err(Into::into)
}

fn attr_type_name(attr: &PackageAttribute) -> String {
  if attr.r#type == "String" {
    "String".to_string()
  } else {
    attr.r#type.to_upper_camel_case()
  }
}

fn attr_parse_expr(package_type: &PackageType, attr: &PackageAttribute) -> Result<Expr> {
  if package_type.name == "Relationship" && attr.field == "type" {
    parse2(quote! {
      crate::common::normalize_relationship_type(
        &attr.decode_and_unescape_value(xml_reader.decoder())?,
      ).to_string()
    })
    .map_err(Into::into)
  } else if attr.r#type == "String" {
    parse2(quote! {
      attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned()
    })
    .map_err(Into::into)
  } else {
    let attr_type: Type = parse_str(&attr.r#type.to_upper_camel_case())?;
    parse2(quote! {
      <#attr_type as std::str::FromStr>::from_str(&attr.decode_and_unescape_value(xml_reader.decoder())?)?
    })
    .map_err(Into::into)
  }
}

fn attr_build_expr(attr: &PackageAttribute) -> Result<TokenStream> {
  let field_ident: Ident = parse_str(&escape_snake_case(attr.field.clone()))?;
  if attr.required {
    let field_name = attr.field.as_str();
    Ok(quote! {
      #field_ident: #field_ident.ok_or_else(|| crate::common::SdkError::CommonError(#field_name.to_string()))?
    })
  } else {
    Ok(quote! { #field_ident })
  }
}

fn gen_text_child_arm(child: &PackageTextChild) -> Result<Arm> {
  let field_ident: Ident = parse_str(&escape_snake_case(child.field.clone()))?;
  let q_name = child.q_name.as_str();
  let q_name_literal: LitByteStr = parse_str(&format!("b\"{q_name}\""))?;
  Ok(parse2(quote! {
    #q_name_literal => {
      if e_empty {
        #field_ident = Some(String::new());
      } else {
        if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
          #field_ident = Some(t.decode()?.into_owned());
        }
        xml_reader.next()?;
      }
    }
  })?)
}

fn gen_attr_writers(package_type: &PackageType) -> Result<Vec<Stmt>> {
  let mut writers: Vec<Stmt> = vec![];

  if package_type.has_xmlns_fields {
    writers.push(parse2(quote! {
      if let Some(xmlns) = &self.xmlns {
        crate::common::write_xmlns_attr(writer, None, xmlns)?;
      }
    })?);
    writers.push(parse2(quote! {
      {
        let mut xmlns_entries: Vec<_> = self.xmlns_map.iter().collect();
        xmlns_entries.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
        for (k, v) in xmlns_entries {
          crate::common::write_xmlns_attr(writer, Some(k), v)?;
        }
      }
    })?);
    writers.push(parse2(quote! {
      if let Some(mc_ignorable) = &self.mc_ignorable {
        crate::common::write_attr_value(writer, "mc:Ignorable", mc_ignorable)?;
      }
    })?);
  }

  for attr in &package_type.attributes {
    let field_ident: Ident = parse_str(&escape_snake_case(attr.field.clone()))?;
    let q_name = attr.q_name.as_str();
    if attr.required {
      writers.push(parse2(quote! {
        crate::common::write_attr_value(writer, #q_name, &self.#field_ident)?;
      })?);
    } else {
      writers.push(parse2(quote! {
        if let Some(value) = &self.#field_ident {
          crate::common::write_attr_value(writer, #q_name, value)?;
        }
      })?);
    }
  }

  Ok(writers)
}

fn gen_child_writers(package_type: &PackageType) -> Result<Vec<TokenStream>> {
  let mut writers = vec![];

  for child in &package_type.text_children {
    let field_ident: Ident = parse_str(&escape_snake_case(child.field.clone()))?;
    let q_name = child.q_name.as_str();
    let fixed_attr_writers = gen_fixed_attr_writers(&child.fixed_attributes);
    writers.push(quote! {
      if let Some(value) = &self.#field_ident {
        writer.write_char('<')?;
        writer.write_str(#q_name)?;
        #( #fixed_attr_writers )*
        writer.write_char('>')?;
        writer.write_str(&quick_xml::escape::escape(value))?;
        writer.write_str("</")?;
        writer.write_str(#q_name)?;
        writer.write_char('>')?;
      }
    });
  }

  for child_field in &package_type.child_fields {
    let field_ident: Ident = parse_str(&escape_snake_case(child_field.field.clone()))?;
    match child_field.kind {
      PackageChildFieldKind::Vec => {
        writers.push(quote! {
          for child in &self.#field_ident {
            child.write_xml(writer, xmlns_prefix)?;
          }
        });
      }
      PackageChildFieldKind::ChoiceVec => {
        let mut variant_arms = vec![];
        for variant in &child_field.variants {
          let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
          let enum_ident: Ident = parse_str(&child_field.enum_name)?;
          variant_arms.push(quote! {
            #enum_ident::#variant_ident(value) => value.write_xml(writer, xmlns_prefix)?,
          });
        }
        writers.push(quote! {
          for child in &self.#field_ident {
            match child {
              #( #variant_arms )*
            }
          }
        });
      }
    }
  }

  Ok(writers)
}

fn gen_fixed_attr_writers(attrs: &[PackageFixedAttribute]) -> Vec<TokenStream> {
  attrs
    .iter()
    .map(|attr| {
      let q_name = attr.q_name.as_str();
      let value = attr.value.as_str();
      quote! {
        writer.write_char(' ')?;
        writer.write_str(#q_name)?;
        writer.write_str("=\"")?;
        writer.write_str(#value)?;
        writer.write_char('"')?;
      }
    })
    .collect()
}

fn package_prefixed_tag(prefix: &str, tag: &str) -> String {
  if prefix.is_empty() {
    tag.to_string()
  } else {
    format!("{prefix}:{tag}")
  }
}

fn find_package_type<'a>(schema: &'a PackageSchema, type_name: &str) -> Result<&'a PackageType> {
  schema
    .types
    .iter()
    .find(|package_type| package_type.name == type_name)
    .ok_or_else(|| format!("missing package type {type_name} in {}", schema.module_name).into())
}
