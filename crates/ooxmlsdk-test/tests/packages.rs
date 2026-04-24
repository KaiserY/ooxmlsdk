#![cfg(feature = "parts")]

use std::io::Cursor;

use ooxmlsdk::parts::{
  PartRef, PartRootCache, main_document_part::MainDocumentPart,
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  style_definitions_part::StyleDefinitionsPart, wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::sdk::SdkPartHandle;
use ooxmlsdk_test::fixtures;

fn doc_sample(file_name: &str) -> std::path::PathBuf {
  let path = fixtures::doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn main_document_body_child_count(
  document: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
) -> usize {
  document
    .body
    .as_ref()
    .map(|body| body.body_choice.len() + usize::from(body.w_sect_pr.is_some()))
    .unwrap_or_default()
}

#[test]
fn wordprocessing_document_supports_eager_and_lazy_root_loading() {
  let path = doc_sample("Of16-01.docx");

  let mut eager = WordprocessingDocument::new_from_file(&path).unwrap();
  let eager_main = eager.main_document_part().unwrap();
  let eager_root = eager_main.root_element(&mut eager).unwrap();
  assert!(main_document_body_child_count(eager_root) > 0);

  let mut lazy = WordprocessingDocument::new_from_file_lazy(&path).unwrap();
  let lazy_main = lazy.main_document_part().unwrap();
  let lazy_root = lazy_main.root_element(&mut lazy).unwrap();
  assert!(main_document_body_child_count(lazy_root) > 0);
}

#[test]
fn package_relationships_resolve_with_container_local_part_factory() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let main_part_id = package.get_id_of_part(main_part).unwrap();

  let resolved = package
    .try_get_part_by_id(main_part_id)
    .and_then(PartRef::downcast::<MainDocumentPart>)
    .unwrap();

  assert_eq!(resolved.part_id(), main_part.part_id());
  assert_eq!(package.parts().count(), package.relationships().len());
  assert_eq!(
    package
      .get_sub_part_of_type::<MainDocumentPart>()
      .unwrap()
      .part_id(),
    main_part.part_id()
  );
  assert_eq!(package.get_parts_of_type::<MainDocumentPart>().count(), 1);
}

#[test]
fn wordprocessing_child_accessors_are_relationship_backed_handles() {
  let package = WordprocessingDocument::new_from_file(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let styles_part = main_part.style_definitions_part(&package).unwrap();

  assert!(main_part.relationships(&package).is_some());
  assert_eq!(
    main_part.get_id_of_part(&package, styles_part),
    main_part
      .relationships(&package)
      .unwrap()
      .iter()
      .find(|relationship| {
        ooxmlsdk::common::relationship_type_matches(
          relationship.relationship_type(),
          StyleDefinitionsPart::RELATIONSHIP_TYPE,
        )
      })
      .map(|relationship| relationship.id())
  );
  assert_eq!(
    main_part
      .get_sub_part_of_type::<_, StyleDefinitionsPart>(&package)
      .unwrap()
      .part_id(),
    styles_part.part_id()
  );
  assert!(styles_part.path(&package).unwrap().ends_with("styles.xml"));
  assert!(
    styles_part
      .content_type(&package)
      .unwrap()
      .ends_with("+xml")
  );
  assert!(!styles_part.data(&package).unwrap().is_empty());
  assert!(main_part.document_settings_part(&package).is_some());
  assert_eq!(
    main_part
      .parts(&package)
      .filter(|pair| pair.part.downcast::<MainDocumentPart>().is_some())
      .count(),
    0
  );
}

#[test]
fn root_cache_reports_and_unloads_lazy_roots() {
  let mut package = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let main_part = package.main_document_part().unwrap();
  let part_id = main_part.part_id();

  assert!(!package.is_root_element_loaded(part_id));
  assert!(main_part.root_element(&mut package).is_ok());
  assert!(package.is_root_element_loaded(part_id));
  assert!(package.unload_root_element(part_id).is_some());
  assert!(!package.is_root_element_loaded(part_id));
}

#[test]
fn spreadsheet_child_accessors_resolve_repeated_parts() {
  let package = SpreadsheetDocument::new_from_file(doc_sample("basicspreadsheet.xlsx")).unwrap();
  let workbook_part = package.workbook_part().unwrap();

  assert!(workbook_part.relationships(&package).is_some());
  assert!(workbook_part.theme_part(&package).is_some());
  assert!(workbook_part.worksheet_parts(&package).count() >= 1);
}

#[test]
fn presentation_child_accessors_resolve_repeated_parts() {
  let package = PresentationDocument::new_from_file(doc_sample("mcppt.pptx")).unwrap();
  let presentation_part = package.presentation_part().unwrap();

  assert!(presentation_part.relationships(&package).is_some());
  assert!(presentation_part.slide_parts(&package).count() >= 1);
  assert!(presentation_part.slide_master_parts(&package).count() >= 1);
}

#[test]
fn package_save_roundtrips_unmodified_storage() {
  let original = WordprocessingDocument::new_from_file_lazy(doc_sample("Of16-01.docx")).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let mut roundtripped = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  let main_part = roundtripped.main_document_part().unwrap();
  let root = main_part.root_element(&mut roundtripped).unwrap();
  assert!(main_document_body_child_count(root) > 0);
}
