use std::io::{Cursor, Read};

#[cfg(feature = "microsoft365")]
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  BodyChoice, Document, Paragraph, ParagraphChoice, ParagraphProperties, Run, Text,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures};

fn doc_sample_part(file_name: &str, part_name: &str) -> String {
  let bytes = std::fs::read(fixtures::doc_sample_path(file_name)).unwrap();
  let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
  let mut part = archive.by_name(part_name).unwrap();
  let mut xml = String::new();
  part.read_to_string(&mut xml).unwrap();
  xml
}

fn first_paragraph(document: &Document) -> &Paragraph {
  document
    .body
    .as_ref()
    .expect("expected body")
    .body_choice
    .iter()
    .find_map(|choice| match choice {
      BodyChoice::WP(paragraph) => Some(paragraph.as_ref()),
      _ => None,
    })
    .expect("expected paragraph")
}

fn first_run(paragraph: &Paragraph) -> &Run {
  paragraph
    .paragraph_choice
    .iter()
    .find_map(|choice| match choice {
      ParagraphChoice::WR(run) => Some(run.as_ref()),
      _ => None,
    })
    .expect("expected run")
}

fn paragraph_properties(paragraph: &Paragraph) -> &ParagraphProperties {
  paragraph
    .paragraph_properties
    .as_ref()
    .expect("expected paragraph properties")
}

#[test]
fn mcsupport_load_attribute_test() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   LoadAttributeTest
  let xml = doc_sample_part("mcdoc.docx", "word/document.xml");

  let (document, serialized, reparsed) = assert_stable_roundtrip::<Document>(&xml);

  assert_eq!(document.mc_ignorable.as_deref(), Some("w14 wp14"));
  assert_eq!(reparsed.mc_ignorable.as_deref(), Some("w14 wp14"));
  assert!(serialized.contains(r#"mc:Ignorable="w14 wp14""#));
  assert!(serialized.contains(r#"mc:PreserveAttributes="w14:myattr""#));
  assert!(serialized.contains(r#"mc:PreserveAttributes="w14:*""#));
}

#[test]
fn mcsupport_load_preserve_attr() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   LoadPreserveAttr
  let xml = doc_sample_part("mcdoc.docx", "word/document.xml");

  let (document, _, _) = assert_stable_roundtrip::<Document>(&xml);
  let paragraph = first_paragraph(&document);
  let properties = paragraph_properties(paragraph);
  let spacing = properties
    .spacing_between_lines
    .as_ref()
    .expect("expected spacing");
  let run = first_run(paragraph);
  let run_properties = run
    .run_properties
    .as_ref()
    .expect("expected run properties");

  assert_eq!(properties.w14_myattr.as_deref(), Some("myattr"));
  assert_eq!(spacing.w14_myattr.as_deref(), Some("myattr"));
  assert_eq!(run.w14_myattr.as_deref(), Some("myattr"));
  assert_eq!(
    run_properties.w14_myanother_attr.as_deref(),
    Some("anotherattr")
  );
}

#[cfg(feature = "mce")]
#[test]
#[ignore = "calibration: Office2007 inherited Ignorable processing is not implemented yet"]
fn mcsupport_load_ignorable() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   LoadIgnorable
  let xml = doc_sample_part("mcdoc.docx", "word/document.xml");

  let document = xml.parse::<Document>().unwrap();
  let paragraph = first_paragraph(&document);

  assert!(
    paragraph.w14_edit_id.is_none(),
    "ProcessLoadedPartsOnly + Office2007 drops ignored w14:editId in the upstream SDK"
  );
}

#[cfg(feature = "microsoft365")]
#[test]
fn mcsupport_load_process_content() {
  // Source: test/DocumentFormat.OpenXml.Tests/ofapiTest/MCSupport.cs
  //   LoadProcessContent
  let xml = doc_sample_part("MCExecl.xlsx", "xl/sharedStrings.xml");

  let (table, serialized, _) = assert_stable_roundtrip::<SharedStringTable>(&xml);
  let item = table.x_si.first().expect("expected shared string item");
  let placeholder = item.w14_placeholder.as_ref().expect("expected placeholder");
  let text = placeholder
    .text
    .as_ref()
    .expect("expected placeholder text");

  assert_eq!(item.mc_ignorable.as_deref(), Some("w14"));
  assert_eq!(item.w14_attr.as_deref(), Some("value"));
  assert_eq!(
    placeholder.mc_process_content.as_deref(),
    Some("w14:placeholder")
  );
  assert_eq!(
    placeholder.mc_preserve_attributes.as_deref(),
    Some("w14:a w14:b")
  );
  assert_eq!(text.w14_a.as_deref(), Some("a"));
  assert_eq!(text.w14_b.as_deref(), Some("b"));
  assert!(serialized.contains(r#"mc:ProcessContent="w14:placeholder""#));
}

#[test]
fn markup_compatibility_ignore_whitespaces_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Ignore_Whitespaces_FullMode
  let xml = r#"<w:t xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="  &#x9;&#xA;&#xD; ">text</w:t>"#;

  let (text, serialized, reparsed) = assert_stable_roundtrip::<Text>(xml);

  assert_eq!(text.mc_ignorable.as_deref(), Some("  \t\n\r "));
  assert_eq!(reparsed.mc_ignorable.as_deref(), Some("  \t\n\r "));
  assert!(serialized.contains("mc:Ignorable=\"  \t\n\r \""));
}

#[test]
fn markup_compatibility_ignored_known_attribute_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Ignored_KnownAttribute_FullMode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14" w14:myattr="attribute1 from unknown namespace1."><w:keepNext/></w:pPr>"#;

  let (properties, serialized, reparsed) = assert_stable_roundtrip::<ParagraphProperties>(xml);

  assert_eq!(
    properties.w14_myattr.as_deref(),
    Some("attribute1 from unknown namespace1.")
  );
  assert_eq!(
    reparsed.w14_myattr.as_deref(),
    Some("attribute1 from unknown namespace1.")
  );
  assert!(serialized.contains(r#"mc:Ignorable="w14""#));
  assert!(serialized.contains(r#"w14:myattr="attribute1 from unknown namespace1.""#));
}

#[cfg(feature = "mce")]
#[test]
#[ignore = "calibration: Office2007 ignored known attribute processing is not implemented yet"]
fn markup_compatibility_ignored_known_attribute_o12_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Ignored_KnownAttribute_O12Mode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14" w14:myattr="attribute1 from unknown namespace1."><w:keepNext/></w:pPr>"#;

  let properties = xml.parse::<ParagraphProperties>().unwrap();

  assert!(
    properties.w14_myattr.is_none(),
    "ProcessAllParts/Office2007 removes ignored known extension attributes upstream"
  );
}

#[test]
fn markup_compatibility_process_content_ignored_unknown_element_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   ProcessContent_Ignored_UnknownElement_FullMode
  let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:uns1="http://test.openxmlsdk.microsoft.com/unknownns1" mc:Ignorable="uns1" mc:ProcessContent="uns1:e1uk1"><mc:Choice Requires="uns1"><uns1:e1uk1><uns1:child/></uns1:e1uk1></mc:Choice><mc:Fallback/></mc:AlternateContent>"#;

  let (alternate_content, serialized, _) = assert_stable_roundtrip::<
    ooxmlsdk::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
  >(xml);

  assert_eq!(alternate_content.mc_ignorable.as_deref(), Some("uns1"));
  assert_eq!(
    alternate_content.mc_process_content.as_deref(),
    Some("uns1:e1uk1")
  );
  assert!(serialized.contains(r#"mc:ProcessContent="uns1:e1uk1""#));
  assert!(serialized.contains("<uns1:e1uk1>"));
}

#[cfg(feature = "mce")]
#[test]
fn markup_compatibility_process_content_ignored_known_element_o12_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   ProcessContent_Ignored_KnownElement_O12Mode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w" mc:ProcessContent="w:keepNext"><w:keepNext/></w:pPr>"#;

  let properties = xml.parse::<ParagraphProperties>().unwrap();

  assert!(
    properties.keep_next.is_some(),
    "ProcessContent keeps children of an ignored known element upstream"
  );
}

#[test]
fn markup_compatibility_process_content_xml_space_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   ProcessContent_xmlSpace_FullMode
  let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:uns1="http://test.openxmlsdk.microsoft.com/unknownns1" xmlns:xml="http://www.w3.org/XML/1998/namespace" mc:Ignorable="uns1" mc:ProcessContent="xml:space"><mc:Choice Requires="uns1"><uns1:e1uk1 xml:space="preserve"> spaced </uns1:e1uk1></mc:Choice><mc:Fallback/></mc:AlternateContent>"#;

  let (alternate_content, serialized, _) = assert_stable_roundtrip::<
    ooxmlsdk::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
  >(xml);

  assert_eq!(
    alternate_content.mc_process_content.as_deref(),
    Some("xml:space")
  );
  assert!(serialized.contains(r#"xml:space="preserve""#));
}

#[test]
fn markup_compatibility_preserve_ignored_unknown_element_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Preserve_Ignored_UnknownElement_FullMode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:uns1="http://test.openxmlsdk.microsoft.com/unknownns1" mc:Ignorable="uns1" mc:PreserveElements="uns1:e1uk1" mc:PreserveAttributes="uns1:a1uk1"><w:keepNext/></w:pPr>"#;

  let (_, serialized, _) = assert_stable_roundtrip::<ParagraphProperties>(xml);

  assert!(serialized.contains(r#"mc:Ignorable="uns1""#));
  assert!(serialized.contains(r#"mc:PreserveElements="uns1:e1uk1""#));
  assert!(serialized.contains(r#"mc:PreserveAttributes="uns1:a1uk1""#));
}

#[test]
fn markup_compatibility_preserve_ignored_unknown_element_wildcard_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Preserve_Ignored_UnknownElement_Wildcard_FullMode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14" mc:PreserveAttributes="*" w14:myattr="attribute1 from unknown namespace1."><w:keepNext/></w:pPr>"#;

  let (properties, serialized, reparsed) = assert_stable_roundtrip::<ParagraphProperties>(xml);

  assert_eq!(properties.mc_preserve_attributes.as_deref(), Some("*"));
  assert_eq!(
    properties.w14_myattr.as_deref(),
    Some("attribute1 from unknown namespace1.")
  );
  assert_eq!(
    reparsed.w14_myattr.as_deref(),
    Some("attribute1 from unknown namespace1.")
  );
  assert!(serialized.contains(r#"mc:PreserveAttributes="*""#));
}

#[cfg(feature = "mce")]
#[test]
fn markup_compatibility_preserve_ignored_unknown_element_wildcard_o12_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   Preserve_Ignored_UnknownElement_Wildcard_O12Mode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14" mc:PreserveAttributes="*" w14:myattr="attribute1 from unknown namespace1."><w:keepNext/></w:pPr>"#;

  let properties = xml.parse::<ParagraphProperties>().unwrap();

  assert_eq!(
    properties.w14_myattr.as_deref(),
    Some("attribute1 from unknown namespace1."),
    "PreserveAttributes=* keeps ignored extension attributes upstream"
  );
}

#[test]
fn markup_compatibility_must_understand_ignored_unknown_element_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   MustUnderstand_Ignored_UnknownElement_FullMode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:uns1="http://test.openxmlsdk.microsoft.com/unknownns1" mc:Ignorable="uns1" mc:MustUnderstand="uns1"><w:keepNext/></w:pPr>"#;

  let (_, serialized, _) = assert_stable_roundtrip::<ParagraphProperties>(xml);

  assert!(serialized.contains(r#"mc:Ignorable="uns1""#));
  assert!(serialized.contains(r#"mc:MustUnderstand="uns1""#));
}

#[cfg(feature = "mce")]
#[test]
#[ignore = "calibration: MustUnderstand prefix validation is not implemented yet"]
fn markup_compatibility_must_understand_ignored_unknown_element_o12_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   MustUnderstand_Ignored_UnknownElement_O12Mode
  let xml = r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:MustUnderstand="uns1"><w:keepNext/></w:pPr>"#;

  let parsed = xml.parse::<ParagraphProperties>();

  assert!(
    parsed.is_err(),
    "MCE processing should reject an unresolved MustUnderstand prefix upstream"
  );
}

#[test]
fn markup_compatibility_must_understand_unselected_full_mode() {
  // Source: test/DocumentFormat.OpenXml.Tests/OpenXmlDomTest/MarkupCompatibilityTest.cs
  //   MustUnderstand_Unselected_FullMode
  let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:uns1="http://test.openxmlsdk.microsoft.com/unknownns1"><mc:Choice Requires="uns1" mc:MustUnderstand="uns1"><uns1:e1uk1/></mc:Choice></mc:AlternateContent>"#;

  let (alternate_content, serialized, _) = assert_stable_roundtrip::<
    ooxmlsdk::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
  >(xml);

  assert_eq!(alternate_content.alternate_content_choice.len(), 1);
  assert!(serialized.contains(r#"mc:MustUnderstand="uns1""#));
}
