use ooxmlsdk::schemas::schemas_microsoft_com_office_office::ShapeDefaults;
use ooxmlsdk::schemas::schemas_microsoft_com_office_word_2012_wordml::ChartTrackingRefBased;
use ooxmlsdk::schemas::schemas_microsoft_com_vml::Shape;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::{
  Variant, VariantChoice,
};
use ooxmlsdk::simple_type::{
  Base64BinaryValue, BooleanValue, ByteValue, DateTimeValue, DecimalValue, DoubleValue,
  HexBinaryValue, HexBinaryValueExt, Int16Value, Int32Value, Int64Value, IntegerValue, ListValue,
  OnOffValue, SByteValue, SingleValue, StringValue, TrueFalseBlankValue, TrueFalseValue,
  UInt16Value, UInt32Value, UInt64Value, hex_binary_from_bytes, hex_binary_to_bytes,
  is_valid_hex_binary,
};
use ooxmlsdk_test::trim_xml_declaration;

fn assert_canonical<T>(input: &str, expected: &str)
where
  T: std::str::FromStr + std::fmt::Display + PartialEq + std::fmt::Debug,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  let parsed = input.parse::<T>().unwrap();
  assert_eq!(parsed.to_string(), expected);
}

fn assert_round_trip<T>(input: &str)
where
  T: std::str::FromStr + std::fmt::Display + PartialEq + std::fmt::Debug,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  let parsed = input.parse::<T>().unwrap();
  assert_eq!(parsed.to_string(), input);
  assert_eq!(parsed, input.parse::<T>().unwrap());
}

#[test]
fn string_value_round_trip_test() {
  assert_round_trip::<StringValue>("abcdef");
  assert_round_trip::<StringValue>("hello world");
}

#[test]
fn base64_binary_value_round_trip_test() {
  assert_round_trip::<Base64BinaryValue>("YWJj");
}

#[test]
fn boolean_value_round_trip_test() {
  assert_round_trip::<BooleanValue>("true");
  assert_round_trip::<BooleanValue>("false");
}

#[test]
fn boolean_simple_type_aliases_keep_rust_bool_lexical_form() {
  assert!("1".parse::<BooleanValue>().is_err());
  assert!("0".parse::<OnOffValue>().is_err());
  assert!("on".parse::<OnOffValue>().is_err());
  assert!("t".parse::<TrueFalseValue>().is_err());
  assert!("".parse::<TrueFalseBlankValue>().is_err());
}

#[test]
fn byte_value_round_trip_test() {
  assert_round_trip::<ByteValue>("0");
  assert_round_trip::<ByteValue>("1");
  assert_round_trip::<ByteValue>("255");
}

#[test]
fn date_time_value_round_trip_test() {
  assert_round_trip::<DateTimeValue>("2024-10-26T22:14:00Z");
}

#[test]
fn decimal_value_round_trip_test() {
  assert_round_trip::<DecimalValue>("123.450");
}

#[test]
fn double_value_round_trip_test() {
  assert_round_trip::<DoubleValue>("1.5");
  assert_round_trip::<DoubleValue>("-2.25");
}

#[test]
fn hex_binary_value_round_trip_test() {
  assert_round_trip::<HexBinaryValue>("00ffaa");
}

#[test]
fn hex_binary_value_helpers_match_upstream_byte_semantics() {
  for (value, expected) in [
    ("", true),
    ("a", false),
    ("A", false),
    ("zz", false),
    ("gg", false),
    ("bb", true),
    ("0A", true),
    ("5A", true),
    ("5dA", false),
    ("5AbC5AbC5AbC5AbC5AbC5AbC5AbC5AbC", true),
    ("5AbC5AbC5AbC5AbC5AbC5AbC5AbC5Ab", false),
  ] {
    assert_eq!(is_valid_hex_binary(value), expected, "{value}");
    assert_eq!(value.is_valid_hex_binary(), expected, "{value}");
  }

  assert_eq!(hex_binary_to_bytes("00"), Some(vec![0]));
  assert_eq!("01".try_get_bytes(), Some(vec![1]));
  assert_eq!("FF".try_get_bytes(), Some(vec![0xFF]));
  assert_eq!("FFF".try_get_bytes(), None);
  assert_eq!("FF01".try_get_bytes(), Some(vec![0xFF, 0x01]));

  assert_eq!(hex_binary_from_bytes([]), "");
  assert_eq!(hex_binary_from_bytes([0]), "00");
  assert_eq!(hex_binary_from_bytes([1]), "01");
  assert_eq!(hex_binary_from_bytes([0xFF]), "FF");
  assert_eq!(hex_binary_from_bytes([0xFF, 0x01]), "FF01");
}

#[test]
fn int16_value_round_trip_test() {
  assert_round_trip::<Int16Value>("0");
  assert_round_trip::<Int16Value>("-1");
  assert_round_trip::<Int16Value>("32767");
  assert_canonical::<Int16Value>("+10", "10");
}

#[test]
fn int32_value_round_trip_test() {
  assert_round_trip::<Int32Value>("0");
  assert_round_trip::<Int32Value>("-1");
  assert_round_trip::<Int32Value>("2147483647");
  assert_canonical::<Int32Value>("+10", "10");
}

#[test]
fn int64_value_round_trip_test() {
  assert_round_trip::<Int64Value>("0");
  assert_round_trip::<Int64Value>("-1");
  assert_round_trip::<Int64Value>("9223372036854775807");
  assert_canonical::<Int64Value>("+10", "10");
}

#[test]
fn integer_value_round_trip_test() {
  assert_round_trip::<IntegerValue>("-1");
  assert_round_trip::<IntegerValue>("0");
  assert_round_trip::<IntegerValue>("123456789");
  assert_canonical::<IntegerValue>("+10", "10");
}

#[test]
fn list_value_string_round_trip_test() {
  assert_round_trip::<ListValue<StringValue>>("1:22");
  assert_round_trip::<ListValue<StringValue>>("alpha beta gamma");
}

#[test]
fn list_value_normalizes_xml_whitespace() {
  assert_canonical::<ListValue<StringValue>>("  alpha\tbeta\r\n gamma  ", "alpha beta gamma");
  assert_canonical::<ListValue<Int32Value>>(" +1\t-2\r\n3 ", "1 -2 3");
}

#[test]
fn list_value_int32_round_trip_test() {
  assert_round_trip::<ListValue<Int32Value>>("-1 0 42");
}

#[test]
fn on_off_value_round_trip_test() {
  assert_round_trip::<OnOffValue>("true");
  assert_round_trip::<OnOffValue>("false");
}

#[test]
fn sbyte_value_round_trip_test() {
  assert_round_trip::<SByteValue>("-128");
  assert_round_trip::<SByteValue>("0");
  assert_round_trip::<SByteValue>("127");
  assert_canonical::<SByteValue>("+10", "10");
}

#[test]
fn single_value_round_trip_test() {
  assert_round_trip::<SingleValue>("1.5");
  assert_round_trip::<SingleValue>("-2.25");
}

#[test]
fn true_false_value_round_trip_test() {
  assert_round_trip::<TrueFalseValue>("true");
  assert_round_trip::<TrueFalseValue>("false");
}

#[test]
fn true_false_blank_value_round_trip_test() {
  assert_round_trip::<TrueFalseBlankValue>("true");
  assert_round_trip::<TrueFalseBlankValue>("false");
}

#[test]
fn uint16_value_round_trip_test() {
  assert_round_trip::<UInt16Value>("0");
  assert_round_trip::<UInt16Value>("1");
  assert_round_trip::<UInt16Value>("65535");
  assert_canonical::<UInt16Value>("+10", "10");
}

#[test]
fn uint32_value_round_trip_test() {
  assert_round_trip::<UInt32Value>("0");
  assert_round_trip::<UInt32Value>("1");
  assert_round_trip::<UInt32Value>("4294967295");
  assert_canonical::<UInt32Value>("+10", "10");
}

#[test]
fn uint64_value_round_trip_test() {
  assert_round_trip::<UInt64Value>("0");
  assert_round_trip::<UInt64Value>("1");
  assert_round_trip::<UInt64Value>("18446744073709551615");
  assert_canonical::<UInt64Value>("+10", "10");
}

#[test]
fn true_false_attribute_accepts_upstream_lexical_forms_and_writes_canonical_form() {
  for (raw, expected, canonical) in [
    ("true", true, "true"),
    ("t", true, "true"),
    ("false", false, "false"),
    ("f", false, "false"),
  ] {
    let xml = format!(
      r#"<o:shapedefaults xmlns:o="urn:schemas-microsoft-com:office:office" fill="{raw}"/>"#
    );
    let parsed = xml.parse::<ShapeDefaults>().unwrap();
    assert_eq!(parsed.be_filled, Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"fill="{canonical}""#)));
  }
}

#[test]
fn true_false_blank_attribute_accepts_empty_string_and_writes_canonical_form() {
  for (raw, expected, canonical) in [
    ("true", true, "true"),
    ("t", true, "true"),
    ("false", false, "false"),
    ("f", false, "false"),
    ("", false, "false"),
  ] {
    let xml = format!(
      r#"<v:shape xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office" o:ole="{raw}"/>"#
    );
    let parsed = xml.parse::<Shape>().unwrap();
    assert_eq!(parsed.ole, Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"o:ole="{canonical}""#)));
  }
}

#[test]
fn on_off_attribute_accepts_upstream_lexical_forms_and_writes_canonical_form() {
  for (raw, expected, canonical) in [
    ("true", true, "true"),
    ("1", true, "true"),
    ("on", true, "true"),
    ("false", false, "false"),
    ("0", false, "false"),
    ("off", false, "false"),
  ] {
    let xml = format!(
      r#"<w15:chartTrackingRefBased xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:val="{raw}"/>"#
    );
    let parsed = xml.parse::<ChartTrackingRefBased>().unwrap();
    assert_eq!(parsed.val, Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"w:val="{canonical}""#)));
  }
}

#[test]
fn xml_schema_float_text_children_use_openxml_special_values() {
  let value = Variant {
    variant_choice: Some(VariantChoice::VtR4(f32::NEG_INFINITY)),
  };
  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<vt:r4>-INF</vt:r4>"));

  let reparsed = serialized.parse::<Variant>().unwrap();
  let Some(VariantChoice::VtR4(parsed)) = reparsed.variant_choice else {
    panic!("expected vt:r4");
  };
  assert!(parsed.is_infinite());
  assert!(parsed.is_sign_negative());
}
