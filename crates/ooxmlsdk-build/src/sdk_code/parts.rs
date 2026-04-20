use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemMod, ItemStruct, Stmt, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::part_codegen_ir::{PartChildCardinality, PartFieldKind, PartModuleDecl};
use crate::sdk_code::versioning::{features_cfg_attrs, versioned_tokens};

pub fn gen_part_module(part: &PartModuleDecl) -> Result<TokenStream> {
  let relationship_type_str = part.relationship_type.as_str();
  let relationship_type_stmt: Stmt = parse2(quote! {
    pub const RELATIONSHIP_TYPE: &str = #relationship_type_str;
  })?;

  let path_prefix_str = part.path_prefix.as_str();
  let path_prefix_stmt: Stmt = parse2(quote! {
    pub const PATH_PREFIX: &str = #path_prefix_str;
  })?;

  let struct_name_ident: Ident = parse_str(&part.struct_name)?;
  let mut fields: Vec<TokenStream> = vec![];

  for field in &part.fields {
    fields.push(field_tokens(field)?);
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

pub fn gen_parts_mod(parts: &[&PartModuleDecl]) -> Result<TokenStream> {
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

fn part_module_attrs(part: &PartModuleDecl) -> Vec<syn::Attribute> {
  let filtered_features: Vec<String> = part
    .features
    .iter()
    .filter(|feature| feature.as_str() != "parts")
    .cloned()
    .collect();

  features_cfg_attrs(&filtered_features)
}

fn field_tokens(field: &crate::sdk_code::part_codegen_ir::PartFieldDecl) -> Result<TokenStream> {
  let field_ident: Ident = parse_str(&field.rust_name)?;
  let field_type: Type = parse_str(&field.rust_type)?;
  let sdk_attr = field_sdk_attr(&field.kind);
  let versioned = match &field.kind {
    PartFieldKind::Child { cardinality, .. } => {
      let rendered_ty = match cardinality {
        PartChildCardinality::Repeated => quote! { Vec<#field_type> },
        PartChildCardinality::Required => quote! { std::boxed::Box<#field_type> },
        PartChildCardinality::Optional => quote! { Option<std::boxed::Box<#field_type>> },
      };
      versioned_tokens(
        &field.version,
        quote! {
          #sdk_attr
          pub #field_ident: #rendered_ty,
        },
      )
    }
    _ => versioned_tokens(
      &field.version,
      quote! {
        #sdk_attr
        pub #field_ident: #field_type,
      },
    ),
  };

  Ok(versioned)
}

fn field_sdk_attr(kind: &PartFieldKind) -> TokenStream {
  match kind {
    PartFieldKind::Rid => quote! { #[sdk(part_rid)] },
    PartFieldKind::ContentTypes => quote! { #[sdk(part_content_types)] },
    PartFieldKind::Relationships => quote! { #[sdk(part_relationships)] },
    PartFieldKind::RelsPath => quote! { #[sdk(part_rels_path)] },
    PartFieldKind::InnerPath => quote! { #[sdk(part_inner_path)] },
    PartFieldKind::RootElement => quote! { #[sdk(part_root)] },
    PartFieldKind::TextContent => quote! { #[sdk(part_content(kind = "text"))] },
    PartFieldKind::BinaryContent => quote! { #[sdk(part_content(kind = "binary"))] },
    PartFieldKind::Child {
      relationship_type,
      cardinality,
    } => {
      let kind = match cardinality {
        PartChildCardinality::Optional => "optional",
        PartChildCardinality::Required => "required",
        PartChildCardinality::Repeated => "repeated",
      };
      quote! {
        #[sdk(part_child(relationship_type = #relationship_type, kind = #kind))]
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_code::part_codegen_ir::{
    PartChildCardinality, PartFieldDecl, PartFieldKind, PartModuleDecl,
  };

  #[test]
  fn generates_part_child_attrs_from_codegen_ir() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      relationship_type: "rel".to_string(),
      path_prefix: "word".to_string(),
      fields: vec![
        PartFieldDecl {
          rust_name: "r_id".to_string(),
          rust_type: "String".to_string(),
          kind: PartFieldKind::Rid,
          ..Default::default()
        },
        PartFieldDecl {
          rust_name: "theme_part".to_string(),
          rust_type: "crate::parts::theme_part::ThemePart".to_string(),
          kind: PartFieldKind::Child {
            relationship_type: "theme-rel".to_string(),
            cardinality: PartChildCardinality::Optional,
          },
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("# [sdk (part_rid)] pub r_id : String"));
    assert!(
      rendered.contains(
        "# [sdk (part_child (relationship_type = \"theme-rel\" , kind = \"optional\"))] pub theme_part : Option < std :: boxed :: Box < crate :: parts :: theme_part :: ThemePart > >"
      )
    );
  }
}
