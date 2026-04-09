#![cfg(feature = "parts")]

use std::io::Cursor;

use ooxmlsdk::common::SdkError;
use ooxmlsdk::parts::alternative_format_import_part::AlternativeFormatImportPart;
use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_core_properties::KeywordsChildChoice;
use ooxmlsdk::schemas::opc_relationships::Relationship;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  AltChunk, Body, BodyChildChoice, CommentChoice, Document, Hyperlink, HyperlinkChildChoice,
  Paragraph, ParagraphChoice, Run, RunChildChoice, SdtPropertiesChildChoice,
};
use ooxmlsdk_test::fixtures;

const ALT_CHUNK_ID: &str = "XmlAltChunkId-1";
const ALT_CHUNK_TARGET: &str = "afchunk.xml";
const ALT_CHUNK_XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><root><element>Some text</element></root>";

fn keywords_text(value: Option<&ooxmlsdk::schemas::opc_core_properties::Keywords>) -> Option<&str> {
  value.and_then(|keywords| {
    keywords.children.iter().find_map(|child| match child {
      KeywordsChildChoice::Text(value) => Some(value.as_str()),
      KeywordsChildChoice::CpValue(value) => value.xml_content.as_deref(),
    })
  })
}

fn test_file_path(file_name: &str) -> std::path::PathBuf {
  let path = fixtures::doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn roundtrip_wordprocessing_document(
  path: &std::path::Path,
) -> (WordprocessingDocument, WordprocessingDocument) {
  let original = WordprocessingDocument::new_from_file(path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();

  (original, roundtripped)
}

fn roundtrip_spreadsheet_document(
  path: &std::path::Path,
) -> (SpreadsheetDocument, SpreadsheetDocument) {
  let original = SpreadsheetDocument::new_from_file(path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped = SpreadsheetDocument::new(Cursor::new(buffer.into_inner())).unwrap();

  (original, roundtripped)
}

fn roundtrip_presentation_document(
  path: &std::path::Path,
) -> (PresentationDocument, PresentationDocument) {
  let original = PresentationDocument::new_from_file(path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped = PresentationDocument::new(Cursor::new(buffer.into_inner())).unwrap();

  (original, roundtripped)
}

fn stylesheet_extensions(
  package: &SpreadsheetDocument,
) -> &[ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::StylesheetExtension] {
  package
    .workbook_part
    .workbook_styles_part
    .as_ref()
    .expect("expected workbook styles part")
    .root_element
    .stylesheet_extension_list
    .as_ref()
    .expect("expected stylesheet extension list")
    .x_ext
    .as_slice()
}

fn main_document_body_child_count(
  document: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
) -> usize {
  document
    .body
    .as_deref()
    .map(|body| body.children.len())
    .unwrap_or_default()
}

fn first_body_child(document: &Document) -> Option<&BodyChildChoice> {
  document
    .body
    .as_deref()
    .and_then(|body| body.children.first())
}

fn first_body_mut(document: &mut Document) -> Option<&mut Body> {
  document.body.as_deref_mut()
}

fn first_body(document: &Document) -> Option<&Body> {
  document.body.as_deref()
}

fn assert_unexpected_tag(
  result: Result<WordprocessingDocument, SdkError>,
  expected_ty: &'static str,
  expected_found: &str,
) {
  match result {
    Err(SdkError::UnexpectedTag { ty, found, .. }) => {
      assert_eq!(ty, expected_ty);
      assert_eq!(found, expected_found);
    }
    Err(err) => panic!("expected UnexpectedTag, got {err:?}"),
    Ok(_) => panic!("expected failure, got Ok(_)"),
  }
}

fn first_paragraph(
  body: &Body,
) -> Option<&ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph> {
  body.children.iter().find_map(|child| match child {
    BodyChildChoice::WP(paragraph) => Some(paragraph.as_ref()),
    _ => None,
  })
}

fn body_paragraph_count(body: &Body) -> usize {
  body
    .children
    .iter()
    .filter(|child| matches!(child, BodyChildChoice::WP(_)))
    .count()
}

fn first_hyperlink(paragraph: &Paragraph) -> Option<&Hyperlink> {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(|child| match child {
      ParagraphChoice::WHyperlink(hyperlink) => Some(hyperlink.as_ref()),
      _ => None,
    })
}

fn paragraph_bookmark_start_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| matches!(child, ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ParagraphChoice::WBookmarkStart(_)))
    .count()
}

fn paragraph_bookmark_end_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| matches!(child, ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ParagraphChoice::WBookmarkEnd(_)))
    .count()
}

fn paragraph_comment_range_start_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| matches!(child, ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ParagraphChoice::WCommentRangeStart(_)))
    .count()
}

fn paragraph_comment_range_end_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| matches!(child, ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ParagraphChoice::WCommentRangeEnd(_)))
    .count()
}

fn paragraph_comment_reference_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter_map(|child| match child {
      ParagraphChoice::WR(run) => Some(run.as_ref()),
      _ => None,
    })
    .map(|run| {
      run
        .children
        .iter()
        .filter(|run_child| matches!(run_child, RunChildChoice::WCommentReference(_)))
        .count()
    })
    .sum()
}

fn first_paragraph_text(paragraph: &Paragraph) -> Option<&str> {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(|child| match child {
      ParagraphChoice::WR(run) => run.children.iter().find_map(|run_child| match run_child {
        RunChildChoice::WT(text) => text.xml_content.as_deref(),
        _ => None,
      }),
      _ => None,
    })
}

fn append_run_text(run: &Run, out: &mut String) {
  for run_child in &run.children {
    if let RunChildChoice::WT(text) = run_child
      && let Some(value) = text.xml_content.as_deref()
    {
      out.push_str(value);
    }
  }
}

fn paragraph_text(paragraph: &Paragraph) -> String {
  let mut text = String::new();

  for child in &paragraph.paragraph_choice {
    match child {
      ParagraphChoice::WR(run) => append_run_text(run, &mut text),
      ParagraphChoice::WHyperlink(hyperlink) => {
        for hyperlink_child in &hyperlink.children {
          if let HyperlinkChildChoice::WR(run) = hyperlink_child {
            append_run_text(run.as_ref(), &mut text);
          }
        }
      }
      _ => {}
    }
  }

  text
}

fn comment_text(
  comment: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comment,
) -> String {
  let mut text = String::new();

  for child in &comment.comment_choice {
    if let CommentChoice::WP(paragraph) = child {
      text.push_str(&paragraph_text(paragraph.as_ref()));
    }
  }

  text
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
fn open_document_dotx_asset_from_openxml_sdk() {
  let path = test_file_path("Document.dotx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.document_settings_part.is_some());
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert_eq!(
    main_document_body_child_count(&package.main_document_part.root_element),
    6
  );
}

#[cfg(feature = "strict")]
#[test]
fn open_strict01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Strict01.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
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
fn open_presentation_potx_asset_from_openxml_sdk() {
  let path = test_file_path("Presentation.potx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.inner_path, "");
  assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
  assert!(package.presentation_part.relationships.is_some());
  assert!(!package.presentation_part.slide_parts.is_empty());
  assert!(!package.presentation_part.slide_master_parts.is_empty());
  assert!(package.presentation_part.theme_part.is_some());
}

#[test]
fn open_spreadsheet_xltx_asset_from_openxml_sdk() {
  let path = test_file_path("Spreadsheet.xltx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.inner_path, "");
  assert_eq!(package.workbook_part.inner_path, "xl/workbook.xml");
  assert!(package.workbook_part.relationships.is_some());
  assert!(!package.workbook_part.worksheet_parts.is_empty());
  assert!(!package.workbook_part.root_element.sheets.x_sheet.is_empty());
}

#[test]
fn open_mcinleaf_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mcinleaf.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-01.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_02_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-02.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_03_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-03.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_04_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-04.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_05_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-05.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_06_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-06.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_07_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-07.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_08_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-08.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_10_symex_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-10-SymEx.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_5errors_docx_asset_currently_fails() {
  let path = test_file_path("5Errors.docx");

  assert_unexpected_tag(
    WordprocessingDocument::new_from_file(&path),
    "ParagraphProperties",
    "w:pPr",
  );
}

#[test]
fn open_of16_09_unknown_element_docx_asset_currently_fails_on_cx_chart_data_intentionally_changed()
{
  let path = test_file_path("Of16-09-UnknownElement.docx");

  assert_unexpected_tag(
    WordprocessingDocument::new_from_file(&path),
    "ChartSpace",
    "cx:chartDataIntentionallyChanged",
  );
}

#[test]
fn open_unknown_element_docx_asset_currently_fails_on_w_p2() {
  let path = test_file_path("UnknownElement.docx");

  assert_unexpected_tag(WordprocessingDocument::new_from_file(&path), "Body", "w:p2");
}

#[test]
fn open_complex0_docx_asset_currently_fails_on_do_not_embed_smart_tags() {
  let path = test_file_path("complex0.docx");

  assert_unexpected_tag(
    WordprocessingDocument::new_from_file(&path),
    "Settings",
    "w:doNotEmbedSmartTags",
  );
}

#[test]
fn open_complex2010_docx_asset_currently_fails_on_do_not_embed_smart_tags() {
  let path = test_file_path("complex2010.docx");

  assert_unexpected_tag(
    WordprocessingDocument::new_from_file(&path),
    "Settings",
    "w:doNotEmbedSmartTags",
  );
}

#[test]
fn open_mailmerge_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mailmerge.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_svg_docx_asset_from_openxml_sdk() {
  let path = test_file_path("svg.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_of16_01_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-01.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
  assert!(package.presentation_part.relationships.is_some());
  assert!(!package.presentation_part.slide_parts.is_empty());
}

#[test]
fn open_of16_02_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-02.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
  assert!(package.presentation_part.relationships.is_some());
  assert!(!package.presentation_part.slide_parts.is_empty());
}

#[test]
fn open_of16_03_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-03.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
  assert!(package.presentation_part.relationships.is_some());
  assert!(!package.presentation_part.slide_parts.is_empty());
}

#[test]
fn round_trip_of16_01_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-01.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.inner_path,
    roundtripped.presentation_part.inner_path
  );
  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert!(!original.presentation_part.slide_parts.is_empty());
  assert!(!roundtripped.presentation_part.slide_parts.is_empty());
}

#[test]
fn round_trip_of16_02_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-02.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.inner_path,
    roundtripped.presentation_part.inner_path
  );
  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert!(!original.presentation_part.slide_parts.is_empty());
  assert!(!roundtripped.presentation_part.slide_parts.is_empty());
}

#[test]
fn round_trip_of16_03_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-03.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.inner_path,
    roundtripped.presentation_part.inner_path
  );
  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert!(!original.presentation_part.slide_parts.is_empty());
  assert!(!roundtripped.presentation_part.slide_parts.is_empty());
}

#[test]
fn open_data_bound_content_controls_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Data-Bound-Content-Controls.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
  assert!(package.main_document_part.style_definitions_part.is_some());
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

  assert!(sdt.sdt_properties.is_some());
  assert!(sdt.sdt_content_block.is_some());

  let Some(properties) = sdt.sdt_properties.as_deref() else {
    panic!("expected w:sdtPr");
  };
  let Some(SdtPropertiesChildChoice::WAlias(alias)) = properties.children.first() else {
    panic!("expected w:alias");
  };

  assert_eq!(alias.val.as_str(), "SDT1");
}

#[test]
fn open_hyperlink_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Hyperlink.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert_eq!(
    package
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(6)
  );

  let body = first_body(&package.main_document_part.root_element).expect("expected body");
  assert_eq!(body_paragraph_count(body), 2);

  let hyperlink_paragraph = body
    .children
    .iter()
    .filter_map(|child| match child {
      BodyChildChoice::WP(paragraph)
        if paragraph
          .paragraph_choice
          .iter()
          .any(|choice| matches!(choice, ParagraphChoice::WHyperlink(_))) =>
      {
        Some(paragraph.as_ref())
      }
      _ => None,
    })
    .next()
    .expect("expected paragraph with hyperlink");

  let hyperlink = first_hyperlink(hyperlink_paragraph).expect("expected hyperlink");
  assert_eq!(hyperlink.id.as_deref(), Some("rId4"));
  assert_eq!(hyperlink.history, Some(true));
  assert_eq!(paragraph_text(hyperlink_paragraph), "EricWhite.com");
}

#[test]
fn open_empty_relationship_element_docx_asset_from_openxml_sdk() {
  let path = test_file_path("EmptyRelationshipElement.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert_eq!(
    package
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(6)
  );

  let body = first_body(&package.main_document_part.root_element).expect("expected body");
  assert_eq!(body_paragraph_count(body), 2);
}

#[test]
fn open_no_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("NoDocProps.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.core_file_properties_part.is_none());
  assert!(package.extended_file_properties_part.is_none());
  assert!(package.custom_file_properties_part.is_none());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
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
fn round_trip_spreadsheet_xltx_asset_from_openxml_sdk() {
  let path = test_file_path("Spreadsheet.xltx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(
    original.workbook_part.inner_path,
    roundtripped.workbook_part.inner_path
  );
  assert_eq!(
    original.workbook_part.root_element.sheets.x_sheet.len(),
    roundtripped.workbook_part.root_element.sheets.x_sheet.len()
  );
  assert!(
    !original
      .workbook_part
      .root_element
      .sheets
      .x_sheet
      .is_empty()
  );
  assert!(
    !roundtripped
      .workbook_part
      .root_element
      .sheets
      .x_sheet
      .is_empty()
  );
}

#[test]
fn round_trip_mcinleaf_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mcinleaf.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_mailmerge_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mailmerge.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_svg_docx_asset_from_openxml_sdk() {
  let path = test_file_path("svg.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_presentation_potx_asset_from_openxml_sdk() {
  let path = test_file_path("Presentation.potx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.inner_path,
    roundtripped.presentation_part.inner_path
  );
  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    roundtripped
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len())
  );
  assert!(
    original
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len())
      .unwrap_or_default()
      > 0
  );
}

#[test]
fn open_comments_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Comments.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.workbook_part.root_element.sheets.x_sheet.len(), 1);
  assert_eq!(package.workbook_part.worksheet_parts.len(), 1);

  let worksheet_part = &package.workbook_part.worksheet_parts[0];
  assert_eq!(worksheet_part.inner_path, "xl/worksheets/sheet1.xml");
  assert!(worksheet_part.worksheet_comments_part.is_some());

  let comments_part = worksheet_part
    .worksheet_comments_part
    .as_ref()
    .expect("expected worksheet comments part");
  assert_eq!(comments_part.inner_path, "xl/comments1.xml");
  assert_eq!(comments_part.root_element.authors.x_author.len(), 1);
  assert_eq!(comments_part.root_element.comment_list.x_comment.len(), 1);

  let comment = &comments_part.root_element.comment_list.x_comment[0];
  assert_eq!(comment.reference.as_str(), "A1");
  assert_eq!(comment.author_id, 0);
  assert_eq!(
    comments_part.root_element.authors.x_author[0]
      .xml_content
      .as_deref(),
    Some("robermc")
  );

  let style_extensions = stylesheet_extensions(&package);
  assert_eq!(style_extensions.len(), 1);
  assert_eq!(
    style_extensions[0].xmlns_map.get("x14").map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
}

#[test]
fn open_malformed_uri_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("malformed_uri.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  let worksheet_part = &package.workbook_part.worksheet_parts[0];
  let hyperlink = worksheet_part
    .root_element
    .x_hyperlinks
    .as_ref()
    .and_then(|hyperlinks| hyperlinks.x_hyperlink.first())
    .expect("expected worksheet hyperlink");
  let relationship = worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected hyperlink relationship");

  assert_eq!(hyperlink.id.as_deref(), Some("rId1"));
  assert_eq!(relationship.target.as_str(), "mailto:one@");
  assert!(matches!(
    relationship.target_mode,
    Some(ooxmlsdk::schemas::opc_relationships::TargetMode::External)
  ));
}

#[test]
fn open_malformed_uri_long_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("malformed_uri_long.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  let worksheet_part = &package.workbook_part.worksheet_parts[0];
  let hyperlink = worksheet_part
    .root_element
    .x_hyperlinks
    .as_ref()
    .and_then(|hyperlinks| hyperlinks.x_hyperlink.first())
    .expect("expected worksheet hyperlink");
  let relationship = worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected hyperlink relationship");

  assert_eq!(hyperlink.id.as_deref(), Some("rId1"));
  assert_eq!(
    relationship.target.as_str(),
    "mailto:test@test.com;%20test2@test.com;%252test3@test.com;%20test3@test.com;%20test4@test.com;%20test5@test.com?subject=Unsubscribe%20Request&body=Please%20unsubscribe%20me%20from%20all%20future%20communications"
  );
  assert!(matches!(
    relationship.target_mode,
    Some(ooxmlsdk::schemas::opc_relationships::TargetMode::External)
  ));
}

#[cfg(feature = "microsoft365")]
#[test]
fn open_youtube_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Youtube.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  let worksheet_part = &package.workbook_part.worksheet_parts[0];
  let drawings_part = worksheet_part
    .drawings_part
    .as_ref()
    .expect("expected worksheet drawings part");

  assert_eq!(drawings_part.web_extension_parts.len(), 1);
  assert_eq!(drawings_part.image_parts.len(), 1);
}

#[test]
fn open_missingcalcchainpart_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("missingcalcchainpart.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.workbook_part.worksheet_parts.len(), 6);
  assert!(package.workbook_part.shared_string_table_part.is_some());
  assert!(package.workbook_part.calculation_chain_part.is_none());
}

#[test]
fn open_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("DocProps.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.title.as_deref()),
    Some(Some("Test-Title"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.subject.as_deref()),
    Some(Some("Test-Subject"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.creator.as_deref()),
    Some(Some("Eric White"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| keywords_text(part.root_element.keywords.as_deref())),
    Some(Some("Test-Keywords"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.description.as_deref()),
    Some(Some("Test-Comments"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.category.as_deref()),
    Some(Some("Test-Category"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.content_status.as_deref()),
    Some(Some("Test-Status"))
  );
  assert!(package.extended_file_properties_part.is_some());
  assert!(package.custom_file_properties_part.is_some());
  assert_eq!(
    package
      .main_document_part
      .wordprocessing_comments_part
      .as_ref()
      .map(|part| part.root_element.w_comment.len()),
    Some(1)
  );
}

#[test]
fn open_more_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("MoreDocProps.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.creator.as_deref()),
    Some(Some("Eric White"))
  );
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.revision.as_deref()),
    Some(Some("6"))
  );
  assert!(package.extended_file_properties_part.is_some());
  assert!(package.custom_file_properties_part.is_none());
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert!(package.main_document_part.theme_part.is_some());
}

#[test]
fn open_notes_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Notes.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.creator.as_deref()),
    Some(Some("Eric"))
  );
  assert_eq!(package.main_document_part.image_parts.len(), 2);
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert!(
    package
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(package.main_document_part.web_settings_part.is_some());
}

#[test]
fn open_complex01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.header_parts.len(), 9);
  assert_eq!(package.main_document_part.footer_parts.len(), 9);
  assert_eq!(package.main_document_part.image_parts.len(), 3);
  assert_eq!(package.main_document_part.chart_parts.len(), 1);
  assert_eq!(package.main_document_part.diagram_data_parts.len(), 1);
  assert_eq!(
    package
      .main_document_part
      .diagram_layout_definition_parts
      .len(),
    1
  );
  assert_eq!(package.main_document_part.diagram_style_parts.len(), 1);
  assert_eq!(package.main_document_part.diagram_colors_parts.len(), 1);
  assert_eq!(package.main_document_part.embedded_package_parts.len(), 1);
  assert!(
    package.main_document_part.chart_parts[0]
      .embedded_package_part
      .is_some()
  );
  assert!(
    package
      .main_document_part
      .wordprocessing_comments_part
      .is_some()
  );
  assert!(
    package
      .main_document_part
      .numbering_definitions_part
      .is_some()
  );
}

#[test]
fn open_bad_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("BadDocProps.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert!(package.core_file_properties_part.is_some());
  assert_eq!(
    package
      .core_file_properties_part
      .as_ref()
      .map(|part| part.root_element.creator.as_deref()),
    Some(Some("Eric White"))
  );
}

#[test]
fn open_invalid_doc_propsct_docx_asset_from_openxml_sdk() {
  let path = test_file_path("InvalidDocPropsct.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert!(package.core_file_properties_part.is_some());
}

#[test]
fn open_mcdoc_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mcdoc.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.custom_xml_parts.len(), 1);
  assert!(package.main_document_part.style_definitions_part.is_some());
  assert!(
    package
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(package.main_document_part.theme_part.is_some());
  assert!(package.main_document_part.web_settings_part.is_some());
}

#[test]
fn open_autosave_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("autosave.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.slide_parts.len(), 2);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert!(package.presentation_part.theme_part.is_some());
  assert!(
    package
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert!(package.presentation_part.view_properties_part.is_some());
  assert!(package.presentation_part.table_styles_part.is_some());
}

#[test]
fn open_mediareference_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("mediareference.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.slide_parts.len(), 2);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert_eq!(
    package.presentation_part.slide_parts[0].image_parts.len(),
    1
  );
  assert!(package.presentation_part.theme_part.is_some());
}

#[test]
fn open_algn_tab_tab_alignment_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Algn_tab_TabAlignment.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.slide_parts.len(), 1);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert!(package.presentation_part.theme_part.is_some());
  assert!(
    package
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert!(package.presentation_part.view_properties_part.is_some());
}

#[test]
fn open_animation_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("animation.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.slide_parts.len(), 2);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert_eq!(package.presentation_part.custom_xml_parts.len(), 1);
  assert_eq!(
    package.presentation_part.slide_parts[1].image_parts.len(),
    1
  );
}

#[test]
fn open_excel14_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("excel14.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();
  let sheet1_part = package
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(package.workbook_part.worksheet_parts.len(), 3);
  assert!(package.workbook_part.shared_string_table_part.is_some());
  assert!(package.workbook_part.workbook_styles_part.is_some());
  assert!(sheet1_part.drawings_part.is_some());
  assert_eq!(
    sheet1_part
      .drawings_part
      .as_ref()
      .expect("expected drawing part")
      .image_parts
      .len(),
    1
  );
}

#[test]
fn open_extlst_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("extlst.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();
  let sheet1_part = package
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(package.workbook_part.worksheet_parts.len(), 4);
  assert_eq!(
    package
      .workbook_part
      .pivot_table_cache_definition_parts
      .len(),
    1
  );
  assert!(sheet1_part.drawings_part.is_some());
  assert_eq!(sheet1_part.pivot_table_parts.len(), 1);
}

#[test]
fn open_revision_name_comment_change_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Revision_NameCommentChange.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  assert_eq!(package.workbook_part.worksheet_parts.len(), 3);
  assert!(
    package
      .workbook_part
      .workbook_revision_header_part
      .is_some()
  );
  assert!(package.workbook_part.workbook_user_data_part.is_some());
  assert!(package.workbook_part.workbook_styles_part.is_some());
}

#[test]
fn open_vmldrawingroot_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("vmldrawingroot.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();
  let sheet1_part = package
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(package.workbook_part.worksheet_parts.len(), 3);
  assert!(package.workbook_part.workbook_styles_part.is_some());
  assert_eq!(sheet1_part.vml_drawing_parts.len(), 1);
  assert!(sheet1_part.worksheet_comments_part.is_some());
  assert!(sheet1_part.drawings_part.is_none());
}

#[test]
fn round_trip_data_bound_content_controls_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Data-Bound-Content-Controls.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-01.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_02_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-02.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_03_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-03.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_04_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-04.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_05_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-05.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_06_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-06.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_07_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-07.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_of16_08_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-08.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn open_complex01_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();

  let workbook = &package.workbook_part.root_element;
  assert_eq!(workbook.mc_ignorable.as_deref(), Some("x15"));
  assert_eq!(
    workbook
      .file_version
      .as_ref()
      .and_then(|file_version| file_version.application_name.as_deref()),
    Some("xl")
  );
  assert_eq!(workbook.sheets.x_sheet.len(), 2);
  assert_eq!(workbook.sheets.x_sheet[0].name.as_str(), "Sheet1");
  assert_eq!(workbook.sheets.x_sheet[1].name.as_str(), "Sheet2");
  assert_eq!(package.workbook_part.worksheet_parts.len(), 2);
  assert!(workbook.calculation_properties.is_some());

  let style_extensions = stylesheet_extensions(&package);
  assert_eq!(style_extensions.len(), 2);
  assert_eq!(
    style_extensions[0].xmlns_map.get("x14").map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
  assert_eq!(
    style_extensions[1].xmlns_map.get("x15").map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2010/11/main")
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

#[test]
fn add_alternative_format_import_part_to_wordprocessing_document_from_openxml_sdk() {
  let path = test_file_path("Hyperlink.docx");
  let mut package = WordprocessingDocument::new_from_file(&path).unwrap();

  let body =
    first_body_mut(&mut package.main_document_part.root_element).expect("expected document body");
  body
    .children
    .push(BodyChildChoice::WAltChunk(Box::new(AltChunk {
      id: Some(ALT_CHUNK_ID.to_string()),
      alt_chunk_properties: None,
    })));

  let alt_chunk_part = AlternativeFormatImportPart {
    r_id: ALT_CHUNK_ID.to_string(),
    inner_path: format!("word/{ALT_CHUNK_TARGET}"),
    part_content: ALT_CHUNK_XML.as_bytes().to_vec(),
  };
  package
    .main_document_part
    .alternative_format_import_parts
    .push(alt_chunk_part);

  let relationships = package
    .main_document_part
    .relationships
    .as_mut()
    .expect("expected main document relationships");
  relationships.relationship.push(Relationship {
    target_mode: None,
    target: ALT_CHUNK_TARGET.to_string(),
    r#type: ooxmlsdk::parts::alternative_format_import_part::RELATIONSHIP_TYPE.to_string(),
    id: ALT_CHUNK_ID.to_string(),
  });

  assert!(
    package
      .main_document_part
      .root_element
      .body
      .as_deref()
      .is_some_and(|body| {
        body
          .children
          .iter()
          .any(|body_child| matches!(body_child, BodyChildChoice::WAltChunk(_)))
      })
  );

  let mut buffer = Cursor::new(Vec::new());
  package.save(&mut buffer).unwrap();

  let reopened = WordprocessingDocument::new(Cursor::new(buffer.into_inner())).unwrap();
  assert!(
    reopened
      .main_document_part
      .root_element
      .body
      .as_deref()
      .is_some_and(|body| {
        body
          .children
          .iter()
          .any(|body_child| matches!(body_child, BodyChildChoice::WAltChunk(_)))
      })
  );
  assert_eq!(
    reopened
      .main_document_part
      .alternative_format_import_parts
      .len(),
    1
  );
  let reopened_alt_chunk = &reopened.main_document_part.alternative_format_import_parts[0];
  assert_eq!(reopened_alt_chunk.r_id, ALT_CHUNK_ID);
  assert_eq!(reopened_alt_chunk.inner_path, "word/afchunk.xml");
  assert_eq!(
    std::str::from_utf8(&reopened_alt_chunk.part_content).unwrap(),
    ALT_CHUNK_XML
  );
}

#[test]
fn round_trip_plain_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Plain.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = first_paragraph(original_body).expect("expected paragraph");
  let roundtripped_paragraph = first_paragraph(roundtripped_body).expect("expected paragraph");

  assert_eq!(original_body.children.len(), 2);
  assert_eq!(roundtripped_body.children.len(), 2);
  assert_eq!(paragraph_bookmark_start_count(original_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(original_paragraph), 1);
  assert_eq!(paragraph_bookmark_start_count(roundtripped_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(roundtripped_paragraph), 1);
  assert!(
    first_paragraph_text(original_paragraph)
      .expect("expected paragraph text")
      .starts_with("Video provides a powerful way")
  );
  assert!(
    first_paragraph_text(roundtripped_paragraph)
      .expect("expected roundtripped paragraph text")
      .starts_with("Video provides a powerful way")
  );
}

#[test]
fn round_trip_hello_world_docx_asset_from_openxml_sdk() {
  let path = test_file_path("HelloWorld.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = first_paragraph(original_body).expect("expected paragraph");
  let roundtripped_paragraph = first_paragraph(roundtripped_body).expect("expected paragraph");

  assert_eq!(original_body.children.len(), 2);
  assert_eq!(roundtripped_body.children.len(), 2);
  assert_eq!(paragraph_bookmark_start_count(original_paragraph), 0);
  assert_eq!(paragraph_bookmark_end_count(original_paragraph), 0);
  assert_eq!(paragraph_bookmark_start_count(roundtripped_paragraph), 0);
  assert_eq!(paragraph_bookmark_end_count(roundtripped_paragraph), 0);
  assert_eq!(
    first_paragraph_text(original_paragraph),
    Some("Hello World!")
  );
  assert_eq!(
    first_paragraph_text(roundtripped_paragraph),
    Some("Hello World!")
  );
}

#[test]
fn round_trip_document_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Document.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert!(original_body.children.len() > 10);
  assert!(roundtripped_body.children.len() > 10);
  assert!(
    original_body
      .children
      .iter()
      .any(|child| matches!(child, BodyChildChoice::WSdt(_)))
  );
  assert!(
    roundtripped_body
      .children
      .iter()
      .any(|child| matches!(child, BodyChildChoice::WSdt(_)))
  );
  assert!(
    original
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
      .unwrap_or_default()
      > 5
  );
  assert!(
    roundtripped
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
      .unwrap_or_default()
      > 5
  );
}

#[test]
fn round_trip_document_dotx_asset_from_openxml_sdk() {
  let path = test_file_path("Document.dotx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert_eq!(
    main_document_body_child_count(&original.main_document_part.root_element),
    6
  );
  assert_eq!(
    main_document_body_child_count(&roundtripped.main_document_part.root_element),
    6
  );

  let original_first_paragraph = first_paragraph(original_body).expect("expected paragraph");
  let roundtripped_first_paragraph =
    first_paragraph(roundtripped_body).expect("expected paragraph");
  assert_eq!(
    first_paragraph_text(original_first_paragraph),
    Some("Document Title")
  );
  assert_eq!(
    first_paragraph_text(roundtripped_first_paragraph),
    Some("Document Title")
  );
}

#[cfg(feature = "strict")]
#[test]
fn round_trip_strict01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Strict01.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert!(original.main_document_part.document_settings_part.is_some());
  assert!(
    roundtripped
      .main_document_part
      .document_settings_part
      .is_some()
  );
  assert!(original_body.children.len() > 1);
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
}

#[cfg(feature = "strict")]
#[test]
fn open_annotation_ref_docx_asset_from_openxml_sdk() {
  let path = test_file_path("AnnotationRef.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[cfg(feature = "strict")]
#[test]
fn round_trip_annotation_ref_docx_asset_from_openxml_sdk() {
  let path = test_file_path("AnnotationRef.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(!original_body.children.is_empty());
  assert!(!roundtripped_body.children.is_empty());
}

#[test]
fn round_trip_hello_o14_docx_asset_from_openxml_sdk() {
  let path = test_file_path("HelloO14.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = original_body
    .children
    .iter()
    .find_map(|child| match child {
      BodyChildChoice::WP(paragraph)
        if paragraph_text(paragraph.as_ref()).contains("Hello O14") =>
      {
        Some(paragraph.as_ref())
      }
      _ => None,
    })
    .expect("expected paragraph text");
  let roundtripped_paragraph = roundtripped_body
    .children
    .iter()
    .find_map(|child| match child {
      BodyChildChoice::WP(paragraph)
        if paragraph_text(paragraph.as_ref()).contains("Hello O14") =>
      {
        Some(paragraph.as_ref())
      }
      _ => None,
    })
    .expect("expected paragraph text");

  assert!(paragraph_text(original_paragraph).contains("Hello O14"));
  assert!(paragraph_text(roundtripped_paragraph).contains("Hello O14"));
  assert!(original.main_document_part.theme_part.is_some());
  assert!(roundtripped.main_document_part.theme_part.is_some());
  assert!(original.main_document_part.font_table_part.is_some());
  assert!(roundtripped.main_document_part.font_table_part.is_some());
  assert!(original.main_document_part.document_settings_part.is_some());
  assert!(
    roundtripped
      .main_document_part
      .document_settings_part
      .is_some()
  );
  assert!(original.main_document_part.web_settings_part.is_some());
  assert!(roundtripped.main_document_part.web_settings_part.is_some());
}

#[test]
fn round_trip_may_12_04_docx_asset_from_openxml_sdk() {
  let path = test_file_path("May_12_04.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(4)
  );
  assert_eq!(
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(4)
  );
  assert_eq!(
    original
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(84)
  );
  assert_eq!(
    roundtripped
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(84)
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
  assert!(
    original_body
      .children
      .iter()
      .any(|child| matches!(child, BodyChildChoice::WSectPr(_)))
  );
  assert!(
    roundtripped_body
      .children
      .iter()
      .any(|child| matches!(child, BodyChildChoice::WSectPr(_)))
  );

  let original_first_paragraph = original_body
    .children
    .iter()
    .find_map(|child| match child {
      BodyChildChoice::WP(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
    .expect("expected first paragraph");
  let roundtripped_first_paragraph = roundtripped_body
    .children
    .iter()
    .find_map(|child| match child {
      BodyChildChoice::WP(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
    .expect("expected first paragraph");

  assert!(
    first_paragraph_text(original_first_paragraph)
      .expect("expected first paragraph text")
      .starts_with("IASWR Listserv Announcements")
  );
  assert!(
    first_paragraph_text(roundtripped_first_paragraph)
      .expect("expected first paragraph text")
      .starts_with("IASWR Listserv Announcements")
  );

  let original_paragraph_texts: Vec<String> = original_body
    .children
    .iter()
    .filter_map(|child| match child {
      BodyChildChoice::WP(paragraph) => Some(paragraph_text(paragraph.as_ref())),
      _ => None,
    })
    .collect();
  let roundtripped_paragraph_texts: Vec<String> = roundtripped_body
    .children
    .iter()
    .filter_map(|child| match child {
      BodyChildChoice::WP(paragraph) => Some(paragraph_text(paragraph.as_ref())),
      _ => None,
    })
    .collect();

  assert!(
    original_paragraph_texts
      .iter()
      .any(|text| text.contains("www.iaswresearch.org"))
  );
  assert!(
    roundtripped_paragraph_texts
      .iter()
      .any(|text| text.contains("www.iaswresearch.org"))
  );
  assert!(
    original_paragraph_texts
      .iter()
      .any(|text| text.contains("http://cosw.sc.edu/conf/policy/index.htm"))
  );
  assert!(
    roundtripped_paragraph_texts
      .iter()
      .any(|text| text.contains("http://cosw.sc.edu/conf/policy/index.htm"))
  );
}

#[test]
fn round_trip_comments_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Comments.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = first_paragraph(original_body).expect("expected paragraph");
  let roundtripped_paragraph = first_paragraph(roundtripped_body).expect("expected paragraph");

  assert_eq!(
    original
      .main_document_part
      .wordprocessing_comments_part
      .as_ref()
      .map(|part| part.root_element.w_comment.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .main_document_part
      .wordprocessing_comments_part
      .as_ref()
      .map(|part| part.root_element.w_comment.len()),
    Some(1)
  );

  let original_comment = original
    .main_document_part
    .wordprocessing_comments_part
    .as_ref()
    .expect("expected comments part")
    .root_element
    .w_comment
    .first()
    .expect("expected comment");
  let roundtripped_comment = roundtripped
    .main_document_part
    .wordprocessing_comments_part
    .as_ref()
    .expect("expected comments part")
    .root_element
    .w_comment
    .first()
    .expect("expected comment");

  assert_eq!(original_comment.id.as_str(), "1");
  assert_eq!(roundtripped_comment.id.as_str(), "1");
  assert_eq!(original_comment.author.as_str(), "Eric White");
  assert_eq!(roundtripped_comment.author.as_str(), "Eric White");
  assert_eq!(original_comment.initials.as_deref(), Some("EW"));
  assert_eq!(roundtripped_comment.initials.as_deref(), Some("EW"));
  assert_eq!(comment_text(original_comment), "This is a comment.");
  assert_eq!(comment_text(roundtripped_comment), "This is a comment.");

  assert!(
    first_paragraph_text(original_paragraph)
      .expect("expected paragraph text")
      .starts_with("When ")
  );
  assert!(
    first_paragraph_text(roundtripped_paragraph)
      .expect("expected paragraph text")
      .starts_with("When ")
  );
  assert_eq!(paragraph_bookmark_start_count(original_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(original_paragraph), 1);
  assert_eq!(paragraph_comment_range_start_count(original_paragraph), 1);
  assert_eq!(paragraph_comment_range_end_count(original_paragraph), 1);
  assert_eq!(paragraph_comment_reference_count(original_paragraph), 1);
  assert_eq!(paragraph_bookmark_start_count(roundtripped_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(roundtripped_paragraph), 1);
  assert_eq!(
    paragraph_comment_range_start_count(roundtripped_paragraph),
    1
  );
  assert_eq!(paragraph_comment_range_end_count(roundtripped_paragraph), 1);
  assert_eq!(paragraph_comment_reference_count(roundtripped_paragraph), 1);
}

#[test]
fn round_trip_hyperlink_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Hyperlink.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(body_paragraph_count(original_body), 2);
  assert_eq!(body_paragraph_count(roundtripped_body), 2);

  let original_hyperlink_paragraph = original_body
    .children
    .iter()
    .filter_map(|child| match child {
      BodyChildChoice::WP(paragraph)
        if paragraph
          .paragraph_choice
          .iter()
          .any(|choice| matches!(choice, ParagraphChoice::WHyperlink(_))) =>
      {
        Some(paragraph.as_ref())
      }
      _ => None,
    })
    .next()
    .expect("expected paragraph with hyperlink");
  let roundtripped_hyperlink_paragraph = roundtripped_body
    .children
    .iter()
    .filter_map(|child| match child {
      BodyChildChoice::WP(paragraph)
        if paragraph
          .paragraph_choice
          .iter()
          .any(|choice| matches!(choice, ParagraphChoice::WHyperlink(_))) =>
      {
        Some(paragraph.as_ref())
      }
      _ => None,
    })
    .next()
    .expect("expected paragraph with hyperlink");

  let original_hyperlink =
    first_hyperlink(original_hyperlink_paragraph).expect("expected hyperlink");
  let roundtripped_hyperlink =
    first_hyperlink(roundtripped_hyperlink_paragraph).expect("expected hyperlink");

  assert_eq!(original_hyperlink.id.as_deref(), Some("rId4"));
  assert_eq!(roundtripped_hyperlink.id.as_deref(), Some("rId4"));
  assert_eq!(original_hyperlink.history, Some(true));
  assert_eq!(roundtripped_hyperlink.history, Some(true));
  assert_eq!(
    paragraph_text(original_hyperlink_paragraph),
    "EricWhite.com"
  );
  assert_eq!(
    paragraph_text(roundtripped_hyperlink_paragraph),
    "EricWhite.com"
  );
}

#[test]
fn round_trip_empty_relationship_element_docx_asset_from_openxml_sdk() {
  let path = test_file_path("EmptyRelationshipElement.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(body_paragraph_count(original_body), 2);
  assert_eq!(body_paragraph_count(roundtripped_body), 2);
  assert_eq!(
    original
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(6)
  );
  assert_eq!(
    roundtripped
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    Some(6)
  );
}

#[test]
fn round_trip_no_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("NoDocProps.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert!(original.core_file_properties_part.is_none());
  assert!(original.extended_file_properties_part.is_none());
  assert!(original.custom_file_properties_part.is_none());
  assert!(roundtripped.core_file_properties_part.is_none());
  assert!(roundtripped.extended_file_properties_part.is_none());
  assert!(roundtripped.custom_file_properties_part.is_none());
  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original_body.children.len(),
    roundtripped_body.children.len()
  );
}

#[test]
fn round_trip_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("DocProps.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_core_properties = original
    .core_file_properties_part
    .as_ref()
    .expect("expected core properties part");
  let roundtripped_core_properties = roundtripped
    .core_file_properties_part
    .as_ref()
    .expect("expected core properties part");

  assert_eq!(
    original_core_properties.root_element.title.as_deref(),
    Some("Test-Title")
  );
  assert_eq!(
    roundtripped_core_properties.root_element.title.as_deref(),
    Some("Test-Title")
  );
  assert_eq!(
    original_core_properties.root_element.subject.as_deref(),
    Some("Test-Subject")
  );
  assert_eq!(
    roundtripped_core_properties.root_element.subject.as_deref(),
    Some("Test-Subject")
  );
  assert_eq!(
    original_core_properties.root_element.creator.as_deref(),
    Some("Eric White")
  );
  assert_eq!(
    roundtripped_core_properties.root_element.creator.as_deref(),
    Some("Eric White")
  );
  assert_eq!(
    keywords_text(original_core_properties.root_element.keywords.as_deref()),
    Some("Test-Keywords")
  );
  assert_eq!(
    keywords_text(
      roundtripped_core_properties
        .root_element
        .keywords
        .as_deref()
    ),
    Some("Test-Keywords")
  );
  assert_eq!(
    original_core_properties.root_element.description.as_deref(),
    Some("Test-Comments")
  );
  assert_eq!(
    roundtripped_core_properties
      .root_element
      .description
      .as_deref(),
    Some("Test-Comments")
  );
  assert_eq!(
    original_core_properties.root_element.category.as_deref(),
    Some("Test-Category")
  );
  assert_eq!(
    roundtripped_core_properties
      .root_element
      .category
      .as_deref(),
    Some("Test-Category")
  );
  assert_eq!(
    original_core_properties
      .root_element
      .content_status
      .as_deref(),
    Some("Test-Status")
  );
  assert_eq!(
    roundtripped_core_properties
      .root_element
      .content_status
      .as_deref(),
    Some("Test-Status")
  );
  assert!(original.extended_file_properties_part.is_some());
  assert!(roundtripped.extended_file_properties_part.is_some());
  assert!(original.custom_file_properties_part.is_some());
  assert!(roundtripped.custom_file_properties_part.is_some());
  assert_eq!(
    original
      .main_document_part
      .wordprocessing_comments_part
      .as_ref()
      .map(|part| part.root_element.w_comment.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .main_document_part
      .wordprocessing_comments_part
      .as_ref()
      .map(|part| part.root_element.w_comment.len()),
    Some(1)
  );
}

#[test]
fn round_trip_more_doc_props_docx_asset_from_openxml_sdk() {
  let path = test_file_path("MoreDocProps.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_core_properties = original
    .core_file_properties_part
    .as_ref()
    .expect("expected core properties part");
  let roundtripped_core_properties = roundtripped
    .core_file_properties_part
    .as_ref()
    .expect("expected core properties part");

  assert_eq!(
    original_core_properties.root_element.creator.as_deref(),
    Some("Eric White")
  );
  assert_eq!(
    roundtripped_core_properties.root_element.creator.as_deref(),
    Some("Eric White")
  );
  assert_eq!(
    original_core_properties.root_element.revision.as_deref(),
    Some("6")
  );
  assert_eq!(
    roundtripped_core_properties
      .root_element
      .revision
      .as_deref(),
    Some("6")
  );
  assert!(original.extended_file_properties_part.is_some());
  assert!(roundtripped.extended_file_properties_part.is_some());
  assert!(original.custom_file_properties_part.is_none());
  assert!(roundtripped.custom_file_properties_part.is_none());
}

#[test]
fn round_trip_mcdoc_docx_asset_from_openxml_sdk() {
  let path = test_file_path("mcdoc.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  assert_eq!(
    original.main_document_part.custom_xml_parts.len(),
    roundtripped.main_document_part.custom_xml_parts.len()
  );
  assert!(original.main_document_part.style_definitions_part.is_some());
  assert!(
    roundtripped
      .main_document_part
      .style_definitions_part
      .is_some()
  );
  assert!(
    original
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(
    roundtripped
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(original.main_document_part.theme_part.is_some());
  assert!(roundtripped.main_document_part.theme_part.is_some());
  assert!(original.main_document_part.web_settings_part.is_some());
  assert!(roundtripped.main_document_part.web_settings_part.is_some());
}

#[test]
fn round_trip_notes_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Notes.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  assert_eq!(original.main_document_part.image_parts.len(), 2);
  assert_eq!(roundtripped.main_document_part.image_parts.len(), 2);
  assert!(
    original
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(
    roundtripped
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  assert!(original.main_document_part.web_settings_part.is_some());
  assert!(roundtripped.main_document_part.web_settings_part.is_some());
}

#[test]
fn round_trip_complex01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  assert_eq!(
    original.main_document_part.header_parts.len(),
    roundtripped.main_document_part.header_parts.len()
  );
  assert_eq!(
    original.main_document_part.footer_parts.len(),
    roundtripped.main_document_part.footer_parts.len()
  );
  assert_eq!(
    original.main_document_part.image_parts.len(),
    roundtripped.main_document_part.image_parts.len()
  );
  assert_eq!(
    original.main_document_part.chart_parts.len(),
    roundtripped.main_document_part.chart_parts.len()
  );
  assert_eq!(
    original.main_document_part.diagram_data_parts.len(),
    roundtripped.main_document_part.diagram_data_parts.len()
  );
  assert_eq!(
    original
      .main_document_part
      .diagram_layout_definition_parts
      .len(),
    roundtripped
      .main_document_part
      .diagram_layout_definition_parts
      .len()
  );
  assert_eq!(
    original.main_document_part.diagram_style_parts.len(),
    roundtripped.main_document_part.diagram_style_parts.len()
  );
  assert_eq!(
    original.main_document_part.diagram_colors_parts.len(),
    roundtripped.main_document_part.diagram_colors_parts.len()
  );
  assert_eq!(
    original.main_document_part.embedded_package_parts.len(),
    roundtripped.main_document_part.embedded_package_parts.len()
  );
  assert_eq!(
    original.main_document_part.chart_parts[0]
      .embedded_package_part
      .is_some(),
    roundtripped.main_document_part.chart_parts[0]
      .embedded_package_part
      .is_some()
  );
}

#[test]
fn round_trip_autosave_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("autosave.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
  assert!(original.presentation_part.theme_part.is_some());
  assert!(roundtripped.presentation_part.theme_part.is_some());
  assert!(
    original
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert!(
    roundtripped
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert!(original.presentation_part.view_properties_part.is_some());
  assert!(
    roundtripped
      .presentation_part
      .view_properties_part
      .is_some()
  );
}

#[test]
fn round_trip_mediareference_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("mediareference.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_parts[0].image_parts.len(),
    1
  );
  assert_eq!(
    roundtripped.presentation_part.slide_parts[0]
      .image_parts
      .len(),
    1
  );
}

#[test]
fn round_trip_algn_tab_tab_alignment_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Algn_tab_TabAlignment.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
  assert!(original.presentation_part.theme_part.is_some());
  assert!(roundtripped.presentation_part.theme_part.is_some());
}

#[test]
fn round_trip_animation_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("animation.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.custom_xml_parts.len(),
    roundtripped.presentation_part.custom_xml_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_parts[1].image_parts.len(),
    1
  );
  assert_eq!(
    roundtripped.presentation_part.slide_parts[1]
      .image_parts
      .len(),
    1
  );
}

#[test]
fn round_trip_excel14_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("excel14.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);
  let original_sheet1_part = original
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");
  let roundtripped_sheet1_part = roundtripped
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
  assert!(original.workbook_part.shared_string_table_part.is_some());
  assert!(
    roundtripped
      .workbook_part
      .shared_string_table_part
      .is_some()
  );
  assert!(original_sheet1_part.drawings_part.is_some());
  assert!(roundtripped_sheet1_part.drawings_part.is_some());
  assert_eq!(
    original_sheet1_part
      .drawings_part
      .as_ref()
      .expect("expected drawing part")
      .image_parts
      .len(),
    1
  );
  assert_eq!(
    roundtripped_sheet1_part
      .drawings_part
      .as_ref()
      .expect("expected drawing part")
      .image_parts
      .len(),
    1
  );
}

#[test]
fn round_trip_extlst_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("extlst.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);
  let original_sheet1_part = original
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");
  let roundtripped_sheet1_part = roundtripped
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
  assert_eq!(
    original
      .workbook_part
      .pivot_table_cache_definition_parts
      .len(),
    roundtripped
      .workbook_part
      .pivot_table_cache_definition_parts
      .len()
  );
  assert!(original_sheet1_part.drawings_part.is_some());
  assert!(roundtripped_sheet1_part.drawings_part.is_some());
  assert_eq!(
    original_sheet1_part.pivot_table_parts.len(),
    roundtripped_sheet1_part.pivot_table_parts.len()
  );
}

#[test]
fn round_trip_revision_name_comment_change_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Revision_NameCommentChange.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
  assert!(
    original
      .workbook_part
      .workbook_revision_header_part
      .is_some()
  );
  assert!(
    roundtripped
      .workbook_part
      .workbook_revision_header_part
      .is_some()
  );
  assert!(original.workbook_part.workbook_user_data_part.is_some());
  assert!(roundtripped.workbook_part.workbook_user_data_part.is_some());
}

#[test]
fn round_trip_vmldrawingroot_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("vmldrawingroot.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);
  let original_sheet1_part = original
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");
  let roundtripped_sheet1_part = roundtripped
    .workbook_part
    .worksheet_parts
    .iter()
    .find(|part| part.inner_path == "xl/worksheets/sheet1.xml")
    .expect("expected sheet1 worksheet part");

  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
  assert_eq!(
    original_sheet1_part.vml_drawing_parts.len(),
    roundtripped_sheet1_part.vml_drawing_parts.len()
  );
  assert!(original_sheet1_part.worksheet_comments_part.is_some());
  assert!(roundtripped_sheet1_part.worksheet_comments_part.is_some());
}

#[test]
fn round_trip_missingcalcchainpart_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("missingcalcchainpart.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
  assert!(original.workbook_part.calculation_chain_part.is_none());
  assert!(roundtripped.workbook_part.calculation_chain_part.is_none());
  assert!(original.workbook_part.shared_string_table_part.is_some());
  assert!(
    roundtripped
      .workbook_part
      .shared_string_table_part
      .is_some()
  );
}

#[test]
fn round_trip_spreadsheet_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Spreadsheet.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(original.workbook_part.root_element.sheets.x_sheet.len(), 2);
  assert_eq!(
    roundtripped.workbook_part.root_element.sheets.x_sheet.len(),
    2
  );
  assert_eq!(
    original.workbook_part.root_element.sheets.x_sheet[0]
      .name
      .as_str(),
    "Sheet1"
  );
  assert_eq!(
    original.workbook_part.root_element.sheets.x_sheet[1]
      .name
      .as_str(),
    "Sheet2"
  );
  assert_eq!(
    roundtripped.workbook_part.root_element.sheets.x_sheet[0]
      .name
      .as_str(),
    "Sheet1"
  );
  assert_eq!(
    roundtripped.workbook_part.root_element.sheets.x_sheet[1]
      .name
      .as_str(),
    "Sheet2"
  );
  assert!(original.workbook_part.shared_string_table_part.is_some());
  assert!(
    roundtripped
      .workbook_part
      .shared_string_table_part
      .is_some()
  );
}

#[test]
fn round_trip_basicspreadsheet_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("basicspreadsheet.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(original.workbook_part.root_element.sheets.x_sheet.len(), 3);
  assert_eq!(
    roundtripped.workbook_part.root_element.sheets.x_sheet.len(),
    3
  );
  assert!(original.workbook_part.root_element.pivot_caches.is_some());
  assert!(
    roundtripped
      .workbook_part
      .root_element
      .pivot_caches
      .is_some()
  );
  assert!(original.workbook_part.root_element.web_publishing.is_some());
  assert!(
    roundtripped
      .workbook_part
      .root_element
      .web_publishing
      .is_some()
  );
}

#[test]
fn round_trip_presentation_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("Presentation.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(2)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(2)
  );
  assert!(original.presentation_part.slide_parts.len() >= 2);
  assert!(roundtripped.presentation_part.slide_parts.len() >= 2);
}

#[test]
fn round_trip_3dtestdot_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("3dtestdot.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(1)
  );
  assert!(!original.presentation_part.slide_parts.is_empty());
  assert!(!roundtripped.presentation_part.slide_parts.is_empty());
}

#[test]
fn round_trip_3dtestdash_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("3dtestdash.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    original
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(1)
  );
  assert_eq!(
    roundtripped
      .presentation_part
      .root_element
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(1)
  );
  assert!(!original.presentation_part.slide_parts.is_empty());
  assert!(!roundtripped.presentation_part.slide_parts.is_empty());
}

#[test]
fn round_trip_simple_sdt_docx_asset_from_openxml_sdk() {
  let path = test_file_path("simpleSdt.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    main_document_body_child_count(&original.main_document_part.root_element),
    main_document_body_child_count(&roundtripped.main_document_part.root_element)
  );

  let Some(BodyChildChoice::WSdt(original_sdt)) =
    first_body_child(&original.main_document_part.root_element)
  else {
    panic!("expected first body child to be w:sdt");
  };
  let Some(BodyChildChoice::WSdt(roundtripped_sdt)) =
    first_body_child(&roundtripped.main_document_part.root_element)
  else {
    panic!("expected first body child to be w:sdt");
  };

  let Some(original_properties) = original_sdt.sdt_properties.as_deref() else {
    panic!("expected original w:sdtPr");
  };
  let Some(roundtripped_properties) = roundtripped_sdt.sdt_properties.as_deref() else {
    panic!("expected roundtripped w:sdtPr");
  };
  let Some(SdtPropertiesChildChoice::WAlias(original_alias)) = original_properties.children.first()
  else {
    panic!("expected original w:alias");
  };
  let Some(SdtPropertiesChildChoice::WAlias(roundtripped_alias)) =
    roundtripped_properties.children.first()
  else {
    panic!("expected roundtripped w:alias");
  };

  assert_eq!(original_alias.val.as_str(), "SDT1");
  assert_eq!(roundtripped_alias.val.as_str(), "SDT1");
}

#[test]
fn round_trip_mc_execl_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("MCExecl.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original.workbook_part.shared_string_table_part.is_some(),
    roundtripped
      .workbook_part
      .shared_string_table_part
      .is_some()
  );

  let original_shared_string_table = &original
    .workbook_part
    .shared_string_table_part
    .as_ref()
    .expect("expected shared string table")
    .root_element;
  let roundtripped_shared_string_table = &roundtripped
    .workbook_part
    .shared_string_table_part
    .as_ref()
    .expect("expected shared string table")
    .root_element;

  assert_eq!(original_shared_string_table.x_si.len(), 1);
  assert_eq!(roundtripped_shared_string_table.x_si.len(), 1);
  assert_eq!(
    original_shared_string_table
      .x_si
      .first()
      .and_then(|item| item.text.as_ref())
      .and_then(|text| text.xml_content.as_deref()),
    Some("abc")
  );
  assert_eq!(
    roundtripped_shared_string_table
      .x_si
      .first()
      .and_then(|item| item.text.as_ref())
      .and_then(|text| text.xml_content.as_deref()),
    Some("abc")
  );
}

#[test]
fn round_trip_comments_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Comments.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  assert_eq!(original.workbook_part.root_element.sheets.x_sheet.len(), 1);
  assert_eq!(
    roundtripped.workbook_part.root_element.sheets.x_sheet.len(),
    1
  );
  assert_eq!(original.workbook_part.worksheet_parts.len(), 1);
  assert_eq!(roundtripped.workbook_part.worksheet_parts.len(), 1);

  let original_comments_part = original.workbook_part.worksheet_parts[0]
    .worksheet_comments_part
    .as_ref()
    .expect("expected original worksheet comments part");
  let roundtripped_comments_part = roundtripped.workbook_part.worksheet_parts[0]
    .worksheet_comments_part
    .as_ref()
    .expect("expected roundtripped worksheet comments part");

  assert_eq!(original_comments_part.inner_path, "xl/comments1.xml");
  assert_eq!(roundtripped_comments_part.inner_path, "xl/comments1.xml");
  assert_eq!(
    original_comments_part.root_element.authors.x_author.len(),
    1
  );
  assert_eq!(
    roundtripped_comments_part
      .root_element
      .authors
      .x_author
      .len(),
    1
  );
  assert_eq!(
    original_comments_part
      .root_element
      .comment_list
      .x_comment
      .len(),
    1
  );
  assert_eq!(
    roundtripped_comments_part
      .root_element
      .comment_list
      .x_comment
      .len(),
    1
  );

  let original_comment = &original_comments_part.root_element.comment_list.x_comment[0];
  let roundtripped_comment = &roundtripped_comments_part
    .root_element
    .comment_list
    .x_comment[0];
  assert_eq!(original_comment.reference.as_str(), "A1");
  assert_eq!(roundtripped_comment.reference.as_str(), "A1");
  assert_eq!(original_comment.author_id, 0);
  assert_eq!(roundtripped_comment.author_id, 0);
  assert_eq!(
    original_comments_part.root_element.authors.x_author[0]
      .xml_content
      .as_deref(),
    Some("robermc")
  );
  assert_eq!(
    roundtripped_comments_part.root_element.authors.x_author[0]
      .xml_content
      .as_deref(),
    Some("robermc")
  );

  let original_style_extensions = stylesheet_extensions(&original);
  let roundtripped_style_extensions = stylesheet_extensions(&roundtripped);
  assert_eq!(original_style_extensions.len(), 1);
  assert_eq!(roundtripped_style_extensions.len(), 1);
  assert_eq!(
    original_style_extensions[0]
      .xmlns_map
      .get("x14")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
  assert_eq!(
    roundtripped_style_extensions[0]
      .xmlns_map
      .get("x14")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
}

#[test]
fn round_trip_malformed_uri_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("malformed_uri.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  let original_worksheet_part = &original.workbook_part.worksheet_parts[0];
  let roundtripped_worksheet_part = &roundtripped.workbook_part.worksheet_parts[0];
  let original_relationship = original_worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected original hyperlink relationship");
  let roundtripped_relationship = roundtripped_worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected roundtripped hyperlink relationship");

  assert_eq!(original_relationship.target.as_str(), "mailto:one@");
  assert_eq!(roundtripped_relationship.target.as_str(), "mailto:one@");
}

#[test]
fn round_trip_malformed_uri_long_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("malformed_uri_long.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  let original_worksheet_part = &original.workbook_part.worksheet_parts[0];
  let roundtripped_worksheet_part = &roundtripped.workbook_part.worksheet_parts[0];
  let original_relationship = original_worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected original hyperlink relationship");
  let roundtripped_relationship = roundtripped_worksheet_part
    .relationships
    .as_ref()
    .and_then(|rels| rels.relationship.iter().find(|rel| rel.id == "rId1"))
    .expect("expected roundtripped hyperlink relationship");

  assert_eq!(
    original_relationship.target.as_str(),
    "mailto:test@test.com;%20test2@test.com;%252test3@test.com;%20test3@test.com;%20test4@test.com;%20test5@test.com?subject=Unsubscribe%20Request&body=Please%20unsubscribe%20me%20from%20all%20future%20communications"
  );
  assert_eq!(
    roundtripped_relationship.target.as_str(),
    "mailto:test@test.com;%20test2@test.com;%252test3@test.com;%20test3@test.com;%20test4@test.com;%20test5@test.com?subject=Unsubscribe%20Request&body=Please%20unsubscribe%20me%20from%20all%20future%20communications"
  );
}

#[cfg(feature = "microsoft365")]
#[test]
fn round_trip_youtube_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Youtube.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  let original_drawings_part = original.workbook_part.worksheet_parts[0]
    .drawings_part
    .as_ref()
    .expect("expected original worksheet drawings part");
  let roundtripped_drawings_part = roundtripped.workbook_part.worksheet_parts[0]
    .drawings_part
    .as_ref()
    .expect("expected roundtripped worksheet drawings part");

  assert_eq!(original_drawings_part.web_extension_parts.len(), 1);
  assert_eq!(roundtripped_drawings_part.web_extension_parts.len(), 1);
  assert_eq!(original_drawings_part.image_parts.len(), 1);
  assert_eq!(roundtripped_drawings_part.image_parts.len(), 1);
}

#[test]
fn round_trip_complex01_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);

  let original_workbook = &original.workbook_part.root_element;
  let roundtripped_workbook = &roundtripped.workbook_part.root_element;

  assert_eq!(original_workbook.mc_ignorable.as_deref(), Some("x15"));
  assert_eq!(roundtripped_workbook.mc_ignorable.as_deref(), Some("x15"));
  assert_eq!(original_workbook.sheets.x_sheet.len(), 2);
  assert_eq!(roundtripped_workbook.sheets.x_sheet.len(), 2);
  assert_eq!(original_workbook.sheets.x_sheet[0].name.as_str(), "Sheet1");
  assert_eq!(
    roundtripped_workbook.sheets.x_sheet[0].name.as_str(),
    "Sheet1"
  );
  assert_eq!(original_workbook.sheets.x_sheet[1].name.as_str(), "Sheet2");
  assert_eq!(
    roundtripped_workbook.sheets.x_sheet[1].name.as_str(),
    "Sheet2"
  );

  let original_style_extensions = stylesheet_extensions(&original);
  let roundtripped_style_extensions = stylesheet_extensions(&roundtripped);
  assert_eq!(original_style_extensions.len(), 2);
  assert_eq!(roundtripped_style_extensions.len(), 2);
  assert_eq!(
    original_style_extensions[0]
      .xmlns_map
      .get("x14")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
  assert_eq!(
    roundtripped_style_extensions[0]
      .xmlns_map
      .get("x14")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2009/9/main")
  );
  assert_eq!(
    original_style_extensions[1]
      .xmlns_map
      .get("x15")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2010/11/main")
  );
  assert_eq!(
    roundtripped_style_extensions[1]
      .xmlns_map
      .get("x15")
      .map(String::as_str),
    Some("http://schemas.microsoft.com/office/spreadsheetml/2010/11/main")
  );
  assert_eq!(original.workbook_part.worksheet_parts.len(), 2);
  assert_eq!(roundtripped.workbook_part.worksheet_parts.len(), 2);
}

#[test]
fn round_trip_mcppt_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("mcppt.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );

  let original_table_style_list = &original
    .presentation_part
    .table_styles_part
    .as_ref()
    .expect("expected table styles part")
    .root_element;
  let roundtripped_table_style_list = &roundtripped
    .presentation_part
    .table_styles_part
    .as_ref()
    .expect("expected table styles part")
    .root_element;

  assert_eq!(
    original_table_style_list.default.as_str(),
    "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"
  );
  assert_eq!(
    roundtripped_table_style_list.default.as_str(),
    "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"
  );
}

#[test]
fn open_o09_performance_typical_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("o09_Performance_typical.pptx");
  let package = PresentationDocument::new_from_file(&path).unwrap();

  assert_eq!(package.presentation_part.slide_parts.len(), 11);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert!(package.presentation_part.notes_master_part.is_some());
  assert!(
    package
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert!(package.presentation_part.view_properties_part.is_some());
  assert!(package.presentation_part.table_styles_part.is_some());
  assert!(
    package
      .presentation_part
      .slide_parts
      .iter()
      .any(|slide| slide.notes_slide_part.is_some())
  );
}

#[test]
fn round_trip_o09_performance_typical_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("o09_Performance_typical.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
  assert_eq!(
    original.presentation_part.notes_master_part.is_some(),
    roundtripped.presentation_part.notes_master_part.is_some()
  );
  assert_eq!(
    original
      .presentation_part
      .presentation_properties_part
      .is_some(),
    roundtripped
      .presentation_part
      .presentation_properties_part
      .is_some()
  );
  assert_eq!(
    original.presentation_part.view_properties_part.is_some(),
    roundtripped
      .presentation_part
      .view_properties_part
      .is_some()
  );
}
