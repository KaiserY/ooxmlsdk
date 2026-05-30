//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom File Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Custom File Property.
  #[sdk(child(qname = "op:property"))]
  pub custom_document_property: Vec<CustomDocumentProperty>,
}
/// Custom File Property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:property")]
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
  #[sdk(
        choice(
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
  pub custom_document_property_choice: Option<CustomDocumentPropertyChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomDocumentPropertyChoice {
  /// Vector.
  VtVector(std::boxed::Box<crate::schemas::vt::VtVector>),
  /// Array.
  VtArray(std::boxed::Box<crate::schemas::vt::VtArray>),
  /// Binary Blob.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:blob"))]
  VtBlob(crate::schemas::vt::VtBlob),
  /// Binary Blob Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:oblob"))]
  VtoBlob(crate::schemas::vt::VtoBlob),
  /// Empty.
  #[sdk(empty_child(qname = "vt:empty"))]
  VtEmpty,
  /// Null.
  #[sdk(empty_child(qname = "vt:null"))]
  VtNull,
  /// 1-Byte Signed Integer.
  #[sdk(text_child(simple_type = "SByteValue", qname = "vt:i1"))]
  VtByte(crate::schemas::vt::VtByte),
  /// 2-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int16Value", qname = "vt:i2"))]
  VtShort(crate::schemas::vt::VtShort),
  /// 4-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:i4"))]
  VtInt32(crate::schemas::vt::VtInt32),
  /// 8-Byte Signed Integer.
  #[sdk(text_child(simple_type = "Int64Value", qname = "vt:i8"))]
  VtInt64(crate::schemas::vt::VtInt64),
  /// Integer.
  #[sdk(text_child(simple_type = "Int32Value", qname = "vt:int"))]
  VtInteger(crate::schemas::vt::VtInteger),
  /// 1-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "ByteValue", qname = "vt:ui1"))]
  VtUnsignedByte(crate::schemas::vt::VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt16Value", qname = "vt:ui2"))]
  VtUnsignedShort(crate::schemas::vt::VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:ui4"))]
  VtUnsignedInt32(crate::schemas::vt::VtUnsignedInt32),
  /// 8-Byte Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt64Value", qname = "vt:ui8"))]
  VtUnsignedInt64(crate::schemas::vt::VtUnsignedInt64),
  /// Unsigned Integer.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "vt:uint"))]
  VtUnsignedInteger(crate::schemas::vt::VtUnsignedInteger),
  /// 4-Byte Real Number.
  #[sdk(text_child(simple_type = "SingleValue", qname = "vt:r4"))]
  VtFloat(crate::schemas::vt::VtFloat),
  /// 8-Byte Real Number.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "vt:r8"))]
  VtDouble(crate::schemas::vt::VtDouble),
  /// Decimal.
  #[sdk(text_child(simple_type = "DecimalValue", qname = "vt:decimal"))]
  VtDecimal(crate::schemas::vt::VtDecimal),
  /// LPSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpstr"))]
  Vtlpstr(crate::schemas::vt::Vtlpstr),
  /// LPWSTR.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:lpwstr"))]
  Vtlpwstr(crate::schemas::vt::Vtlpwstr),
  /// Basic String.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:bstr"))]
  VtbString(crate::schemas::vt::VtbString),
  /// Date and Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:date"))]
  VtDate(crate::schemas::vt::VtDate),
  /// File Time.
  #[sdk(text_child(simple_type = "DateTimeValue", qname = "vt:filetime"))]
  VtFileTime(crate::schemas::vt::VtFileTime),
  /// Boolean.
  #[sdk(text_child(simple_type = "BooleanValue", qname = "vt:bool"))]
  VtBool(crate::schemas::vt::VtBool),
  /// Currency.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:cy"))]
  VtCurrency(crate::schemas::vt::VtCurrency),
  /// Error Status Code.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:error"))]
  VtError(crate::schemas::vt::VtError),
  /// Binary Stream.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:stream"))]
  VtStreamData(crate::schemas::vt::VtStreamData),
  /// Binary Stream Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:ostream"))]
  VtoStreamData(crate::schemas::vt::VtoStreamData),
  /// Binary Storage.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:storage"))]
  VtStorage(crate::schemas::vt::VtStorage),
  /// Binary Storage Object.
  #[sdk(text_child(simple_type = "Base64BinaryValue", qname = "vt:ostorage"))]
  VtoStorage(crate::schemas::vt::VtoStorage),
  /// Binary Versioned Stream.
  VtvStreamData(std::boxed::Box<crate::schemas::vt::VtvStreamData>),
  /// Class ID.
  #[sdk(text_child(simple_type = "StringValue", qname = "vt:clsid"))]
  VtClassId(crate::schemas::vt::VtClassId),
  /// Clipboard Data.
  VtClipboardData(std::boxed::Box<crate::schemas::vt::VtClipboardData>),
}
