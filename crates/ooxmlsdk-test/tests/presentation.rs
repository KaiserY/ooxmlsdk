use ooxmlsdk::common::XmlHeaderType;
#[cfg(feature = "microsoft365")]
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ResetShapeProperties;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::{
  NonVisualDrawingProperties, Presentation, SlideSize,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures};

#[test]
fn non_visual_drawing_properties_round_trip_with_embedded_xml() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<NonVisualDrawingProperties>(
    fixtures::PRESENTATION_NON_VISUAL_DRAWING_PROPERTIES_XML,
  );

  assert_eq!(parsed.id, 4);
  assert_eq!(parsed.name, "[WorkArea]");
  assert_eq!(parsed.hidden, Some(true));
  assert!(
    parsed
      .description
      .as_deref()
      .is_some_and(|value| value.contains("<?xml version=\"1.0\" encoding=\"utf-16\"?>"))
  );
  assert!(serialized.starts_with("<p:cNvPr "));
  assert!(serialized.contains("id=\"4\""));
  assert!(serialized.contains("name=\"[WorkArea]\""));
  assert!(serialized.contains("hidden=\"1\""));
  assert!(
    reparsed
      .description
      .as_deref()
      .is_some_and(|value| value.contains("<GridTheme"))
  );
}

#[test]
fn boolean_value_attribute_accepts_upstream_lexical_forms_and_writes_canonical_form() {
  for (raw, expected, canonical) in [
    ("true", true, "1"),
    ("1", true, "1"),
    ("false", false, "0"),
    ("0", false, "0"),
  ] {
    let xml = format!(
      r#"<p:cNvPr xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" id="+4" name="shape" hidden="{raw}"/>"#
    );
    let parsed = xml.parse::<NonVisualDrawingProperties>().unwrap();
    assert_eq!(parsed.id, 4);
    assert_eq!(parsed.hidden, Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(r#"id="4""#));
    assert!(serialized.contains(&format!(r#"hidden="{canonical}""#)));
  }
}

#[test]
fn static_empty_element_serialization_uses_upstream_slash_spacing() {
  let parsed = r#"<p:sldSz xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" cx="12192000" cy="6858000"/>"#
    .parse::<SlideSize>()
    .unwrap();

  assert_eq!(
    parsed.to_xml().unwrap(),
    r#"<p:sldSz cx="12192000" cy="6858000" />"#
  );
}

#[test]
#[cfg(feature = "microsoft365")]
fn empty_child_unit_fields_are_serialized() {
  let parsed = r#"<oac:spPr xmlns:oac="http://schemas.microsoft.com/office/drawing/2013/main/command"><oac:fill/></oac:spPr>"#
    .parse::<ResetShapeProperties>()
    .unwrap();

  assert_eq!(parsed.fill_empty, Some(()));
  assert_eq!(
    parsed.to_xml().unwrap(),
    r#"<oac:spPr><oac:fill /></oac:spPr>"#
  );
}

#[test]
fn presentation_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Presentation>(fixtures::PRESENTATION_PRESENTATION_XML);

  assert_eq!(parsed.xml_header, XmlHeaderType::Standalone);
  assert_eq!(
    parsed
      .slide_master_id_list
      .as_ref()
      .map(|list| list.p_sld_master_id.len()),
    Some(1)
  );
  assert_eq!(
    parsed
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(2)
  );
  assert_eq!(parsed.auto_compress_pictures, Some(false));
  assert!(
    serialized.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")
  );
  assert!(serialized.contains("autoCompressPictures=\"0\""));
  assert!(serialized.contains(r#"<p:sldSz cx="12192000" cy="6858000" />"#));
  assert_eq!(reparsed.xml_header, XmlHeaderType::Standalone);
  assert_eq!(
    reparsed
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(2)
  );
}
