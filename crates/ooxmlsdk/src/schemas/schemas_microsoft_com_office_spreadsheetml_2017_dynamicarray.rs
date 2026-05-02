//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DynamicArrayProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xda:CT_DynamicArrayProperties/xda:dynamicArrayProperties"
)]
pub struct DynamicArrayProperties {
  /// fDynamic
  #[sdk(attr(office2019, qname = ":fDynamic"))]
  pub f_dynamic: Option<crate::simple_type::BooleanValue>,
  /// fCollapsed
  #[sdk(attr(office2019, qname = ":fCollapsed"))]
  pub f_collapsed: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xda:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "x:CT_ExtensionList/xda:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
