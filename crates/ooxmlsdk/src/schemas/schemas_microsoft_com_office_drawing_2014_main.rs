//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CreationId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a16:CT_CreationId/a16:creationId")]
pub struct CreationId {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Defines the PredecessorDrawingElementReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "a16:CT_PredecessorDrawingElementReference/a16:predDERef"
)]
pub struct PredecessorDrawingElementReference {
  /// pred
  #[sdk(attr(office2016, qname = ":pred"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub pred: Option<crate::simple_type::StringValue>,
}
/// Defines the ConnectableReferences Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a16:CT_ConnectableReferences/a16:cxnDERefs")]
pub struct ConnectableReferences {
  /// st
  #[sdk(attr(office2016, qname = ":st"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub st: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(office2016, qname = ":end"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub end: Option<crate::simple_type::StringValue>,
}
/// Defines the RowIdIdentifier Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a16:CT_Identifier/a16:rowId")]
pub struct RowIdIdentifier {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ColIdIdentifier Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a16:CT_Identifier/a16:colId")]
pub struct ColIdIdentifier {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
