use ooxmlsdk::schemas::schemas_microsoft_com_office_office::ShapeDefaults;
use ooxmlsdk::schemas::schemas_microsoft_com_office_word_2012_wordml::ChartTrackingRefBased;
use ooxmlsdk::schemas::schemas_microsoft_com_vml::Shape;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::{
  Variant, VariantChoice,
};
use ooxmlsdk::sdk::SdkType;
use ooxmlsdk::simple_type::{
  Base64BinaryValue, BooleanValue, ByteValue, DateTimeValue, DecimalValue, DoubleValue,
  HexBinaryValue, HexBinaryValueExt, Int16Value, Int32Value, Int64Value, IntegerValue, OnOffValue,
  SByteValue, SingleValue, StringValue, TrueFalseBlankValue, TrueFalseValue, UInt16Value,
  UInt32Value, UInt64Value, hex_binary_from_bytes, hex_binary_to_bytes, is_valid_hex_binary,
};
use ooxmlsdk_test::trim_xml_declaration;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_value<T: Hash>(value: &T) -> u64 {
  let mut hasher = DefaultHasher::new();
  value.hash(&mut hasher);
  hasher.finish()
}

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
fn comparable_simple_reference_equivalents_use_rust_string_semantics() {
  // Source: test/DocumentFormat.OpenXml.Tests/SimpleTypes/OpenXmlComparableSimpleReferenceTests.cs
  let small1: Option<StringValue> = Some("alpha".to_string());
  let small2: Option<StringValue> = Some("alpha".to_string());
  let large: Option<StringValue> = Some("omega".to_string());
  let null1: Option<StringValue> = None;
  let null2: Option<StringValue> = None;

  assert_eq!(small1, small2);
  assert_eq!(null1, null2);
  assert_ne!(small1, large);
  assert_ne!(small1, null1);
  assert!(small1 < large);
  assert!(large > small2);
  assert!(null1 < small1);
  assert!(large >= small1);
  assert_eq!(hash_value(&small1), hash_value(&small2));
  assert_eq!(hash_value(&null1), hash_value(&null2));
  assert_ne!(hash_value(&small1), hash_value(&large));
}

#[test]
fn comparable_simple_value_equivalents_use_rust_numeric_semantics() {
  // Source: test/DocumentFormat.OpenXml.Tests/SimpleTypes/OpenXmlComparableSimpleValueTests.cs
  let small1: Int32Value = 12;
  let small2: Int32Value = 12;
  let large: Int32Value = 42;

  assert_eq!(small1, small2);
  assert_ne!(small1, large);
  assert!(small1 < large);
  assert!(small1 <= small2);
  assert!(large > small1);
  assert!(large >= small2);
  assert_eq!(small1.cmp(&small2), std::cmp::Ordering::Equal);
  assert_eq!(large.cmp(&small1), std::cmp::Ordering::Greater);
  assert_eq!(small1.cmp(&large), std::cmp::Ordering::Less);
  assert_eq!(hash_value(&small1), hash_value(&small2));
  assert_ne!(hash_value(&small1), hash_value(&large));
}

#[test]
fn base64_binary_value_round_trip_test() {
  assert_round_trip::<Base64BinaryValue>("YWJj");
}

#[test]
fn boolean_value_round_trip_test() {
  assert_round_trip::<BooleanValue>("true");
  assert_round_trip::<BooleanValue>("false");
  assert_round_trip::<BooleanValue>("1");
  assert_round_trip::<BooleanValue>("0");
}

#[test]
fn boolean_simple_type_helpers_use_upstream_bool_defaults() {
  assert_eq!(BooleanValue::from_bool(true).to_string(), "1");
  assert_eq!(BooleanValue::from_bool(false).to_string(), "0");
  assert_eq!(OnOffValue::from_bool(true).to_string(), "true");
  assert_eq!(OnOffValue::from_bool(false).to_string(), "false");
  assert_eq!(TrueFalseValue::from_bool(true).to_string(), "true");
  assert_eq!(TrueFalseValue::from_bool(false).to_string(), "false");
  assert_eq!(TrueFalseBlankValue::from_bool(true).to_string(), "true");
  assert_eq!(TrueFalseBlankValue::from_bool(false).to_string(), "false");

  assert!(BooleanValue::One.as_bool());
  assert!(!OnOffValue::Off.as_bool());
  assert!(TrueFalseValue::T.as_bool());
  assert!(!TrueFalseBlankValue::Blank.as_bool());
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
fn on_off_value_round_trip_test() {
  assert_round_trip::<OnOffValue>("true");
  assert_round_trip::<OnOffValue>("false");
  assert_round_trip::<OnOffValue>("on");
  assert_round_trip::<OnOffValue>("off");
  assert_round_trip::<OnOffValue>("1");
  assert_round_trip::<OnOffValue>("0");
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
  assert_round_trip::<TrueFalseValue>("t");
  assert_round_trip::<TrueFalseValue>("f");
}

#[test]
fn true_false_blank_value_round_trip_test() {
  assert_round_trip::<TrueFalseBlankValue>("true");
  assert_round_trip::<TrueFalseBlankValue>("false");
  assert_round_trip::<TrueFalseBlankValue>("t");
  assert_round_trip::<TrueFalseBlankValue>("f");
  assert_round_trip::<TrueFalseBlankValue>("");
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
fn true_false_attribute_accepts_upstream_lexical_forms_and_preserves_lexical_form() {
  for (raw, expected) in [("true", true), ("t", true), ("false", false), ("f", false)] {
    let xml = format!(
      r#"<o:shapedefaults xmlns:o="urn:schemas-microsoft-com:office:office" fill="{raw}"/>"#
    );
    let parsed = xml.parse::<ShapeDefaults>().unwrap();
    assert_eq!(
      parsed.be_filled.map(|value| value.as_bool()),
      Some(expected)
    );

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"fill="{raw}""#)));
  }
}

#[test]
fn true_false_blank_attribute_accepts_empty_string_and_preserves_lexical_form() {
  for (raw, expected) in [
    ("true", true),
    ("t", true),
    ("false", false),
    ("f", false),
    ("", false),
  ] {
    let xml = format!(
      r#"<v:shape xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office" o:ole="{raw}"/>"#
    );
    let parsed = xml.parse::<Shape>().unwrap();
    assert_eq!(parsed.ole.map(|value| value.as_bool()), Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"o:ole="{raw}""#)));
  }
}

#[test]
fn on_off_attribute_accepts_upstream_lexical_forms_and_preserves_lexical_form() {
  for (raw, expected) in [
    ("true", true),
    ("1", true),
    ("on", true),
    ("false", false),
    ("0", false),
    ("off", false),
  ] {
    let xml = format!(
      r#"<w15:chartTrackingRefBased xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:val="{raw}"/>"#
    );
    let parsed = xml.parse::<ChartTrackingRefBased>().unwrap();
    assert_eq!(parsed.val.map(|value| value.as_bool()), Some(expected));

    let serialized = parsed.to_xml().unwrap();
    assert!(serialized.contains(&format!(r#"w:val="{raw}""#)));
  }
}

#[test]
fn xml_schema_float_text_children_use_openxml_special_values() {
  let value = Variant {
    variant_choice: Some(VariantChoice::VtFloat(f32::NEG_INFINITY)),
  };
  let xml = value.to_xml().unwrap();
  let serialized = trim_xml_declaration(&xml);
  assert!(serialized.contains("<vt:r4>-INF</vt:r4>"));

  let reparsed = serialized.parse::<Variant>().unwrap();
  let Some(VariantChoice::VtFloat(parsed)) = reparsed.variant_choice else {
    panic!("expected vt:r4");
  };
  assert!(parsed.is_infinite());
  assert!(parsed.is_sign_negative());
}
