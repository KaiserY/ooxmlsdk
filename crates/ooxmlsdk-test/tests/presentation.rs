use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::NonVisualDrawingProperties;
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
  assert!(serialized.starts_with("<p:cNvPr id=\"4\" name=\"[WorkArea]\""));
  assert!(serialized.contains("hidden=\"true\""));
  assert!(
    reparsed
      .description
      .as_deref()
      .is_some_and(|value| value.contains("<GridTheme"))
  );
}
