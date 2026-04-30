//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DesignerTagList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p202:designTagLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p202:CT_DesignerTagList/p202:designTagLst")]
pub struct DesignerTagList {
  /// _
  #[sdk(child(qname = "p202:CT_DesignerTag/p202:designTag"))]
  pub p202_design_tag: Vec<DesignerTag>,
}
/// Defines the DesignerDrawingProps Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p202:designPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p202:CT_DesignerDrawingProps/p202:designPr")]
pub struct DesignerDrawingProps {
  /// edtDesignElem
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :edtDesignElem
  #[sdk(attr(qname = ":edtDesignElem"))]
  pub edt_design_elem: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p202:CT_DesignerTagList/p202:designTagLst"))]
  pub designer_tag_list: Option<DesignerTagList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p202:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DesignerTag Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p202:designTag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p202:CT_DesignerTag/p202:designTag")]
pub struct DesignerTag {
  /// name
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// val
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p202:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p202:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
