//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DesignerTagList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p202:designTagLst")]
pub struct DesignerTagList {
  /// Defines the DesignerTag Class.
  #[sdk(child(office2021, qname = "p202:designTag"))]
  pub designer_tag: Vec<DesignerTag>,
}
/// Defines the DesignerDrawingProps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p202:designPr")]
pub struct DesignerDrawingProps {
  /// edtDesignElem
  #[sdk(attr(office2021, qname = ":edtDesignElem"))]
  pub edt_design_elem: Option<crate::simple_type::BooleanValue>,
  /// Defines the DesignerTagList Class.
  #[sdk(child(office2021, qname = "p202:designTagLst"))]
  pub designer_tag_list: Option<DesignerTagList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "p202:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DesignerTag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p202:designTag")]
pub struct DesignerTag {
  /// name
  #[sdk(attr(office2021, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// val
  #[sdk(attr(office2021, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p202:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
