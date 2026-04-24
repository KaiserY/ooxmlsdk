use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemMod, ItemStruct, Stmt, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::part_codegen_ir::{
  PartChildCardinality, PartFieldDecl, PartFieldKind, PartModuleDecl,
};
use crate::sdk_code::versioning::features_cfg_attrs;

pub fn gen_part_module(part: &PartModuleDecl) -> Result<TokenStream> {
  let relationship_type_str = part.relationship_type.as_str();
  let relationship_type_stmt: Stmt = parse2(quote! {
    pub const RELATIONSHIP_TYPE: &str = #relationship_type_str;
  })?;

  let path_prefix_str = part.path_prefix.as_str();
  let path_prefix_stmt: Stmt = parse2(quote! {
    pub const PATH_PREFIX: &str = #path_prefix_str;
  })?;

  let content_type_str = part.content_type.as_str();
  let content_type_stmt: Stmt = parse2(quote! {
    pub const CONTENT_TYPE: &str = #content_type_str;
  })?;

  let target_name_str = part.target_name.as_str();
  let target_name_stmt: Stmt = parse2(quote! {
    pub const TARGET_NAME: &str = #target_name_str;
  })?;

  let extension_str = part.extension.as_str();
  let extension_stmt: Stmt = parse2(quote! {
    pub const EXTENSION: &str = #extension_str;
  })?;

  if is_package_part(part) {
    return gen_package_module(
      part,
      relationship_type_stmt,
      path_prefix_stmt,
      content_type_stmt,
      target_name_stmt,
      extension_stmt,
    );
  }

  gen_part_handle_module(
    part,
    relationship_type_stmt,
    path_prefix_stmt,
    content_type_stmt,
    target_name_stmt,
    extension_stmt,
  )
}

fn gen_package_module(
  part: &PartModuleDecl,
  relationship_type_stmt: Stmt,
  path_prefix_stmt: Stmt,
  content_type_stmt: Stmt,
  target_name_stmt: Stmt,
  extension_stmt: Stmt,
) -> Result<TokenStream> {
  let struct_name_ident: Ident = parse_str(&part.struct_name)?;
  let marker_fields = package_marker_fields(part)?;
  let package_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
    pub struct #struct_name_ident {
      pub(crate) storage: crate::common::SdkPackageStorage,
      pub(crate) main_part_id: Option<crate::common::PartId>,
      pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
      #( #marker_fields )*
    }
  })?;

  Ok(quote! {
    #relationship_type_stmt
    #path_prefix_stmt
    #content_type_stmt
    #target_name_stmt
    #extension_stmt
    #package_struct
  })
}

fn package_marker_fields(part: &PartModuleDecl) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();
  let main_part_accessor = part.main_part_accessor_name.as_deref();

  for field in &part.fields {
    let PartFieldKind::ChildPart {
      relationship_type,
      cardinality,
    } = &field.kind
    else {
      continue;
    };
    if relationship_type.is_empty() {
      continue;
    }

    let attrs = part_field_attrs(field);
    let field_ident: Ident = parse_str(&field.rust_name)?;
    let part_ty: Type = parse_str(&field.rust_type)?;
    let marker_ty = part_child_marker_type(*cardinality, &part_ty);
    let main_attr = if Some(field.rust_name.as_str()) == main_part_accessor {
      let accessor = field.rust_name.as_str();
      quote! { #[sdk(package_main(accessor = #accessor))] }
    } else {
      quote! {}
    };
    fields.push(quote! {
      #( #attrs )*
      #main_attr
      #[sdk(part_child(relationship_type = #relationship_type))]
      pub(crate) #field_ident: #marker_ty,
    });
  }

  Ok(fields)
}

fn gen_part_handle_module(
  part: &PartModuleDecl,
  relationship_type_stmt: Stmt,
  path_prefix_stmt: Stmt,
  content_type_stmt: Stmt,
  target_name_stmt: Stmt,
  extension_stmt: Stmt,
) -> Result<TokenStream> {
  let struct_name_ident: Ident = parse_str(&part.struct_name)?;
  let marker_fields = part_marker_fields(part)?;
  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
    pub struct #struct_name_ident {
      pub(crate) id: crate::common::PartId,
      #( #marker_fields )*
    }
  })?;

  Ok(quote! {
    #relationship_type_stmt
    #path_prefix_stmt
    #content_type_stmt
    #target_name_stmt
    #extension_stmt
    #part_struct
  })
}

fn part_marker_fields(part: &PartModuleDecl) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  if let Some((root_type, accessor_name)) = part_root_element_info(part) {
    let root_ty: Type = parse_str(&root_type)?;
    let accessor_name = accessor_name.as_str();
    fields.push(quote! {
      #[sdk(part_root(accessor = #accessor_name))]
      pub(crate) root_element: crate::sdk::PartRoot<#root_ty>,
    });
  }

  for field in &part.fields {
    let PartFieldKind::ChildPart {
      relationship_type,
      cardinality,
    } = &field.kind
    else {
      continue;
    };
    if relationship_type.is_empty() {
      continue;
    }

    let attrs = part_field_attrs(field);
    let field_ident: Ident = parse_str(&field.rust_name)?;
    let part_ty: Type = parse_str(&field.rust_type)?;
    let marker_ty = part_child_marker_type(*cardinality, &part_ty);
    fields.push(quote! {
      #( #attrs )*
      #[sdk(part_child(relationship_type = #relationship_type))]
      pub(crate) #field_ident: #marker_ty,
    });
  }

  Ok(fields)
}

fn part_child_marker_type(cardinality: PartChildCardinality, part_ty: &Type) -> TokenStream {
  match cardinality {
    PartChildCardinality::Optional => quote! { crate::sdk::OptionalPart<#part_ty> },
    PartChildCardinality::Required => quote! { crate::sdk::RequiredPart<#part_ty> },
    PartChildCardinality::Repeated => quote! { crate::sdk::RepeatedPart<#part_ty> },
  }
}

pub fn gen_parts_mod(parts: &[&PartModuleDecl]) -> Result<TokenStream> {
  let mut mod_list: Vec<ItemMod> = vec![];
  let mut part_ref_variants: Vec<TokenStream> = vec![];
  let mut part_ref_part_id_arms: Vec<TokenStream> = vec![];
  let mut part_ref_downcast_arms: Vec<TokenStream> = vec![];
  let mut part_ref_from_relationship_type_branches: Vec<TokenStream> = vec![];
  let mut root_variants: Vec<TokenStream> = vec![];
  let mut root_part_id_arms: Vec<TokenStream> = vec![];
  let mut root_accessor_methods: Vec<TokenStream> = vec![];
  let mut root_from_part_id_branches: Vec<TokenStream> = vec![];
  let mut root_to_xml_arms: Vec<TokenStream> = vec![];

  for part in parts {
    let mod_ident: Ident = parse_str(&part.module_name)?;
    let part_attrs = part_module_attrs(part);
    let struct_ident: Ident = parse_str(&part.struct_name)?;
    let part_ty: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      part.module_name, part.struct_name
    ))?;
    mod_list.push(parse2(quote! {
      #( #part_attrs )*
      pub mod #mod_ident;
    })?);
    if is_package_part(part) {
      continue;
    }

    if let Some((root_type, accessor_name)) = part_root_element_info(part) {
      let root_ty: Type = parse_str(&root_type)?;
      let root_accessor_ident: Ident = parse_str(&accessor_name)?;
      let root_accessor_ident_mut: Ident = parse_str(&format!("{accessor_name}_mut"))?;
      root_variants.push(quote! {
        #( #part_attrs )*
        #struct_ident(Box<#root_ty>),
      });
      root_part_id_arms.push(quote! {
        #( #part_attrs )*
        PartRootElement::#struct_ident(_) => stringify!(#struct_ident)
      });
      root_accessor_methods.push(quote! {
        #( #part_attrs )*
        pub fn #root_accessor_ident(&self) -> Option<&#root_ty> {
          match self {
            PartRootElement::#struct_ident(root) => Some(root.as_ref()),
            _ => None,
          }
        }

        #( #part_attrs )*
        pub fn #root_accessor_ident_mut(&mut self) -> Option<&mut #root_ty> {
          match self {
            PartRootElement::#struct_ident(root) => Some(root.as_mut()),
            _ => None,
          }
        }
      });
      root_to_xml_arms.push(quote! {
        #( #part_attrs )*
        PartRootElement::#struct_ident(root) => Ok(root.to_xml_bytes()?),
      });
      let content_type_str = part.content_type.as_str();
      root_from_part_id_branches.push(quote! {
        #( #part_attrs )*
        if !matches!(#content_type_str, "" | "application/xml" | "text/xml")
          && part.content_type() == #content_type_str
        {
          return #root_ty::from_bytes(part.data().bytes())
            .map(Box::new)
            .map(PartRootElement::#struct_ident)
            .map(Some);
        }
      });
    }

    part_ref_variants.push(quote! {
      #( #part_attrs )*
      #struct_ident(#part_ty),
    });
    part_ref_part_id_arms.push(quote! {
      #( #part_attrs )*
      PartRef::#struct_ident(part) => part.part_id()
    });
    part_ref_downcast_arms.push(quote! {
      #( #part_attrs )*
      PartRef::#struct_ident(part) => {
        let any: &dyn std::any::Any = &part;
        any.downcast_ref::<T>().copied()
      }
    });
    let relationship_type_str = part.relationship_type.as_str();
    part_ref_from_relationship_type_branches.push(quote! {
      #( #part_attrs )*
      if crate::common::relationship_type_matches(relationship_type, #relationship_type_str) {
        return Some(PartRef::#struct_ident(
          <#part_ty as crate::sdk::SdkPartHandle>::from_part_id(part_id),
        ));
      }
    });
  }

  Ok(quote! {
    #( #mod_list )*

    #[derive(Clone, Copy, Debug)]
    pub enum PartRef {
      #( #part_ref_variants )*
    }

    impl PartRef {
      pub fn part_id(self) -> crate::common::PartId {
        match self {
          #( #part_ref_part_id_arms, )*
        }
      }

      pub fn downcast<T: crate::sdk::SdkPartHandle + 'static>(self) -> Option<T> {
        match self {
          #( #part_ref_downcast_arms, )*
        }
      }

      pub fn from_part_id<P: crate::sdk::SdkPackage>(
        package: &P,
        part_id: crate::common::PartId,
      ) -> Option<Self> {
        let part = package.storage().part(part_id)?;
        let relationship_type = part.relationship_type()?;
        #( #part_ref_from_relationship_type_branches )*
        None
      }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct IdPartPair<'a> {
      pub relationship_id: &'a str,
      pub part: PartRef,
    }

    impl<'a> IdPartPair<'a> {
      pub const fn new(relationship_id: &'a str, part: PartRef) -> Self {
        Self {
          relationship_id,
          part,
        }
      }
    }

    #[derive(Clone, Debug)]
    pub enum PartRootElement {
      #( #root_variants )*
    }

    impl PartRootElement {
      pub fn part_type_name(&self) -> &'static str {
        match self {
          #( #root_part_id_arms, )*
        }
      }

      #( #root_accessor_methods )*

      pub fn to_xml_bytes(&self) -> Result<Vec<u8>, crate::common::SdkError> {
        match self {
          #( #root_to_xml_arms )*
        }
      }

      pub fn from_part_id(
        storage: &crate::common::SdkPackageStorage,
        part_id: crate::common::PartId,
      ) -> Result<Option<Self>, crate::common::SdkError> {
        let Some(part) = storage.part(part_id) else {
          return Ok(None);
        };
        #( #root_from_part_id_branches )*

        Ok(None)
      }
    }

    pub fn initialize_root_elements(
      storage: &crate::common::SdkPackageStorage,
      open_mode: crate::common::PackageOpenMode,
    ) -> Result<Vec<Option<crate::parts::PartRootElement>>, crate::common::SdkError> {
      let mut root_elements = vec![None; storage.parts().len()];
      if matches!(open_mode, crate::common::PackageOpenMode::Eager) {
        for (index, slot) in root_elements.iter_mut().enumerate() {
          let part_id = crate::common::PartId::from_index(index);
          if let Some(root_element) = crate::parts::PartRootElement::from_part_id(storage, part_id)? {
            *slot = Some(root_element);
          }
        }
      }
      Ok(root_elements)
    }

    pub trait PartRootCache: crate::sdk::SdkPackage {
      fn root_element(
        &self,
        part_id: crate::common::PartId,
      ) -> Option<&crate::parts::PartRootElement>;

      fn root_element_slot_mut(
        &mut self,
        part_id: crate::common::PartId,
      ) -> Option<&mut Option<crate::parts::PartRootElement>>;

      fn push_root_element_slot(&mut self);

      #[inline]
      fn is_root_element_loaded(&self, part_id: crate::common::PartId) -> bool {
        self.root_element(part_id).is_some()
      }

      #[inline]
      fn unload_root_element(
        &mut self,
        part_id: crate::common::PartId,
      ) -> Option<crate::parts::PartRootElement> {
        self.root_element_slot_mut(part_id)?.take()
      }
    }

    pub fn save_package<P, W>(
      package: &P,
      writer: W,
    ) -> Result<(), crate::common::SdkError>
    where
      P: crate::parts::PartRootCache,
      W: std::io::Write + std::io::Seek,
    {
      use std::io::Write as _;

      let mut zip = zip::ZipWriter::new(writer);
      let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
      let mut entry_set = std::collections::HashSet::<String>::new();
      let storage = crate::sdk::SdkPackage::storage(package);

      zip.start_file("[Content_Types].xml", options)?;
      zip.write_all(&storage.content_types().to_xml_bytes()?)?;

      if !storage.package_relationships().is_empty() {
        if entry_set.insert("_rels".to_string()) {
          zip.add_directory("_rels", options)?;
        }
        zip.start_file("_rels/.rels", options)?;
        zip.write_all(&storage.package_relationships().to_relationships().to_xml_bytes()?)?;
      }

      for (index, part) in storage.parts().iter().enumerate() {
        let part_id = crate::common::PartId::from_index(index);
        let parent_path = crate::common::parent_zip_path(part.path());
        let directory_path = parent_path.strip_suffix('/').unwrap_or(&parent_path);
        if !directory_path.is_empty() && entry_set.insert(directory_path.to_string()) {
          zip.add_directory(directory_path, options)?;
        }

        let relationships = part.relationships();
        if !relationships.is_empty() {
          let rels_dir_path = crate::common::part_relationships_directory_path(part.path());
          if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
            zip.add_directory(&rels_dir_path, options)?;
          }
          let rels_path = crate::common::part_relationships_path(part.path());
          zip.start_file(&rels_path, options)?;
          zip.write_all(&relationships.to_relationships().to_xml_bytes()?)?;
        }

        zip.start_file(part.path(), options)?;
        if let Some(root_element) = package.root_element(part_id) {
          zip.write_all(&root_element.to_xml_bytes()?)?;
        } else {
          zip.write_all(part.data().bytes())?;
        }
      }

      zip.finish()?;
      Ok(())
    }
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

fn part_field_attrs(field: &PartFieldDecl) -> Vec<syn::Attribute> {
  let filtered_features: Vec<String> = field
    .features
    .iter()
    .filter(|feature| feature.as_str() != "parts")
    .cloned()
    .collect();

  features_cfg_attrs(&filtered_features)
}

fn is_package_part(part: &PartModuleDecl) -> bool {
  part.is_package
    || part.relationship_type.is_empty()
      && part
        .fields
        .iter()
        .any(|field| matches!(field.kind, PartFieldKind::ContentTypes))
}

fn part_root_element_info(part: &PartModuleDecl) -> Option<(String, String)> {
  if let (Some(root_type), Some(accessor_name)) = (
    part.root_element_type.as_deref(),
    part.root_element_accessor_name.as_deref(),
  ) {
    return Some((root_type.to_string(), accessor_name.to_string()));
  }

  part
    .fields
    .iter()
    .find(|field| matches!(field.kind, PartFieldKind::RootElement))
    .map(|field| (field.rust_type.clone(), format!("as_{}", part.module_name)))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_code::part_codegen_ir::{PartChildCardinality, PartFieldDecl, PartModuleDecl};

  #[test]
  fn generates_part_handle_from_codegen_ir() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      relationship_type: "rel".to_string(),
      path_prefix: "word".to_string(),
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("pub struct MainDocumentPart"));
    assert!(rendered.contains("id : crate :: common :: PartId"));
    assert!(rendered.contains("ooxmlsdk_derive :: SdkPart"));
    assert!(!rendered.contains("impl crate :: sdk :: SdkPartHandle for MainDocumentPart"));
    assert!(!rendered.contains("pub fn relationships"));
  }

  #[test]
  fn generates_root_element_method_for_xml_part() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      root_element_type: Some(
        "crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document"
          .to_string(),
      ),
      root_element_accessor_name: Some("as_main_document_part".to_string()),
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("root_element : crate :: sdk :: PartRoot"));
    assert!(rendered.contains("part_root"));
    assert!(rendered.contains("as_main_document_part"));
  }

  #[test]
  fn generates_part_ref_enum_from_codegen_ir() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      root_element_type: Some(
        "crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document"
          .to_string(),
      ),
      root_element_accessor_name: Some("as_main_document_part".to_string()),
      ..Default::default()
    };

    let rendered = gen_parts_mod(&[&part]).unwrap().to_string();
    assert!(rendered.contains("pub enum PartRef"));
    assert!(
      rendered
        .contains("MainDocumentPart (crate :: parts :: main_document_part :: MainDocumentPart)")
    );
    assert!(rendered.contains("pub fn downcast < T : crate :: sdk :: SdkPartHandle"));
    assert!(rendered.contains("pub struct IdPartPair < 'a >"));
    assert!(rendered.contains("pub enum PartRootElement"));
    assert!(rendered.contains("pub trait PartRootCache"));
    assert!(rendered.contains("pub fn initialize_root_elements"));
    assert!(rendered.contains("pub fn from_part_id"));
    assert!(rendered.contains("PackageOpenMode :: Eager"));
    assert!(rendered.contains(
      "MainDocumentPart (Box < crate :: schemas :: schemas_openxmlformats_org_wordprocessingml_2006_main :: Document >)"
    ));
    assert!(rendered.contains("pub fn as_main_document_part"));
    assert!(rendered.contains("pub fn as_main_document_part_mut"));
  }

  #[test]
  fn generates_package_module_with_storage_fields() {
    let part = PartModuleDecl {
      module_name: "wordprocessing_document".to_string(),
      struct_name: "WordprocessingDocument".to_string(),
      is_package: true,
      main_part_module_name: Some("main_document_part".to_string()),
      main_part_struct_name: Some("MainDocumentPart".to_string()),
      main_part_accessor_name: Some("main_document_part".to_string()),
      fields: vec![PartFieldDecl {
        rust_name: "main_document_part".to_string(),
        rust_type: "crate::parts::main_document_part::MainDocumentPart".to_string(),
        kind: PartFieldKind::ChildPart {
          relationship_type:
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
              .to_string(),
          cardinality: PartChildCardinality::Required,
        },
        ..Default::default()
      }],
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("ooxmlsdk_derive :: SdkPackage"));
    assert!(rendered.contains("storage : crate :: common :: SdkPackageStorage"));
    assert!(rendered.contains("main_part_id : Option < crate :: common :: PartId >"));
    assert!(
      rendered.contains("root_elements : Vec < Option < crate :: parts :: PartRootElement > >")
    );
    assert!(rendered.contains("package_main"));
    assert!(rendered.contains("main_document_part : crate :: sdk :: RequiredPart"));
    assert!(!rendered.contains("ooxmlsdk_derive :: SdkPart"));
  }

  #[test]
  fn excludes_packages_from_part_ref_enum() {
    let package = PartModuleDecl {
      module_name: "wordprocessing_document".to_string(),
      struct_name: "WordprocessingDocument".to_string(),
      is_package: true,
      ..Default::default()
    };
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      ..Default::default()
    };

    let rendered = gen_parts_mod(&[&package, &part]).unwrap().to_string();
    assert!(rendered.contains("pub mod wordprocessing_document"));
    assert!(!rendered.contains("WordprocessingDocument (& 'a"));
    assert!(rendered.contains("MainDocumentPart (crate :: parts"));
    assert!(!rendered.contains("impl crate :: parts :: PartRootCache"));
  }
}
