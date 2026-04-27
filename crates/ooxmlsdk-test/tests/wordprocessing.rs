use ooxmlsdk::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContentChoice;
#[cfg(feature = "microsoft365")]
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::LevelJustification;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  Body, BodyChoice, CommentChoice, Comments, Document, Hyperlink, HyperlinkChoice, Justification,
  Paragraph, ParagraphChoice, ParagraphChoice2, Run, RunChoice, SdtBlock, SdtPropertiesChoice,
  TabStop, TableJustification, Text, TextDirection,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

fn first_body(document: &Document) -> &Body {
  document.body.as_ref().expect("expected document body")
}

fn first_paragraph(body: &Body) -> &Paragraph {
  body
    .body_choice
    .iter()
    .find_map(body_choice_paragraph)
    .expect("expected body paragraph")
}

fn first_run(paragraph: &Paragraph) -> &Run {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(paragraph_choice_run)
    .expect("expected paragraph run")
}

fn first_hyperlink(paragraph: &Paragraph) -> &Hyperlink {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(paragraph_choice_hyperlink)
    .expect("expected paragraph hyperlink")
}

fn first_hyperlink_run(hyperlink: &Hyperlink) -> &Run {
  hyperlink
    .xml_children
    .iter()
    .find_map(|child| match child {
      HyperlinkChoice::WR(run) => Some(run.as_ref()),
      _ => None,
    })
    .expect("expected hyperlink run")
}

fn first_sdt_block(body: &Body) -> &SdtBlock {
  body
    .body_choice
    .iter()
    .find_map(body_choice_sdt_block)
    .expect("expected body sdt block")
}

fn first_text(run: &Run) -> &Text {
  run
    .run_choice
    .iter()
    .find_map(|child| match child {
      RunChoice::WT(text) => Some(text.as_ref()),
      _ => None,
    })
    .expect("expected run text")
}

fn run_texts(run: &Run) -> Vec<&Text> {
  run
    .run_choice
    .iter()
    .filter_map(|child| match child {
      RunChoice::WT(text) => Some(text.as_ref()),
      _ => None,
    })
    .collect()
}

fn append_run_text(run: &Run, out: &mut String) {
  for text in run_texts(run) {
    if let Some(value) = text.xml_content.as_deref() {
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

fn paragraph_run_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_run(child).is_some())
    .count()
}

fn paragraph_sdt_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_is_sdt(child))
    .count()
}

fn paragraph_bookmark_start_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_bookmark_start(child))
    .count()
}

fn paragraph_bookmark_end_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_bookmark_end(child))
    .count()
}

fn paragraph_comment_range_start_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_comment_range_start(child))
    .count()
}

fn paragraph_comment_range_end_count(paragraph: &Paragraph) -> usize {
  paragraph
    .paragraph_choice
    .iter()
    .filter(|child| paragraph_choice_has_comment_range_end(child))
    .count()
}

fn paragraph_comment_reference_count(paragraph: &Paragraph) -> usize {
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

fn comment_text(
  comment: &ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comment,
) -> String {
  let mut text = String::new();

  for child in &comment.comment_choice {
    if let Some(paragraph) = comment_choice_paragraph(child) {
      for paragraph_child in &paragraph.paragraph_choice {
        if let Some(run) = paragraph_choice_run(paragraph_child) {
          append_run_text(run, &mut text);
        }
      }
    }
  }

  text
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

fn body_choice_sdt_block(choice: &BodyChoice) -> Option<&SdtBlock> {
  match choice {
    BodyChoice::WSdt(sdt) => Some(sdt.as_ref()),
    _ => None,
  }
}

fn body_choice_has_bookmark_start(choice: &BodyChoice) -> bool {
  matches!(choice, BodyChoice::WBookmarkStart(_))
}

fn body_choice_has_bookmark_end(choice: &BodyChoice) -> bool {
  matches!(choice, BodyChoice::WBookmarkEnd(_))
}

fn comment_choice_paragraph(choice: &CommentChoice) -> Option<&Paragraph> {
  match choice {
    CommentChoice::WP(paragraph) => Some(paragraph.as_ref()),
    _ => None,
  }
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

fn paragraph_choice_is_sdt(choice: &ParagraphChoice) -> bool {
  matches!(choice, ParagraphChoice::WSdt(_))
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

#[test]
fn document_attribute_translation_test() {
  let xml = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" conformance="strict" />"#;
  let document = xml.parse::<Document>().unwrap();

  assert!(document.w_conformance.is_none());
}

#[test]
fn justification_attribute_translation_test() {
  const NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  for (value, expected) in [("start", "left"), ("end", "right")] {
    let xml = format!(r#"<w:jc xmlns:w="{NAMESPACE}" w:val="{value}" />"#);
    let element = xml.parse::<Justification>().unwrap();

    assert_eq!(element.val.to_string(), expected);
  }
}

#[test]
fn table_justification_attribute_translation_test() {
  const NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  for (value, expected) in [("start", "left"), ("end", "right")] {
    let xml = format!(r#"<w:jc xmlns:w="{NAMESPACE}" w:val="{value}" />"#);
    let element = xml.parse::<TableJustification>().unwrap();

    assert_eq!(element.val.to_string(), expected);
  }
}

#[test]
fn tab_stop_attribute_translation_test() {
  const NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  for (value, expected) in [("start", "left"), ("end", "right")] {
    let xml = format!(r#"<w:tab xmlns:w="{NAMESPACE}" w:val="{value}" w:pos="12" />"#);
    let element = xml.parse::<TabStop>().unwrap();

    assert_eq!(element.val.to_string(), expected);
  }
}

#[test]
fn text_direction_attribute_translation_test() {
  const NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  for (value, expected) in [
    ("lr", "btLr"),
    ("tb", "lrTb"),
    ("tbV", "lrTbV"),
    ("lrV", "tbLrV"),
    ("rl", "tbRl"),
    ("rlV", "tbRlV"),
  ] {
    let xml = format!(r#"<w:textDirection xmlns:w="{NAMESPACE}" w:val="{value}" />"#);
    let element = xml.parse::<TextDirection>().unwrap();

    assert_eq!(element.val.to_string(), expected);
  }
}

#[cfg(feature = "microsoft365")]
#[test]
fn level_justification_attribute_translation_test() {
  const NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  for (value, expected) in [("start", "left"), ("end", "right")] {
    let xml = format!(r#"<w:lvlJc xmlns:w="{NAMESPACE}" w:val="{value}" />"#);
    let element = xml.parse::<LevelJustification>().unwrap();

    assert_eq!(element.w_val.to_string(), expected);
  }
}

#[test]
fn document_round_trip_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_XML);

  assert_eq!(
    ooxmlsdk::common::find_xmlns_uri(&parsed.xmlns, "w"),
    Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
  );
  assert!(!serialized.starts_with("<?xml"));
  assert!(
    serialized.contains("xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"")
  );

  let paragraph = first_paragraph(first_body(&parsed));
  assert_eq!(paragraph.rsid_paragraph_properties.as_deref(), Some("001"));
  assert!(reparsed.body.is_some());
}

#[test]
fn document_round_trip_drops_misc_node_from_part_reader_misc_node_test() {
  assert!(fixtures::WORDPROCESSING_DOCUMENT_XML.contains("<!-- start body -->"));

  let (_parsed, serialized, _reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_XML);

  assert!(!serialized.contains("<!-- start body -->"));
  assert!(serialized.contains("<w:body"));
}

#[test]
fn document_round_trip_with_two_paragraphs_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TWO_PARAGRAPHS_XML);

  let body = first_body(&parsed);
  assert_eq!(body.body_choice.len(), 2);
  assert!(body_choice_paragraph(&body.body_choice[0]).is_some());
  assert!(body_choice_paragraph(&body.body_choice[1]).is_some());
  assert!(
    serialized.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")
  );
  assert!(reparsed.body.is_some());
}

#[test]
fn document_round_trip_preserves_whitespace_only_text_from_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_WHITESPACE_TEXT_XML);

  let text = first_text(first_run(first_paragraph(first_body(&parsed))));
  assert_eq!(text.xml_content.as_deref(), Some("  "));
  assert!(serialized.contains("<w:t") && serialized.contains(">  </w:t>"));

  let reparsed_text = first_text(first_run(first_paragraph(first_body(&reparsed))));
  assert_eq!(reparsed_text.xml_content.as_deref(), Some("  "));
}

#[test]
fn document_round_trip_from_formatted_openxml_reader_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_FORMATTED_XML);

  let text = first_text(first_run(first_paragraph(first_body(&parsed))));
  assert_eq!(text.xml_content.as_deref(), Some("First Text"));
  assert!(serialized.contains("<w:t"));
  assert!(serialized.contains(">First Text</w:t>"));

  let reparsed_text = first_text(first_run(first_paragraph(first_body(&reparsed))));
  assert_eq!(reparsed_text.xml_content.as_deref(), Some("First Text"));
}

#[test]
fn document_round_trip_with_trailing_whitespace_after_last_element() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_TRAILING_WHITESPACE_XML);

  let paragraph = first_paragraph(first_body(&parsed));
  assert_eq!(paragraph_run_count(paragraph), 0);
  assert!(
    serialized.contains("<w:p/>")
      || serialized.contains("<w:p></w:p>")
      || serialized.contains("<w:p ")
  );

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
fn document_round_trip_preserves_hyperlink_structure_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_HYPERLINK_XML);

  let body = first_body(&parsed);
  assert_eq!(body_paragraph_count(body), 2);
  assert!(body.body_choice.iter().any(|_| body.w_sect_pr.is_some()));

  let hyperlink_paragraph = body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .any(|child| paragraph_choice_hyperlink(child).is_some())
    })
    .expect("expected paragraph with hyperlink");

  let hyperlink = first_hyperlink(hyperlink_paragraph);
  assert_eq!(hyperlink.id.as_deref(), Some("rId4"));
  assert_eq!(hyperlink.history, Some(true));
  let run = first_hyperlink_run(hyperlink);
  let text = first_text(run);
  assert_eq!(text.xml_content.as_deref(), Some("EricWhite.com"));
  assert!(serialized.contains("<w:hyperlink"));
  assert!(serialized.contains("r:id=\"rId4\""));
  assert!(serialized.contains("w:history=\"true\"") || serialized.contains("w:history=\"1\""));
  assert!(serialized.contains("EricWhite.com"));

  let reparsed_body = first_body(&reparsed);
  assert_eq!(body_paragraph_count(reparsed_body), 2);
  let reparsed_hyperlink_paragraph = reparsed_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .any(|child| paragraph_choice_hyperlink(child).is_some())
    })
    .expect("expected reparsed paragraph with hyperlink");
  let reparsed_hyperlink = first_hyperlink(reparsed_hyperlink_paragraph);
  assert_eq!(reparsed_hyperlink.id.as_deref(), Some("rId4"));
  assert_eq!(
    first_text(first_hyperlink_run(reparsed_hyperlink))
      .xml_content
      .as_deref(),
    Some("EricWhite.com")
  );
}

#[test]
fn document_round_trip_preserves_bookmarks_and_text_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_PLAIN_XML);

  let body = first_body(&parsed);
  let paragraph = first_paragraph(body);
  assert_eq!(body_paragraph_count(body), 1);
  assert_eq!(paragraph_bookmark_start_count(paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(paragraph), 1);

  let text = first_text(first_run(paragraph));
  assert!(
    text
      .xml_content
      .as_deref()
      .unwrap_or_default()
      .starts_with("Video provides a powerful way")
  );
  assert!(serialized.contains("<w:bookmarkStart"));
  assert!(serialized.contains("w:name=\"_GoBack\""));
  assert!(serialized.contains("w:id=\"0\""));
  assert!(serialized.contains("<w:bookmarkEnd"));
  assert!(serialized.contains("Video provides a powerful way"));

  let reparsed_body = first_body(&reparsed);
  let reparsed_paragraph = first_paragraph(reparsed_body);
  assert_eq!(body_paragraph_count(reparsed_body), 1);
  assert_eq!(paragraph_bookmark_start_count(reparsed_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(reparsed_paragraph), 1);
  assert!(
    first_text(first_run(reparsed_paragraph))
      .xml_content
      .as_deref()
      .unwrap_or_default()
      .starts_with("Video provides a powerful way")
  );
}

#[test]
fn document_round_trip_preserves_hello_world_text_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_HELLO_WORLD_XML);

  let body = first_body(&parsed);
  let paragraph = first_paragraph(body);
  assert_eq!(body_paragraph_count(body), 1);
  assert_eq!(paragraph_run_count(paragraph), 1);
  assert_eq!(
    first_text(first_run(paragraph)).xml_content.as_deref(),
    Some("Hello World!")
  );
  assert!(serialized.contains("<w:t"));
  assert!(serialized.contains(">Hello World!</w:t>"));

  let reparsed_body = first_body(&reparsed);
  let reparsed_paragraph = first_paragraph(reparsed_body);
  assert_eq!(body_paragraph_count(reparsed_body), 1);
  assert_eq!(paragraph_run_count(reparsed_paragraph), 1);
  assert_eq!(
    first_text(first_run(reparsed_paragraph))
      .xml_content
      .as_deref(),
    Some("Hello World!")
  );
}

#[test]
fn document_round_trip_preserves_hello_o14_structure_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_HELLO_O14_XML);

  let body = first_body(&parsed);
  let paragraph = body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| paragraph_text(paragraph).contains("Hello O14"))
    .expect("expected paragraph with Hello O14");
  assert_eq!(paragraph_bookmark_start_count(paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(paragraph), 1);
  assert!(paragraph_text(paragraph).contains("Hello O14"));
  assert!(serialized.contains("Hello O14"));
  assert!(serialized.contains("<w:bookmarkStart"));
  assert!(serialized.contains("<w:sectPr"));

  let reparsed_body = first_body(&reparsed);
  let reparsed_paragraph = reparsed_body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find(|paragraph| paragraph_text(paragraph).contains("Hello O14"))
    .expect("expected paragraph with Hello O14");
  assert_eq!(paragraph_bookmark_start_count(reparsed_paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(reparsed_paragraph), 1);
  assert!(paragraph_text(reparsed_paragraph).contains("Hello O14"));
}

#[test]
fn document_round_trip_preserves_mce_attributes_and_alternate_content() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   LoadAttributeTest
  //   LoadPreserveAttr
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   NonIgnored_UnknownElement_FullMode
  //   ProcessContent_NonIgnored_UnknownElement_FullMode
  //   MustUnderstand_NonIgnored_UnknownElement_FullMode
  let xml = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14"><w:body><w:p><w:r mc:PreserveAttributes="w14:editId" w14:myattr="myattr"><mc:AlternateContent mc:MustUnderstand="w14" mc:ProcessContent="w14:unknown"><mc:Choice Requires="w14"><w14:unknown attr="1">choice</w14:unknown></mc:Choice><mc:Fallback><w:t>fallback</w:t></mc:Fallback></mc:AlternateContent><w:t>after</w:t></w:r></w:p></w:body></w:document>"#;

  let (document, serialized, _) = assert_stable_roundtrip::<Document>(xml);

  assert_eq!(document.mc_ignorable.as_deref(), Some("w14"));
  let run = first_run(first_paragraph(first_body(&document)));
  assert_eq!(run.mc_preserve_attributes.as_deref(), Some("w14:editId"));
  assert_eq!(run.w14_myattr.as_deref(), Some("myattr"));

  let alternate_content = run
    .run_choice
    .iter()
    .find_map(|choice| match choice {
      RunChoice::McAlternateContent(alternate_content) => Some(alternate_content.as_ref()),
      _ => None,
    })
    .expect("expected alternate content in run");
  assert_eq!(alternate_content.mc_must_understand.as_deref(), Some("w14"));
  assert_eq!(
    alternate_content.mc_process_content.as_deref(),
    Some("w14:unknown")
  );

  let choice = alternate_content
    .alternate_content_choice
    .iter()
    .find_map(|choice| match choice {
      AlternateContentChoice::McChoice(choice) => Some(choice.as_ref()),
      _ => None,
    })
    .expect("expected mc:Choice");
  assert_eq!(choice.requires.as_deref(), Some("w14"));
  assert_eq!(
    choice.xml_children,
    vec![r#"<w14:unknown attr="1">choice</w14:unknown>"#.to_string()]
  );

  let fallback = alternate_content
    .alternate_content_choice
    .iter()
    .find_map(|choice| match choice {
      AlternateContentChoice::McFallback(fallback) => Some(fallback.as_ref()),
      _ => None,
    })
    .expect("expected mc:Fallback");
  assert_eq!(fallback.xml_children, vec![r#"<w:t>fallback</w:t>"#]);
  assert!(serialized.contains(r#"mc:Ignorable="w14""#));
  assert!(serialized.contains(r#"mc:PreserveAttributes="w14:editId""#));
  assert!(serialized.contains(r#"mc:MustUnderstand="w14""#));
  assert!(serialized.contains(r#"mc:ProcessContent="w14:unknown""#));
  assert!(serialized.contains(r#"<w14:unknown attr="1">choice</w14:unknown>"#));
}

#[test]
fn document_round_trip_preserves_comments_document_structure_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_COMMENTS_XML);

  let body = first_body(&parsed);
  let paragraph = first_paragraph(body);
  assert_eq!(paragraph_bookmark_start_count(paragraph), 1);
  assert_eq!(paragraph_bookmark_end_count(paragraph), 1);
  assert_eq!(paragraph_comment_range_start_count(paragraph), 1);
  assert_eq!(paragraph_comment_range_end_count(paragraph), 1);
  assert_eq!(paragraph_comment_reference_count(paragraph), 1);
  assert!(
    first_text(first_run(paragraph))
      .xml_content
      .as_deref()
      .unwrap_or_default()
      .starts_with("When ")
  );
  assert!(serialized.contains("<w:commentRangeStart"));
  assert!(serialized.contains("<w:commentRangeEnd"));
  assert!(serialized.contains("<w:commentReference"));
  assert!(serialized.contains("you click Online Video"));
  assert!(serialized.contains("<w:sectPr"));
  assert!(reparsed.body.is_some());

  let reparsed_body = first_body(&reparsed);
  let reparsed_paragraph = first_paragraph(reparsed_body);
  assert_eq!(paragraph_comment_range_start_count(reparsed_paragraph), 1);
  assert_eq!(paragraph_comment_range_end_count(reparsed_paragraph), 1);
  assert_eq!(paragraph_comment_reference_count(reparsed_paragraph), 1);
}

#[test]
fn comments_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Comments>(fixtures::WORDPROCESSING_COMMENTS_XML);

  assert_eq!(parsed.w_comment.len(), 1);
  let comment = parsed.w_comment.first().expect("expected comment");
  assert_eq!(comment.id.as_str(), "1");
  assert_eq!(comment.author.as_str(), "Eric White");
  assert_eq!(comment.initials.as_deref(), Some("EW"));
  assert_eq!(comment_text(comment), "This is a comment.");
  assert!(serialized.contains("<w:comment"));
  assert!(serialized.contains("Eric White"));
  assert!(serialized.contains("This is a comment."));
  assert_eq!(reparsed.w_comment.len(), 1);
}

#[test]
fn document_round_trip_preserves_rich_content_and_hyperlinks_from_openxml_asset() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Document>(fixtures::WORDPROCESSING_DOCUMENT_DOCUMENT_XML);

  let body = first_body(&parsed);
  assert!(
    body
      .body_choice
      .iter()
      .any(|child| body_choice_sdt_block(child).is_some())
  );
  assert!(body.body_choice.iter().any(body_choice_has_bookmark_start));
  assert!(body.body_choice.iter().any(body_choice_has_bookmark_end));
  assert!(
    body
      .body_choice
      .iter()
      .any(|child| body_choice_paragraph(child).is_some())
  );

  let sdt = first_sdt_block(body);
  let Some(properties) = sdt.sdt_properties.as_ref() else {
    panic!("expected w:sdtPr");
  };
  let Some(SdtPropertiesChoice::WAlias(alias)) = properties.xml_children.first() else {
    panic!("expected sdt alias");
  };
  assert_eq!(alias.val.as_str(), "RichTextContentControl");

  let hyperlink = body
    .body_choice
    .iter()
    .filter_map(body_choice_paragraph)
    .find_map(|paragraph| {
      paragraph.paragraph_choice.iter().find_map(|child| {
        let hyperlink = paragraph_choice_hyperlink(child)?;
        hyperlink
          .xml_children
          .iter()
          .any(|hyperlink_child| match hyperlink_child {
            HyperlinkChoice::WR(run) => {
              first_text(run.as_ref()).xml_content.as_deref() == Some("EricWhite.com")
            }
            _ => false,
          })
          .then_some(hyperlink)
      })
    })
    .expect("expected EricWhite.com hyperlink");
  assert_eq!(hyperlink.id.as_deref(), Some("rId26"));
  assert_eq!(hyperlink.history, Some(true));
  assert_eq!(
    first_text(first_hyperlink_run(hyperlink))
      .xml_content
      .as_deref(),
    Some("EricWhite.com")
  );
  assert!(serialized.contains("RichTextContentControl"));
  assert!(serialized.contains("EricWhite.com"));
  assert!(serialized.contains("Heading1"));

  let reparsed_body = first_body(&reparsed);
  assert!(
    reparsed_body
      .body_choice
      .iter()
      .any(|child| body_choice_sdt_block(child).is_some())
  );
  assert!(
    reparsed_body
      .body_choice
      .iter()
      .any(body_choice_has_bookmark_start)
  );
  assert!(
    reparsed_body
      .body_choice
      .iter()
      .any(body_choice_has_bookmark_end)
  );
}

#[test]
fn paragraph_round_trip_from_openxml_element_equality_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_XML);

  assert_eq!(parsed.rsid_paragraph_properties.as_deref(), Some("001"));
  assert!(serialized.starts_with("<w:p "));
  assert!(serialized.contains("w:rsidP=\"001\""));
  assert!(serialized.contains("<w:r"));
  assert!(serialized.contains(">Run Text.</w:t>"));
  assert!(serialized.contains(">Run 2.</w:t>"));

  let run = first_run(&parsed);
  assert_eq!(run_texts(run).len(), 2);
  assert_eq!(paragraph_run_count(&reparsed), 1);
}

#[test]
fn paragraph_round_trip_from_attribute_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Paragraph>(fixtures::WORDPROCESSING_PARAGRAPH_RSID_P_002_XML);

  assert_eq!(parsed.rsid_paragraph_properties.as_deref(), Some("002"));
  assert!(serialized.starts_with("<w:p "));
  assert!(serialized.contains("w:rsidP=\"002\""));
  assert!(serialized.contains("<w:r"));
  assert!(serialized.contains(">Run Text.</w:t>"));
  assert!(serialized.contains(">Run 2.</w:t>"));

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
  assert!(serialized.starts_with("<w:p "));
  assert!(serialized.contains("w:rsidP=\"001\""));
  assert!(serialized.contains("<w:r"));
  assert!(serialized.contains(">Run Text.</w:t>"));
  assert!(serialized.contains(">Run 1.</w:t>"));
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
      || trim_xml_declaration(&serialized).contains("<w:r ")
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
  assert!(trim_xml_declaration(&serialized).starts_with("<w:p "));
  assert!(trim_xml_declaration(&serialized).contains("w:rsidP=\"001\""));
  assert!(trim_xml_declaration(&serialized).contains(">Run Text.</w:t>"));
  assert!(trim_xml_declaration(&serialized).contains(">Run 2.</w:t>"));

  assert_eq!(run_texts(first_run(&reparsed)).len(), 2);
}
