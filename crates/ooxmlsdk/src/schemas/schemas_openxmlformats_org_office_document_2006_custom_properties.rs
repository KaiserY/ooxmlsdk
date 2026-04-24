//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom File Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is op:Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:CT_Properties/op:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "op:CT_Property/op:property"))]
  pub op_property: Vec<CustomDocumentProperty>,
}
/// Custom File Property.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is op:property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "op:CT_Property/op:property")]
pub struct CustomDocumentProperty {
  /// Format ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fmtid
  #[sdk(attr(qname = ":fmtid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\s*\\{[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\\}\\s*"
  ))]
  pub format_id: crate::simple_type::StringValue,
  /// Property ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pid
  #[sdk(attr(qname = ":pid"))]
  pub property_id: crate::simple_type::Int32Value,
  /// Custom File Property Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Bookmark Link Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :linkTarget
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
  pub xml_children: Option<CustomDocumentPropertyChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomDocumentPropertyChoice {
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
    VtVector(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
        >,
    ),
    #[sdk(child(qname = "vt:CT_Array/vt:array"))]
    VtArray(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtArray,
        >,
    ),
    #[sdk(text_child(qname = "xsd:base64Binary/vt:blob"))]
    VtBlob(crate::simple_type::Base64BinaryValue),
    #[sdk(text_child(qname = "xsd:base64Binary/vt:oblob"))]
    VtOblob(crate::simple_type::Base64BinaryValue),
    #[sdk(child(qname = "vt:CT_Empty/vt:empty"))]
    VtEmpty(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtEmpty,
        >,
    ),
    #[sdk(child(qname = "vt:CT_Null/vt:null"))]
    VtNull(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtNull,
        >,
    ),
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
    VtVstream(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtvStreamData,
        >,
    ),
    #[sdk(text_child(qname = "vt:ST_Clsid/vt:clsid"))]
    VtClsid(crate::simple_type::StringValue),
    #[sdk(child(qname = "vt:CT_Cf/vt:cf"))]
    VtCf(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtClipboardData,
        >,
    ),
}
