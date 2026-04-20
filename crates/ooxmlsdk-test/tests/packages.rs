#![cfg(feature = "parts")]

#[cfg(feature = "microsoft365")]
use std::fs;
use std::io::Cursor;

use ooxmlsdk::common::SdkError;
use ooxmlsdk::parts::alternative_format_import_part::AlternativeFormatImportPart;
use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_relationships::Relationship;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  AltChunk, Body, BodyChoice, CommentChoice, Document, Hyperlink, HyperlinkChoice, Paragraph,
  ParagraphChoice, ParagraphChoice2, Run, RunChoice, SdtPropertiesChoice,
};
use ooxmlsdk_test::fixtures;
#[cfg(feature = "microsoft365")]
use std::io::Read;
#[cfg(feature = "microsoft365")]
use zip::ZipArchive;

const ALT_CHUNK_ID: &str = "XmlAltChunkId-1";
const ALT_CHUNK_TARGET: &str = "afchunk.xml";
const ALT_CHUNK_XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><root><element>Some text</element></root>";

fn keywords_text(value: Option<&ooxmlsdk::schemas::opc_core_properties::Keywords>) -> Option<&str> {
  value.and_then(|keywords| keywords.xml_content.as_deref())
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

#[cfg(feature = "microsoft365")]
fn zip_entry_text(bytes: &[u8], package_name: &str, entry_name: &str) -> String {
  let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap_or_else(|err| {
    panic!("failed to open zip for {package_name}: {err}");
  });
  let mut file = archive.by_name(entry_name).unwrap_or_else(|err| {
    panic!("failed to read {entry_name} from {package_name}: {err}");
  });
  let mut xml = String::new();
  file.read_to_string(&mut xml).unwrap_or_else(|err| {
    panic!("failed to read {entry_name} text from {package_name}: {err}");
  });
  xml
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
    .as_ref()
    .map(|body| body.body_choice.len() + usize::from(body.w_sect_pr.is_some()))
    .unwrap_or_default()
}

fn first_body_child(document: &Document) -> Option<&BodyChoice> {
  document
    .body
    .as_ref()
    .and_then(|body| body.body_choice.first())
}

fn first_body_mut(document: &mut Document) -> Option<&mut Body> {
  document.body.as_mut().map(std::boxed::Box::as_mut)
}

fn first_body(document: &Document) -> Option<&Body> {
  document.body.as_ref().map(std::boxed::Box::as_ref)
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
  body.body_choice.iter().find_map(body_choice_paragraph)
}

fn body_paragraph_count(body: &Body) -> usize {
  body
    .body_choice
    .iter()
    .filter(|child| body_choice_paragraph(child).is_some())
    .count()
}

fn body_choice_paragraph(choice: &BodyChoice) -> Option<&Paragraph> {
  match choice {
    BodyChoice::WP(paragraph) => Some(paragraph.as_ref()),
    _ => None,
  }
}

fn body_choice_sdt_block(
  choice: &BodyChoice,
) -> Option<&ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtBlock> {
  match choice {
    BodyChoice::WSdt(sdt) => Some(sdt.as_ref()),
    _ => None,
  }
}

fn first_hyperlink(paragraph: &Paragraph) -> Option<&Hyperlink> {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(paragraph_choice_hyperlink)
}

fn paragraph_bookmark_start_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_bookmark_start(child))
    .count()
}

fn paragraph_bookmark_end_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_bookmark_end(child))
    .count()
}

fn paragraph_comment_range_start_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_comment_range_start(child))
    .count()
}

fn paragraph_comment_range_end_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_comment_range_end(child))
    .count()
}

fn paragraph_comment_reference_count(
  paragraph: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph,
) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter_map(paragraph_choice_run)
    .map(|run| {
      run
        .run_choice
        .iter()
        .filter(|run_child| matches!(run_child, RunChoice::WCommentReference(_)))
        .count()
    })
    .sum()
}

fn first_paragraph_text(paragraph: &Paragraph) -> Option<&str> {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(|child| match paragraph_choice_run(child) {
      Some(run) => run.run_choice.iter().find_map(|run_child| match run_child {
        RunChoice::WT(text) => text.xml_content.as_deref(),
        _ => None,
      }),
      _ => None,
    })
}

fn append_run_text(run: &Run, out: &mut String) {
  for run_child in &run.run_choice {
    if let RunChoice::WT(text) = run_child
      && let Some(value) = text.xml_content.as_deref()
    {
      out.push_str(value);
    }
  }
}

fn paragraph_text(paragraph: &Paragraph) -> String {
  let mut text = String::new();

  for child in &paragraph.paragraph_choice {
    if let Some(run) = paragraph_choice_run(child) {
      append_run_text(run, &mut text);
    }

    if let Some(hyperlink) = paragraph_choice_hyperlink(child) {
      for hyperlink_child in &hyperlink.xml_children {
        if let HyperlinkChoice::WR(run) = hyperlink_child {
          append_run_text(run.as_ref(), &mut text);
        }
      }
    }
  }

  text
}

fn paragraph_choice_run(choice: &ParagraphChoice) -> Option<&Run> {
  match choice {
    ParagraphChoice::WR(run) => Some(run.as_ref()),
    _ => None,
  }
}

fn paragraph_choice_hyperlink(choice: &ParagraphChoice) -> Option<&Hyperlink> {
  match choice {
    ParagraphChoice::WHyperlink(hyperlink) => Some(hyperlink.as_ref()),
    _ => None,
  }
}

fn paragraph_choice_has_bookmark_start(choice: &ParagraphChoice) -> bool {
  paragraph_choice_has_range_markup(choice, |choice| {
    matches!(choice, ParagraphChoice2::WBookmarkStart(_))
  })
}

fn paragraph_choice_has_bookmark_end(choice: &ParagraphChoice) -> bool {
  paragraph_choice_has_range_markup(choice, |choice| {
    matches!(choice, ParagraphChoice2::WBookmarkEnd(_))
  })
}

fn paragraph_choice_has_comment_range_start(choice: &ParagraphChoice) -> bool {
  paragraph_choice_has_range_markup(choice, |choice| {
    matches!(choice, ParagraphChoice2::WCommentRangeStart(_))
  })
}

fn paragraph_choice_has_comment_range_end(choice: &ParagraphChoice) -> bool {
  paragraph_choice_has_range_markup(choice, |choice| {
    matches!(choice, ParagraphChoice2::WCommentRangeEnd(_))
  })
}

fn paragraph_choice_has_range_markup(
  choice: &ParagraphChoice,
  predicate: impl Fn(&ParagraphChoice2) -> bool,
) -> bool {
  let ParagraphChoice::EgRunLevelElts(choice) = choice else {
    return false;
  };
  predicate(choice.as_ref())
}

fn comment_text(
  comment: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comment,
) -> String {
  let mut text = String::new();

  for child in &comment.comment_choice {
    if let Some(paragraph) = comment_choice_paragraph(child) {
      text.push_str(&paragraph_text(paragraph));
    }
  }

  text
}

fn comment_choice_paragraph(choice: &CommentChoice) -> Option<&Paragraph> {
  match choice {
    CommentChoice::WP(paragraph) => Some(paragraph.as_ref()),
    _ => None,
  }
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

#[test]
#[cfg(feature = "microsoft365")]
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
fn open_unknown_element_docx_asset_currently_fails_on_w_p2() {
  let path = test_file_path("UnknownElement.docx");

  assert_unexpected_tag(WordprocessingDocument::new_from_file(&path), "Body", "w:p2");
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
#[cfg(feature = "microsoft365")]
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

  let Some(sdt) =
    first_body_child(&package.main_document_part.root_element).and_then(body_choice_sdt_block)
  else {
    panic!("expected first body child to be w:sdt");
  };

  assert!(sdt.sdt_properties.is_some());
  assert!(sdt.sdt_content_block.is_some());

  let Some(properties) = sdt.sdt_properties.as_ref() else {
    panic!("expected w:sdtPr");
  };
  let Some(SdtPropertiesChoice::WAlias(alias)) = properties.xml_children.first() else {
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
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .any(|choice| paragraph_choice_hyperlink(choice).is_some())
    })
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
  assert_eq!(first_item.w14_attr.as_deref(), Some("value"));
  assert_eq!(first_item.mc_ignorable.as_deref(), Some("w14"));
  let placeholder = first_item
    .w14_placeholder
    .as_ref()
    .expect("expected w14:placeholder");
  assert_eq!(
    placeholder.mc_process_content.as_deref(),
    Some("w14:placeholder")
  );
  assert_eq!(
    placeholder.mc_preserve_attributes.as_deref(),
    Some("w14:a w14:b")
  );
  let placeholder_text = placeholder
    .text
    .as_ref()
    .expect("expected placeholder text");
  assert_eq!(placeholder_text.w14_a.as_deref(), Some("a"));
  assert_eq!(placeholder_text.w14_b.as_deref(), Some("b"));
  assert_eq!(placeholder_text.w14_c.as_deref(), Some("c"));
  assert_eq!(placeholder_text.xml_content.as_deref(), Some("ddd"));
  assert!(first_item.w14_no.is_some());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
}

#[test]
#[cfg(feature = "microsoft365")]
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
      .map(|part| keywords_text(part.root_element.keywords.as_ref())),
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
  #[cfg(feature = "microsoft365")]
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
  assert_eq!(
    package
      .main_document_part
      .wordprocessing_text_box_parts
      .len(),
    1
  );
  assert!(
    package.main_document_part.wordprocessing_text_box_parts[0]
      .part_content
      .contains("<w14:txbx")
  );
  assert!(package.main_document_part.style_definitions_part.is_some());
  #[cfg(feature = "microsoft365")]
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
  let audio_inner_paths: Vec<&str> = package
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| part.inner_path.as_str())
    })
    .collect();
  let media_inner_paths: Vec<&str> = package
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .media_reference_relationships
        .iter()
        .map(|part| part.inner_path.as_str())
    })
    .collect();
  let image_count: usize = package
    .presentation_part
    .slide_parts
    .iter()
    .map(|slide| slide.image_parts.len())
    .sum();

  assert_eq!(package.presentation_part.slide_parts.len(), 2);
  assert_eq!(package.presentation_part.slide_master_parts.len(), 1);
  assert_eq!(audio_inner_paths, vec!["ppt/media/media1.wav"]);
  assert_eq!(
    media_inner_paths,
    vec!["ppt/media/media1.wav", "ppt/media/media1.wav"]
  );
  assert_eq!(image_count, 2);
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
}

#[cfg(feature = "microsoft365")]
#[test]
fn round_trip_of16_10_symex_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Of16-10-SymEx.docx");
  #[cfg(feature = "microsoft365")]
  let original_bytes = fs::read(&path).unwrap();
  let original = WordprocessingDocument::new_from_file(&path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped_bytes = buffer.into_inner();
  let reopened = WordprocessingDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
  let original_xml = zip_entry_text(&original_bytes, "Of16-10-SymEx.docx", "word/document.xml");
  let roundtripped_xml = zip_entry_text(
    &roundtripped_bytes,
    "Of16-10-SymEx.docx",
    "word/document.xml",
  );

  assert_eq!(
    original.main_document_part.inner_path,
    reopened.main_document_part.inner_path
  );
  assert!(original_xml.contains("<w16se:sym "));
  assert!(original_xml.contains("w:font=\"Webdings\""));
  assert!(original_xml.contains("w:char=\"F04E\""));
  assert!(!original_xml.contains("<w16se:symEx"));
  assert!(roundtripped_xml.contains("<w16se:sym "));
  assert!(roundtripped_xml.contains("w:font=\"Webdings\""));
  assert!(roundtripped_xml.contains("w:char=\"F04E\""));
  assert!(!roundtripped_xml.contains("<w16se:symEx"));
}

#[test]
fn open_complex01_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.xlsx");
  let package = SpreadsheetDocument::new_from_file(&path).unwrap();
  let drawings_part = package.workbook_part.worksheet_parts[0]
    .drawings_part
    .as_ref()
    .expect("expected worksheet drawings part");

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
  assert_eq!(drawings_part.hd_photo_parts.len(), 1);
  assert_eq!(drawings_part.image_parts.len(), 1);
  assert_eq!(
    drawings_part.hd_photo_parts[0].inner_path,
    "xl/media/hdphoto1.wdp"
  );

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
    .body_choice
    .push(BodyChoice::WAltChunk(Box::new(AltChunk {
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
      .as_ref()
      .is_some_and(|body| {
        body
          .body_choice
          .iter()
          .any(|body_child| matches!(body_child, BodyChoice::WAltChunk(_)))
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
      .as_ref()
      .is_some_and(|body| {
        body
          .body_choice
          .iter()
          .any(|body_child| matches!(body_child, BodyChoice::WAltChunk(_)))
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

  assert_eq!(
    original_body.body_choice.len() + usize::from(original_body.w_sect_pr.is_some()),
    2
  );
  assert_eq!(
    roundtripped_body.body_choice.len() + usize::from(roundtripped_body.w_sect_pr.is_some()),
    2
  );
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
#[cfg(feature = "microsoft365")]
fn round_trip_hello_world_docx_asset_from_openxml_sdk() {
  let path = test_file_path("HelloWorld.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = first_paragraph(original_body).expect("expected paragraph");
  let roundtripped_paragraph = first_paragraph(roundtripped_body).expect("expected paragraph");

  assert_eq!(
    original_body.body_choice.len() + usize::from(original_body.w_sect_pr.is_some()),
    2
  );
  assert_eq!(
    roundtripped_body.body_choice.len() + usize::from(roundtripped_body.w_sect_pr.is_some()),
    2
  );
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

  assert!(original_body.body_choice.len() > 10);
  assert!(roundtripped_body.body_choice.len() > 10);
  assert!(
    original_body
      .body_choice
      .iter()
      .any(|child| body_choice_sdt_block(child).is_some())
  );
  assert!(
    roundtripped_body
      .body_choice
      .iter()
      .any(|child| body_choice_sdt_block(child).is_some())
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

  #[cfg(feature = "microsoft365")]
  {
    let original_numbering = &original
      .main_document_part
      .numbering_definitions_part
      .as_ref()
      .expect("expected numbering definitions part")
      .root_element;
    let roundtripped_numbering = &roundtripped
      .main_document_part
      .numbering_definitions_part
      .as_ref()
      .expect("expected numbering definitions part")
      .root_element;

    assert!(!original_numbering.w_abstract_num.is_empty());
    assert_eq!(
      original_numbering.w_abstract_num[0]
        .w15_restart_numbering_after_break
        .as_ref()
        .map(std::string::ToString::to_string),
      Some("0".to_string())
    );
    assert_eq!(
      roundtripped_numbering.w_abstract_num[0]
        .w15_restart_numbering_after_break
        .as_ref()
        .map(std::string::ToString::to_string),
      Some("0".to_string())
    );
  }
}

#[test]
fn round_trip_document_dotx_asset_from_openxml_sdk() {
  let path = test_file_path("Document.dotx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");

  assert_eq!(
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
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

#[test]
#[cfg(feature = "microsoft365")]
fn round_trip_strict01_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Strict01.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.document_settings_part.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

#[test]
fn open_annotation_ref_docx_asset_from_openxml_sdk() {
  let path = test_file_path("AnnotationRef.docx");
  let package = WordprocessingDocument::new_from_file(&path).unwrap();

  assert_eq!(package.main_document_part.inner_path, "word/document.xml");
  assert!(package.main_document_part.relationships.is_some());
  assert!(main_document_body_child_count(&package.main_document_part.root_element) > 0);
}

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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(!original_body.body_choice.is_empty());
  assert!(!roundtripped_body.body_choice.is_empty());
}

#[test]
fn round_trip_hello_o14_docx_asset_from_openxml_sdk() {
  let path = test_file_path("HelloO14.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  let original_body = first_body(&original.main_document_part.root_element).expect("expected body");
  let roundtripped_body =
    first_body(&roundtripped.main_document_part.root_element).expect("expected body");
  let original_paragraph = original_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| paragraph_text(paragraph).contains("Hello O14"))
    .expect("expected paragraph text");
  let roundtripped_paragraph = roundtripped_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| paragraph_text(paragraph).contains("Hello O14"))
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
  );
  assert!(
    original_body
      .body_choice
      .iter()
      .any(|_| original_body.w_sect_pr.is_some())
  );
  assert!(
    roundtripped_body
      .body_choice
      .iter()
      .any(|_| roundtripped_body.w_sect_pr.is_some())
  );

  let original_first_paragraph = original_body
    .body_choice
    .iter()
    .find_map(body_choice_paragraph)
    .expect("expected first paragraph");
  let roundtripped_first_paragraph = roundtripped_body
    .body_choice
    .iter()
    .find_map(body_choice_paragraph)
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
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .map(paragraph_text)
    .collect();
  let roundtripped_paragraph_texts: Vec<String> = roundtripped_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .map(paragraph_text)
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
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .any(|choice| paragraph_choice_hyperlink(choice).is_some())
    })
    .expect("expected paragraph with hyperlink");
  let roundtripped_hyperlink_paragraph = roundtripped_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .any(|choice| paragraph_choice_hyperlink(choice).is_some())
    })
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
    original_body.body_choice.len(),
    roundtripped_body.body_choice.len()
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
    keywords_text(original_core_properties.root_element.keywords.as_ref()),
    Some("Test-Keywords")
  );
  assert_eq!(
    keywords_text(roundtripped_core_properties.root_element.keywords.as_ref()),
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
  #[cfg(feature = "microsoft365")]
  assert!(
    original
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  #[cfg(feature = "microsoft365")]
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
  assert_eq!(
    original
      .main_document_part
      .wordprocessing_text_box_parts
      .len(),
    roundtripped
      .main_document_part
      .wordprocessing_text_box_parts
      .len()
  );
  assert_eq!(
    original.main_document_part.wordprocessing_text_box_parts[0].part_content,
    roundtripped
      .main_document_part
      .wordprocessing_text_box_parts[0]
      .part_content
  );
}

#[test]
fn round_trip_notes_docx_asset_from_openxml_sdk() {
  let path = test_file_path("Notes.docx");
  let (original, roundtripped) = roundtrip_wordprocessing_document(&path);

  assert_eq!(original.main_document_part.image_parts.len(), 2);
  assert_eq!(roundtripped.main_document_part.image_parts.len(), 2);
  #[cfg(feature = "microsoft365")]
  assert!(
    original
      .main_document_part
      .styles_with_effects_part
      .is_some()
  );
  #[cfg(feature = "microsoft365")]
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

  let original_layout_xml = original.main_document_part.diagram_layout_definition_parts[0]
    .root_element
    .to_xml()
    .unwrap();
  let roundtripped_layout_xml = roundtripped
    .main_document_part
    .diagram_layout_definition_parts[0]
    .root_element
    .to_xml()
    .unwrap();
  for layout_xml in [&original_layout_xml, &roundtripped_layout_xml] {
    assert!(layout_xml.contains("val=\"INF\""));
    assert!(layout_xml.contains("fact=\"NaN\""));
    assert!(layout_xml.contains("max=\"NaN\""));
  }
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
  let original_audio_parts: Vec<(&str, &Vec<u8>)> = original
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();
  let roundtripped_audio_parts: Vec<(&str, &Vec<u8>)> = roundtripped
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();
  let original_media_parts: Vec<(&str, &Vec<u8>)> = original
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .media_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();
  let roundtripped_media_parts: Vec<(&str, &Vec<u8>)> = roundtripped
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .media_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();
  let original_image_count: usize = original
    .presentation_part
    .slide_parts
    .iter()
    .map(|slide| slide.image_parts.len())
    .sum();
  let roundtripped_image_count: usize = roundtripped
    .presentation_part
    .slide_parts
    .iter()
    .map(|slide| slide.image_parts.len())
    .sum();

  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
  assert_eq!(original_audio_parts, roundtripped_audio_parts);
  assert_eq!(original_media_parts, roundtripped_media_parts);
  assert_eq!(original_image_count, roundtripped_image_count);
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

  fn find_cell<'a>(
    package: &'a SpreadsheetDocument,
    sheet_path: &str,
    cell_ref: &str,
  ) -> &'a ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Cell {
    package
      .workbook_part
      .worksheet_parts
      .iter()
      .find(|part| part.inner_path == sheet_path)
      .and_then(|part| {
        part
          .root_element
          .x_sheet_data
          .x_row
          .iter()
          .flat_map(|row| row.x_c.iter())
          .find(|cell| cell.cell_reference.as_deref() == Some(cell_ref))
      })
      .unwrap_or_else(|| panic!("expected {sheet_path} {cell_ref}"))
  }

  let original_c21 = find_cell(&original, "xl/worksheets/sheet5.xml", "C21");
  let roundtripped_c21 = find_cell(&roundtripped, "xl/worksheets/sheet5.xml", "C21");

  assert_eq!(
    original_c21
      .cell_formula
      .as_ref()
      .and_then(|formula| formula.xml_content.as_deref()),
    Some("IFERROR(C20*0.15,\" \")")
  );
  assert_eq!(
    roundtripped_c21
      .cell_formula
      .as_ref()
      .and_then(|formula| formula.xml_content.as_deref()),
    Some("IFERROR(C20*0.15,\" \")")
  );
  assert_eq!(
    original_c21
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref()),
    Some(" ")
  );
  assert_eq!(
    roundtripped_c21
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref()),
    Some(" ")
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

  let Some(original_sdt) =
    first_body_child(&original.main_document_part.root_element).and_then(body_choice_sdt_block)
  else {
    panic!("expected first body child to be w:sdt");
  };
  let Some(roundtripped_sdt) =
    first_body_child(&roundtripped.main_document_part.root_element).and_then(body_choice_sdt_block)
  else {
    panic!("expected first body child to be w:sdt");
  };

  let Some(original_properties) = original_sdt.sdt_properties.as_ref() else {
    panic!("expected original w:sdtPr");
  };
  let Some(roundtripped_properties) = roundtripped_sdt.sdt_properties.as_ref() else {
    panic!("expected roundtripped w:sdtPr");
  };
  let Some(SdtPropertiesChoice::WAlias(original_alias)) = original_properties.xml_children.first()
  else {
    panic!("expected original w:alias");
  };
  let Some(SdtPropertiesChoice::WAlias(roundtripped_alias)) =
    roundtripped_properties.xml_children.first()
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
  let original_item = original_shared_string_table
    .x_si
    .first()
    .expect("expected original si");
  let roundtripped_item = roundtripped_shared_string_table
    .x_si
    .first()
    .expect("expected roundtripped si");
  assert_eq!(original_item.w14_attr.as_deref(), Some("value"));
  assert_eq!(roundtripped_item.w14_attr.as_deref(), Some("value"));
  assert_eq!(original_item.mc_ignorable.as_deref(), Some("w14"));
  assert_eq!(roundtripped_item.mc_ignorable.as_deref(), Some("w14"));
  assert!(original_item.w14_placeholder.is_some());
  assert!(roundtripped_item.w14_placeholder.is_some());
  assert!(original_item.w14_no.is_some());
  assert!(roundtripped_item.w14_no.is_some());
  assert_eq!(
    original_item
      .text
      .as_ref()
      .and_then(|text| text.xml_content.as_deref()),
    Some("abc")
  );
  assert_eq!(
    roundtripped_item
      .text
      .as_ref()
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
  #[cfg(feature = "microsoft365")]
  let original_bytes = fs::read(&path).unwrap();
  let original = SpreadsheetDocument::new_from_file(&path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped_bytes = buffer.into_inner();
  let roundtripped = SpreadsheetDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
  #[cfg(feature = "microsoft365")]
  let original_workbook_xml = zip_entry_text(
    &original_bytes,
    "malformed_uri_long.xlsx",
    "xl/workbook.xml",
  );
  #[cfg(feature = "microsoft365")]
  let roundtripped_workbook_xml = zip_entry_text(
    &roundtripped_bytes,
    "malformed_uri_long.xlsx",
    "xl/workbook.xml",
  );

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
  assert_eq!(
    original_worksheet_part.root_element.xr_uid.as_deref(),
    Some("{00000000-0001-0000-0000-000000000000}")
  );
  assert_eq!(
    roundtripped_worksheet_part.root_element.xr_uid.as_deref(),
    Some("{00000000-0001-0000-0000-000000000000}")
  );
  let original_hyperlink = original_worksheet_part
    .root_element
    .x_hyperlinks
    .as_ref()
    .and_then(|hyperlinks| hyperlinks.x_hyperlink.first())
    .expect("expected original hyperlink element");
  let roundtripped_hyperlink = roundtripped_worksheet_part
    .root_element
    .x_hyperlinks
    .as_ref()
    .and_then(|hyperlinks| hyperlinks.x_hyperlink.first())
    .expect("expected roundtripped hyperlink element");
  assert_eq!(
    original_hyperlink.xr_uid.as_deref(),
    Some("{00000000-0004-0000-0000-000000000000}")
  );
  assert_eq!(
    roundtripped_hyperlink.xr_uid.as_deref(),
    Some("{00000000-0004-0000-0000-000000000000}")
  );
  #[cfg(feature = "microsoft365")]
  assert!(original_workbook_xml.contains("<xr:revisionPtr "));
  #[cfg(feature = "microsoft365")]
  assert!(original_workbook_xml.contains("xr6:coauthVersionLast=\"46\""));
  #[cfg(feature = "microsoft365")]
  assert!(original_workbook_xml.contains("xr6:coauthVersionMax=\"46\""));
  #[cfg(feature = "microsoft365")]
  assert!(
    original_workbook_xml.contains("xr10:uidLastSave=\"{00000000-0000-0000-0000-000000000000}\"")
  );
  #[cfg(feature = "microsoft365")]
  assert!(original_workbook_xml.contains("xr2:uid=\"{00000000-000D-0000-FFFF-FFFF00000000}\""));
  #[cfg(feature = "microsoft365")]
  assert!(roundtripped_workbook_xml.contains("<xr:revisionPtr "));
  #[cfg(feature = "microsoft365")]
  assert!(roundtripped_workbook_xml.contains("xr6:coauthVersionLast=\"46\""));
  #[cfg(feature = "microsoft365")]
  assert!(roundtripped_workbook_xml.contains("xr6:coauthVersionMax=\"46\""));
  #[cfg(feature = "microsoft365")]
  assert!(
    roundtripped_workbook_xml
      .contains("xr10:uidLastSave=\"{00000000-0000-0000-0000-000000000000}\"")
  );
  #[cfg(feature = "microsoft365")]
  assert!(roundtripped_workbook_xml.contains("xr2:uid=\"{00000000-000D-0000-FFFF-FFFF00000000}\""));
}

#[cfg(feature = "microsoft365")]
#[test]
fn round_trip_youtube_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Youtube.xlsx");
  let original_bytes = fs::read(&path).unwrap();
  let original = SpreadsheetDocument::new_from_file(&path).unwrap();
  let mut buffer = Cursor::new(Vec::new());

  original.save(&mut buffer).unwrap();

  let roundtripped_bytes = buffer.into_inner();
  let roundtripped = SpreadsheetDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
  let original_workbook_xml = zip_entry_text(&original_bytes, "Youtube.xlsx", "xl/workbook.xml");
  let roundtripped_workbook_xml =
    zip_entry_text(&roundtripped_bytes, "Youtube.xlsx", "xl/workbook.xml");

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
  assert!(original_workbook_xml.contains("<x15:absPath "));
  assert!(!original_workbook_xml.contains("<x15ac:absPath"));
  assert!(roundtripped_workbook_xml.contains("<x15:absPath "));
  assert!(!roundtripped_workbook_xml.contains("<x15ac:absPath"));
}

#[test]
fn round_trip_complex01_xlsx_asset_from_openxml_sdk() {
  let path = test_file_path("Complex01.xlsx");
  let (original, roundtripped) = roundtrip_spreadsheet_document(&path);
  let original_drawings_part = original.workbook_part.worksheet_parts[0]
    .drawings_part
    .as_ref()
    .expect("expected original worksheet drawings part");
  let roundtripped_drawings_part = roundtripped.workbook_part.worksheet_parts[0]
    .drawings_part
    .as_ref()
    .expect("expected roundtripped worksheet drawings part");

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
  assert_eq!(
    original_drawings_part.hd_photo_parts.len(),
    roundtripped_drawings_part.hd_photo_parts.len()
  );
  assert_eq!(
    original_drawings_part.hd_photo_parts[0].inner_path,
    roundtripped_drawings_part.hd_photo_parts[0].inner_path
  );
  assert_eq!(
    original_drawings_part.hd_photo_parts[0].part_content,
    roundtripped_drawings_part.hd_photo_parts[0].part_content
  );
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
  let audio_inner_paths: Vec<&str> = package
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| part.inner_path.as_str())
    })
    .collect();
  let vml_image_inner_paths: Vec<&str> = package
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide.vml_drawing_parts.iter().flat_map(|part| {
        part
          .image_parts
          .iter()
          .map(|image| image.inner_path.as_str())
      })
    })
    .collect();
  let mut sorted_vml_image_inner_paths = vml_image_inner_paths.clone();
  sorted_vml_image_inner_paths.sort_unstable();

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
  assert_eq!(audio_inner_paths, vec!["ppt/media/audio1.wav"]);
  assert_eq!(
    sorted_vml_image_inner_paths,
    vec![
      "ppt/media/image1.emf",
      "ppt/media/image2.emf",
      "ppt/media/image3.wmf",
      "ppt/media/image4.wmf",
      "ppt/media/image5.wmf",
      "ppt/media/image6.wmf",
      "ppt/media/image7.wmf",
    ]
  );
}

#[test]
fn round_trip_o09_performance_typical_pptx_asset_from_openxml_sdk() {
  let path = test_file_path("o09_Performance_typical.pptx");
  let (original, roundtripped) = roundtrip_presentation_document(&path);
  let original_audio_parts: Vec<(&str, &Vec<u8>)> = original
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();
  let original_vml_image_parts: Vec<(&str, &Vec<u8>)> = original
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide.vml_drawing_parts.iter().flat_map(|part| {
        part
          .image_parts
          .iter()
          .map(|image| (image.inner_path.as_str(), &image.part_content))
      })
    })
    .collect();
  let roundtripped_vml_image_parts: Vec<(&str, &Vec<u8>)> = roundtripped
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide.vml_drawing_parts.iter().flat_map(|part| {
        part
          .image_parts
          .iter()
          .map(|image| (image.inner_path.as_str(), &image.part_content))
      })
    })
    .collect();
  let roundtripped_audio_parts: Vec<(&str, &Vec<u8>)> = roundtripped
    .presentation_part
    .slide_parts
    .iter()
    .flat_map(|slide| {
      slide
        .audio_reference_relationships
        .iter()
        .map(|part| (part.inner_path.as_str(), &part.part_content))
    })
    .collect();

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
  assert_eq!(original_audio_parts, roundtripped_audio_parts);
  assert_eq!(original_vml_image_parts, roundtripped_vml_image_parts);
}
