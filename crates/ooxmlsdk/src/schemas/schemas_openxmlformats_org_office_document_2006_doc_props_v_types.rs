//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VectorBaseValues {
  #[sdk(rename = "variant")]
  #[default]
  Variant,
  #[sdk(rename = "i1")]
  OneByteSignedInteger,
  #[sdk(rename = "i2")]
  TwoBytesSignedInteger,
  #[sdk(rename = "i4")]
  FourBytesSignedInteger,
  #[sdk(rename = "i8")]
  EightBytesSignedInteger,
  #[sdk(rename = "ui1")]
  OneByteUnsignedInteger,
  #[sdk(rename = "ui2")]
  TwoBytesUnsignedInteger,
  #[sdk(rename = "ui4")]
  FourBytesUnsignedInteger,
  #[sdk(rename = "ui8")]
  EightBytesUnsignedInteger,
  #[sdk(rename = "r4")]
  FourBytesReal,
  #[sdk(rename = "r8")]
  EightBytesReal,
  #[sdk(rename = "lpstr")]
  Lpstr,
  #[sdk(rename = "lpwstr")]
  Lpwstr,
  #[sdk(rename = "bstr")]
  Bstr,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "filetime")]
  Filetime,
  #[sdk(rename = "bool")]
  Bool,
  #[sdk(rename = "cy")]
  Currency,
  #[sdk(rename = "error")]
  Error,
  #[sdk(rename = "clsid")]
  ClassId,
  #[sdk(rename = "cf")]
  ClipboardData,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ArrayBaseValues {
  #[sdk(rename = "variant")]
  #[default]
  Variant,
  #[sdk(rename = "i1")]
  OneByteSignedInteger,
  #[sdk(rename = "i2")]
  TwoBytesSignedInteger,
  #[sdk(rename = "i4")]
  FourBytesSignedInteger,
  #[sdk(rename = "int")]
  Integer,
  #[sdk(rename = "ui1")]
  OneByteUnsignedInteger,
  #[sdk(rename = "ui2")]
  TwoBytesUnsignedInteger,
  #[sdk(rename = "ui4")]
  FourBytesUnsignedInteger,
  #[sdk(rename = "uint")]
  UnsignedInteger,
  #[sdk(rename = "r4")]
  FourBytesReal,
  #[sdk(rename = "r8")]
  EightBytesReal,
  #[sdk(rename = "decimal")]
  Decimal,
  #[sdk(rename = "bstr")]
  Bstr,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "bool")]
  Bool,
  #[sdk(rename = "cy")]
  Currency,
  #[sdk(rename = "error")]
  Error,
}
/// Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Variant/vt:variant")]
pub struct Variant {
  #[sdk(choice(
    qname = "vt:CT_Variant/vt:variant",
    qname = "vt:CT_Vector/vt:vector",
    qname = "vt:CT_Array/vt:array",
    qname = "xsd:base64Binary/vt:blob",
    qname = "xsd:base64Binary/vt:oblob",
    qname = "vt:CT_Empty/vt:empty",
    qname = "vt:CT_Null/vt:null",
    qname = "xsd:byte/vt:i1",
    qname = "xsd:short/vt:i2",
    qname = "xsd:int/vt:i4",
    qname = "xsd:long/vt:i8",
    qname = "xsd:int/vt:int",
    qname = "xsd:unsignedByte/vt:ui1",
    qname = "xsd:unsignedShort/vt:ui2",
    qname = "xsd:unsignedInt/vt:ui4",
    qname = "xsd:unsignedLong/vt:ui8",
    qname = "xsd:unsignedInt/vt:uint",
    qname = "xsd:float/vt:r4",
    qname = "xsd:double/vt:r8",
    qname = "xsd:decimal/vt:decimal",
    qname = "xsd:string/vt:lpstr",
    qname = "xsd:string/vt:lpwstr",
    qname = "xsd:string/vt:bstr",
    qname = "xsd:dateTime/vt:date",
    qname = "xsd:dateTime/vt:filetime",
    qname = "xsd:boolean/vt:bool",
    qname = "vt:ST_Cy/vt:cy",
    qname = "vt:ST_Error/vt:error",
    qname = "xsd:base64Binary/vt:stream",
    qname = "xsd:base64Binary/vt:ostream",
    qname = "xsd:base64Binary/vt:storage",
    qname = "xsd:base64Binary/vt:ostorage",
    qname = "vt:CT_Vstream/vt:vstream",
    qname = "vt:ST_Clsid/vt:clsid",
    qname = "vt:CT_Cf/vt:cf"
  ))]
  pub variant_choice: Option<VariantChoice>,
}
/// Vector.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Vector/vt:vector")]
pub struct VtVector {
  /// Vector Base Type
  #[sdk(attr(qname = ":baseType"))]
  pub base_type: VectorBaseValues,
  /// Vector Size
  #[sdk(attr(qname = ":size"))]
  pub size: crate::simple_type::UInt32Value,
  #[sdk(choice(
    qname = "vt:CT_Variant/vt:variant",
    qname = "xsd:byte/vt:i1",
    qname = "xsd:short/vt:i2",
    qname = "xsd:int/vt:i4",
    qname = "xsd:long/vt:i8",
    qname = "xsd:unsignedByte/vt:ui1",
    qname = "xsd:unsignedShort/vt:ui2",
    qname = "xsd:unsignedInt/vt:ui4",
    qname = "xsd:unsignedLong/vt:ui8",
    qname = "xsd:float/vt:r4",
    qname = "xsd:double/vt:r8",
    qname = "xsd:string/vt:lpstr",
    qname = "xsd:string/vt:lpwstr",
    qname = "xsd:string/vt:bstr",
    qname = "xsd:dateTime/vt:date",
    qname = "xsd:dateTime/vt:filetime",
    qname = "xsd:boolean/vt:bool",
    qname = "vt:ST_Cy/vt:cy",
    qname = "vt:ST_Error/vt:error",
    qname = "vt:ST_Clsid/vt:clsid",
    qname = "vt:CT_Cf/vt:cf"
  ))]
  pub vt_vector_choice: Vec<VtVectorChoice>,
}
/// Array.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Array/vt:array")]
pub struct VtArray {
  /// Array Lower Bounds Attribute
  #[sdk(attr(qname = ":lBound"))]
  pub lower_bounds: crate::simple_type::Int32Value,
  /// Array Upper Bounds Attribute
  #[sdk(attr(qname = ":uBound"))]
  pub upper_bounds: crate::simple_type::Int32Value,
  /// Array Base Type
  #[sdk(attr(qname = ":baseType"))]
  pub base_type: ArrayBaseValues,
  #[sdk(choice(
    qname = "vt:CT_Variant/vt:variant",
    qname = "xsd:byte/vt:i1",
    qname = "xsd:short/vt:i2",
    qname = "xsd:int/vt:i4",
    qname = "xsd:int/vt:int",
    qname = "xsd:unsignedByte/vt:ui1",
    qname = "xsd:unsignedShort/vt:ui2",
    qname = "xsd:unsignedInt/vt:ui4",
    qname = "xsd:unsignedInt/vt:uint",
    qname = "xsd:float/vt:r4",
    qname = "xsd:double/vt:r8",
    qname = "xsd:decimal/vt:decimal",
    qname = "xsd:string/vt:bstr",
    qname = "xsd:dateTime/vt:date",
    qname = "xsd:boolean/vt:bool",
    qname = "vt:ST_Error/vt:error",
    qname = "vt:ST_Cy/vt:cy"
  ))]
  pub vt_array_choice: Vec<VtArrayChoice>,
}
/// Binary Blob.
pub type VtBlob = crate::simple_type::Base64BinaryValue;
/// Binary Blob Object.
pub type VtoBlob = crate::simple_type::Base64BinaryValue;
/// Binary Stream.
pub type VtStreamData = crate::simple_type::Base64BinaryValue;
/// Binary Stream Object.
pub type VtoStreamData = crate::simple_type::Base64BinaryValue;
/// Binary Storage.
pub type VtStorage = crate::simple_type::Base64BinaryValue;
/// Binary Storage Object.
pub type VtoStorage = crate::simple_type::Base64BinaryValue;
/// 1-Byte Signed Integer.
pub type VtByte = crate::simple_type::SByteValue;
/// 2-Byte Signed Integer.
pub type VtShort = crate::simple_type::Int16Value;
/// 4-Byte Signed Integer.
pub type VtInt32 = crate::simple_type::Int32Value;
/// Integer.
pub type VtInteger = crate::simple_type::Int32Value;
/// 8-Byte Signed Integer.
pub type VtInt64 = crate::simple_type::Int64Value;
/// 1-Byte Unsigned Integer.
pub type VtUnsignedByte = crate::simple_type::ByteValue;
/// 2-Byte Unsigned Integer.
pub type VtUnsignedShort = crate::simple_type::UInt16Value;
/// 4-Byte Unsigned Integer.
pub type VtUnsignedInt32 = crate::simple_type::UInt32Value;
/// Unsigned Integer.
pub type VtUnsignedInteger = crate::simple_type::UInt32Value;
/// 8-Byte Unsigned Integer.
pub type VtUnsignedInt64 = crate::simple_type::UInt64Value;
/// 4-Byte Real Number.
pub type VtFloat = crate::simple_type::SingleValue;
/// 8-Byte Real Number.
pub type VtDouble = crate::simple_type::DoubleValue;
/// Decimal.
pub type VtDecimal = crate::simple_type::DecimalValue;
/// LPSTR.
pub type Vtlpstr = crate::simple_type::StringValue;
/// LPWSTR.
pub type Vtlpwstr = crate::simple_type::StringValue;
/// Basic String.
pub type VtbString = crate::simple_type::StringValue;
/// Date and Time.
pub type VtDate = crate::simple_type::DateTimeValue;
/// File Time.
pub type VtFileTime = crate::simple_type::DateTimeValue;
/// Boolean.
pub type VtBool = crate::simple_type::BooleanValue;
/// Currency.
pub type VtCurrency = crate::simple_type::StringValue;
/// Error Status Code.
pub type VtError = crate::simple_type::StringValue;
/// Binary Versioned Stream.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Vstream/vt:vstream")]
pub struct VtvStreamData {
  /// VSTREAM Version Attribute
  #[sdk(attr(qname = ":version"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\s*\\{[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\\}\\s*"
  ))]
  pub version: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Base64BinaryValue>,
}
/// Class ID.
pub type VtClassId = crate::simple_type::StringValue;
/// Clipboard Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Cf/vt:cf")]
pub struct VtClipboardData {
  /// Format Attribute
  #[sdk(attr(qname = ":format"))]
  #[sdk(number_range(source = 0u32, min = "-3", min_inclusive = true, max_inclusive = false))]
  pub format: Option<crate::simple_type::Int32Value>,
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Base64BinaryValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VariantChoice {
  /// Variant.
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  /// Vector.
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  VtVector(std::boxed::Box<VtVector>),
  /// Array.
  #[sdk(child(qname = "vt:CT_Array/vt:array"))]
  VtArray(std::boxed::Box<VtArray>),
  /// Binary Blob.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:blob"))]
  VtBlob(crate::simple_type::Base64BinaryValue),
  /// Binary Blob Object.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:oblob"))]
  VtOblob(crate::simple_type::Base64BinaryValue),
  /// Empty.
  #[sdk(empty_child(qname = "vt:CT_Empty/vt:empty"))]
  VtEmpty,
  /// Null.
  #[sdk(empty_child(qname = "vt:CT_Null/vt:null"))]
  VtNull,
  /// 1-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:long/vt:i8"))]
  VtI8(crate::simple_type::Int64Value),
  /// Integer.
  #[sdk(text_child(qname = "xsd:int/vt:int"))]
  VtInt(crate::simple_type::Int32Value),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedLong/vt:ui8"))]
  VtUi8(crate::simple_type::UInt64Value),
  /// Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:uint"))]
  VtUint(crate::simple_type::UInt32Value),
  /// 4-Byte Real Number.
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  /// 8-Byte Real Number.
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  /// Decimal.
  #[sdk(text_child(qname = "xsd:decimal/vt:decimal"))]
  VtDecimal(crate::simple_type::DecimalValue),
  /// LPSTR.
  #[sdk(text_child(qname = "xsd:string/vt:lpstr"))]
  VtLpstr(crate::simple_type::StringValue),
  /// LPWSTR.
  #[sdk(text_child(qname = "xsd:string/vt:lpwstr"))]
  VtLpwstr(crate::simple_type::StringValue),
  /// Basic String.
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  /// Date and Time.
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  /// File Time.
  #[sdk(text_child(qname = "xsd:dateTime/vt:filetime"))]
  VtFiletime(crate::simple_type::DateTimeValue),
  /// Boolean.
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  /// Currency.
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
  /// Error Status Code.
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  /// Binary Stream.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:stream"))]
  VtStream(crate::simple_type::Base64BinaryValue),
  /// Binary Stream Object.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:ostream"))]
  VtOstream(crate::simple_type::Base64BinaryValue),
  /// Binary Storage.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:storage"))]
  VtStorage(crate::simple_type::Base64BinaryValue),
  /// Binary Storage Object.
  #[sdk(text_child(qname = "xsd:base64Binary/vt:ostorage"))]
  VtOstorage(crate::simple_type::Base64BinaryValue),
  /// Binary Versioned Stream.
  #[sdk(child(qname = "vt:CT_Vstream/vt:vstream"))]
  VtVstream(std::boxed::Box<VtvStreamData>),
  /// Class ID.
  #[sdk(text_child(qname = "vt:ST_Clsid/vt:clsid"))]
  VtClsid(crate::simple_type::StringValue),
  /// Clipboard Data.
  #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
  VtCf(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtVectorChoice {
  /// Variant.
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  /// 1-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:long/vt:i8"))]
  VtI8(crate::simple_type::Int64Value),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedLong/vt:ui8"))]
  VtUi8(crate::simple_type::UInt64Value),
  /// 4-Byte Real Number.
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  /// 8-Byte Real Number.
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  /// LPSTR.
  #[sdk(text_child(qname = "xsd:string/vt:lpstr"))]
  VtLpstr(crate::simple_type::StringValue),
  /// LPWSTR.
  #[sdk(text_child(qname = "xsd:string/vt:lpwstr"))]
  VtLpwstr(crate::simple_type::StringValue),
  /// Basic String.
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  /// Date and Time.
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  /// File Time.
  #[sdk(text_child(qname = "xsd:dateTime/vt:filetime"))]
  VtFiletime(crate::simple_type::DateTimeValue),
  /// Boolean.
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  /// Currency.
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
  /// Error Status Code.
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  /// Class ID.
  #[sdk(text_child(qname = "vt:ST_Clsid/vt:clsid"))]
  VtClsid(crate::simple_type::StringValue),
  /// Clipboard Data.
  #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
  VtCf(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtArrayChoice {
  /// Variant.
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  /// 1-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  /// Integer.
  #[sdk(text_child(qname = "xsd:int/vt:int"))]
  VtInt(crate::simple_type::Int32Value),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  /// Unsigned Integer.
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:uint"))]
  VtUint(crate::simple_type::UInt32Value),
  /// 4-Byte Real Number.
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  /// 8-Byte Real Number.
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  /// Decimal.
  #[sdk(text_child(qname = "xsd:decimal/vt:decimal"))]
  VtDecimal(crate::simple_type::DecimalValue),
  /// Basic String.
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  /// Date and Time.
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  /// Boolean.
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  /// Error Status Code.
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  /// Currency.
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
}
