use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::{
  CellValue, ColorScale, ConditionalFormatValueObjectValues, SharedStringTable,
};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

fn assert_cell_value_xml(serialized: &str, expected_value: &str) {
  let serialized = trim_xml_declaration(serialized);

  assert!(serialized.starts_with("<x:v"));
  assert!(serialized.ends_with("</x:v>"));
  assert!(serialized.contains(expected_value));
}

#[test]
fn shared_string_table_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<SharedStringTable>(fixtures::SPREADSHEET_SHARED_STRING_TABLE_XML);

  assert_eq!(
    parsed.xmlns_map.get("x").map(String::as_str),
    Some("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
  );
  assert_eq!(parsed.x_si.len(), 1);
  let item = &parsed.x_si[0];
  assert_eq!(
    item.text.as_ref().and_then(|text| text.xml_content.as_deref()),
    Some("Test")
  );
  assert_eq!(
    trim_xml_declaration(&serialized),
    trim_xml_declaration(fixtures::SPREADSHEET_SHARED_STRING_TABLE_XML)
  );
  assert_eq!(reparsed.x_si.len(), 1);
}

#[test]
fn empty_shared_string_table_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<SharedStringTable>(
    fixtures::SPREADSHEET_SHARED_STRING_TABLE_EMPTY_XML,
  );

  assert_eq!(parsed.x_si.len(), 0);
  let serialized = trim_xml_declaration(&serialized);
  assert!(
    serialized.starts_with("<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"")
  );
  assert!(serialized == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"></x:sst>" || serialized == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"/>");
  assert_eq!(reparsed.x_si.len(), 0);
}

#[test]
fn color_scale_round_trip_from_bug_regression_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<ColorScale>(fixtures::SPREADSHEET_COLOR_SCALE_XML);

  assert_eq!(parsed.x_cfvo.len(), 1);
  assert_eq!(parsed.x_color.len(), 2);
  assert!(matches!(
    parsed.x_cfvo[0].r#type,
    ConditionalFormatValueObjectValues::Min
  ));
  assert_eq!(parsed.x_cfvo[0].val.as_deref(), Some("0"));
  assert_eq!(parsed.x_color[0].theme, Some(4));
  assert_eq!(parsed.x_color[1].theme, Some(6));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<x:colorScale"));
  assert!(serialized.contains("<x:cfvo type=\"min\" val=\"0\""));
  assert!(serialized.contains("<x:color theme=\"4\""));
  assert!(serialized.contains("<x:color theme=\"6\""));
  assert_eq!(reparsed.x_color.len(), 2);
}

#[test]
fn cell_value_double_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DOUBLE_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("-1.5"));
  assert_cell_value_xml(&serialized, "-1.5");
  assert_eq!(reparsed.xml_content.as_deref(), Some("-1.5"));
}

#[test]
fn cell_value_double_exponential_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<CellValue>(
    fixtures::SPREADSHEET_CELL_VALUE_DOUBLE_EXPONENTIAL_XML,
  );

  assert_eq!(parsed.xml_content.as_deref(), Some("987.6E+30"));
  assert_cell_value_xml(&serialized, "987.6E+30");
  assert_eq!(reparsed.xml_content.as_deref(), Some("987.6E+30"));
}

#[test]
fn cell_value_int_exponential_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<CellValue>(
    fixtures::SPREADSHEET_CELL_VALUE_INT_EXPONENTIAL_XML,
  );

  assert_eq!(parsed.xml_content.as_deref(), Some("987E+5"));
  assert_cell_value_xml(&serialized, "987E+5");
  assert_eq!(reparsed.xml_content.as_deref(), Some("987E+5"));
}

#[test]
fn cell_value_boolean_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_BOOLEAN_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("true"));
  assert_cell_value_xml(&serialized, "true");
  assert_eq!(reparsed.xml_content.as_deref(), Some("true"));
}

#[test]
fn cell_value_int_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_INT_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("2147483647"));
  assert_cell_value_xml(&serialized, "2147483647");
  assert_eq!(reparsed.xml_content.as_deref(), Some("2147483647"));
}

#[test]
fn cell_value_decimal_exponential_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<CellValue>(
    fixtures::SPREADSHEET_CELL_VALUE_DECIMAL_EXPONENTIAL_XML,
  );

  assert_eq!(parsed.xml_content.as_deref(), Some("987.6E+8"));
  assert_cell_value_xml(&serialized, "987.6E+8");
  assert_eq!(reparsed.xml_content.as_deref(), Some("987.6E+8"));
}

#[test]
fn cell_value_datetime_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DATETIME_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("2017-11-28T12:25:02.123"));
  assert_cell_value_xml(&serialized, "2017-11-28T12:25:02.123");
  assert_eq!(reparsed.xml_content.as_deref(), Some("2017-11-28T12:25:02.123"));
}

#[test]
fn cell_value_datetime_offset_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DATETIME_OFFSET_XML);

  assert_eq!(
    parsed.xml_content.as_deref(),
    Some("2017-11-28T12:25:02.123+00:00")
  );
  assert_cell_value_xml(&serialized, "2017-11-28T12:25:02.123+00:00");
  assert_eq!(
    reparsed.xml_content.as_deref(),
    Some("2017-11-28T12:25:02.123+00:00")
  );
}
