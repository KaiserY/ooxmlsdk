//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Locked Canvas Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShape/lc:lockedCanvas")]
pub struct LockedCanvas {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties:
    Option<std::boxed::Box<crate::schemas::a::NonVisualGroupShapeProperties>>,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/a:grpSpPr"))]
  pub visual_group_shape_properties:
    Option<std::boxed::Box<crate::schemas::a::VisualGroupShapeProperties>>,
  #[sdk(choice(
    qname = "a:CT_GvmlTextShape/a:txSp",
    qname = "a:CT_GvmlShape/a:sp",
    qname = "a:CT_GvmlConnector/a:cxnSp",
    qname = "a:CT_GvmlPicture/a:pic",
    qname = "a14:CT_GvmlContentPart/a14:contentPart",
    qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame",
    qname = "a:CT_GvmlGroupShape/a:grpSp",
    text,
    any
  ))]
  pub locked_canvas_choice: Vec<LockedCanvasChoice>,
  /// Defines the GvmlGroupShapeExtensionList Class.
  #[sdk(child(qname = "a:CT_GvmlGroupShapeExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::GvmlGroupShapeExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LockedCanvasChoice {
  #[sdk(child(qname = "a:CT_GvmlTextShape/a:txSp"))]
  ATxSp(std::boxed::Box<crate::schemas::a::TextShape>),
  #[sdk(child(qname = "a:CT_GvmlShape/a:sp"))]
  ASp(std::boxed::Box<crate::schemas::a::Shape>),
  #[sdk(child(qname = "a:CT_GvmlConnector/a:cxnSp"))]
  ACxnSp(std::boxed::Box<crate::schemas::a::ConnectionShape>),
  #[sdk(child(qname = "a:CT_GvmlPicture/a:pic"))]
  APic(std::boxed::Box<crate::schemas::a::Picture>),
  #[sdk(child(office2010, qname = "a14:CT_GvmlContentPart/a14:contentPart"))]
  A14ContentPart(std::boxed::Box<crate::schemas::a14::GvmlContentPart>),
  #[sdk(child(qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame"))]
  AGraphicFrame(std::boxed::Box<crate::schemas::a::GraphicFrame>),
  #[sdk(child(qname = "a:CT_GvmlGroupShape/a:grpSp"))]
  AGrpSp(std::boxed::Box<crate::schemas::a::GroupShape>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
