use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::{
  CustomDocumentPropertyChildChoice, Properties as CustomProperties,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VectorBaseValues;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::{
  Properties as ExtendedProperties, PropertiesChildChoice,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

#[test]
fn extended_properties_titles_of_parts_round_trip_from_bug225919_test() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<ExtendedProperties>(
    fixtures::EXTENDED_PROPERTIES_TITLES_OF_PARTS_XML,
  );

  assert_eq!(parsed.children.len(), 1);
  let Some(PropertiesChildChoice::ApTitlesOfParts(titles)) = parsed.children.first() else {
    panic!("expected ap:TitlesOfParts");
  };

  assert_eq!(titles.vt_vector.size, 1);
  assert!(matches!(
    titles.vt_vector.base_type,
    VectorBaseValues::Lpstr
  ));
  assert_eq!(titles.vt_vector.children.len(), 1);
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<ap:Properties"));
  assert!(serialized.contains("<ap:TitlesOfParts>"));
  assert!(serialized.contains("<vt:vector baseType=\"lpstr\" size=\"1\">"));
  assert!(serialized.contains("<vt:lpstr"));
  assert_eq!(reparsed.children.len(), 1);
}

#[test]
fn custom_properties_bool_round_trip_from_bug225919_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CustomProperties>(fixtures::CUSTOM_PROPERTIES_BOOL_XML);

  assert_eq!(parsed.op_property.len(), 1);
  let property = &parsed.op_property[0];
  assert_eq!(property.name.as_deref(), Some("crap"));
  assert_eq!(property.property_id, 2);
  assert_eq!(
    property.format_id.as_str(),
    "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}"
  );
  assert_eq!(property.children.len(), 1);
  let Some(CustomDocumentPropertyChildChoice::VtBool(value)) = property.children.first() else {
    panic!("expected vt:bool");
  };
  assert_eq!(value.0, Some(true));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<op:Properties"));
  assert!(serialized.contains("fmtid=\"{D5CDD505-2E9C-101B-9397-08002B2CF9AE}\""));
  assert!(serialized.contains("pid=\"2\""));
  assert!(serialized.contains("name=\"crap\""));
  assert!(serialized.contains("<vt:bool>true</vt:bool>"));
  assert_eq!(reparsed.op_property.len(), 1);
}
