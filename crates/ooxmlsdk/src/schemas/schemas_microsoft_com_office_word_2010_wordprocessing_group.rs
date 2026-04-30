//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:wgp")]
pub struct WordprocessingGroup {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
  ))]
  pub non_visual_group_drawing_shape_properties:
    std::boxed::Box<NonVisualGroupDrawingShapeProperties>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice(
    qname = "wps:CT_WordprocessingShape/wps:wsp",
    qname = "wpg:CT_WordprocessingGroup/wpg:grpSp",
    qname = "wpg:CT_GraphicFrame/wpg:graphicFrame",
    qname = "pic:CT_Picture/pic:pic",
    qname = "w14:CT_WordContentPart/w14:contentPart"
  ))]
  pub wordprocessing_group_choice: Vec<WordprocessingGroupChoice>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub wpg_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the GroupShape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:grpSp")]
pub struct GroupShape {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
  ))]
  pub non_visual_group_drawing_shape_properties:
    std::boxed::Box<NonVisualGroupDrawingShapeProperties>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice(
    qname = "wps:CT_WordprocessingShape/wps:wsp",
    qname = "wpg:CT_WordprocessingGroup/wpg:grpSp",
    qname = "wpg:CT_GraphicFrame/wpg:graphicFrame",
    qname = "pic:CT_Picture/pic:pic",
    qname = "w14:CT_WordContentPart/w14:contentPart"
  ))]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub wpg_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr")]
pub struct NonVisualDrawingProperties {
    pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    /// Hyperlink associated with hovering over the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    /// Future extension
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualGraphicFrameProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr")]
pub struct NonVisualGraphicFrameProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrameLocks,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/wpg:xfrm")]
pub struct Transform2D {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the NonVisualGroupDrawingShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr"
)]
pub struct NonVisualGroupDrawingShapeProperties {
    /// _
    #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
    pub group_shape_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Defines the GroupShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_GroupShapeProperties/wpg:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup>,
  >,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the GraphicFrame Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_GraphicFrame/wpg:graphicFrame")]
pub struct GraphicFrame {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_Transform2D/wpg:xfrm"))]
  pub transform2_d: std::boxed::Box<Transform2D>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingGroupChoice {
  #[sdk(child(office2010, qname = "wps:CT_WordprocessingShape/wps:wsp"))]
    WpsWsp(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
        >,
    ),
    #[sdk(child(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:grpSp"))]
    WpgGrpSp(std::boxed::Box<GroupShape>),
    #[sdk(child(office2010, qname = "wpg:CT_GraphicFrame/wpg:graphicFrame"))]
    WpgGraphicFrame(std::boxed::Box<GraphicFrame>),
    #[sdk(child(qname = "pic:CT_Picture/pic:pic"))]
    PicPic(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture,
        >,
    ),
    #[sdk(child(office2010, qname = "w14:CT_WordContentPart/w14:contentPart"))]
    W14ContentPart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(office2010, qname = "wps:CT_WordprocessingShape/wps:wsp"))]
    WpsWsp(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
        >,
    ),
    #[sdk(child(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:grpSp"))]
    WpgGrpSp(std::boxed::Box<GroupShape>),
    #[sdk(child(office2010, qname = "wpg:CT_GraphicFrame/wpg:graphicFrame"))]
    WpgGraphicFrame(std::boxed::Box<GraphicFrame>),
    #[sdk(child(qname = "pic:CT_Picture/pic:pic"))]
    PicPic(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture,
        >,
    ),
    #[sdk(child(office2010, qname = "w14:CT_WordContentPart/w14:contentPart"))]
    W14ContentPart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
