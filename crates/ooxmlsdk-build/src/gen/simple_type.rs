use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;

pub fn gen_simple_type() -> Result<TokenStream, Box<dyn Error>> {
  Ok(quote! {
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
    pub type IntegerValue = String;
    pub type OnOffValue = bool;
    pub type SByteValue = String;
    pub type SingleValue = f32;
    pub type StringValue = String;
    pub type TrueFalseBlankValue = bool;
    pub type TrueFalseValue = bool;
    pub type UInt16Value = u16;
    pub type UInt32Value = u32;
    pub type UInt64Value = u64;
  })
}
