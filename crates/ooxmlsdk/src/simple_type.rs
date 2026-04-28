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

pub trait HexBinaryValueExt {
  fn is_valid_hex_binary(&self) -> bool;
  fn try_get_bytes(&self) -> Option<Vec<u8>>;
}

impl<T> HexBinaryValueExt for T
where
  T: AsRef<str> + ?Sized,
{
  fn is_valid_hex_binary(&self) -> bool {
    is_valid_hex_binary(self.as_ref())
  }

  fn try_get_bytes(&self) -> Option<Vec<u8>> {
    hex_binary_to_bytes(self.as_ref())
  }
}

pub fn is_valid_hex_binary(value: &str) -> bool {
  value.len().is_multiple_of(2) && value.as_bytes().iter().all(|byte| byte.is_ascii_hexdigit())
}

pub fn hex_binary_to_bytes(value: &str) -> Option<Vec<u8>> {
  if !is_valid_hex_binary(value) {
    return None;
  }

  value
    .as_bytes()
    .chunks_exact(2)
    .map(|pair| Some((hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?))
    .collect()
}

pub fn hex_binary_from_bytes(bytes: impl AsRef<[u8]>) -> HexBinaryValue {
  const HEX: &[u8; 16] = b"0123456789ABCDEF";
  let bytes = bytes.as_ref();
  let mut value = String::with_capacity(bytes.len() * 2);
  for byte in bytes {
    value.push(HEX[(byte >> 4) as usize] as char);
    value.push(HEX[(byte & 0x0F) as usize] as char);
  }
  value
}

fn hex_nibble(byte: u8) -> Option<u8> {
  match byte {
    b'0'..=b'9' => Some(byte - b'0'),
    b'a'..=b'f' => Some(byte - b'a' + 10),
    b'A'..=b'F' => Some(byte - b'A' + 10),
    _ => None,
  }
}
