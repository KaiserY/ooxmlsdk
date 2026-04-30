//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlrvrel:CT_RichValueRels/xlrvrel:richValueRels")]
pub struct RichValueRels {
  /// _
  #[sdk(child(
    microsoft365,
    qname = "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel"
  ))]
  pub xlrvrel_rel: Vec<RichValueRelRelationship>,
  /// _
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
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
