//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:wgp")]
pub struct WordprocessingGroup {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// Defines the NonVisualGroupDrawingShapeProperties Class.
  #[sdk(child(
    office2010,
    qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
  ))]
  pub non_visual_group_drawing_shape_properties:
    std::boxed::Box<NonVisualGroupDrawingShapeProperties>,
  /// Defines the GroupShapeProperties Class.
  #[sdk(child(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = WordprocessingShape, qname = "wps:wsp"),
            child(variant = GroupShape, qname = "wpg:grpSp"),
            child(variant = GraphicFrame, qname = "wpg:graphicFrame"),
            child(variant = Picture, qname = "pic:pic"),
            child(variant = ContentPart, qname = "w14:contentPart")
        )
    )]
  pub wordprocessing_group_choice: Vec<WordprocessingGroupChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GroupShape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:grpSp")]
pub struct GroupShape {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// Defines the NonVisualGroupDrawingShapeProperties Class.
  #[sdk(child(
    office2010,
    qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
  ))]
  pub non_visual_group_drawing_shape_properties:
    std::boxed::Box<NonVisualGroupDrawingShapeProperties>,
  /// Defines the GroupShapeProperties Class.
  #[sdk(child(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = WordprocessingShape, qname = "wps:wsp"),
            child(variant = GroupShape, qname = "wpg:grpSp"),
            child(variant = GraphicFrame, qname = "wpg:graphicFrame"),
            child(variant = Picture, qname = "pic:pic"),
            child(variant = ContentPart, qname = "w14:contentPart")
        )
    )]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr")]
pub struct NonVisualDrawingProperties {
  /// Application defined unique identifier.
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name compatible with Object Model (non-unique).
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Description of the drawing element.
  #[sdk(attr(qname = ":descr"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Flag determining to show or hide this element.
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Hyperlink associated with clicking or selecting the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Defines the NonVisualGraphicFrameProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr")]
pub struct NonVisualGraphicFrameProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<crate::schemas::a::GraphicFrameLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/wpg:xfrm")]
pub struct Transform2D {
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  /// Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the NonVisualGroupDrawingShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
)]
pub struct NonVisualGroupDrawingShapeProperties {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<crate::schemas::a::GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<crate::schemas::a::NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Defines the GroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<crate::schemas::a::TransformGroup>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the GraphicFrame Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_GraphicFrame/wpg:graphicFrame")]
pub struct GraphicFrame {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Defines the NonVisualGraphicFrameProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// Defines the Transform2D Class.
  #[sdk(child(office2010, qname = "a:CT_Transform2D/wpg:xfrm"))]
  pub transform2_d: std::boxed::Box<Transform2D>,
  /// Graphic Object.
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingGroupChoice {
  WordprocessingShape(std::boxed::Box<crate::schemas::wps::WordprocessingShape>),
  /// Defines the GroupShape Class.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Defines the GraphicFrame Class.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  Picture(std::boxed::Box<crate::schemas::pic::Picture>),
  ContentPart(std::boxed::Box<crate::schemas::w14::ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  WordprocessingShape(std::boxed::Box<crate::schemas::wps::WordprocessingShape>),
  /// Defines the GroupShape Class.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Defines the GraphicFrame Class.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  Picture(std::boxed::Box<crate::schemas::pic::Picture>),
  ContentPart(std::boxed::Box<crate::schemas::w14::ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
