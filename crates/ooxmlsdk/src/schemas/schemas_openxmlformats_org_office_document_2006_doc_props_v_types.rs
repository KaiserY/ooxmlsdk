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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:variant.
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
  pub xml_children: Option<VariantChoice>,
}
/// Vector.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:vector.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Vector/vt:vector")]
pub struct VtVector {
  /// Vector Base Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseType
  #[sdk(attr(qname = ":baseType"))]
  pub base_type: VectorBaseValues,
  /// Vector Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :size
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
  pub xml_children: Vec<VtVectorChoice>,
}
/// Array.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:array.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Array/vt:array")]
pub struct VtArray {
  /// Array Lower Bounds Attribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lBound
  #[sdk(attr(qname = ":lBound"))]
  pub lower_bounds: crate::simple_type::Int32Value,
  /// Array Upper Bounds Attribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uBound
  #[sdk(attr(qname = ":uBound"))]
  pub upper_bounds: crate::simple_type::Int32Value,
  /// Array Base Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseType
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
  pub xml_children: Vec<VtArrayChoice>,
}
/// Binary Blob.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:blob.
pub type VtBlob = crate::simple_type::Base64BinaryValue;
/// Binary Blob Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:oblob.
pub type VtoBlob = crate::simple_type::Base64BinaryValue;
/// Binary Stream.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:stream.
pub type VtStreamData = crate::simple_type::Base64BinaryValue;
/// Binary Stream Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ostream.
pub type VtoStreamData = crate::simple_type::Base64BinaryValue;
/// Binary Storage.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:storage.
pub type VtStorage = crate::simple_type::Base64BinaryValue;
/// Binary Storage Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ostorage.
pub type VtoStorage = crate::simple_type::Base64BinaryValue;
/// Empty.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:empty.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Empty/vt:empty")]
pub struct VtEmpty {}
/// Null.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:null.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Null/vt:null")]
pub struct VtNull {}
/// 1-Byte Signed Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:i1.
pub type VtByte = crate::simple_type::SByteValue;
/// 2-Byte Signed Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:i2.
pub type VtShort = crate::simple_type::Int16Value;
/// 4-Byte Signed Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:i4.
pub type VtInt32 = crate::simple_type::Int32Value;
/// Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:int.
pub type VtInteger = crate::simple_type::Int32Value;
/// 8-Byte Signed Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:i8.
pub type VtInt64 = crate::simple_type::Int64Value;
/// 1-Byte Unsigned Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ui1.
pub type VtUnsignedByte = crate::simple_type::ByteValue;
/// 2-Byte Unsigned Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ui2.
pub type VtUnsignedShort = crate::simple_type::UInt16Value;
/// 4-Byte Unsigned Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ui4.
pub type VtUnsignedInt32 = crate::simple_type::UInt32Value;
/// Unsigned Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:uint.
pub type VtUnsignedInteger = crate::simple_type::UInt32Value;
/// 8-Byte Unsigned Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:ui8.
pub type VtUnsignedInt64 = crate::simple_type::UInt64Value;
/// 4-Byte Real Number.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:r4.
pub type VtFloat = crate::simple_type::SingleValue;
/// 8-Byte Real Number.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:r8.
pub type VtDouble = crate::simple_type::DoubleValue;
/// Decimal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:decimal.
pub type VtDecimal = crate::simple_type::DecimalValue;
/// LPSTR.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:lpstr.
pub type Vtlpstr = crate::simple_type::StringValue;
/// LPWSTR.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:lpwstr.
pub type Vtlpwstr = crate::simple_type::StringValue;
/// Basic String.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:bstr.
pub type VtbString = crate::simple_type::StringValue;
/// Date and Time.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:date.
pub type VtDate = crate::simple_type::DateTimeValue;
/// File Time.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:filetime.
pub type VtFileTime = crate::simple_type::DateTimeValue;
/// Boolean.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:bool.
pub type VtBool = crate::simple_type::BooleanValue;
/// Currency.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:cy.
pub type VtCurrency = crate::simple_type::StringValue;
/// Error Status Code.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:error.
pub type VtError = crate::simple_type::StringValue;
/// Binary Versioned Stream.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:vstream.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Vstream/vt:vstream")]
pub struct VtvStreamData {
  /// VSTREAM Version Attribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :version
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:clsid.
pub type VtClassId = crate::simple_type::StringValue;
/// Clipboard Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is vt:cf.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "vt:CT_Cf/vt:cf")]
pub struct VtClipboardData {
  /// Format Attribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  #[sdk(number_range(source = 0u32, min = "-3", min_inclusive = true, max_inclusive = false))]
  pub format: Option<crate::simple_type::Int32Value>,
  /// size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Base64BinaryValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VariantChoice {
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  VtVector(std::boxed::Box<VtVector>),
  #[sdk(child(qname = "vt:CT_Array/vt:array"))]
  VtArray(std::boxed::Box<VtArray>),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:blob"))]
  VtBlob(crate::simple_type::Base64BinaryValue),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:oblob"))]
  VtOblob(crate::simple_type::Base64BinaryValue),
  #[sdk(child(qname = "vt:CT_Empty/vt:empty"))]
  VtEmpty(std::boxed::Box<VtEmpty>),
  #[sdk(child(qname = "vt:CT_Null/vt:null"))]
  VtNull(std::boxed::Box<VtNull>),
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "xsd:long/vt:i8"))]
  VtI8(crate::simple_type::Int64Value),
  #[sdk(text_child(qname = "xsd:int/vt:int"))]
  VtInt(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:unsignedLong/vt:ui8"))]
  VtUi8(crate::simple_type::UInt64Value),
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:uint"))]
  VtUint(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  #[sdk(text_child(qname = "xsd:decimal/vt:decimal"))]
  VtDecimal(crate::simple_type::DecimalValue),
  #[sdk(text_child(qname = "xsd:string/vt:lpstr"))]
  VtLpstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/vt:lpwstr"))]
  VtLpwstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  #[sdk(text_child(qname = "xsd:dateTime/vt:filetime"))]
  VtFiletime(crate::simple_type::DateTimeValue),
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:stream"))]
  VtStream(crate::simple_type::Base64BinaryValue),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:ostream"))]
  VtOstream(crate::simple_type::Base64BinaryValue),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:storage"))]
  VtStorage(crate::simple_type::Base64BinaryValue),
  #[sdk(text_child(qname = "xsd:base64Binary/vt:ostorage"))]
  VtOstorage(crate::simple_type::Base64BinaryValue),
  #[sdk(child(qname = "vt:CT_Vstream/vt:vstream"))]
  VtVstream(std::boxed::Box<VtvStreamData>),
  #[sdk(text_child(qname = "vt:ST_Clsid/vt:clsid"))]
  VtClsid(crate::simple_type::StringValue),
  #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
  VtCf(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtVectorChoice {
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "xsd:long/vt:i8"))]
  VtI8(crate::simple_type::Int64Value),
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:unsignedLong/vt:ui8"))]
  VtUi8(crate::simple_type::UInt64Value),
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  #[sdk(text_child(qname = "xsd:string/vt:lpstr"))]
  VtLpstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/vt:lpwstr"))]
  VtLpwstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  #[sdk(text_child(qname = "xsd:dateTime/vt:filetime"))]
  VtFiletime(crate::simple_type::DateTimeValue),
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "vt:ST_Clsid/vt:clsid"))]
  VtClsid(crate::simple_type::StringValue),
  #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
  VtCf(std::boxed::Box<VtClipboardData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VtArrayChoice {
  #[sdk(child(qname = "vt:CT_Variant/vt:variant"))]
  VtVariant(std::boxed::Box<Variant>),
  #[sdk(text_child(qname = "xsd:byte/vt:i1"))]
  VtI1(crate::simple_type::SByteValue),
  #[sdk(text_child(qname = "xsd:short/vt:i2"))]
  VtI2(crate::simple_type::Int16Value),
  #[sdk(text_child(qname = "xsd:int/vt:i4"))]
  VtI4(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "xsd:int/vt:int"))]
  VtInt(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::simple_type::ByteValue),
  #[sdk(text_child(qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::simple_type::UInt16Value),
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:unsignedInt/vt:uint"))]
  VtUint(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:float/vt:r4"))]
  VtR4(crate::simple_type::SingleValue),
  #[sdk(text_child(qname = "xsd:double/vt:r8"))]
  VtR8(crate::simple_type::DoubleValue),
  #[sdk(text_child(qname = "xsd:decimal/vt:decimal"))]
  VtDecimal(crate::simple_type::DecimalValue),
  #[sdk(text_child(qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::simple_type::DateTimeValue),
  #[sdk(text_child(qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::simple_type::BooleanValue),
  #[sdk(text_child(qname = "vt:ST_Error/vt:error"))]
  VtError(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::simple_type::StringValue),
}
