use ooxmlsdk::simple_type::{
  Base64BinaryValue, BooleanValue, ByteValue, DateTimeValue, DecimalValue, DoubleValue,
  HexBinaryValue, Int16Value, Int32Value, Int64Value, IntegerValue, OnOffValue, SByteValue,
  SingleValue, StringValue, TrueFalseBlankValue, TrueFalseValue, UInt16Value, UInt32Value,
  UInt64Value,
};

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
fn int16_value_round_trip_test() {
  assert_round_trip::<Int16Value>("0");
  assert_round_trip::<Int16Value>("-1");
  assert_round_trip::<Int16Value>("32767");
}

#[test]
fn int32_value_round_trip_test() {
  assert_round_trip::<Int32Value>("0");
  assert_round_trip::<Int32Value>("-1");
  assert_round_trip::<Int32Value>("2147483647");
}

#[test]
fn int64_value_round_trip_test() {
  assert_round_trip::<Int64Value>("0");
  assert_round_trip::<Int64Value>("-1");
  assert_round_trip::<Int64Value>("9223372036854775807");
}

#[test]
fn integer_value_round_trip_test() {
  assert_round_trip::<IntegerValue>("123456789");
}

#[test]
fn on_off_value_round_trip_test() {
  assert_round_trip::<OnOffValue>("true");
  assert_round_trip::<OnOffValue>("false");
}

#[test]
fn sbyte_value_round_trip_test() {
  assert_round_trip::<SByteValue>("abc");
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
}

#[test]
fn uint32_value_round_trip_test() {
  assert_round_trip::<UInt32Value>("0");
  assert_round_trip::<UInt32Value>("1");
  assert_round_trip::<UInt32Value>("4294967295");
}

#[test]
fn uint64_value_round_trip_test() {
  assert_round_trip::<UInt64Value>("0");
  assert_round_trip::<UInt64Value>("1");
  assert_round_trip::<UInt64Value>("18446744073709551615");
}
