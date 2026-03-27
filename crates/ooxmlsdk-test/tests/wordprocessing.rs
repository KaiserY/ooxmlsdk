use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  BodyChildChoice, Document, DocumentChildChoice, Paragraph, ParagraphChildChoice, RunChildChoice,
  TableCellWidth, TableWidthUnitValues, Text,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

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
fn document_round_trip_with_two_paragraphs_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TWO_PARAGRAPHS_XML);

  let Some(DocumentChildChoice::WBody(body)) = parsed.children.first() else {
    panic!("expected document body");
  };

  assert_eq!(body.children.len(), 2);
  assert!(matches!(body.children[0], BodyChildChoice::WP(_)));
  assert!(matches!(body.children[1], BodyChildChoice::WP(_)));
  assert!(
    serialized.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")
  );
  assert_eq!(reparsed.children.len(), 1);
}

#[test]
fn document_round_trip_preserves_whitespace_only_text_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_WHITESPACE_TEXT_XML);

  let Some(DocumentChildChoice::WBody(body)) = parsed.children.first() else {
    panic!("expected document body");
  };
  let Some(BodyChildChoice::WP(paragraph)) = body.children.first() else {
    panic!("expected first body child to be paragraph");
  };
  let Some(ParagraphChildChoice::WR(run)) = paragraph.children.first() else {
    panic!("expected first paragraph child to be run");
  };
  let Some(RunChildChoice::WT(text)) = run.children.first() else {
    panic!("expected first run child to be text");
  };

  assert_eq!(text.xml_content.as_deref(), Some("  "));
  assert!(serialized.contains("<w:t>  </w:t>") || serialized.contains("<w:t xml:space=\"preserve\">  </w:t>"));

  let Some(DocumentChildChoice::WBody(reparsed_body)) = reparsed.children.first() else {
    panic!("expected reparsed document body");
  };
  let Some(BodyChildChoice::WP(reparsed_paragraph)) = reparsed_body.children.first() else {
    panic!("expected reparsed paragraph");
  };
  let Some(ParagraphChildChoice::WR(reparsed_run)) = reparsed_paragraph.children.first() else {
    panic!("expected reparsed run");
  };
  let Some(RunChildChoice::WT(reparsed_text)) = reparsed_run.children.first() else {
    panic!("expected reparsed text");
  };

  assert_eq!(reparsed_text.xml_content.as_deref(), Some("  "));
}

#[test]
fn document_round_trip_from_formatted_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_FORMATTED_XML);

  let Some(DocumentChildChoice::WBody(body)) = parsed.children.first() else {
    panic!("expected document body");
  };
  let Some(BodyChildChoice::WP(paragraph)) = body.children.first() else {
    panic!("expected paragraph");
  };
  let Some(ParagraphChildChoice::WR(run)) = paragraph.children.first() else {
    panic!("expected run");
  };
  let Some(RunChildChoice::WT(text)) = run.children.first() else {
    panic!("expected text");
  };

  assert_eq!(text.xml_content.as_deref(), Some("First Text"));
  assert!(serialized.contains("<w:t>First Text</w:t>"));

  let Some(DocumentChildChoice::WBody(reparsed_body)) = reparsed.children.first() else {
    panic!("expected reparsed body");
  };
  let Some(BodyChildChoice::WP(reparsed_paragraph)) = reparsed_body.children.first() else {
    panic!("expected reparsed paragraph");
  };
  let Some(ParagraphChildChoice::WR(reparsed_run)) = reparsed_paragraph.children.first() else {
    panic!("expected reparsed run");
  };
  let Some(RunChildChoice::WT(reparsed_text)) = reparsed_run.children.first() else {
    panic!("expected reparsed text");
  };
  assert_eq!(reparsed_text.xml_content.as_deref(), Some("First Text"));
}

#[test]
fn document_round_trip_with_trailing_whitespace_after_last_element() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TRAILING_WHITESPACE_XML);

  let Some(DocumentChildChoice::WBody(body)) = parsed.children.first() else {
    panic!("expected document body");
  };
  let Some(BodyChildChoice::WP(paragraph)) = body.children.first() else {
    panic!("expected paragraph");
  };

  assert_eq!(paragraph.children.len(), 0);
  assert!(serialized.contains("<w:p/>") || serialized.contains("<w:p></w:p>"));

  let Some(DocumentChildChoice::WBody(reparsed_body)) = reparsed.children.first() else {
    panic!("expected reparsed body");
  };
  let Some(BodyChildChoice::WP(reparsed_paragraph)) = reparsed_body.children.first() else {
    panic!("expected reparsed paragraph");
  };
  assert_eq!(reparsed_paragraph.children.len(), 0);
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
fn paragraph_round_trip_preserves_child_order_for_sdt_and_run() {
  let (parsed_1, serialized_1, reparsed_1) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_SDT_THEN_RUN_XML);
  let (parsed_2, serialized_2, reparsed_2) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RUN_THEN_SDT_XML);

  assert!(matches!(parsed_1.children[0], ParagraphChildChoice::WSdt(_)));
  assert!(matches!(parsed_1.children[1], ParagraphChildChoice::WR(_)));
  assert!(matches!(parsed_2.children[0], ParagraphChildChoice::WR(_)));
  assert!(matches!(parsed_2.children[1], ParagraphChildChoice::WSdt(_)));
  assert_ne!(
    trim_xml_declaration(&serialized_1),
    trim_xml_declaration(&serialized_2)
  );
  assert!(matches!(reparsed_1.children[0], ParagraphChildChoice::WSdt(_)));
  assert!(matches!(reparsed_1.children[1], ParagraphChildChoice::WR(_)));
  assert!(matches!(reparsed_2.children[0], ParagraphChildChoice::WR(_)));
  assert!(matches!(reparsed_2.children[1], ParagraphChildChoice::WSdt(_)));
}

#[test]
fn paragraph_round_trip_normalizes_rsid_attribute_order() {
  let (parsed_1, serialized_1, reparsed_1) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_ORDER_1_XML);
  let (parsed_2, serialized_2, reparsed_2) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_ORDER_2_XML);

  assert_eq!(parsed_1.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(parsed_1.rsid_paragraph_addition.as_deref(), Some("123"));
  assert_eq!(parsed_1.children.len(), 0);
  assert_eq!(parsed_2.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(parsed_2.rsid_paragraph_addition.as_deref(), Some("123"));
  assert_eq!(parsed_2.children.len(), 0);
  assert_eq!(
    trim_xml_declaration(&serialized_1),
    trim_xml_declaration(&serialized_2)
  );
  assert_eq!(reparsed_1.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(reparsed_2.rsid_paragraph_addition.as_deref(), Some("123"));
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
