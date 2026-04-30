//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Relative Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_RelSizeAnchor/cdr:relSizeAnchor")]
pub struct RelativeAnchorSize {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Starting Anchor Point
  #[sdk(child(qname = "cdr:CT_Marker/cdr:from"))]
  pub from_anchor: std::boxed::Box<FromAnchor>,
  /// Ending Anchor Point
  #[sdk(child(qname = "cdr:CT_Marker/cdr:to"))]
  pub to_anchor: std::boxed::Box<ToAnchor>,
  #[sdk(choice(
    qname = "cdr:CT_Shape/cdr:sp",
    qname = "cdr:CT_GroupShape/cdr:grpSp",
    qname = "cdr:CT_GraphicFrame/cdr:graphicFrame",
    qname = "cdr:CT_Connector/cdr:cxnSp",
    qname = "cdr:CT_Picture/cdr:pic",
    qname = "cdr14:CT_ContentPart/cdr14:contentPart"
  ))]
  pub relative_anchor_size_choice: Option<RelativeAnchorSizeChoice>,
}
/// Absolute Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_AbsSizeAnchor/cdr:absSizeAnchor")]
pub struct AbsoluteAnchorSize {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// _
  #[sdk(child(qname = "cdr:CT_Marker/cdr:from"))]
  pub from_anchor: std::boxed::Box<FromAnchor>,
  /// Shape Extent
  #[sdk(child(qname = "a:CT_PositiveSize2D/cdr:ext"))]
  pub extent: std::boxed::Box<Extent>,
  #[sdk(choice(
    qname = "cdr:CT_Shape/cdr:sp",
    qname = "cdr:CT_GroupShape/cdr:grpSp",
    qname = "cdr:CT_GraphicFrame/cdr:graphicFrame",
    qname = "cdr:CT_Connector/cdr:cxnSp",
    qname = "cdr:CT_Picture/cdr:pic",
    qname = "cdr14:CT_ContentPart/cdr14:contentPart"
  ))]
  pub absolute_anchor_size_choice: Option<AbsoluteAnchorSizeChoice>,
}
/// Shape Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Shape/cdr:sp")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Text Link
  #[sdk(attr(qname = ":textlink"))]
  pub text_link: Option<crate::simple_type::StringValue>,
  /// Lock Text
  #[sdk(attr(qname = ":fLocksText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// Publish to Server
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Shape Properties
  #[sdk(child(qname = "cdr:CT_ShapeNonVisual/cdr:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/cdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/cdr:style"))]
  pub style: Option<std::boxed::Box<Style>>,
  /// Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/cdr:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
}
/// Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_GroupShape/cdr:grpSp")]
pub struct GroupShape {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Group Shape Properties
  #[sdk(child(qname = "cdr:CT_GroupShapeNonVisual/cdr:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/cdr:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice(
    qname = "cdr:CT_Shape/cdr:sp",
    qname = "cdr:CT_GroupShape/cdr:grpSp",
    qname = "cdr:CT_GraphicFrame/cdr:graphicFrame",
    qname = "cdr:CT_Connector/cdr:cxnSp",
    qname = "cdr:CT_Picture/cdr:pic",
    qname = "cdr14:CT_ContentPart/cdr14:contentPart",
    text,
    any
  ))]
  pub group_shape_choice: Vec<GroupShapeChoice>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_GraphicFrame/cdr:graphicFrame")]
pub struct GraphicFrame {
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish To Server
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Graphic Frame Properties
  #[sdk(child(qname = "cdr:CT_GraphicFrameNonVisual/cdr:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// Graphic Frame Transform
  #[sdk(child(qname = "a:CT_Transform2D/cdr:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// Graphical Object
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Connector/cdr:cxnSp")]
pub struct ConnectionShape {
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Connector Non Visual Properties
  #[sdk(child(qname = "cdr:CT_ConnectorNonVisual/cdr:nvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/cdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Connection Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/cdr:style"))]
  pub style: Option<std::boxed::Box<Style>>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Picture/cdr:pic")]
pub struct Picture {
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Picture Properties
  #[sdk(child(qname = "cdr:CT_PictureNonVisual/cdr:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties/cdr:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/cdr:style"))]
  pub style: Option<std::boxed::Box<Style>>,
}
/// Chart Non Visual Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr")]
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
/// Non-Visual Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/cdr:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_ShapeNonVisual/cdr:nvSpPr")]
pub struct NonVisualShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Chart Non Visual Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/cdr:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/cdr:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Shape Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/cdr:style")]
pub struct Style {
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectReference,
  >,
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Shape Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/cdr:txBody")]
pub struct TextBody {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Non-Visual Connection Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/cdr:cNvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
  >,
  /// Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Connector Non Visual Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_ConnectorNonVisual/cdr:nvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Chart Non Visual Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connection Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/cdr:cNvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/cdr:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    #[sdk(attr(qname = ":preferRelativeResize"))]
    pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
    /// _
    #[sdk(child(qname = "a:CT_PictureLocking/a:picLocks"))]
    pub picture_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst"))]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Picture Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_PictureNonVisual/cdr:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/cdr:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties/cdr:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip:
    Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle>,
  #[sdk(choice(
    qname = "a:CT_TileInfoProperties/a:tile",
    qname = "a:CT_StretchInfoProperties/a:stretch"
  ))]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/cdr:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
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
/// Non-Visual Graphic Frame Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_GraphicFrameNonVisual/cdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/cdr:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// Graphic Frame Transform.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/cdr:xfrm")]
pub struct Transform {
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
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/cdr:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
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
/// Relative X Coordinate.
pub type XPosition = crate::simple_type::DoubleValue;
/// Relative Y Coordinate.
pub type YPosition = crate::simple_type::DoubleValue;
/// Starting Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Marker/cdr:from")]
pub struct FromAnchor {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Relative X Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:x"))]
  pub x_position: crate::simple_type::DoubleValue,
  /// Relative Y Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:y"))]
  pub y_position: crate::simple_type::DoubleValue,
}
/// Ending Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Marker/cdr:to")]
pub struct ToAnchor {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Relative X Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:x"))]
  pub x_position: crate::simple_type::DoubleValue,
  /// Relative Y Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:y"))]
  pub y_position: crate::simple_type::DoubleValue,
}
/// Defines the MarkerType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Marker/")]
pub struct MarkerType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  #[sdk(choice(
    qname = "cdr:ST_MarkerCoordinate/cdr:x",
    qname = "cdr:ST_MarkerCoordinate/cdr:y"
  ))]
  pub xml_children: Vec<MarkerTypeChoice>,
}
/// Shape Extent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/cdr:ext")]
pub struct Extent {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: crate::simple_type::Int64Value,
}
/// Non-Visual Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_GroupShapeNonVisual/cdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Chart Non Visual Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/cdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/cdr:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/cdr:grpSpPr")]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RelativeAnchorSizeChoice {
  #[sdk(child(qname = "cdr:CT_Shape/cdr:sp"))]
  CdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "cdr:CT_GroupShape/cdr:grpSp"))]
  CdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "cdr:CT_GraphicFrame/cdr:graphicFrame"))]
  CdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "cdr:CT_Connector/cdr:cxnSp"))]
  CdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "cdr:CT_Picture/cdr:pic"))]
  CdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "cdr14:CT_ContentPart/cdr14:contentPart"))]
  Cdr14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AbsoluteAnchorSizeChoice {
  #[sdk(child(qname = "cdr:CT_Shape/cdr:sp"))]
  CdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "cdr:CT_GroupShape/cdr:grpSp"))]
  CdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "cdr:CT_GraphicFrame/cdr:graphicFrame"))]
  CdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "cdr:CT_Connector/cdr:cxnSp"))]
  CdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "cdr:CT_Picture/cdr:pic"))]
  CdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "cdr14:CT_ContentPart/cdr14:contentPart"))]
  Cdr14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(qname = "cdr:CT_Shape/cdr:sp"))]
  CdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "cdr:CT_GroupShape/cdr:grpSp"))]
  CdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "cdr:CT_GraphicFrame/cdr:graphicFrame"))]
  CdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "cdr:CT_Connector/cdr:cxnSp"))]
  CdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "cdr:CT_Picture/cdr:pic"))]
  CdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "cdr14:CT_ContentPart/cdr14:contentPart"))]
  Cdr14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    >,
  ),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
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
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MarkerTypeChoice {
  /// Relative X Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:x"))]
  CdrX(crate::simple_type::DoubleValue),
  /// Relative Y Coordinate
  #[sdk(text_child(qname = "cdr:ST_MarkerCoordinate/cdr:y"))]
  CdrY(crate::simple_type::DoubleValue),
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
