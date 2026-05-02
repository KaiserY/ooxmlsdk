//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlrvrel:CT_RichValueRels/xlrvrel:richValueRels")]
pub struct RichValueRels {
  /// Defines the RichValueRelRelationship Class.
  #[sdk(child(
    microsoft365,
    qname = "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel"
  ))]
  pub xlrvrel_rel: Vec<RichValueRelRelationship>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xlrvrel:extLst"))]
  pub xlrvrel_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValueRelRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel"
)]
pub struct RichValueRelRelationship {
  /// id
  #[sdk(attr(microsoft365, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "x:CT_ExtensionList/xlrvrel:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
