use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  BodyChildChoice, Document, DocumentChildChoice, Paragraph, ParagraphChildChoice, RunChildChoice,
  TableCellWidth, TableWidthUnitValues, Text,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures};

#[test]
fn document_round_trip_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_XML);

  assert_eq!(
    parsed.xmlns_map.get("w").map(String::as_str),
    Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
  );
  assert!(
    serialized.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")
  );
  assert!(
    serialized.contains("xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"")
  );

  let Some(DocumentChildChoice::WBody(body)) = parsed.children.first() else {
    panic!("expected document body");
  };
  let Some(BodyChildChoice::WP(paragraph)) = body.children.first() else {
    panic!("expected first body child to be paragraph");
  };

  assert_eq!(paragraph.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(reparsed.children.len(), 1);
}

#[test]
fn paragraph_round_trip_from_openxml_element_equality_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);

  assert_eq!(parsed.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(
    serialized,
    "<w:p w:rsidP=\"001\"><w:r><w:t>Run Text.</w:t><w:t>Run 2.</w:t></w:r></w:p>"
  );

  let Some(ParagraphChildChoice::WR(run)) = parsed.children.first() else {
    panic!("expected first paragraph child to be run");
  };

  assert_eq!(run.children.len(), 2);
  assert!(matches!(run.children[0], RunChildChoice::WT(_)));
  assert!(matches!(run.children[1], RunChildChoice::WT(_)));
  assert_eq!(reparsed.children.len(), 1);
}

#[test]
fn text_round_trip_from_openxml_element_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Text>(fixtures::WORDPROCESSING_TEXT_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("Run Text."));
  assert_eq!(serialized, "<w:t>Run Text.</w:t>");
  assert_eq!(reparsed.xml_content.as_deref(), Some("Run Text."));
}

#[test]
fn table_cell_width_round_trip() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<TableCellWidth>(fixtures::WORDPROCESSING_TABLE_CELL_WIDTH_XML);

  assert_eq!(parsed.width.as_deref(), Some("2400"));
  assert!(matches!(parsed.r#type, Some(TableWidthUnitValues::Dxa)));
  assert_eq!(serialized, "<w:tcW w:w=\"2400\" w:type=\"dxa\"/>");
  assert_eq!(reparsed.width.as_deref(), Some("2400"));
  assert!(matches!(reparsed.r#type, Some(TableWidthUnitValues::Dxa)));
}
