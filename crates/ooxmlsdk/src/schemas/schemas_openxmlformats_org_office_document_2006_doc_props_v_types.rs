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
#[sdk(qname = "vt:variant")]
pub struct Variant {
  #[sdk(
        choice(
            child(variant = Variant, qname = "vt:variant"),
            child(variant = VtVector, qname = "vt:vector"),
            child(variant = VtArray, qname = "vt:array"),
            text_child(variant = VtBlob, qname = "vt:blob"),
            text_child(variant = VtoBlob, qname = "vt:oblob"),
            empty_child(variant = VtEmpty, qname = "vt:empty"),
            empty_child(variant = VtNull, qname = "vt:null"),
            text_child(variant = VtByte, qname = "vt:i1"),
            text_child(variant = VtShort, qname = "vt:i2"),
            text_child(variant = VtInt32, qname = "vt:i4"),
            text_child(variant = VtInt64, qname = "vt:i8"),
            text_child(variant = VtInteger, qname = "vt:int"),
            text_child(variant = VtUnsignedByte, qname = "vt:ui1"),
            text_child(variant = VtUnsignedShort, qname = "vt:ui2"),
            text_child(variant = VtUnsignedInt32, qname = "vt:ui4"),
            text_child(variant = VtUnsignedInt64, qname = "vt:ui8"),
            text_child(variant = VtUnsignedInteger, qname = "vt:uint"),
            text_child(variant = VtFloat, qname = "vt:r4"),
            text_child(variant = VtDouble, qname = "vt:r8"),
            text_child(variant = VtDecimal, qname = "vt:decimal"),
            text_child(variant = Vtlpstr, qname = "vt:lpstr"),
            text_child(variant = Vtlpwstr, qname = "vt:lpwstr"),
            text_child(variant = VtbString, qname = "vt:bstr"),
            text_child(variant = VtDate, qname = "vt:date"),
            text_child(variant = VtFileTime, qname = "vt:filetime"),
            text_child(variant = VtBool, qname = "vt:bool"),
            text_child(variant = VtCurrency, qname = "vt:cy"),
            text_child(variant = VtError, qname = "vt:error"),
            text_child(variant = VtStreamData, qname = "vt:stream"),
            text_child(variant = VtoStreamData, qname = "vt:ostream"),
            text_child(variant = VtStorage, qname = "vt:storage"),
            text_child(variant = VtoStorage, qname = "vt:ostorage"),
            child(variant = VtvStreamData, qname = "vt:vstream"),
            text_child(variant = VtClassId, qname = "vt:clsid"),
            child(variant = VtClipboardData, qname = "vt:cf")
        )
    )]
  pub variant_choice: Option<VariantChoice>,
}
/// Vector.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:vector")]
pub struct VtVector {
  /// Vector Base Type
  #[sdk(attr(qname = ":baseType"))]
  pub base_type: VectorBaseValues,
  /// Vector Size
  #[sdk(attr(qname = ":size"))]
  pub size: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = Variant, qname = "vt:variant"),
            text_child(variant = VtByte, qname = "vt:i1"),
            text_child(variant = VtShort, qname = "vt:i2"),
            text_child(variant = VtInt32, qname = "vt:i4"),
            text_child(variant = VtInt64, qname = "vt:i8"),
            text_child(variant = VtUnsignedByte, qname = "vt:ui1"),
            text_child(variant = VtUnsignedShort, qname = "vt:ui2"),
            text_child(variant = VtUnsignedInt32, qname = "vt:ui4"),
            text_child(variant = VtUnsignedInt64, qname = "vt:ui8"),
            text_child(variant = VtFloat, qname = "vt:r4"),
            text_child(variant = VtDouble, qname = "vt:r8"),
            text_child(variant = Vtlpstr, qname = "vt:lpstr"),
            text_child(variant = Vtlpwstr, qname = "vt:lpwstr"),
            text_child(variant = VtbString, qname = "vt:bstr"),
            text_child(variant = VtDate, qname = "vt:date"),
            text_child(variant = VtFileTime, qname = "vt:filetime"),
            text_child(variant = VtBool, qname = "vt:bool"),
            text_child(variant = VtCurrency, qname = "vt:cy"),
            text_child(variant = VtError, qname = "vt:error"),
            text_child(variant = VtClassId, qname = "vt:clsid"),
            child(variant = VtClipboardData, qname = "vt:cf")
        )
    )]
  pub vt_vector_choice: Vec<VtVectorChoice>,
}
/// Array.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:array")]
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
  #[sdk(
        choice(
            child(variant = Variant, qname = "vt:variant"),
            text_child(variant = VtByte, qname = "vt:i1"),
            text_child(variant = VtShort, qname = "vt:i2"),
            text_child(variant = VtInt32, qname = "vt:i4"),
            text_child(variant = VtInteger, qname = "vt:int"),
            text_child(variant = VtUnsignedByte, qname = "vt:ui1"),
            text_child(variant = VtUnsignedShort, qname = "vt:ui2"),
            text_child(variant = VtUnsignedInt32, qname = "vt:ui4"),
            text_child(variant = VtUnsignedInteger, qname = "vt:uint"),
            text_child(variant = VtFloat, qname = "vt:r4"),
            text_child(variant = VtDouble, qname = "vt:r8"),
            text_child(variant = VtDecimal, qname = "vt:decimal"),
            text_child(variant = VtbString, qname = "vt:bstr"),
            text_child(variant = VtDate, qname = "vt:date"),
            text_child(variant = VtBool, qname = "vt:bool"),
            text_child(variant = VtError, qname = "vt:error"),
            text_child(variant = VtCurrency, qname = "vt:cy")
        )
    )]
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
#[sdk(qname = "vt:vstream")]
pub struct VtvStreamData {
  /// VSTREAM Version Attribute
  #[sdk(attr(qname = ":version"))]
  #[sdk(pattern(
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
#[sdk(qname = "vt:cf")]
pub struct VtClipboardData {
  /// Format Attribute
  #[sdk(attr(qname = ":format"))]
  #[sdk(number_range(range = -3..))]
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
  Variant(std::boxed::Box<Variant>),
  /// Vector.
  VtVector(std::boxed::Box<VtVector>),
  /// Array.
  VtArray(std::boxed::Box<VtArray>),
  /// Binary Blob.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:blob"))]
  VtBlob(VtBlob),
  /// Binary Blob Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:oblob"))]
  VtoBlob(VtoBlob),
  /// Empty.
  #[sdk(empty_child(qname = "vt:empty"))]
  VtEmpty,
  /// Null.
  #[sdk(empty_child(qname = "vt:null"))]
  VtNull,
  /// 1-Byte Signed Integer.
  #[sdk(text_child(simple_type = "SByteValue", qname = "vt:i1"))]
  VtByte(VtByte),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int16Value", qname = "vt:i2"))]
  VtShort(VtShort),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:i4"))]
  VtInt32(VtInt32),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int64Value", qname = "vt:i8"))]
  VtInt64(VtInt64),
  /// Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:int"))]
  VtInteger(VtInteger),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "ByteValue", qname = "vt:ui1"))]
  VtUnsignedByte(VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt16Value", qname = "vt:ui2"))]
  VtUnsignedShort(VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:ui4"))]
  VtUnsignedInt32(VtUnsignedInt32),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt64Value", qname = "vt:ui8"))]
  VtUnsignedInt64(VtUnsignedInt64),
  /// Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:uint"))]
  VtUnsignedInteger(VtUnsignedInteger),
  /// 4-Byte Real Number.
  #[sdk(text_child(simple_type = "SingleValue", qname = "vt:r4"))]
  VtFloat(VtFloat),
  /// 8-Byte Real Number.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "vt:r8"))]
  VtDouble(VtDouble),
  /// Decimal.
  #[sdk(text_child(simple_type = "DecimalValue", qname = "vt:decimal"))]
  VtDecimal(VtDecimal),
  /// LPSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpstr"))]
  Vtlpstr(Vtlpstr),
  /// LPWSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpwstr"))]
  Vtlpwstr(Vtlpwstr),
  /// Basic String.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:bstr"))]
  VtbString(VtbString),
  /// Date and Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:date"))]
  VtDate(VtDate),
  /// File Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:filetime"))]
  VtFileTime(VtFileTime),
  /// Boolean.
  #[sdk(text_child(simple_type = "BooleanValue", qname = "vt:bool"))]
  VtBool(VtBool),
  /// Currency.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:cy"))]
  VtCurrency(VtCurrency),
  /// Error Status Code.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:error"))]
  VtError(VtError),
  /// Binary Stream.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:stream"))]
  VtStreamData(VtStreamData),
  /// Binary Stream Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:ostream"))]
  VtoStreamData(VtoStreamData),
  /// Binary Storage.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:storage"))]
  VtStorage(VtStorage),
  /// Binary Storage Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:ostorage"))]
  VtoStorage(VtoStorage),
  /// Binary Versioned Stream.
  VtvStreamData(std::boxed::Box<VtvStreamData>),
  /// Class ID.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:clsid"))]
  VtClassId(VtClassId),
  /// Clipboard Data.
  VtClipboardData(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtVectorChoice {
  /// Variant.
  Variant(std::boxed::Box<Variant>),
  /// 1-Byte Signed Integer.
  #[sdk(text_child(simple_type = "SByteValue", qname = "vt:i1"))]
  VtByte(VtByte),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int16Value", qname = "vt:i2"))]
  VtShort(VtShort),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:i4"))]
  VtInt32(VtInt32),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int64Value", qname = "vt:i8"))]
  VtInt64(VtInt64),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "ByteValue", qname = "vt:ui1"))]
  VtUnsignedByte(VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt16Value", qname = "vt:ui2"))]
  VtUnsignedShort(VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:ui4"))]
  VtUnsignedInt32(VtUnsignedInt32),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt64Value", qname = "vt:ui8"))]
  VtUnsignedInt64(VtUnsignedInt64),
  /// 4-Byte Real Number.
  #[sdk(text_child(simple_type = "SingleValue", qname = "vt:r4"))]
  VtFloat(VtFloat),
  /// 8-Byte Real Number.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "vt:r8"))]
  VtDouble(VtDouble),
  /// LPSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpstr"))]
  Vtlpstr(Vtlpstr),
  /// LPWSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpwstr"))]
  Vtlpwstr(Vtlpwstr),
  /// Basic String.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:bstr"))]
  VtbString(VtbString),
  /// Date and Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:date"))]
  VtDate(VtDate),
  /// File Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:filetime"))]
  VtFileTime(VtFileTime),
  /// Boolean.
  #[sdk(text_child(simple_type = "BooleanValue", qname = "vt:bool"))]
  VtBool(VtBool),
  /// Currency.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:cy"))]
  VtCurrency(VtCurrency),
  /// Error Status Code.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:error"))]
  VtError(VtError),
  /// Class ID.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:clsid"))]
  VtClassId(VtClassId),
  /// Clipboard Data.
  VtClipboardData(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtArrayChoice {
  /// Variant.
  Variant(std::boxed::Box<Variant>),
  /// 1-Byte Signed Integer.
  #[sdk(text_child(simple_type = "SByteValue", qname = "vt:i1"))]
  VtByte(VtByte),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int16Value", qname = "vt:i2"))]
  VtShort(VtShort),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:i4"))]
  VtInt32(VtInt32),
  /// Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:int"))]
  VtInteger(VtInteger),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "ByteValue", qname = "vt:ui1"))]
  VtUnsignedByte(VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt16Value", qname = "vt:ui2"))]
  VtUnsignedShort(VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:ui4"))]
  VtUnsignedInt32(VtUnsignedInt32),
  /// Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:uint"))]
  VtUnsignedInteger(VtUnsignedInteger),
  /// 4-Byte Real Number.
  #[sdk(text_child(simple_type = "SingleValue", qname = "vt:r4"))]
  VtFloat(VtFloat),
  /// 8-Byte Real Number.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "vt:r8"))]
  VtDouble(VtDouble),
  /// Decimal.
  #[sdk(text_child(simple_type = "DecimalValue", qname = "vt:decimal"))]
  VtDecimal(VtDecimal),
  /// Basic String.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:bstr"))]
  VtbString(VtbString),
  /// Date and Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:date"))]
  VtDate(VtDate),
  /// Boolean.
  #[sdk(text_child(simple_type = "BooleanValue", qname = "vt:bool"))]
  VtBool(VtBool),
  /// Error Status Code.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:error"))]
  VtError(VtError),
  /// Currency.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:cy"))]
  VtCurrency(VtCurrency),
}
