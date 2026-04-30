//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Locked Canvas Container.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is lc:lockedCanvas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShape/lc:lockedCanvas")]
pub struct LockedCanvas {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupShapeProperties,
  >,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/a:grpSpPr"))]
  pub visual_group_shape_properties: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::VisualGroupShapeProperties,
  >,
  #[sdk(choice(
    qname = "a:CT_GvmlTextShape/a:txSp",
    qname = "a:CT_GvmlShape/a:sp",
    qname = "a:CT_GvmlConnector/a:cxnSp",
    qname = "a:CT_GvmlPicture/a:pic",
    qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame",
    qname = "a:CT_GvmlGroupShape/a:grpSp",
    text,
    any
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "a14:CT_GvmlContentPart/a14:contentPart"))
  )]
  pub locked_canvas_choice: Vec<LockedCanvasChoice>,
  /// _
  #[sdk(child(qname = "a:CT_GvmlGroupShapeExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GvmlGroupShapeExtensionList,
  >,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LockedCanvasChoice {
  #[sdk(child(qname = "a:CT_GvmlTextShape/a:txSp"))]
  ATxSp(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextShape>),
  #[sdk(child(qname = "a:CT_GvmlShape/a:sp"))]
  ASp(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape>),
  #[sdk(child(qname = "a:CT_GvmlConnector/a:cxnSp"))]
  ACxnSp(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShape,
    >,
  ),
  #[sdk(child(qname = "a:CT_GvmlPicture/a:pic"))]
  APic(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Picture>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "a14:CT_GvmlContentPart/a14:contentPart"))]
  A14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::GvmlContentPart,
    >,
  ),
  #[sdk(child(qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame"))]
  AGraphicFrame(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrame>,
  ),
  #[sdk(child(qname = "a:CT_GvmlGroupShape/a:grpSp"))]
  AGrpSp(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShape>,
  ),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
