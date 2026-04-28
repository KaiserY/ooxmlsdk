//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DynamicArrayProperties Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xda:dynamicArrayProperties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xda:CT_DynamicArrayProperties/xda:dynamicArrayProperties")]
pub struct DynamicArrayProperties {
  /// fDynamic
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :fDynamic
  #[sdk(attr(qname = ":fDynamic"))]
  pub f_dynamic: Option<crate::simple_type::BooleanValue>,
  /// fCollapsed
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :fCollapsed
  #[sdk(attr(qname = ":fCollapsed"))]
  pub f_collapsed: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xda:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xda:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xda:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
