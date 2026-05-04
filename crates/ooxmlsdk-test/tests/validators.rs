#![cfg(feature = "validators")]

use ooxmlsdk::schemas::schemas_microsoft_com_office_2006_01_customui::{
  Item, QuickAccessToolbarControlClone,
};
use ooxmlsdk::schemas::schemas_microsoft_com_office_word::TopBorder;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart::{
  BubbleScale, LogBase, MajorUnit,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorTransformCategory;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::ChildExtents;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_bibliography::Sources;
use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_math::ArgumentSize;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSize;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::{
  ConditionalFormatStyle, DocPartId, DocumentProtection, Panose1Number, UniqueTag,
};
use ooxmlsdk::validator::ValidationErrorType;
use ooxmlsdk::validator::validate_number_type;
use ooxmlsdk::validator::{SdkValidator, StringFormatKind, validate_string_format};

fn repeated_a(count: usize) -> String {
  "A".repeat(count)
}

#[test]
fn verify_token_from_token_restriction_tests() {
  let cases = [
    ("", true),
    (" ", false),
    ("a ", false),
    ("a\nd", false),
    ("a\td", false),
    ("a\rd", false),
    ("abc def", true),
    ("abc  def", false),
  ];

  for (text, expected) in cases {
    let actual = validate_string_format(
      "TokenRestrictionTests",
      "text",
      &text,
      StringFormatKind::Token,
    )
    .is_ok();
    assert_eq!(actual, expected, "input: {text:?}");
  }
}

#[test]
fn positive_integer_attribute_validation_test() {
  let mut element = TopBorder {
    width: Some(1),
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.width = Some(2);
  assert!(element.validate().is_empty());

  element.width = Some(1000);
  assert!(element.validate().is_empty());

  element.width = Some(0);
  assert!(!element.validate().is_empty());

  element.width = Some(-3);
  assert!(!element.validate().is_empty());
}

#[test]
fn integer_attribute_validation_test() {
  let mut element = ArgumentSize { val: -2 };
  assert!(element.validate().is_empty());

  element.val = 2;
  assert!(element.validate().is_empty());

  element.val = 0;
  assert!(element.validate().is_empty());

  element.val = -3;
  assert!(!element.validate().is_empty());

  element.val = 3;
  assert!(!element.validate().is_empty());
}

#[test]
fn uint32_attribute_validation_test() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs UInt32AttributeValidationTest
  let mut element = BubbleScale { val: Some(0) };
  assert!(element.validate().is_empty());

  element.val = Some(100);
  assert!(element.validate().is_empty());

  element.val = Some(300);
  assert!(element.validate().is_empty());

  element.val = Some(301);
  assert!(!element.validate().is_empty());

  element.val = Some(600);
  assert!(!element.validate().is_empty());
}

#[test]
fn int32_attribute_validation_test2() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs Int32AttributeValidationTest2
  let mut element = SlideSize {
    cx: 914400,
    cy: 914400,
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.cx = 51206400;
  assert!(element.validate().is_empty());

  element.cx = -1;
  assert!(!element.validate().is_empty());

  element.cx = 913999;
  assert!(!element.validate().is_empty());

  element.cx = 51206401;
  assert!(!element.validate().is_empty());
}

#[test]
fn int64_attribute_validation_test() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs Int64AttributeValidationTest
  let mut element = ChildExtents {
    cx: 914400,
    cy: 914400,
  };
  assert!(element.validate().is_empty());

  element.cx = 2_147_483_647;
  assert!(element.validate().is_empty());

  element.cx = -1;
  assert!(!element.validate().is_empty());

  element.cx = 2_147_483_648;
  assert!(!element.validate().is_empty());
}

#[test]
fn double_attribute_validation_test() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs DoubleAttributeValidationTest
  let mut element = MajorUnit { val: 10000.001 };
  assert!(element.validate().is_empty());

  element.val = 10.23e4;
  assert!(element.validate().is_empty());

  element.val = f64::MAX;
  assert!(element.validate().is_empty());

  element.val = f64::EPSILON;
  assert!(element.validate().is_empty());

  element.val = f64::INFINITY;
  assert!(element.validate().is_empty());

  element.val = 0.0;
  assert!(!element.validate().is_empty());

  element.val = 0.0 - f64::EPSILON;
  assert!(!element.validate().is_empty());

  element.val = f64::NEG_INFINITY;
  assert!(!element.validate().is_empty());

  element.val = f64::NAN;
  assert!(!element.validate().is_empty());

  let mut log_base = LogBase { val: 2.0 };
  assert!(log_base.validate().is_empty());

  log_base.val = 200.0;
  assert!(log_base.validate().is_empty());

  log_base.val = 1000.0;
  assert!(log_base.validate().is_empty());

  log_base.val = -1.0;
  assert!(!log_base.validate().is_empty());

  log_base.val = 1.9;
  assert!(!log_base.validate().is_empty());

  log_base.val = 1000.1;
  assert!(!log_base.validate().is_empty());
}

#[test]
fn string_attribute_validation_test() {
  let mut element = ConditionalFormatStyle {
    val: "010101010101".to_string(),
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.val = String::new();
  assert!(!element.validate().is_empty());

  element.val = "0101".to_string();
  assert!(!element.validate().is_empty());

  element.val = "0101010101010".to_string();
  assert!(!element.validate().is_empty());

  element.val = "010101010102".to_string();
  assert!(!element.validate().is_empty());

  element.val = "invalid".to_string();
  assert!(!element.validate().is_empty());

  let mut sources = Sources {
    style_name: Some(String::new()),
    ..Default::default()
  };
  assert!(sources.validate().is_empty());

  sources.style_name = Some("Style1".to_string());
  assert!(sources.validate().is_empty());

  sources.style_name = Some(repeated_a(255));
  assert!(sources.validate().is_empty());

  sources.style_name = Some(repeated_a(256));
  assert!(!sources.validate().is_empty());
}

#[test]
fn hex_binary_attribute_validation_test() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs HexBinaryAttributeValidationTest
  let mut element = Panose1Number {
    val: "1234567890ABCDEFabcd".to_string(),
  };
  assert!(element.validate().is_empty());

  element.val = "ABCDEFabcdef12345678".to_string();
  assert!(element.validate().is_empty());

  element.val = String::new();
  assert!(!element.validate().is_empty());

  element.val = "1234".to_string();
  assert!(!element.validate().is_empty());

  element.val = "ABCDEFabcdef123456789".to_string();
  assert!(!element.validate().is_empty());

  element.val = "ABCDEFabcdef1234567890".to_string();
  assert!(!element.validate().is_empty());

  element.val = "ABCDEFabcdef1234567X".to_string();
  assert!(!element.validate().is_empty());

  element.val = "ABCDEFabcdefX1234567".to_string();
  assert!(!element.validate().is_empty());
}

#[test]
fn base64_binary_attribute_validation_test() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs Base64BinaryAttributeValidationTest
  let mut element = DocumentProtection {
    hash: Some(String::new()),
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.hash = Some("fUmpYmCMpTxTA4pfvlhKSAgB848=".to_string());
  assert!(element.validate().is_empty());

  element.hash = Some("R3k/CLjN768ujxMXkKZOuw==".to_string());
  assert!(element.validate().is_empty());

  element.hash = Some("0".to_string());
  assert!(!element.validate().is_empty());

  element.hash = Some("R3k/CLjN768ujxMXkKZOuw==$".to_string());
  assert!(!element.validate().is_empty());

  element.hash = Some("*R3k/fUmpYmCMpTxTA4pfvlhKSAgB848=".to_string());
  assert!(!element.validate().is_empty());
}

#[test]
fn base64_binary_attribute_validation_test2() {
  // Source: DocumentFormat.OpenXml.Tests/ofapiTest/OpenXmlValidatorTest.cs Base64BinaryAttributeValidationTest2
  let mut element = UniqueTag { val: String::new() };
  assert!(element.validate().is_empty());

  element.val = "fUmpYmCMpTxTA4pfvlhKSAgB848=".to_string();
  assert!(element.validate().is_empty());

  element.val = "R3k/CLjN768ujxMXkKZOuw==".to_string();
  assert!(element.validate().is_empty());

  element.val = "0".to_string();
  assert!(!element.validate().is_empty());

  element.val = "R3k/CLjN768ujxMXkKZOuw==$".to_string();
  assert!(!element.validate().is_empty());

  element.val = "*R3k/fUmpYmCMpTxTA4pfvlhKSAgB848=".to_string();
  assert!(!element.validate().is_empty());
}

#[test]
fn token_attribute_validation_test() {
  let mut element = DocPartId {
    val: Some("{6A9B8B6F-5BD2-4BC8-9F70-7020E1357FB2}".to_string()),
  };
  assert!(element.validate().is_empty());

  element.val = Some(String::new());
  assert!(!element.validate().is_empty());

  element.val = Some("{6A9B8B6F-    -4BC8-9F70-7020E1357FB2}".to_string());
  assert!(!element.validate().is_empty());

  element.val = Some(" 6A9B8B6F-5BD2-4BC8-9F70-7020E1357FB2}".to_string());
  assert!(!element.validate().is_empty());

  element.val = Some("{6A9B8B6F-5BD\t-4BC8-9F70-7020E1357FB2}".to_string());
  assert!(!element.validate().is_empty());

  element.val = Some("1234".to_string());
  assert!(!element.validate().is_empty());

  element.val = Some("{*A9B8B6F-5BD2-4BC8-9F70-7020E1357FB2}".to_string());
  assert!(!element.validate().is_empty());
}

#[test]
fn any_uri_attribute_validation_test() {
  let mut element = ColorTransformCategory {
    r#type: String::new(),
    priority: 1,
  };
  assert!(element.validate().is_empty());

  element.r#type = "http://temp".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "http://microsoft.com".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "http://a/b/c/d;p?q".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "http://a/b/c/g;x?y#s".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "<>".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "http://a/../../g".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "urn:schemas-microsoft-com:office:office".to_string();
  assert!(element.validate().is_empty());

  element.r#type = " http://temp ".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "\thttp://microsoft.com\r\n".to_string();
  assert!(element.validate().is_empty());

  element.r#type = "http://temp##s".to_string();
  assert!(!element.validate().is_empty());

  element.r#type = "http:///temp".to_string();
  assert!(!element.validate().is_empty());

  element.r#type = "http://temp /a".to_string();
  assert!(!element.validate().is_empty());

  element.r#type = "   ".to_string();
  assert!(!element.validate().is_empty());
}

#[test]
fn any_uri_string_format_trims_edge_whitespace_like_upstream() {
  assert!(
    validate_string_format("AnyUriRestrictionTests", "text", &"", StringFormatKind::Uri).is_ok()
  );
  assert!(
    validate_string_format(
      "AnyUriRestrictionTests",
      "text",
      &" http://temp ",
      StringFormatKind::Uri,
    )
    .is_ok()
  );
  assert!(
    validate_string_format(
      "AnyUriRestrictionTests",
      "text",
      &"   ",
      StringFormatKind::Uri
    )
    .is_err()
  );
}

#[test]
fn non_negative_integer_number_type_rejects_negative_values() {
  assert!(validate_number_type("NumberTypeTests", "value", &"0", "xsd:nonNegativeInteger").is_ok());
  assert!(
    validate_number_type("NumberTypeTests", "value", &"42", "xsd:nonNegativeInteger").is_ok()
  );
  assert!(
    validate_number_type("NumberTypeTests", "value", &"-1", "xsd:nonNegativeInteger").is_err()
  );
}

#[test]
fn id_string_attribute_validation_test() {
  let mut element = Item {
    id: Some("A".to_string()),
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.id = Some("\u{4E00}".to_string());
  assert!(element.validate().is_empty());

  element.id = Some("A1".to_string());
  assert!(element.validate().is_empty());

  element.id = Some("_".to_string());
  assert!(element.validate().is_empty());

  element.id = Some("ABCD".to_string());
  assert!(element.validate().is_empty());

  element.id = Some("ABCD_1234-XY.00".to_string());
  assert!(element.validate().is_empty());

  element.id = Some(String::new());
  assert!(!element.validate().is_empty());

  element.id = Some("1A".to_string());
  assert!(!element.validate().is_empty());

  element.id = Some(".B".to_string());
  assert!(!element.validate().is_empty());

  element.id = Some("http:///temp".to_string());
  assert!(!element.validate().is_empty());
}

#[test]
fn qname_attribute_validation_test() {
  let mut element = QuickAccessToolbarControlClone {
    id_q: Some("A".to_string()),
    ..Default::default()
  };
  assert!(element.validate().is_empty());

  element.id_q = Some("A:b".to_string());
  assert!(element.validate().is_empty());

  element.id_q = Some("A1".to_string());
  assert!(element.validate().is_empty());

  element.id_q = Some("_".to_string());
  assert!(element.validate().is_empty());

  element.id_q = Some("ABCD".to_string());
  assert!(element.validate().is_empty());

  element.id_q = Some("ABCD_1234-XY.00".to_string());
  assert!(element.validate().is_empty());

  element.id_q = Some(String::new());
  assert!(!element.validate().is_empty());

  element.id_q = Some(":".to_string());
  assert!(!element.validate().is_empty());

  element.id_q = Some(":A".to_string());
  assert!(!element.validate().is_empty());

  element.id_q = Some("A:".to_string());
  assert!(!element.validate().is_empty());

  element.id_q = Some("1A".to_string());
  assert!(!element.validate().is_empty());

  element.id_q = Some(".B".to_string());
  assert!(!element.validate().is_empty());

  element.id_q = Some("http:///temp".to_string());
  assert!(!element.validate().is_empty());
}

#[test]
fn validate_reports_schema_errors_without_dom_api() {
  let mut element = TopBorder {
    width: Some(0),
    ..Default::default()
  };

  let errors = element.validate();
  assert_eq!(errors.len(), 1);
  assert_eq!(errors[0].error_type, ValidationErrorType::Schema);
  assert_eq!(errors[0].id, Some("number_sign"));

  element.width = Some(1);
  assert!(element.validate().is_empty());
}
