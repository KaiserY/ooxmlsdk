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
pub type ByteValue = u8;
pub type DateTimeValue = String;
pub type DecimalValue = String;
pub type DoubleValue = f64;
pub type HexBinaryValue = String;
pub type Int16Value = i16;
pub type Int32Value = i32;
pub type Int64Value = i64;
pub type IntegerValue = i64;
pub type SByteValue = i8;
pub type SingleValue = f32;
pub type StringValue = String;
pub type UInt16Value = u16;
pub type UInt32Value = u32;
pub type UInt64Value = u64;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanValue {
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "false")]
  False,
  #[sdk(rename = "1")]
  One,
  #[sdk(rename = "0")]
  #[default]
  Zero,
}

impl BooleanValue {
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    if value { Self::One } else { Self::Zero }
  }

  #[inline]
  pub const fn as_bool(self) -> bool {
    matches!(self, Self::True | Self::One)
  }
}

impl From<bool> for BooleanValue {
  #[inline]
  fn from(value: bool) -> Self {
    Self::from_bool(value)
  }
}

impl From<BooleanValue> for bool {
  #[inline]
  fn from(value: BooleanValue) -> Self {
    value.as_bool()
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OnOffValue {
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "on")]
  On,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "1")]
  One,
  #[sdk(rename = "0")]
  Zero,
}

impl OnOffValue {
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    if value { Self::True } else { Self::False }
  }

  #[inline]
  pub const fn as_bool(self) -> bool {
    matches!(self, Self::True | Self::On | Self::One)
  }
}

impl From<bool> for OnOffValue {
  #[inline]
  fn from(value: bool) -> Self {
    Self::from_bool(value)
  }
}

impl From<OnOffValue> for bool {
  #[inline]
  fn from(value: OnOffValue) -> Self {
    value.as_bool()
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TrueFalseValue {
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "f")]
  F,
}

impl TrueFalseValue {
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    if value { Self::True } else { Self::False }
  }

  #[inline]
  pub const fn as_bool(self) -> bool {
    matches!(self, Self::True | Self::T)
  }
}

impl From<bool> for TrueFalseValue {
  #[inline]
  fn from(value: bool) -> Self {
    Self::from_bool(value)
  }
}

impl From<TrueFalseValue> for bool {
  #[inline]
  fn from(value: TrueFalseValue) -> Self {
    value.as_bool()
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TrueFalseBlankValue {
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "f")]
  F,
  #[sdk(rename = "")]
  Blank,
}

impl TrueFalseBlankValue {
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    if value { Self::True } else { Self::False }
  }

  #[inline]
  pub const fn as_bool(self) -> bool {
    matches!(self, Self::True | Self::T)
  }
}

impl From<bool> for TrueFalseBlankValue {
  #[inline]
  fn from(value: bool) -> Self {
    Self::from_bool(value)
  }
}

impl From<TrueFalseBlankValue> for bool {
  #[inline]
  fn from(value: TrueFalseBlankValue) -> Self {
    value.as_bool()
  }
}

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
