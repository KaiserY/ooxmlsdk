//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom File Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:CT_Properties/op:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Custom File Property.
  #[sdk(child(qname = "op:CT_Property/op:property"))]
  pub op_property: Vec<CustomDocumentProperty>,
}
/// Custom File Property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:CT_Property/op:property")]
pub struct CustomDocumentProperty {
  /// Format ID
  #[sdk(attr(qname = ":fmtid"))]
  #[sdk(pattern(
    regex = "\\s*\\{[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\\}\\s*"
  ))]
  pub format_id: crate::simple_type::StringValue,
  /// Property ID
  #[sdk(attr(qname = ":pid"))]
  pub property_id: crate::simple_type::Int32Value,
  /// Custom File Property Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Bookmark Link Target
  #[sdk(attr(qname = ":linkTarget"))]
  pub link_target: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
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
  pub custom_document_property_choice: Option<CustomDocumentPropertyChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomDocumentPropertyChoice {
  /// Vector.
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  VtVector(std::boxed::Box<crate::schemas::vt::VtVector>),
  /// Array.
  #[sdk(child(qname = "vt:CT_Array/vt:array"))]
  VtArray(std::boxed::Box<crate::schemas::vt::VtArray>),
  /// Binary Blob.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "xsd:base64Binary/vt:blob"))]
  VtBlob(crate::schemas::vt::VtBlob),
  /// Binary Blob Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "xsd:base64Binary/vt:oblob"))]
  VtOblob(crate::schemas::vt::VtoBlob),
  /// Empty.
  #[sdk(empty_child(qname = "vt:CT_Empty/vt:empty"))]
  VtEmpty,
  /// Null.
  #[sdk(empty_child(qname = "vt:CT_Null/vt:null"))]
  VtNull,
  /// 1-Byte Signed Integer.
  #[sdk(text_child(simple_type = "SByteValue", qname = "xsd:byte/vt:i1"))]
  VtI1(crate::schemas::vt::VtByte),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int16Value", qname = "xsd:short/vt:i2"))]
  VtI2(crate::schemas::vt::VtShort),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "xsd:int/vt:i4"))]
  VtI4(crate::schemas::vt::VtInt32),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int64Value", qname = "xsd:long/vt:i8"))]
  VtI8(crate::schemas::vt::VtInt64),
  /// Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "xsd:int/vt:int"))]
  VtInt(crate::schemas::vt::VtInteger),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "ByteValue", qname = "xsd:unsignedByte/vt:ui1"))]
  VtUi1(crate::schemas::vt::VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt16Value", qname = "xsd:unsignedShort/vt:ui2"))]
  VtUi2(crate::schemas::vt::VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "xsd:unsignedInt/vt:ui4"))]
  VtUi4(crate::schemas::vt::VtUnsignedInt32),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt64Value", qname = "xsd:unsignedLong/vt:ui8"))]
  VtUi8(crate::schemas::vt::VtUnsignedInt64),
  /// Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "xsd:unsignedInt/vt:uint"))]
  VtUint(crate::schemas::vt::VtUnsignedInteger),
  /// 4-Byte Real Number.
  #[sdk(text_child(simple_type = "SingleValue", qname = "xsd:float/vt:r4"))]
  VtR4(crate::schemas::vt::VtFloat),
  /// 8-Byte Real Number.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "xsd:double/vt:r8"))]
  VtR8(crate::schemas::vt::VtDouble),
  /// Decimal.
  #[sdk(text_child(simple_type = "DecimalValue", qname = "xsd:decimal/vt:decimal"))]
  VtDecimal(crate::schemas::vt::VtDecimal),
  /// LPSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "xsd:string/vt:lpstr"))]
  VtLpstr(crate::schemas::vt::Vtlpstr),
  /// LPWSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "xsd:string/vt:lpwstr"))]
  VtLpwstr(crate::schemas::vt::Vtlpwstr),
  /// Basic String.
  #[sdk(text_child(simple_type = "StringValue", qname = "xsd:string/vt:bstr"))]
  VtBstr(crate::schemas::vt::VtbString),
  /// Date and Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "xsd:dateTime/vt:date"))]
  VtDate(crate::schemas::vt::VtDate),
  /// File Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "xsd:dateTime/vt:filetime"))]
  VtFiletime(crate::schemas::vt::VtFileTime),
  /// Boolean.
  #[sdk(text_child(simple_type = "BooleanValue", qname = "xsd:boolean/vt:bool"))]
  VtBool(crate::schemas::vt::VtBool),
  /// Currency.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:ST_Cy/vt:cy"))]
  VtCy(crate::schemas::vt::VtCurrency),
  /// Error Status Code.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:ST_Error/vt:error"))]
  VtError(crate::schemas::vt::VtError),
  /// Binary Stream.
  #[sdk(text_child(
    simple_type = "Base64BinaryValue",
    qname = "xsd:base64Binary/vt:stream"
  ))]
  VtStream(crate::schemas::vt::VtStreamData),
  /// Binary Stream Object.
  #[sdk(text_child(
    simple_type = "Base64BinaryValue",
    qname = "xsd:base64Binary/vt:ostream"
  ))]
  VtOstream(crate::schemas::vt::VtoStreamData),
  /// Binary Storage.
  #[sdk(text_child(
    simple_type = "Base64BinaryValue",
    qname = "xsd:base64Binary/vt:storage"
  ))]
  VtStorage(crate::schemas::vt::VtStorage),
  /// Binary Storage Object.
  #[sdk(text_child(
    simple_type = "Base64BinaryValue",
    qname = "xsd:base64Binary/vt:ostorage"
  ))]
  VtOstorage(crate::schemas::vt::VtoStorage),
  /// Binary Versioned Stream.
  #[sdk(child(qname = "vt:CT_Vstream/vt:vstream"))]
  VtVstream(std::boxed::Box<crate::schemas::vt::VtvStreamData>),
  /// Class ID.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:ST_Clsid/vt:clsid"))]
  VtClsid(crate::schemas::vt::VtClassId),
  /// Clipboard Data.
  #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
  VtCf(std::boxed::Box<crate::schemas::vt::VtClipboardData>),
}
