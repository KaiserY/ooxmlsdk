//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Locked Canvas Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShape/lc:lockedCanvas")]
pub struct LockedCanvas {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties:
    std::boxed::Box<crate::schemas::a::NonVisualGroupShapeProperties>,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/a:grpSpPr"))]
  pub visual_group_shape_properties: std::boxed::Box<crate::schemas::a::VisualGroupShapeProperties>,
  #[sdk(choice(
    qname = "a:CT_GvmlTextShape/a:txSp",
    qname = "a:CT_GvmlShape/a:sp",
    qname = "a:CT_GvmlConnector/a:cxnSp",
    qname = "a:CT_GvmlPicture/a:pic",
    qname = "a14:CT_GvmlContentPart/a14:contentPart",
    qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame",
    qname = "a:CT_GvmlGroupShape/a:grpSp"
  ))]
  pub choice: Vec<LockedCanvasChoice>,
  /// Defines the GvmlGroupShapeExtensionList Class.
  #[sdk(child(qname = "a:CT_GvmlGroupShapeExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::GvmlGroupShapeExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LockedCanvasChoice {
  /// Text Shape.
  #[sdk(child(qname = "a:CT_GvmlTextShape/a:txSp"))]
  TxSp(std::boxed::Box<crate::schemas::a::TextShape>),
  /// Shape.
  #[sdk(child(qname = "a:CT_GvmlShape/a:sp"))]
  Sp(std::boxed::Box<crate::schemas::a::Shape>),
  /// Connection Shape.
  #[sdk(child(qname = "a:CT_GvmlConnector/a:cxnSp"))]
  CxnSp(std::boxed::Box<crate::schemas::a::ConnectionShape>),
  /// Picture.
  #[sdk(child(qname = "a:CT_GvmlPicture/a:pic"))]
  Pic(std::boxed::Box<crate::schemas::a::Picture>),
  /// Defines the GvmlContentPart Class.
  #[sdk(child(office2010, qname = "a14:CT_GvmlContentPart/a14:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::a14::GvmlContentPart>),
  /// Graphic Frame.
  #[sdk(child(qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame"))]
  GraphicFrame(std::boxed::Box<crate::schemas::a::GraphicFrame>),
  /// Group shape.
  #[sdk(child(qname = "a:CT_GvmlGroupShape/a:grpSp"))]
  GrpSp(std::boxed::Box<crate::schemas::a::GroupShape>),
}
