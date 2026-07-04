//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom File Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, xml_header, qname = "op:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Custom File Property.
  #[sdk(child(qname = "op:property"))]
  pub custom_document_property: Vec<CustomDocumentProperty>,
}
/// Custom File Property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "op:property")]
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
            text_child(
                variant = VtBlob,
                simple_type = "Base64BinaryValue",
                qname = "vt:blob"
            ),
            text_child(
                variant = VtoBlob,
                simple_type = "Base64BinaryValue",
                qname = "vt:oblob"
            ),
            empty_child(variant = VtEmpty, qname = "vt:empty"),
            empty_child(variant = VtNull, qname = "vt:null"),
            text_child(variant = VtByte, simple_type = "SByteValue", qname = "vt:i1"),
            text_child(variant = VtShort, simple_type = "Int16Value", qname = "vt:i2"),
            text_child(variant = VtInt32, simple_type = "Int32Value", qname = "vt:i4"),
            text_child(variant = VtInt64, simple_type = "Int64Value", qname = "vt:i8"),
            text_child(
                variant = VtInteger,
                simple_type = "Int32Value",
                qname = "vt:int"
            ),
            text_child(
                variant = VtUnsignedByte,
                simple_type = "ByteValue",
                qname = "vt:ui1"
            ),
            text_child(
                variant = VtUnsignedShort,
                simple_type = "UInt16Value",
                qname = "vt:ui2"
            ),
            text_child(
                variant = VtUnsignedInt32,
                simple_type = "UInt32Value",
                qname = "vt:ui4"
            ),
            text_child(
                variant = VtUnsignedInt64,
                simple_type = "UInt64Value",
                qname = "vt:ui8"
            ),
            text_child(
                variant = VtUnsignedInteger,
                simple_type = "UInt32Value",
                qname = "vt:uint"
            ),
            text_child(variant = VtFloat, simple_type = "SingleValue", qname = "vt:r4"),
            text_child(variant = VtDouble, simple_type = "DoubleValue", qname = "vt:r8"),
            text_child(
                variant = VtDecimal,
                simple_type = "DecimalValue",
                qname = "vt:decimal"
            ),
            text_child(variant = Vtlpstr, qname = "vt:lpstr"),
            text_child(variant = Vtlpwstr, qname = "vt:lpwstr"),
            text_child(variant = VtbString, qname = "vt:bstr"),
            text_child(
                variant = VtDate,
                simple_type = "DateTimeValue",
                qname = "vt:date"
            ),
            text_child(
                variant = VtFileTime,
                simple_type = "DateTimeValue",
                qname = "vt:filetime"
            ),
            text_child(
                variant = VtBool,
                simple_type = "BooleanValue",
                qname = "vt:bool"
            ),
            text_child(variant = VtCurrency, qname = "vt:cy"),
            text_child(variant = VtError, qname = "vt:error"),
            text_child(
                variant = VtStreamData,
                simple_type = "Base64BinaryValue",
                qname = "vt:stream"
            ),
            text_child(
                variant = VtoStreamData,
                simple_type = "Base64BinaryValue",
                qname = "vt:ostream"
            ),
            text_child(
                variant = VtStorage,
                simple_type = "Base64BinaryValue",
                qname = "vt:storage"
            ),
            text_child(
                variant = VtoStorage,
                simple_type = "Base64BinaryValue",
                qname = "vt:ostorage"
            ),
            child(variant = VtvStreamData, qname = "vt:vstream"),
            text_child(variant = VtClassId, qname = "vt:clsid"),
            child(variant = VtClipboardData, qname = "vt:cf")
        )
    )]
  pub custom_document_property_choice: Option<CustomDocumentPropertyChoice>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomDocumentPropertyChoice {
  /// Vector.
  VtVector(std::boxed::Box<crate::schemas::vt::VtVector>),
  /// Array.
  VtArray(std::boxed::Box<crate::schemas::vt::VtArray>),
  /// Binary Blob.
  VtBlob(crate::schemas::vt::VtBlob),
  /// Binary Blob Object.
  VtoBlob(crate::schemas::vt::VtoBlob),
  /// Empty.
  VtEmpty,
  /// Null.
  VtNull,
  /// 1-Byte Signed Integer.
  VtByte(crate::schemas::vt::VtByte),
  /// 2-Byte Signed Integer.
  VtShort(crate::schemas::vt::VtShort),
  /// 4-Byte Signed Integer.
  VtInt32(crate::schemas::vt::VtInt32),
  /// 8-Byte Signed Integer.
  VtInt64(crate::schemas::vt::VtInt64),
  /// Integer.
  VtInteger(crate::schemas::vt::VtInteger),
  /// 1-Byte Unsigned Integer.
  VtUnsignedByte(crate::schemas::vt::VtUnsignedByte),
  /// 2-Byte Unsigned Integer.
  VtUnsignedShort(crate::schemas::vt::VtUnsignedShort),
  /// 4-Byte Unsigned Integer.
  VtUnsignedInt32(crate::schemas::vt::VtUnsignedInt32),
  /// 8-Byte Unsigned Integer.
  VtUnsignedInt64(crate::schemas::vt::VtUnsignedInt64),
  /// Unsigned Integer.
  VtUnsignedInteger(crate::schemas::vt::VtUnsignedInteger),
  /// 4-Byte Real Number.
  VtFloat(crate::schemas::vt::VtFloat),
  /// 8-Byte Real Number.
  VtDouble(crate::schemas::vt::VtDouble),
  /// Decimal.
  VtDecimal(crate::schemas::vt::VtDecimal),
  /// LPSTR.
  Vtlpstr(crate::schemas::vt::Vtlpstr),
  /// LPWSTR.
  Vtlpwstr(crate::schemas::vt::Vtlpwstr),
  /// Basic String.
  VtbString(crate::schemas::vt::VtbString),
  /// Date and Time.
  VtDate(crate::schemas::vt::VtDate),
  /// File Time.
  VtFileTime(crate::schemas::vt::VtFileTime),
  /// Boolean.
  VtBool(crate::schemas::vt::VtBool),
  /// Currency.
  VtCurrency(crate::schemas::vt::VtCurrency),
  /// Error Status Code.
  VtError(crate::schemas::vt::VtError),
  /// Binary Stream.
  VtStreamData(crate::schemas::vt::VtStreamData),
  /// Binary Stream Object.
  VtoStreamData(crate::schemas::vt::VtoStreamData),
  /// Binary Storage.
  VtStorage(crate::schemas::vt::VtStorage),
  /// Binary Storage Object.
  VtoStorage(crate::schemas::vt::VtoStorage),
  /// Binary Versioned Stream.
  VtvStreamData(std::boxed::Box<crate::schemas::vt::VtvStreamData>),
  /// Class ID.
  VtClassId(crate::schemas::vt::VtClassId),
  /// Clipboard Data.
  VtClipboardData(std::boxed::Box<crate::schemas::vt::VtClipboardData>),
}
