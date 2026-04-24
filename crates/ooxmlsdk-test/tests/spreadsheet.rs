#[cfg(feature = "microsoft365")]
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::{
  ArrayFeatureProperty, ArrayFeaturePropertyChoice, BoolFeatureProperty, IntFeatureProperty,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::{
  CellValue, ColorScale, ConditionalFormatValueObjectValues, SharedStringTable, Workbook, Worksheet,
};
use ooxmlsdk::simple_type::{ListValue, StringValue};
use ooxmlsdk_test::{assert_stable_roundtrip, fixtures, trim_xml_declaration};

fn assert_cell_value_xml(serialized: &str, expected_value: &str) {
  assert_eq!(
    trim_xml_declaration(serialized),
    format!("<x:v>{expected_value}</x:v>")
  );
}

fn shared_string_items(
  table: &SharedStringTable,
) -> Vec<&ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringItem> {
  table.x_si.iter().collect()
}

fn color_scale_cfvo(
  scale: &ColorScale,
) -> Vec<&ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatValueObject>{
  scale.x_cfvo.iter().collect()
}

fn color_scale_colors(
  scale: &ColorScale,
) -> Vec<&ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Color> {
  scale.x_color.iter().collect()
}

#[test]
fn workbook_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Workbook>(fixtures::SPREADSHEET_WORKBOOK_XML);

  assert_eq!(parsed.sheets.x_sheet.len(), 2);
  assert_eq!(parsed.sheets.x_sheet[0].name.as_str(), "Sheet1");
  assert_eq!(parsed.sheets.x_sheet[1].name.as_str(), "Sheet2");
  assert!(serialized.contains("<x:sheet"));
  assert!(serialized.contains("name=\"Sheet1\""));
  assert!(serialized.contains("name=\"Sheet2\""));
  assert_eq!(reparsed.sheets.x_sheet.len(), 2);
}

#[test]
fn workbook_round_trip_from_complex01_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<Workbook>(fixtures::SPREADSHEET_WORKBOOK_COMPLEX01_XML);

  assert_eq!(parsed.mc_ignorable.as_deref(), Some("x15"));
  assert_eq!(
    parsed
      .file_version
      .as_ref()
      .and_then(|file_version| file_version.application_name.as_ref())
      .map(|value| value.as_str()),
    Some("xl")
  );
  assert_eq!(
    parsed
      .file_version
      .as_ref()
      .and_then(|file_version| file_version.last_edited.as_ref())
      .map(|value| value.as_str()),
    Some("6")
  );
  assert_eq!(parsed.sheets.x_sheet.len(), 2);
  assert_eq!(parsed.sheets.x_sheet[0].name.as_str(), "Sheet1");
  assert_eq!(parsed.sheets.x_sheet[1].name.as_str(), "Sheet2");
  assert!(trim_xml_declaration(&serialized).contains("mc:Ignorable=\"x15\""));
  assert!(trim_xml_declaration(&serialized).contains("<x:calcPr"));
  assert!(trim_xml_declaration(&serialized).contains("calcId=\"152511\""));
  assert_eq!(reparsed.mc_ignorable.as_deref(), Some("x15"));
  assert_eq!(reparsed.sheets.x_sheet.len(), 2);
}

#[test]
fn worksheet_round_trip_from_complex01_part_test() {
  let (parsed, _serialized, reparsed) =
    assert_stable_roundtrip::<Worksheet>(fixtures::SPREADSHEET_WORKSHEET_COMPLEX01_SHEET1_XML);

  assert_eq!(parsed.mc_ignorable.as_deref(), Some("x14ac"));
  assert_eq!(
    parsed
      .sheet_dimension
      .as_ref()
      .map(|dimension| dimension.reference.as_str()),
    Some("A1:V19")
  );
  assert_eq!(parsed.x_sheet_data.x_row.len(), 13);
  assert_eq!(
    parsed.x_sheet_data.x_row[0].spans,
    Some(ListValue::<StringValue>(vec!["1:22".to_string()]))
  );
  assert_eq!(
    parsed
      .x_hyperlinks
      .as_ref()
      .map(|links| links.x_hyperlink.len()),
    Some(1)
  );
  assert_eq!(
    parsed
      .x_table_parts
      .as_ref()
      .and_then(|parts| parts.count.as_ref())
      .copied(),
    Some(1)
  );
  assert!(parsed.x_drawing.is_some());
  assert!(parsed.x_legacy_drawing.is_some());
  assert_eq!(
    reparsed
      .x_hyperlinks
      .as_ref()
      .map(|links| links.x_hyperlink.len()),
    Some(1)
  );
  assert_eq!(reparsed.x_sheet_data.x_row.len(), 13);
  assert_eq!(
    reparsed.x_sheet_data.x_row[0].spans,
    Some(ListValue::<StringValue>(vec!["1:22".to_string()]))
  );
}

#[test]
fn shared_string_table_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<SharedStringTable>(fixtures::SPREADSHEET_SHARED_STRING_TABLE_XML);

  assert_eq!(
    ooxmlsdk::common::find_xmlns_uri(&parsed.xmlns, "x"),
    Some("http://schemas.openxmlformats.org/spreadsheetml/2006/main")
  );
  let items = shared_string_items(&parsed);
  assert_eq!(items.len(), 1);
  let item = items[0];
  assert_eq!(
    item
      .text
      .as_ref()
      .and_then(|text| text.xml_content.as_deref()),
    Some("Test")
  );
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<x:sst "));
  assert!(serialized.contains("<x:si"));
  assert!(serialized.contains("<x:t"));
  assert!(serialized.contains(">Test</x:t>"));
  assert_eq!(shared_string_items(&reparsed).len(), 1);
}

#[test]
fn shared_string_table_serialization_matches_get_stream_write_test() {
  let (_parsed, serialized, reparsed) =
    assert_stable_roundtrip::<SharedStringTable>(fixtures::SPREADSHEET_SHARED_STRING_TABLE_XML);

  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<x:sst "));
  assert!(serialized.contains("<x:si"));
  assert!(serialized.contains("<x:t"));
  assert!(serialized.contains(">Test</x:t>"));
  assert_eq!(shared_string_items(&reparsed).len(), 1);
}

#[test]
fn empty_shared_string_table_round_trip_from_openxml_part_test() {
  let (parsed, serialized, reparsed) = assert_stable_roundtrip::<SharedStringTable>(
    fixtures::SPREADSHEET_SHARED_STRING_TABLE_EMPTY_XML,
  );

  assert_eq!(shared_string_items(&parsed).len(), 0);
  let serialized = trim_xml_declaration(&serialized);
  assert!(
    serialized
      .starts_with("<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"")
  );
  assert!(
    serialized
      == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"></x:sst>"
      || serialized
        == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"/>"
  );
  assert_eq!(shared_string_items(&reparsed).len(), 0);
}

#[test]
fn empty_shared_string_table_serialization_matches_get_stream_write_no_updates_test() {
  let (_parsed, serialized, reparsed) = assert_stable_roundtrip::<SharedStringTable>(
    fixtures::SPREADSHEET_SHARED_STRING_TABLE_EMPTY_XML,
  );

  let serialized = trim_xml_declaration(&serialized);
  assert!(
    serialized == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"/>"
      || serialized
        == "<x:sst xmlns:x=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"></x:sst>"
  );
  assert_eq!(shared_string_items(&reparsed).len(), 0);
}

#[test]
fn color_scale_round_trip_from_bug_regression_test() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<ColorScale>(fixtures::SPREADSHEET_COLOR_SCALE_XML);

  let cfvo = color_scale_cfvo(&parsed);
  let colors = color_scale_colors(&parsed);
  assert_eq!(cfvo.len(), 1);
  assert_eq!(colors.len(), 2);
  assert!(matches!(
    cfvo[0].r#type,
    ConditionalFormatValueObjectValues::Min
  ));
  assert_eq!(cfvo[0].val.as_deref(), Some("0"));
  assert_eq!(colors[0].theme, Some(4));
  assert_eq!(colors[1].theme, Some(6));
  let serialized = trim_xml_declaration(&serialized);
  assert!(serialized.starts_with("<x:colorScale"));
  assert!(serialized.contains("<x:cfvo"));
  assert!(serialized.contains("type=\"min\""));
  assert!(serialized.contains("val=\"0\""));
  assert!(serialized.contains("<x:color"));
  assert!(serialized.contains("theme=\"4\""));
  assert!(serialized.contains("theme=\"6\""));
  assert_eq!(color_scale_colors(&reparsed).len(), 2);
}

#[cfg(feature = "microsoft365")]
#[test]
fn array_feature_property_double_text_child_uses_xml_schema_float_lexical_form() {
  let value = ArrayFeatureProperty {
    k: "sample".to_string(),
    xml_children: vec![ArrayFeaturePropertyChoice::XfpbD(f64::NAN)],
  };

  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<xfpb:d>NaN</xfpb:d>"));

  let reparsed = serialized.parse::<ArrayFeatureProperty>().unwrap();
  let Some(ArrayFeaturePropertyChoice::XfpbD(parsed)) = reparsed.xml_children.first() else {
    panic!("expected xfpb:d");
  };
  assert!(parsed.is_nan());
}

#[cfg(feature = "microsoft365")]
#[test]
fn bool_feature_property_uses_boolean_value_lexical_form() {
  let value = BoolFeatureProperty {
    k: "flag".into(),
    xml_content: Some(true),
  };

  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<xfpb:b"));
  assert!(serialized.contains("k=\"flag\""));
  assert!(serialized.contains(">1</xfpb:b>"));

  let reparsed = serialized.parse::<BoolFeatureProperty>().unwrap();
  assert_eq!(reparsed.k.as_str(), "flag");
  assert_eq!(reparsed.xml_content, Some(true));
}

#[cfg(feature = "microsoft365")]
#[test]
fn int_feature_property_uses_integer_value_numeric_form() {
  let value = IntFeatureProperty {
    k: "count".into(),
    xml_content: Some(-42),
  };

  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<xfpb:i"));
  assert!(serialized.contains("k=\"count\""));
  assert!(serialized.contains(">-42</xfpb:i>"));

  let reparsed = serialized.parse::<IntFeatureProperty>().unwrap();
  assert_eq!(reparsed.k.as_str(), "count");
  assert_eq!(reparsed.xml_content, Some(-42));
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
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DOUBLE_EXPONENTIAL_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("987.6E+30"));
  assert_cell_value_xml(&serialized, "987.6E+30");
  assert_eq!(reparsed.xml_content.as_deref(), Some("987.6E+30"));
}

#[test]
fn cell_value_int_exponential_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_INT_EXPONENTIAL_XML);

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
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DECIMAL_EXPONENTIAL_XML);

  assert_eq!(parsed.xml_content.as_deref(), Some("987.6E+8"));
  assert_cell_value_xml(&serialized, "987.6E+8");
  assert_eq!(reparsed.xml_content.as_deref(), Some("987.6E+8"));
}

#[test]
fn cell_value_datetime_round_trip_from_cell_value_tests() {
  let (parsed, serialized, reparsed) =
    assert_stable_roundtrip::<CellValue>(fixtures::SPREADSHEET_CELL_VALUE_DATETIME_XML);

  assert_eq!(
    parsed.xml_content.as_deref(),
    Some("2017-11-28T12:25:02.123")
  );
  assert_cell_value_xml(&serialized, "2017-11-28T12:25:02.123");
  assert_eq!(
    reparsed.xml_content.as_deref(),
    Some("2017-11-28T12:25:02.123")
  );
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
