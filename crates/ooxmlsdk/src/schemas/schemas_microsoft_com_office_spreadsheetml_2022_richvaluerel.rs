//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRels Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvrel:richValueRels.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvrel:CT_RichValueRels/xlrvrel:richValueRels")]
pub struct RichValueRels {
  /// _
  #[sdk(child(qname = "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel"))]
  pub xlrvrel_rel: Vec<RichValueRelRelationship>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrvrel:extLst"))]
  pub xlrvrel_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValueRelRelationship Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvrel:rel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel")]
pub struct RichValueRelRelationship {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvrel:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xlrvrel:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
