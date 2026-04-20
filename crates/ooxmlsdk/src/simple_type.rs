use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListValue<T>(pub Vec<T>);

impl<T> Display for ListValue<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut values = self.0.iter();
    if let Some(first) = values.next() {
      write!(f, "{first}")?;
      for value in values {
        write!(f, " {value}")?;
      }
    }
    Ok(())
  }
}

impl<T> FromStr for ListValue<T>
where
  T: FromStr,
{
  type Err = crate::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.split_whitespace()
      .map(|value| {
        value
          .parse::<T>()
          .map_err(|_| crate::common::invalid_field_value("ListValue", "value", value))
      })
      .collect::<Result<Vec<_>, _>>()
      .map(Self)
  }
}

pub type Base64BinaryValue = String;
pub type BooleanValue = bool;
pub type ByteValue = u8;
pub type DateTimeValue = String;
pub type DecimalValue = String;
pub type DoubleValue = f64;
pub type HexBinaryValue = String;
pub type Int16Value = i16;
pub type Int32Value = i32;
pub type Int64Value = i64;
pub type IntegerValue = i64;
pub type OnOffValue = bool;
pub type SByteValue = i8;
pub type SingleValue = f32;
pub type StringValue = String;
pub type TrueFalseBlankValue = bool;
pub type TrueFalseValue = bool;
pub type UInt16Value = u16;
pub type UInt32Value = u32;
pub type UInt64Value = u64;
