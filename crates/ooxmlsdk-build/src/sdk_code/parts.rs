use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemMod, ItemStruct, Stmt, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::versioning::{features_cfg_attrs, versioned_tokens};
use crate::sdk_data::sdk_data_model::{Part, PartChild, PartContentKind};

pub fn gen_part_module(part: &Part) -> Result<TokenStream> {
  let relationship_type_str = part.relationship_type.as_str();
  let relationship_type_stmt: Stmt = parse2(quote! {
    pub const RELATIONSHIP_TYPE: &str = #relationship_type_str;
  })?;

  let path_prefix_str = if part.paths.general.is_empty() {
    ""
  } else {
    part.paths.general.as_str()
  };
  let path_prefix_stmt: Stmt = parse2(quote! {
    pub const PATH_PREFIX: &str = #path_prefix_str;
  })?;

  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case())?;
  let mut fields: Vec<TokenStream> = vec![];

  if part.base == "OpenXmlPackage" {
    fields.push(quote! {
      pub content_types: crate::schemas::opc_content_types::Types,
    });
  } else {
    fields.push(quote! {
      pub r_id: String,
    });
  }

  if !part.children.is_empty() {
    fields.push(quote! {
      pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    });
    fields.push(quote! {
      pub rels_path: String,
    });
  }

  fields.push(quote! {
    pub inner_path: String,
  });

  match part.content_kind {
    PartContentKind::Xml => {
      let root_type = part_root_type_tokens(part)?;
      fields.push(quote! {
        pub root_element: #root_type,
      });
    }
    PartContentKind::Text => {
      fields.push(quote! {
        pub part_content: String,
      });
    }
    PartContentKind::Binary => {
      fields.push(quote! {
        pub part_content: Vec<u8>,
      });
    }
    PartContentKind::None => {}
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    fields.push(child_field_tokens(child)?);
  }

  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  })?;

  Ok(quote! {
    #relationship_type_stmt
    #path_prefix_stmt
    #part_struct
  })
}

pub fn gen_parts_mod(parts: &[Part]) -> Result<TokenStream> {
  let mut mod_list: Vec<ItemMod> = vec![];

  for part in parts {
    let mod_ident: Ident = parse_str(&part.module_name)?;
    let part_attrs = part_module_attrs(part);
    mod_list.push(parse2(quote! {
      #( #part_attrs )*
      pub mod #mod_ident;
    })?);
  }

  Ok(quote! {
    #( #mod_list )*
  })
}

fn part_module_attrs(part: &Part) -> Vec<syn::Attribute> {
  let filtered_features: Vec<String> = part
    .features
    .iter()
    .filter(|feature| feature.as_str() != "parts")
    .cloned()
    .collect();

  features_cfg_attrs(&filtered_features)
}

fn part_root_type_tokens(part: &Part) -> Result<Type> {
  if part.name == "CoreFilePropertiesPart" {
    return parse_str("crate::schemas::opc_core_properties::CoreProperties").map_err(Into::into);
  }

  parse_str(&format!(
    "crate::schemas::{}::{}",
    part.schema_module,
    part.root_class_name.to_upper_camel_case()
  ))
  .map_err(Into::into)
}

fn child_type_tokens(child: &PartChild) -> Result<Type> {
  parse_str(&format!(
    "crate::parts::{}::{}",
    child.name.to_snake_case(),
    child.name.to_upper_camel_case(),
  ))
  .map_err(Into::into)
}

fn child_field_tokens(child: &PartChild) -> Result<TokenStream> {
  let field_ident: Ident = parse_str(&child.api_name.to_snake_case())?;
  let child_type = child_type_tokens(child)?;

  if child.max_occurs_great_than_one {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: Vec<#child_type>,
      },
    ))
  } else if child.min_occurs_is_non_zero {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: std::boxed::Box<#child_type>,
      },
    ))
  } else {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: Option<std::boxed::Box<#child_type>>,
      },
    ))
  }
}
