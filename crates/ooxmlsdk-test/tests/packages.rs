#![cfg(feature = "parts")]

use std::io::Cursor;

use ooxmlsdk::parts::alternative_format_import_part::AlternativeFormatImportPart;
use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::opc_relationships::Relationship;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  AltChunk, Body, BodyChildChoice, CommentChoice, Document, HyperlinkChildChoice, Paragraph,
  ParagraphChoice, Run, RunChoice, SdtPropertiesChildChoice,
};
use ooxmlsdk_test::fixtures;

const ALT_CHUNK_ID: &str = "XmlAltChunkId-1";
const ALT_CHUNK_TARGET: &str = "afchunk.xml";
const ALT_CHUNK_XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><root><element>Some text</element></root>";

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

fn first_paragraph(
  body: &Body,
) -> Option<&ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Paragraph> {
  body.children.iter().find_map(|child| match child {
    BodyChildChoice::WP(paragraph) => Some(paragraph.as_ref()),
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
    .find_map(|child| match child {
      ParagraphChoice::WR(run) => run.run_choice.iter().find_map(|run_child| match run_child {
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
