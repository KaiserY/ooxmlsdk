use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  Body, BodyChildChoice, Document, DocumentChildChoice, Paragraph, ParagraphChildChoice, Run,
  RunChildChoice, Text,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

fn first_body(document: &Document) -> &Body {
  document
    .children
    .iter()
    .find_map(|child| match child {
      DocumentChildChoice::WBody(body) => Some(body.as_ref()),
      _ => None,
    })
    .expect("expected document body")
}

fn first_paragraph(body: &Body) -> &Paragraph {
  body
    .children
    .iter()
    .find_map(|child| match child {
      BodyChildChoice::WP(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
    .expect("expected body paragraph")
}

fn first_run(paragraph: &Paragraph) -> &Run {
  paragraph
    .children
    .iter()
    .find_map(|child| match child {
      ParagraphChildChoice::WR(run) => Some(run.as_ref()),
      _ => None,
    })
    .expect("expected paragraph run")
}

fn first_text(run: &Run) -> &Text {
  run
    .children
    .iter()
    .find_map(|child| match child {
      RunChildChoice::WT(text) => Some(text.as_ref()),
      _ => None,
    })
    .expect("expected run text")
}

fn run_texts(run: &Run) -> Vec<&Text> {
  run
    .children
    .iter()
    .filter_map(|child| match child {
      RunChildChoice::WT(text) => Some(text.as_ref()),
      _ => None,
    })
    .collect()
}

fn paragraph_run_count(paragraph: &Paragraph) -> usize {
  paragraph
    .children
    .iter()
    .filter(|child| matches!(child, ParagraphChildChoice::WR(_)))
    .count()
}

fn paragraph_sdt_count(paragraph: &Paragraph) -> usize {
  paragraph
    .children
    .iter()
    .filter(|child| matches!(child, ParagraphChildChoice::WSdt(_)))
    .count()
}

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

  let paragraph = first_paragraph(first_body(&parsed));
  assert_eq!(paragraph.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(reparsed.children.len(), 1);
}

#[test]
fn document_round_trip_drops_misc_node_from_part_reader_misc_node_test() {
  assert!(fixtures::WORDPROCESSING_DOCUMENT_XML.contains("<!-- start body -->"));

  let (_parsed, serialized, _reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_XML);

  assert!(!serialized.contains("<!-- start body -->"));
  assert!(serialized.contains("<w:body>"));
}

#[test]
fn document_round_trip_with_two_paragraphs_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TWO_PARAGRAPHS_XML);

  let body = first_body(&parsed);
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

  let text = first_text(first_run(first_paragraph(first_body(&parsed))));
  assert_eq!(text.xml_content.as_deref(), Some("  "));
  assert!(
    serialized.contains("<w:t>  </w:t>")
      || serialized.contains("<w:t xml:space=\"preserve\">  </w:t>")
  );

  let reparsed_text = first_text(first_run(first_paragraph(first_body(&reparsed))));
  assert_eq!(reparsed_text.xml_content.as_deref(), Some("  "));
}

#[test]
fn document_round_trip_from_formatted_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_FORMATTED_XML);

  let text = first_text(first_run(first_paragraph(first_body(&parsed))));
  assert_eq!(text.xml_content.as_deref(), Some("First Text"));
  assert!(serialized.contains("<w:t>First Text</w:t>"));

  let reparsed_text = first_text(first_run(first_paragraph(first_body(&reparsed))));
  assert_eq!(reparsed_text.xml_content.as_deref(), Some("First Text"));
}

#[test]
fn document_round_trip_with_trailing_whitespace_after_last_element() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TRAILING_WHITESPACE_XML);

  let paragraph = first_paragraph(first_body(&parsed));
  assert_eq!(paragraph_run_count(paragraph), 0);
  assert!(serialized.contains("<w:p/>") || serialized.contains("<w:p></w:p>"));

  let reparsed_paragraph = first_paragraph(first_body(&reparsed));
  assert_eq!(paragraph_run_count(reparsed_paragraph), 0);
}

#[test]
fn document_round_trip_with_trailing_comment_after_document() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TRAILING_COMMENT_XML);

  let paragraph = first_paragraph(first_body(&parsed));
  assert_eq!(paragraph_run_count(paragraph), 0);
  assert!(!serialized.contains("<!--Your comment-->"));

  let reparsed_paragraph = first_paragraph(first_body(&reparsed));
  assert_eq!(paragraph_run_count(reparsed_paragraph), 0);
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

  let run = first_run(&parsed);
  assert_eq!(run_texts(run).len(), 2);
  assert_eq!(paragraph_run_count(&reparsed), 1);
}

#[test]
fn paragraph_round_trip_from_attribute_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_P_002_XML);

  assert_eq!(parsed.rsid_paragraph_properties.as_deref(), Some("002"));
  assert_eq!(
    serialized,
    "<w:p w:rsidP=\"002\"><w:r><w:t>Run Text.</w:t><w:t>Run 2.</w:t></w:r></w:p>"
  );

  let run = first_run(&parsed);
  assert_eq!(run_texts(run).len(), 2);
  assert_eq!(paragraph_run_count(&reparsed), 1);
}

#[test]
fn paragraph_serialization_differs_for_different_attribute_value_test() {
  let (_, serialized_rsid_p_001, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);
  let (_, serialized_rsid_p_002, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_P_002_XML);

  assert_ne!(
    trim_xml_declaration(&serialized_rsid_p_001),
    trim_xml_declaration(&serialized_rsid_p_002)
  );
}

#[test]
fn paragraph_round_trip_from_different_child_value_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RUN_1_XML);

  assert_eq!(parsed.rsid_paragraph_properties.as_deref(), Some("001"));
  let run = first_run(&parsed);
  let texts = run_texts(run);
  assert_eq!(texts.len(), 2);
  assert_eq!(texts[0].xml_content.as_deref(), Some("Run Text."));
  assert_eq!(texts[1].xml_content.as_deref(), Some("Run 1."));
  assert_eq!(
    serialized,
    "<w:p w:rsidP=\"001\"><w:r><w:t>Run Text.</w:t><w:t>Run 1.</w:t></w:r></w:p>"
  );
  assert_eq!(paragraph_run_count(&reparsed), 1);
}

#[test]
fn paragraph_serialization_differs_for_different_child_value_test() {
  let (_, serialized_run_1, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RUN_1_XML);
  let (_, serialized_run_2, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);

  assert_ne!(
    trim_xml_declaration(&serialized_run_1),
    trim_xml_declaration(&serialized_run_2)
  );
}

#[test]
fn paragraph_round_trip_preserves_sdt_and_run_presence() {
  let (parsed_1, serialized_1, reparsed_1) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_SDT_THEN_RUN_XML);
  let (parsed_2, serialized_2, reparsed_2) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RUN_THEN_SDT_XML);

  assert_eq!(paragraph_sdt_count(&parsed_1), 1);
  assert_eq!(paragraph_run_count(&parsed_1), 1);
  assert_eq!(paragraph_sdt_count(&parsed_2), 1);
  assert_eq!(paragraph_run_count(&parsed_2), 1);
  assert_ne!(
    trim_xml_declaration(&serialized_1),
    trim_xml_declaration(&serialized_2)
  );
  assert_eq!(paragraph_sdt_count(&reparsed_1), 1);
  assert_eq!(paragraph_run_count(&reparsed_1), 1);
  assert_eq!(paragraph_sdt_count(&reparsed_2), 1);
  assert_eq!(paragraph_run_count(&reparsed_2), 1);
}

#[test]
fn paragraph_round_trip_normalizes_rsid_attribute_order() {
  let (parsed_1, serialized_1, reparsed_1) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_ORDER_1_XML);
  let (parsed_2, serialized_2, reparsed_2) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_ORDER_2_XML);

  assert_eq!(parsed_1.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(parsed_1.rsid_paragraph_addition.as_deref(), Some("123"));
  assert_eq!(paragraph_run_count(&parsed_1), 0);
  assert_eq!(parsed_2.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(parsed_2.rsid_paragraph_addition.as_deref(), Some("123"));
  assert_eq!(paragraph_run_count(&parsed_2), 0);
  assert_eq!(
    trim_xml_declaration(&serialized_1),
    trim_xml_declaration(&serialized_2)
  );
  assert_eq!(reparsed_1.rsid_paragraph_properties.as_deref(), Some("001"));
  assert_eq!(reparsed_2.rsid_paragraph_addition.as_deref(), Some("123"));
}

#[test]
fn paragraph_round_trip_from_different_amount_of_children_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_ONLY_RUN_XML);

  assert_eq!(paragraph_run_count(&parsed), 1);
  assert!(
    trim_xml_declaration(&serialized) == "<w:p w:rsidP=\"001\"><w:r/></w:p>"
      || trim_xml_declaration(&serialized) == "<w:p w:rsidP=\"001\"><w:r></w:r></w:p>"
  );
  assert_eq!(paragraph_run_count(&reparsed), 1);
}

#[test]
fn paragraph_serialization_differs_for_different_amount_of_children_test() {
  let (_, serialized_run_only, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_ONLY_RUN_XML);
  let (_, serialized_with_text, _) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);

  assert_ne!(
    trim_xml_declaration(&serialized_run_only),
    trim_xml_declaration(&serialized_with_text)
  );
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
fn paragraph_text_content_matches_inner_xml_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);

  let texts = run_texts(first_run(&parsed));
  let concatenated = format!(
    "{}{}",
    texts[0].xml_content.as_deref().unwrap_or_default(),
    texts[1].xml_content.as_deref().unwrap_or_default()
  );

  assert_eq!(concatenated, "Run Text.Run 2.");
  assert_eq!(
    trim_xml_declaration(&serialized),
    "<w:p w:rsidP=\"001\"><w:r><w:t>Run Text.</w:t><w:t>Run 2.</w:t></w:r></w:p>"
  );

  assert_eq!(run_texts(first_run(&reparsed)).len(), 2);
}
