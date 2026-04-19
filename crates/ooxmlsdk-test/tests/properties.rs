use ooxmlsdk::schemas::opc_core_properties::CoreProperties;
use ooxmlsdk::schemas::opc_core_properties::XsiTypeValue;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::{
  CustomDocumentPropertyChoice, Properties as CustomProperties,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VectorBaseValues;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::{
  Variant, VariantChoice,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties as ExtendedProperties;
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

#[test]
fn core_properties_round_trip_from_hello_world_doc_props_test() {
  let parsed = fixtures::OPC_CORE_PROPERTIES_HELLO_WORLD_XML
    .parse::<CoreProperties>()
    .unwrap();
  let serialized = parsed.to_xml().unwrap();
  let reparsed = serialized.parse::<CoreProperties>().unwrap();
  let serialized_twice = reparsed.to_xml().unwrap();

  assert_eq!(serialized, serialized_twice);

  assert_eq!(parsed.title.as_deref(), Some(""));
  assert_eq!(parsed.subject.as_deref(), Some(""));
  assert_eq!(parsed.creator.as_deref(), Some("Thomas Barnekow"));
  assert_eq!(parsed.last_modified_by.as_deref(), Some("Thomas Barnekow"));
  assert_eq!(parsed.revision.as_deref(), Some("1"));
  assert_eq!(
    parsed
      .created
      .as_ref()
      .and_then(|v| v.xml_content.as_deref()),
    Some("2024-10-26T22:14:00Z")
  );
  assert_eq!(
    parsed
      .modified
      .as_ref()
      .and_then(|v| v.xml_content.as_deref()),
    Some("2024-10-26T22:15:00Z")
  );
  assert!(matches!(
    parsed.created.as_ref().and_then(|v| v.xsi_type.as_ref()),
    Some(XsiTypeValue::DctermsW3cdtf)
  ));
  assert!(matches!(
    parsed.modified.as_ref().and_then(|v| v.xsi_type.as_ref()),
    Some(XsiTypeValue::DctermsW3cdtf)
  ));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<cp:coreProperties"));
  assert!(serialized.contains("<dc:creator"));
  assert!(serialized.contains(">Thomas Barnekow</dc:creator>"));
  assert!(serialized.contains("<cp:lastModifiedBy"));
  assert!(serialized.contains(">Thomas Barnekow</cp:lastModifiedBy>"));
  assert_eq!(reparsed.creator.as_deref(), Some("Thomas Barnekow"));
}

#[test]
fn core_properties_round_trip_from_more_doc_props_test() {
  let parsed = fixtures::OPC_CORE_PROPERTIES_MORE_DOC_PROPS_XML
    .parse::<CoreProperties>()
    .unwrap();
  let serialized = parsed.to_xml().unwrap();
  let reparsed = serialized.parse::<CoreProperties>().unwrap();
  let serialized_twice = reparsed.to_xml().unwrap();

  assert_eq!(serialized, serialized_twice);

  assert_eq!(parsed.creator.as_deref(), Some("Eric White"));
  assert_eq!(parsed.last_modified_by.as_deref(), Some("Eric White"));
  assert_eq!(parsed.revision.as_deref(), Some("6"));
  assert_eq!(
    parsed
      .created
      .as_ref()
      .and_then(|v| v.xml_content.as_deref()),
    Some("2014-10-28T11:34:00Z")
  );
  assert_eq!(
    parsed
      .modified
      .as_ref()
      .and_then(|v| v.xml_content.as_deref()),
    Some("2015-06-20T07:40:00Z")
  );
  assert!(matches!(
    parsed.created.as_ref().and_then(|v| v.xsi_type.as_ref()),
    Some(XsiTypeValue::DctermsW3cdtf)
  ));
  assert!(matches!(
    parsed.modified.as_ref().and_then(|v| v.xsi_type.as_ref()),
    Some(XsiTypeValue::DctermsW3cdtf)
  ));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<cp:coreProperties"));
  assert!(serialized.contains("<dc:creator"));
  assert!(serialized.contains(">Eric White</dc:creator>"));
  assert!(serialized.contains("<cp:lastModifiedBy"));
  assert!(serialized.contains(">Eric White</cp:lastModifiedBy>"));
  assert_eq!(reparsed.creator.as_deref(), Some("Eric White"));
}

#[test]
fn extended_properties_round_trip_from_more_doc_props_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<ExtendedProperties>(fixtures::EXTENDED_PROPERTIES_MORE_DOC_PROPS_XML);

  assert_eq!(parsed.template.as_deref(), Some("Normal.dotm"));
  assert_eq!(parsed.application.as_deref(), Some("Microsoft Office Word"));
  assert_eq!(parsed.application_version.as_deref(), Some("15.0000"));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<Properties"));
  assert!(serialized.contains("extended-properties"));
  assert!(serialized.contains("<Template"));
  assert!(serialized.contains(">Normal.dotm</Template>"));
  assert!(serialized.contains("<Application"));
  assert!(serialized.contains(">Microsoft Office Word</Application>"));
  assert!(serialized.contains("<AppVersion"));
  assert!(serialized.contains(">15.0000</AppVersion>"));
  assert_eq!(parsed.template.is_some(), reparsed.template.is_some());
  assert_eq!(parsed.application.is_some(), reparsed.application.is_some());
  assert_eq!(
    parsed.application_version.is_some(),
    reparsed.application_version.is_some()
  );
}

#[test]
fn extended_properties_titles_of_parts_round_trip_from_bug225919_test() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<ExtendedProperties>(
    fixtures::EXTENDED_PROPERTIES_TITLES_OF_PARTS_XML,
  );

  let titles = parsed
    .titles_of_parts
    .as_ref()
    .expect("expected ap:TitlesOfParts");

  assert_eq!(titles.vt_vector.size, 1);
  assert!(matches!(
    titles.vt_vector.base_type,
    VectorBaseValues::Lpstr
  ));
  assert_eq!(titles.vt_vector.xml_children.len(), 1);
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<ap:Properties"));
  assert!(serialized.contains("<ap:TitlesOfParts"));
  assert!(serialized.contains("<vt:vector"));
  assert!(serialized.contains("baseType=\"lpstr\""));
  assert!(serialized.contains("size=\"1\""));
  assert!(serialized.contains("<vt:lpstr"));
  assert!(reparsed.titles_of_parts.is_some());
}

#[test]
fn custom_properties_bool_round_trip_from_bug225919_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CustomProperties>(fixtures::CUSTOM_PROPERTIES_BOOL_XML);

  assert_eq!(parsed.op_property.len(), 1);
  let property = parsed.op_property.first().expect("expected op:property");
  assert_eq!(property.name.as_deref(), Some("crap"));
  assert_eq!(property.property_id, 2);
  assert_eq!(
    property.format_id.as_str(),
    "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}"
  );
  let Some(CustomDocumentPropertyChoice::VtBool(value)) = property.xml_children.as_ref() else {
    panic!("expected vt:bool");
  };
  assert!(value);
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<op:Properties"));
  assert!(serialized.contains("fmtid=\"{D5CDD505-2E9C-101B-9397-08002B2CF9AE}\""));
  assert!(serialized.contains("pid=\"2\""));
  assert!(serialized.contains("name=\"crap\""));
  assert!(serialized.contains("<vt:bool"));
  assert!(serialized.contains(">1</vt:bool>"));
  assert_eq!(reparsed.op_property.len(), 1);
}

#[test]
fn variant_double_text_child_uses_xml_schema_float_lexical_form() {
  let value = Variant {
    xml_children: Some(VariantChoice::VtR8(f64::INFINITY)),
  };

  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<vt:r8>INF</vt:r8>"));

  let reparsed = serialized.parse::<Variant>().unwrap();
  let Some(VariantChoice::VtR8(parsed)) = reparsed.xml_children else {
    panic!("expected vt:r8");
  };
  assert!(parsed.is_infinite());
  assert!(parsed.is_sign_positive());
}
