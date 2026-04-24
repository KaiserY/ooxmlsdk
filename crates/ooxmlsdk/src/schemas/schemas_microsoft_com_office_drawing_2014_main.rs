//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CreationId Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is a16:creationId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_CreationId/a16:creationId")]
pub struct CreationId {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Defines the PredecessorDrawingElementReference Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is a16:predDERef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_PredecessorDrawingElementReference/a16:predDERef")]
pub struct PredecessorDrawingElementReference {
  /// pred
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pred
  #[sdk(attr(qname = ":pred"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub pred: Option<crate::simple_type::StringValue>,
}
/// Defines the ConnectableReferences Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is a16:cxnDERefs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_ConnectableReferences/a16:cxnDERefs")]
pub struct ConnectableReferences {
  /// st
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub st: Option<crate::simple_type::StringValue>,
  /// end
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub end: Option<crate::simple_type::StringValue>,
}
/// Defines the RowIdIdentifier Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is a16:rowId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_Identifier/a16:rowId")]
pub struct RowIdIdentifier {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ColIdIdentifier Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is a16:colId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_Identifier/a16:colId")]
pub struct ColIdIdentifier {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the OpenXmlIdentifierElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a16:CT_Identifier/")]
pub struct OpenXmlIdentifierElement {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
