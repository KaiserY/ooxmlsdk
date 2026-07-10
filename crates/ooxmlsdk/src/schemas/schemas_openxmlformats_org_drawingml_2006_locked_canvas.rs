//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Locked Canvas Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "lc:lockedCanvas")]
pub struct LockedCanvas {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties:
    std::boxed::Box<crate::schemas::a::NonVisualGroupShapeProperties>,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:grpSpPr"))]
  pub visual_group_shape_properties: std::boxed::Box<crate::schemas::a::VisualGroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = TextShape, boxed, qname = "a:txSp"),
            child(variant = Shape, boxed, qname = "a:sp"),
            child(variant = ConnectionShape, boxed, qname = "a:cxnSp"),
            child(variant = Picture, boxed, qname = "a:pic"),
            child(variant = GvmlContentPart, boxed, qname = "a14:contentPart"),
            child(variant = GraphicFrame, boxed, qname = "a:graphicFrame"),
            child(variant = GroupShape, boxed, qname = "a:grpSp")
        )
    )]
  pub locked_canvas_choice: Vec<LockedCanvasChoice>,
  /// Defines the GvmlGroupShapeExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub gvml_group_shape_extension_list: Option<crate::schemas::a::GvmlGroupShapeExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum LockedCanvasChoice {
  TextShape(std::boxed::Box<crate::schemas::a::TextShape>),
  Shape(std::boxed::Box<crate::schemas::a::Shape>),
  ConnectionShape(std::boxed::Box<crate::schemas::a::ConnectionShape>),
  Picture(std::boxed::Box<crate::schemas::a::Picture>),
  GvmlContentPart(std::boxed::Box<crate::schemas::a14::GvmlContentPart>),
  GraphicFrame(std::boxed::Box<crate::schemas::a::GraphicFrame>),
  GroupShape(std::boxed::Box<crate::schemas::a::GroupShape>),
}
