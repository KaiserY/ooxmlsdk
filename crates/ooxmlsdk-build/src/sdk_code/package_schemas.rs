use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Arm, Ident, ItemImpl, ItemStruct, LitByteStr, Type, parse_str, parse2};

use crate::sdk_data::sdk_data_model::{
  PackageAttribute, PackageChildFieldKind, PackageEnum, PackageFixedAttribute, PackageSchema,
  PackageTextChild, PackageType, PackageTypeKind, PackageXmlHeader,
};
use crate::utils::escape_snake_case;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

pub fn gen_package_schema(schema: &PackageSchema) -> Result<TokenStream> {
  let mut token_stream_list = vec![];

  for package_enum in &schema.enums {
    token_stream_list.push(gen_package_enum(package_enum)?);
  }

  for package_type in &schema.types {
    if let Some(choice_enum) = gen_choice_enum(package_type)? {
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
  let mut from_str_arms: Vec<Arm> = vec![];
  let mut as_str_arms: Vec<Arm> = vec![];

  for variant in &package_enum.variants {
    let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
    let variant_value = variant.value.as_str();

    if variant.is_default {
      variants.push(quote! {
        #[default]
        #variant_ident
      });
    } else {
      variants.push(quote! {
        #variant_ident
      });
    }

    from_str_arms.push(parse2(quote! {
      #variant_value => Ok(Self::#variant_ident),
    })?);
    as_str_arms.push(parse2(quote! {
      Self::#variant_ident => #variant_value,
    })?);
  }

  Ok(quote! {
    #[derive(Clone, Debug, Default)]
    pub enum #enum_ident {
      #( #variants, )*
    }

    impl std::str::FromStr for #enum_ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          #( #from_str_arms )*
          _ => Err(crate::common::SdkError::CommonError(s.to_string())),
        }
      }
    }

    impl #enum_ident {
      pub fn as_xml_str(&self) -> &'static str {
        match self {
          #( #as_str_arms )*
        }
      }
    }

    impl std::fmt::Display for #enum_ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_xml_str())
      }
    }
  })
}

fn gen_choice_enum(package_type: &PackageType) -> Result<Option<TokenStream>> {
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
    variants.push(quote! {
      #variant_ident(std::boxed::Box<#variant_type>),
    });
  }

  Ok(Some(quote! {
    #[derive(Clone, Debug)]
    pub enum #enum_ident {
      #( #variants )*
    }
  }))
}

fn gen_package_type(schema: &PackageSchema, package_type: &PackageType) -> Result<TokenStream> {
  let struct_ident: Ident = parse_str(&package_type.name.to_upper_camel_case())?;
  let prefixed_tag = prefixed_tag(&package_type.prefix, &package_type.tag);
  let local_tag = package_type.tag.as_str();
  let prefixed_tag_literal: LitByteStr = parse_str(&format!("b\"{prefixed_tag}\""))?;
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

  let from_reader_fn = quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;
      Self::deserialize_inner(&mut xml_reader, None)
    }
  };

  let deserialize_inner_fn = gen_deserialize_inner_fn(
    schema,
    package_type,
    &prefixed_tag_literal,
    &local_tag_literal,
  )?;
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
  prefixed_tag_literal: &LitByteStr,
  local_tag_literal: &LitByteStr,
) -> Result<TokenStream> {
  let mut declarations = vec![];
  let mut attr_arms: Vec<Arm> = vec![];
  let mut build_fields = vec![];
  let mut child_arms: Vec<Arm> = vec![];

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
    let assign_expr = attr_parse_expr(attr)?;
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
        let child_prefixed = prefixed_tag(&child_schema_type.prefix, child_type_name);
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
          let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
          let variant_type: Type = parse_str(&variant.r#type.to_upper_camel_case())?;
          let variant_schema_type = find_package_type(schema, &variant.r#type)?;
          let tag = variant_schema_type.tag.as_str();
          let prefixed_tag = prefixed_tag(&variant_schema_type.prefix, tag);
          let prefixed_tag_literal: LitByteStr = parse_str(&format!("b\"{prefixed_tag}\""))?;
          let local_tag_literal: LitByteStr = parse_str(&format!("b\"{tag}\""))?;
          child_arms.push(parse2(quote! {
            #prefixed_tag_literal | #local_tag_literal => {
              #field_ident.push(#enum_ident::#variant_ident(std::boxed::Box::new(
                #variant_type::deserialize_inner(xml_reader, Some((e, e_empty)))?,
              )));
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

  let xmlns_match = if package_type.has_xmlns_fields {
    quote! {
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
          continue;
        }

        match key {
          #( #attr_arms )*
          _ => {}
        }
      }
    }
  } else {
    quote! {
      key => match key {
        #( #attr_arms )*
        _ => {}
      }
    }
  };

  Ok(quote! {
    pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
      xml_reader: &mut R,
      xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    ) -> Result<Self, crate::common::SdkError> {
      let (e, _empty_tag) =
        crate::common::expect_event_start!(xml_reader, xml_event, #prefixed_tag_literal, #local_tag_literal);

      #( #declarations )*

      for attr in e.attributes().with_checks(false) {
        let attr = attr?;

        match attr.key.as_ref() {
          #xmlns_match
        }
      }

      #child_match

      Ok(Self {
        #( #build_fields, )*
      })
    }
  })
}

fn gen_package_type_serializer(
  schema: &PackageSchema,
  package_type: &PackageType,
) -> Result<TokenStream> {
  let struct_ident: Ident = parse_str(&package_type.name.to_upper_camel_case())?;
  let root_with_xmlns = if package_type.name == schema.root {
    let xmlns_uri = schema.xmlns_uri.as_str();
    quote! {
      if let Some(xmlns) = &self.xmlns {
        xmlns != #xmlns_uri
      } else {
        true
      }
    }
  } else {
    quote! { false }
  };
  let header = if package_type.name == schema.root {
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
  };
  let prefixed_tag = prefixed_tag(&package_type.prefix, &package_type.tag);
  let local_tag = package_type.tag.as_str();
  let is_leaf = package_type.kind == PackageTypeKind::Leaf;

  let attr_writers = gen_attr_writers(package_type)?;
  let child_writers = gen_child_writers(package_type)?;
  let write_body = if is_leaf {
    quote! {
      writer.write_str("/>")?;
      return Ok(());
    }
  } else {
    quote! {
      writer.write_char('>')?;

      #( #child_writers )*

      writer.write_str("</")?;
      if with_xmlns {
        writer.write_str(#prefixed_tag)?;
      } else {
        writer.write_str(#local_tag)?;
      }
      writer.write_char('>')?;
      Ok(())
    }
  };
  let to_xml_fn = if package_type.name == schema.root {
    quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, #root_with_xmlns)?;
        Ok(writer)
      }
    }
  } else {
    quote! {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, false)?;
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
        with_xmlns: bool,
      ) -> Result<(), std::fmt::Error> {
        #header

        writer.write_char('<')?;
        if with_xmlns {
          writer.write_str(#prefixed_tag)?;
        } else {
          writer.write_str(#local_tag)?;
        }

        #( #attr_writers )*

        #write_body
      }
    }
  })?;

  Ok(quote! {
    #serializer_impl
  })
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

fn attr_parse_expr(attr: &PackageAttribute) -> Result<TokenStream> {
  if attr.r#type == "String" {
    Ok(quote! {
      attr.decode_and_unescape_value(xml_reader.decoder())?.into_owned()
    })
  } else {
    let attr_type: Type = parse_str(&attr.r#type.to_upper_camel_case())?;
    Ok(quote! {
      <#attr_type as std::str::FromStr>::from_str(&attr.decode_and_unescape_value(xml_reader.decoder())?)?
    })
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

fn gen_attr_writers(package_type: &PackageType) -> Result<Vec<TokenStream>> {
  let mut writers = vec![];

  if package_type.has_xmlns_fields {
    writers.push(quote! {
      if let Some(xmlns) = &self.xmlns {
        crate::common::write_xmlns_attr(writer, None, xmlns)?;
      }
    });
    writers.push(quote! {
      {
        let mut xmlns_entries: Vec<_> = self.xmlns_map.iter().collect();
        xmlns_entries.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
        for (k, v) in xmlns_entries {
          crate::common::write_xmlns_attr(writer, Some(k), v)?;
        }
      }
    });
    writers.push(quote! {
      if let Some(mc_ignorable) = &self.mc_ignorable {
        crate::common::write_attr_value(writer, "mc:Ignorable", mc_ignorable)?;
      }
    });
  }

  for attr in &package_type.attributes {
    let field_ident: Ident = parse_str(&escape_snake_case(attr.field.clone()))?;
    let q_name = attr.q_name.as_str();
    if attr.required {
      writers.push(quote! {
        crate::common::write_attr_value(writer, #q_name, &self.#field_ident)?;
      });
    } else {
      writers.push(quote! {
        if let Some(value) = &self.#field_ident {
          crate::common::write_attr_value(writer, #q_name, value)?;
        }
      });
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
            child.write_xml(writer, with_xmlns)?;
          }
        });
      }
      PackageChildFieldKind::ChoiceVec => {
        let mut variant_arms = vec![];
        for variant in &child_field.variants {
          let variant_ident: Ident = parse_str(&variant.name.to_upper_camel_case())?;
          let enum_ident: Ident = parse_str(&child_field.enum_name)?;
          variant_arms.push(quote! {
            #enum_ident::#variant_ident(value) => value.write_xml(writer, with_xmlns)?,
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

fn prefixed_tag(prefix: &str, tag: &str) -> String {
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
