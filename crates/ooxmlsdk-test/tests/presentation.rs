use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::{
  NonVisualDrawingProperties, Presentation,
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
  assert!(serialized.contains("hidden=\"true\""));
  assert!(
    reparsed
      .description
      .as_deref()
      .is_some_and(|value| value.contains("<GridTheme"))
  );
}

#[test]
fn presentation_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Presentation>(fixtures::PRESENTATION_PRESENTATION_XML);

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
    serialized.contains("autoCompressPictures=\"false\"")
      || serialized.contains("autoCompressPictures=\"0\"")
  );
  assert_eq!(
    reparsed
      .slide_id_list
      .as_ref()
      .map(|list| list.p_sld_id.len()),
    Some(2)
  );
}
