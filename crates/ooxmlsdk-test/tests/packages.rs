#![cfg(feature = "parts")]

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  BodyChildChoice, DocumentChildChoice, SdtBlockChildChoice, SdtPropertiesChildChoice,
};
use ooxmlsdk_test::fixtures;

fn test_file_path(file_name: &str) -> std::path::PathBuf {
  let path = fixtures::doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn main_document_body_child_count(
  document: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
) -> usize {
  document
    .children
    .iter()
    .find_map(|child| match child {
      DocumentChildChoice::WBody(body) => Some(body.children.len()),
      _ => None,
    })
    .unwrap_or_default()
}

fn first_body_child(
  document: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
) -> Option<&BodyChildChoice> {
  document.children.iter().find_map(|child| match child {
    DocumentChildChoice::WBody(body) => body.children.first(),
    _ => None,
  })
}

#[test]
fn open_wordprocessing_document_asset_from_openxml_sdk() {
  let path = test_file_path("May_12_04.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.inner_path, "");
  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.relationships.is_some());
  assert!(package.relationships.as_ref().unwrap().relationship.len() >= 3);
  assert!(package.main_document_part.relationships.is_some());
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert!(package.main_document_part.document_settings_part.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_document_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Document.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.document_settings_part.is_some());
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_spreadsheet_document_asset_from_openxml_sdk() {
  let path = test_file_path("basicspreadsheet.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.inner_path, "");
  assert_eq!(package.workbook_part.inner_path, "xl/workbook.xml");
  assert!(package.relationships.is_some());
  assert!(package.relationships.as_ref().unwrap().relationship.len() >= 3);
  assert!(package.workbook_part.relationships.is_some());
  assert!(!package.workbook_part.worksheet_parts.is_empty());
  assert!(package.workbook_part.workbook_styles_part.is_some());
  assert!(package.workbook_part.theme_part.is_some());
  assert!(!package.workbook_part.root_element.sheets.x_sheet.is_empty());
}

#[test]
fn open_spreadsheet_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Spreadsheet.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.workbook_part.inner_path, "xl/workbook.xml");
  assert!(package.workbook_part.relationships.is_some());
  assert!(!package.workbook_part.worksheet_parts.is_empty());
  assert!(package.workbook_part.workbook_styles_part.is_some());
}

#[test]
fn open_presentation_document_asset_from_openxml_sdk() {
  let path = test_file_path("Presentation.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.inner_path, "");
  assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
  assert!(package.relationships.is_some());
  assert!(package.relationships.as_ref().unwrap().relationship.len() >= 3);
  assert!(package.presentation_part.relationships.is_some());
  assert!(!package.presentation_part.slide_parts.is_empty());
  assert!(!package.presentation_part.slide_master_parts.is_empty());
  assert!(package.presentation_part.theme_part.is_some());
  assert!(
    package
      .presentation_part
      .root_element
      .slide_id_list
      .is_some()
  );
}

#[test]
#[cfg(feature = "microsoft365")]
fn open_hello_world_docx_asset_from_openxml_sdk() {
  let path = test_file_path("HelloWorld.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
  assert!(package.main_document_part.style_definitions_part.is_some());
}

#[test]
fn open_simple_sdt_docx_asset_from_openxml_sdk() {
  let path = test_file_path("simpleSdt.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  let Some(BodyChildChoice::WSdt(sdt)) = first_body_child(&package.main_document_part.root_element)
  else {
    panic!("expected first body child to be w:sdt");
  };

  assert!(matches!(
    sdt.children.first(),
    Some(SdtBlockChildChoice::WSdtPr(_))
  ));
  assert!(matches!(
    sdt.children.get(1),
    Some(SdtBlockChildChoice::WSdtContent(_))
  ));

  let Some(SdtBlockChildChoice::WSdtPr(properties)) = sdt.children.first() else {
    panic!("expected w:sdtPr");
  };
  let Some(SdtPropertiesChildChoice::WAlias(alias)) = properties.children.first() else {
    panic!("expected w:alias");
  };

  assert_eq!(alias.val.as_str(), "SDT1");
}

#[test]
fn open_mcexecl_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("MCExecl.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert!(package.workbook_part.shared_string_table_part.is_some());
  let shared_string_table = &package
    .workbook_part
    .shared_string_table_part
    .as_ref()
    .unwrap()
    .root_element;
  assert_eq!(shared_string_table.x_si.len(), 1);

  let first_item = &shared_string_table.x_si[0];
  assert!(first_item.text.is_some());
  assert_eq!(
    first_item
      .text
      .as_ref()
      .and_then(|text| text.xml_content.as_ref())
      .map(|value| value.as_str()),
    Some("abc")
  );
}

#[test]
fn open_mcppt_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("mcppt.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert!(package.presentation_part.table_styles_part.is_some());
  let table_style_list = &package
    .presentation_part
    .table_styles_part
    .as_ref()
    .unwrap()
    .root_element;

  assert_eq!(
    table_style_list.default.as_str(),
    "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"
  );
}
