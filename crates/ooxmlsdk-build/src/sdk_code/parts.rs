use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemMod, ItemStruct, Type, parse_str, parse2};

use crate::Result;
use crate::sdk_code::part_codegen_ir::{
  PartChildCardinality, PartFieldDecl, PartFieldKind, PartModuleDecl,
};
use crate::sdk_code::versioning::features_cfg_attrs;

pub fn gen_part_module(part: &PartModuleDecl) -> Result<TokenStream> {
  gen_part_module_with_relationship_type_variants(part)
}

pub fn gen_part_module_with_relationship_type_variants(
  part: &PartModuleDecl,
) -> Result<TokenStream> {
  if is_package_part(part) {
    return gen_package_module(part);
  }

  gen_part_handle_module(part)
}

fn gen_package_module(part: &PartModuleDecl) -> Result<TokenStream> {
  let struct_name_ident: Ident = parse_str(&part.struct_name)?;
  let marker_fields = package_marker_fields(part)?;
  let package_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
    pub struct #struct_name_ident {
      pub(crate) storage: crate::common::SdkPackageStorage,
      pub(crate) open_settings: crate::sdk::OpenSettings,
      pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
      #( #marker_fields )*
    }
  })?;

  Ok(quote! {
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
    let field_ty = part_child_field_type(*cardinality, &part_ty);
    let main_attr = if Some(field.rust_name.as_str()) == main_part_accessor {
      let accessor = field.rust_name.as_str();
      quote! { #[sdk(package_main(accessor = #accessor))] }
    } else {
      quote! {}
    };
    let kind = part_child_kind_value(*cardinality);
    let kind_attr = if *cardinality == PartChildCardinality::Optional {
      quote! {}
    } else {
      quote! { #[sdk(part_child(kind = #kind))] }
    };
    fields.push(quote! {
      #( #attrs )*
      #main_attr
      #kind_attr
      pub(crate) #field_ident: #field_ty,
    });
  }

  Ok(fields)
}

fn gen_part_handle_module(part: &PartModuleDecl) -> Result<TokenStream> {
  let struct_name_ident: Ident = parse_str(&part.struct_name)?;
  let marker_fields = part_handle_marker_fields(part)?;
  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
    pub struct #struct_name_ident {
      pub(crate) id: crate::common::PartId,
      #( #marker_fields )*
    }
  })?;

  Ok(quote! {
    #part_struct
  })
}

fn part_handle_marker_fields(part: &PartModuleDecl) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  if let Some((root_type, _accessor_name)) = part_root_element_info(part) {
    let root_ty: Type = parse_str(&root_type)?;
    fields.push(quote! {
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
    let kind_attr = match cardinality {
      PartChildCardinality::RequiredRepeated => {
        quote! { #[sdk(part_child(kind = "required_repeated"))] }
      }
      _ => quote! {},
    };
    let field_ty = match cardinality {
      PartChildCardinality::Optional => quote! { crate::sdk::OptionalPart<#part_ty> },
      PartChildCardinality::Required => quote! { crate::sdk::RequiredPart<#part_ty> },
      PartChildCardinality::Repeated | PartChildCardinality::RequiredRepeated => {
        quote! { crate::sdk::RepeatedPart<#part_ty> }
      }
    };
    fields.push(quote! {
      #( #attrs )*
      #kind_attr
      pub(crate) #field_ident: #field_ty,
    });
  }

  Ok(fields)
}

fn part_child_field_type(cardinality: PartChildCardinality, part_ty: &Type) -> TokenStream {
  match cardinality {
    PartChildCardinality::Optional => quote! { crate::sdk::OptionalPart<#part_ty> },
    PartChildCardinality::Required => quote! { crate::sdk::RequiredPart<#part_ty> },
    PartChildCardinality::Repeated | PartChildCardinality::RequiredRepeated => {
      quote! { crate::sdk::RepeatedPart<#part_ty> }
    }
  }
}

fn part_child_kind_value(cardinality: PartChildCardinality) -> &'static str {
  match cardinality {
    PartChildCardinality::Optional => "optional",
    PartChildCardinality::Required => "required",
    PartChildCardinality::Repeated => "repeated",
    PartChildCardinality::RequiredRepeated => "required_repeated",
  }
}

pub fn gen_parts_mod(parts: &[&PartModuleDecl]) -> Result<TokenStream> {
  let mut mod_list: Vec<ItemMod> = vec![];
  let mut part_ref_variants: Vec<TokenStream> = vec![];

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

    let relationship_type = part.relationship_type.as_str();
    let path_prefix = part.path_prefix.as_str();
    let content_type = part.content_type.as_str();
    let target_name = part.target_name.as_str();
    let extension = part.extension.as_str();
    let path_prefix_attr = if path_prefix == "." {
      quote! {}
    } else {
      quote! { , path_prefix = #path_prefix }
    };
    let content_type_attr = if content_type.is_empty() {
      quote! {}
    } else {
      quote! { , content_type = #content_type }
    };
    let extension_attr = if extension.is_empty() {
      quote! {}
    } else {
      quote! { , extension = #extension }
    };
    let root_attr = if let Some((root_type, _accessor_name)) = part_root_element_info(part) {
      let root_ty: Type = parse_str(&root_type)?;
      let root_content_type = root_part_content_type(part);
      let root_content_type_attr = if root_content_type == content_type {
        quote! {}
      } else {
        quote! { , content_type = #root_content_type }
      };
      quote! {
        , root(
          element = #root_ty
          #root_content_type_attr
        )
      }
    } else {
      quote! {}
    };
    part_ref_variants.push(quote! {
      #( #part_attrs )*
      #[sdk(
        relationship_type = #relationship_type,
        target_name = #target_name
        #path_prefix_attr
        #content_type_attr
        #extension_attr
        #root_attr
      )]
      #struct_ident(#part_ty),
    });
  }

  Ok(quote! {
    #( #mod_list )*
    pub mod extended_part;

    #[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPartRef)]
    pub enum PartRef {
      #( #part_ref_variants )*
      ExtendedPart(crate::parts::extended_part::ExtendedPart),
    }

    #[derive(Clone, Debug)]
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

    pub(crate) fn initialize_root_elements(
      storage: &mut crate::common::SdkPackageStorage,
      open_settings: &crate::sdk::OpenSettings,
    ) -> Result<Vec<Option<crate::parts::PartRootElement>>, crate::common::SdkError> {
      let mut root_elements = vec![None; storage.parts().len()];
      if matches!(
        open_settings.root_element_open_mode(),
        crate::common::PackageOpenMode::Eager
      ) {
        for (index, slot) in root_elements.iter_mut().enumerate() {
          let part_id = crate::common::PartId::from_index(index);
          let root_element = crate::parts::PartRootElement::from_part_id(storage, part_id, open_settings)?;
          if let Some(root_element) = root_element {
            storage.discard_cached_part_bytes(part_id);
            *slot = Some(root_element);
          }
        }
      }
      Ok(root_elements)
    }

    pub(crate) fn load_all_part_roots<P>(
      package: &mut P,
    ) -> Result<(), crate::common::SdkError>
    where
      P: crate::sdk::SdkPackage,
    {
      validate_missing_internal_relationships(package)?;

      let part_count = crate::sdk::SdkPackage::storage(package).parts().len();
      for index in 0..part_count {
        let part_id = crate::common::PartId::from_index(index);
        if crate::sdk::SdkPackage::storage(package).part(part_id).is_none()
          || crate::sdk::SdkPackage::is_root_element_loaded(package, part_id)
        {
          continue;
        }

        let root_element = crate::parts::PartRootElement::from_part_id(
          crate::sdk::SdkPackage::storage(package),
          part_id,
          crate::sdk::SdkPackage::open_settings(package),
        )?;
        if root_element.is_some() {
          crate::sdk::SdkPackage::storage_mut(package).discard_cached_part_bytes(part_id);
        }
        if let Some(root_element) = root_element
          && let Some(slot) = crate::sdk::SdkPackage::root_element_slot_mut(package, part_id)
        {
          *slot = Some(root_element);
        }
      }

      Ok(())
    }

    fn validate_missing_internal_relationships<P>(
      package: &P,
    ) -> Result<(), crate::common::SdkError>
    where
      P: crate::sdk::SdkPackage,
    {
      let storage = crate::sdk::SdkPackage::storage(package);
      let open_settings = crate::sdk::SdkPackage::open_settings(package);

      for relationship in storage.package_relationships().iter() {
        validate_missing_internal_relationship(open_settings, relationship)?;
      }

      for part in storage.parts().iter().filter(|part| !part.is_deleted()) {
        for relationship in part.relationships().iter() {
          validate_missing_internal_relationship(open_settings, relationship)?;
        }
      }

      Ok(())
    }

    fn validate_missing_internal_relationship(
      open_settings: &crate::sdk::OpenSettings,
      relationship: &crate::common::RelationshipInfo,
    ) -> Result<(), crate::common::SdkError>
    {
      if !matches!(
        relationship.target_kind(),
        crate::common::RelationshipTargetKind::Missing
      ) || should_ignore_missing_relationship(open_settings, relationship)
      {
        return Ok(());
      }

      Err(crate::common::SdkError::CommonError(format!(
        "relationship {} targets missing part {}",
        relationship.id(),
        relationship.target()
      )))
    }

    fn should_ignore_missing_relationship(
      open_settings: &crate::sdk::OpenSettings,
      relationship: &crate::common::RelationshipInfo,
    ) -> bool
    {
      open_settings.ignore_calculation_chain_part_relationship
        && relationship.relationship_type_bytes()
          == b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain"
    }

    pub fn save_package<P, W>(
      package: &P,
      writer: W,
    ) -> Result<(), crate::common::SdkError>
    where
      P: crate::sdk::SdkPackage,
      W: std::io::Write + std::io::Seek,
    {
      use std::io::Write as _;

      let mut zip = zip::ZipWriter::new(writer);
      let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
      let storage = crate::sdk::SdkPackage::storage(package);
      storage.configure_zip_writer(&mut zip)?;

      let must_serialize_root = storage.parts().iter().enumerate().any(|(index, part)| {
        !part.is_deleted()
          && crate::sdk::SdkPackage::should_serialize_root_element(
            package,
            crate::common::PartId::from_index(index),
          )
      });
      if !must_serialize_root && storage.merge_original_archive_if_unchanged(&mut zip)? {
        zip.finish()?;
        return Ok(());
      }

      for entry in storage.package_save_entries() {
        match entry {
          crate::common::PackageSaveEntry::ArchivedExtra(entry_index) => {
            storage.raw_copy_archived_extra(entry_index, &mut zip)?;
          }
          crate::common::PackageSaveEntry::ContentTypes => {
            if let Some(entry_index) = storage.unchanged_content_types_archive_entry_index() {
              storage.raw_copy_archive_entry(
                entry_index,
                "[Content_Types].xml",
                &mut zip,
              )?;
            } else {
              zip.start_file("[Content_Types].xml", options)?;
              crate::sdk::SdkType::write_to(storage.content_types(), &mut zip)?;
            }
          }
          crate::common::PackageSaveEntry::PackageRelationships => {
            let relationships = storage.package_relationships();
            if let Some(entry_index) = relationships.unchanged_archive_entry_index() {
              storage.raw_copy_archive_entry(entry_index, "_rels/.rels", &mut zip)?;
            } else {
              zip.start_file("_rels/.rels", options)?;
              relationships.write_xml(&mut zip)?;
            }
          }
          crate::common::PackageSaveEntry::PartRelationships(part_id) => {
            let part = storage.part(part_id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {part_id:?} is not present in package storage"
              ))
            })?;
            let relationships = storage.relationships(part_id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {part_id:?} has no relationship set"
              ))
            })?;
            let rels_path = crate::common::part_relationships_path(part.path());
            if let Some(entry_index) = relationships.unchanged_archive_entry_index() {
              storage.raw_copy_archive_entry(entry_index, &rels_path, &mut zip)?;
            } else {
              zip.start_file(&rels_path, options)?;
              relationships.write_xml(&mut zip)?;
            }
          }
          crate::common::PackageSaveEntry::Part(part_id) => {
            let part = storage.part(part_id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {part_id:?} is not present in package storage"
              ))
            })?;
            if crate::sdk::SdkPackage::should_serialize_root_element(package, part_id)
              && let Some(root_element) = crate::sdk::SdkPackage::root_element(package, part_id)
            {
              zip.start_file(part.path(), options)?;
              root_element.write_to(&mut zip)?;
            } else if !storage.raw_copy_part(part_id, &mut zip)? {
              zip.start_file(part.path(), options)?;
              zip.write_all(storage.part_bytes(part_id)?)?;
            }
          }
        }
      }

      zip.finish()?;
      Ok(())
    }

    pub fn save_package_to_file<P>(
      package: &P,
      path: &std::path::Path,
    ) -> Result<(), crate::common::SdkError>
    where
      P: crate::sdk::SdkPackage,
    {
      use std::io::Write as _;

      let (temporary_path, mut file) = crate::common::create_package_temp_file(path)?;
      let write_result = (|| {
        let mut writer = std::io::BufWriter::new(&mut file);
        save_package(package, &mut writer)?;
        writer.flush()?;
        drop(writer);
        Ok::<(), crate::common::SdkError>(())
      })();
      if let Err(error) = write_result {
        let _ = std::fs::remove_file(&temporary_path);
        return Err(error);
      }
      drop(file);

      if let Err(error) = crate::common::replace_package_file(&temporary_path, path) {
        let _ = std::fs::remove_file(&temporary_path);
        return Err(error);
      }
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

fn root_part_content_type(part: &PartModuleDecl) -> &str {
  if !part.content_type.is_empty() {
    return part.content_type.as_str();
  }

  match part.struct_name.as_str() {
    "MainDocumentPart" => {
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
    }
    "WorkbookPart" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
    "PresentationPart" => {
      "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
    }
    _ => "",
  }
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
    assert!(!rendered.contains("impl crate :: sdk :: SdkPart for MainDocumentPart"));
    assert!(!rendered.contains("pub fn relationships"));
  }

  #[test]
  fn generates_root_element_marker_for_xml_part() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
          .to_string(),
      root_element_type: Some(
        "crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document"
          .to_string(),
      ),
      root_element_accessor_name: Some("as_main_document_part".to_string()),
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("PartRoot"));
    assert!(rendered.contains("MainDocumentPart"));
    assert!(!rendered.contains("as_main_document_part"));
  }

  #[test]
  fn generates_child_part_markers() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      relationship_type: "rel".to_string(),
      fields: vec![
        PartFieldDecl {
          rust_name: "theme_part".to_string(),
          rust_type: "crate::parts::theme_part::ThemePart".to_string(),
          kind: PartFieldKind::ChildPart {
            relationship_type:
              "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
                .to_string(),
            cardinality: PartChildCardinality::Optional,
          },
          ..Default::default()
        },
        PartFieldDecl {
          rust_name: "custom_xml_parts".to_string(),
          rust_type: "crate::parts::custom_xml_part::CustomXmlPart".to_string(),
          kind: PartFieldKind::ChildPart {
            relationship_type:
              "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
                .to_string(),
            cardinality: PartChildCardinality::Repeated,
          },
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let rendered = gen_part_module(&part).unwrap().to_string();
    assert!(rendered.contains("OptionalPart < crate :: parts :: theme_part :: ThemePart >"));
    assert!(
      rendered.contains("RepeatedPart < crate :: parts :: custom_xml_part :: CustomXmlPart >")
    );
    assert!(!rendered.contains("child_part_by_relationship_type"));
    assert!(!rendered.contains("child_parts_by_relationship_type"));
  }

  #[test]
  fn generates_part_ref_enum_from_codegen_ir() {
    let part = PartModuleDecl {
      module_name: "main_document_part".to_string(),
      struct_name: "MainDocumentPart".to_string(),
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
          .to_string(),
      root_element_type: Some(
        "crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document"
          .to_string(),
      ),
      root_element_accessor_name: Some("as_main_document_part".to_string()),
      ..Default::default()
    };

    let rendered = gen_parts_mod(&[&part]).unwrap().to_string();
    assert!(rendered.contains("pub enum PartRef"));
    assert!(rendered.contains("ooxmlsdk_derive :: SdkPartRef"));
    assert!(
      rendered.contains(
        "relationship_type = \"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\""
      )
    );
    assert!(rendered.contains("root ("));
    assert!(rendered.contains("pub mod extended_part"));
    assert!(
      rendered
        .contains("MainDocumentPart (crate :: parts :: main_document_part :: MainDocumentPart)")
    );
    assert!(rendered.contains("ExtendedPart (crate :: parts :: extended_part :: ExtendedPart)"));
    assert!(!rendered.contains("PartRefDowncast"));
    assert!(!rendered.contains("pub fn downcast"));
    assert!(rendered.contains("pub struct IdPartPair < 'a >"));
    assert!(!rendered.contains("define_part_root_element"));
    assert!(!rendered.contains("pub trait PartRootCache"));
    assert!(rendered.contains("pub (crate) fn initialize_root_elements"));
    assert!(rendered.contains("PackageOpenMode :: Eager"));
    assert!(rendered.contains(
      "element = crate :: schemas :: schemas_openxmlformats_org_wordprocessingml_2006_main :: Document"
    ));
    assert!(!rendered.contains("as_main_document_part"));
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
    assert!(!rendered.contains("main_part_id"));
    assert!(
      rendered.contains("root_elements : Vec < Option < crate :: parts :: PartRootElement > >")
    );
    assert!(rendered.contains("package_main"));
    assert!(rendered.contains(
      "main_document_part : crate :: sdk :: RequiredPart < crate :: parts :: main_document_part :: MainDocumentPart >"
    ));
    assert!(!rendered.contains("fallback_parts"));
    assert!(!rendered.contains("relationship_order"));
    assert!(!rendered.contains("modeled_relationships"));
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
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
          .to_string(),
      ..Default::default()
    };

    let rendered = gen_parts_mod(&[&package, &part]).unwrap().to_string();
    assert!(rendered.contains("pub mod wordprocessing_document"));
    assert!(!rendered.contains("WordprocessingDocument (& 'a"));
    assert!(rendered.contains("MainDocumentPart (crate :: parts"));
    assert!(!rendered.contains("impl crate :: parts :: PartRootCache"));
  }
}
