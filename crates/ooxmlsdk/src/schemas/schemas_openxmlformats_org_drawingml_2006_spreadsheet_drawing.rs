//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EditAsValues {
  #[sdk(rename = "twoCell")]
  #[default]
  TwoCell,
  #[sdk(rename = "oneCell")]
  OneCell,
  #[sdk(rename = "absolute")]
  Absolute,
}
/// Two Cell Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor")]
pub struct TwoCellAnchor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Positioning and Resizing Behaviors
  #[sdk(attr(qname = ":editAs"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub edit_as: Option<EditAsValues>,
  /// Starting Anchor Point
  #[sdk(child(qname = "xdr:CT_Marker/xdr:from"))]
  pub from_marker: Option<FromMarker>,
  /// Ending Anchor Point
  #[sdk(child(qname = "xdr:CT_Marker/xdr:to"))]
  pub to_marker: Option<ToMarker>,
  #[sdk(choice(
    qname = "xdr:CT_Shape/xdr:sp",
    qname = "xdr:CT_GroupShape/xdr:grpSp",
    qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame",
    qname = "xdr:CT_Connector/xdr:cxnSp",
    qname = "xdr:CT_Picture/xdr:pic",
    qname = "xdr14:CT_ContentPart/xdr:contentPart"
  ))]
  pub two_cell_anchor_choice: Option<TwoCellAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: Option<ClientData>,
}
/// One Cell Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_OneCellAnchor/xdr:oneCellAnchor")]
pub struct OneCellAnchor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Starting Anchor Point.
  #[sdk(child(qname = "xdr:CT_Marker/xdr:from"))]
  pub from_marker: Option<FromMarker>,
  /// Defines the Extent Class.
  #[sdk(child(qname = "a:CT_PositiveSize2D/xdr:ext"))]
  pub extent: Option<Extent>,
  #[sdk(choice(
    qname = "xdr:CT_Shape/xdr:sp",
    qname = "xdr:CT_GroupShape/xdr:grpSp",
    qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame",
    qname = "xdr:CT_Connector/xdr:cxnSp",
    qname = "xdr:CT_Picture/xdr:pic",
    qname = "xdr14:CT_ContentPart/xdr:contentPart"
  ))]
  pub one_cell_anchor_choice: Option<OneCellAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: Option<ClientData>,
}
/// Absolute Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor")]
pub struct AbsoluteAnchor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Position
  #[sdk(child(qname = "a:CT_Point2D/xdr:pos"))]
  pub position: Option<Position>,
  /// Shape Extent
  #[sdk(child(qname = "a:CT_PositiveSize2D/xdr:ext"))]
  pub extent: Option<Extent>,
  #[sdk(choice(
    qname = "xdr:CT_Shape/xdr:sp",
    qname = "xdr:CT_GroupShape/xdr:grpSp",
    qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame",
    qname = "xdr:CT_Connector/xdr:cxnSp",
    qname = "xdr:CT_Picture/xdr:pic",
    qname = "xdr14:CT_ContentPart/xdr:contentPart"
  ))]
  pub absolute_anchor_choice: Option<AbsoluteAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: Option<ClientData>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Shape/xdr:sp")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Text Link
  #[sdk(attr(qname = ":textlink"))]
  pub text_link: Option<crate::simple_type::StringValue>,
  /// Lock Text Flag
  #[sdk(attr(qname = ":fLocksText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Shape
  #[sdk(child(qname = "xdr:CT_ShapeNonVisual/xdr:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/xdr:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
}
/// Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GroupShape/xdr:grpSp")]
pub struct GroupShape {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "xdr:CT_GroupShapeNonVisual/xdr:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: Option<std::boxed::Box<NonVisualGroupShapeProperties>>,
  /// Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/xdr:grpSpPr"))]
  pub group_shape_properties: Option<std::boxed::Box<GroupShapeProperties>>,
  #[sdk(choice(
    qname = "xdr:CT_Shape/xdr:sp",
    qname = "xdr:CT_GroupShape/xdr:grpSp",
    qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame",
    qname = "xdr:CT_Connector/xdr:cxnSp",
    qname = "xdr:CT_Picture/xdr:pic",
    qname = "xdr14:CT_ContentPart/xdr14:contentPart",
    text,
    any
  ))]
  pub group_shape_choice: Vec<GroupShapeChoice>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame")]
pub struct GraphicFrame {
  /// Reference To Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrameNonVisual/xdr:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// 2D Transform for Graphic Frames
  #[sdk(child(qname = "a:CT_Transform2D/xdr:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// Graphic Object.
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Connector/xdr:cxnSp")]
pub struct ConnectionShape {
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "xdr:CT_ConnectorNonVisual/xdr:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Connector Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Picture/xdr:pic")]
pub struct Picture {
  /// Reference To Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "xdr:CT_PictureNonVisual/xdr:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties2/xdr:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// Shape Properties.
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "xdr14:CT_ContentPart/xdr:contentPart")]
pub struct ContentPart {
    /// id
    #[sdk(attr(office2010, qname = "r:id"))]
    pub relationship_id: crate::simple_type::StringValue,
    /// bwMode
    #[sdk(attr(office2010, qname = ":bwMode"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// Defines the ExcelNonVisualContentPartShapeProperties Class.
    #[sdk(
        child(office2010, qname = "xdr14:CT_ContentPartNonVisual/xdr14:nvContentPartPr")
    )]
    pub excel_non_visual_content_part_shape_properties: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ExcelNonVisualContentPartShapeProperties,
        >,
    >,
    /// Defines the ApplicationNonVisualDrawingProperties Class.
    #[sdk(
        child(office2010, qname = "xdr14:CT_ApplicationNonVisualDrawingProps/xdr14:nvPr")
    )]
    pub application_non_visual_drawing_properties: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ApplicationNonVisualDrawingProperties,
    >,
    /// Defines the Transform2D Class.
    #[sdk(child(office2010, qname = "a:CT_Transform2D/xdr14:xfrm"))]
    pub transform2_d: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::Transform2D,
        >,
    >,
    /// Defines the OfficeArtExtensionList Class.
    #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/xdr14:extLst"))]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::OfficeArtExtensionList,
    >,
}
/// Worksheet Drawing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Drawing/xdr:wsDr")]
pub struct WorksheetDrawing {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor",
    qname = "xdr:CT_OneCellAnchor/xdr:oneCellAnchor",
    qname = "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor"
  ))]
  pub worksheet_drawing_choice: Vec<WorksheetDrawingChoice>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_ShapeNonVisual/xdr:nvSpPr")]
pub struct NonVisualShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Connection Non-Visual Shape Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/xdr:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/xdr:spPr")]
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
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Defines the ShapeStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/xdr:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference>,
  /// Effect Reference.
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
#[sdk(qname = "a:CT_TextBody/xdr:txBody")]
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
  /// Text Paragraphs.
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_ConnectorNonVisual/xdr:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/xdr:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_PictureNonVisual/xdr:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Non-Visual Drawing Properties.
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/xdr:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties2/xdr:blipFill")]
pub struct BlipFill {
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
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
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GraphicalObjectFrameNonVisual/xdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/xdr:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// 2D Transform for Graphic Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/xdr:xfrm")]
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
/// Column).
pub type ColumnId = crate::simple_type::Int32Value;
/// Column Offset.
pub type ColumnOffset = crate::simple_type::Int64Value;
/// Row Offset.
pub type RowOffset = crate::simple_type::Int64Value;
/// Row.
pub type RowId = crate::simple_type::Int32Value;
/// Starting Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/xdr:from")]
pub struct FromMarker {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  /// Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  /// Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  /// Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Ending Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/xdr:to")]
pub struct ToMarker {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  /// Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  /// Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  /// Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Client Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_AnchorClientData/xdr:clientData")]
pub struct ClientData {
  /// Locks With Sheet Flag
  #[sdk(attr(qname = ":fLocksWithSheet"))]
  pub lock_with_sheet: Option<crate::simple_type::BooleanValue>,
  /// Prints With Sheet Flag
  #[sdk(attr(qname = ":fPrintsWithSheet"))]
  pub print_with_sheet: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Extent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/xdr:ext")]
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
/// Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/xdr:pos")]
pub struct Position {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr")]
pub struct NonVisualDrawingProperties {
    pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
    /// id
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// descr
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// hidden
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// title
    #[sdk(attr(qname = ":title"))]
    pub title: Option<crate::simple_type::StringValue>,
    /// Defines the HyperlinkOnClick Class.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    /// Defines the HyperlinkOnHover Class.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    /// Defines the NonVisualDrawingPropertiesExtensionList Class.
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Connection Non-Visual Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/xdr:cNvSpPr")]
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
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/xdr:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
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
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/xdr:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    #[sdk(attr(qname = ":preferRelativeResize"))]
    pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
    /// Defines the PictureLocks Class.
    #[sdk(child(qname = "a:CT_PictureLocking/a:picLocks"))]
    pub picture_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
        >,
    >,
    /// Defines the NonVisualPicturePropertiesExtensionList Class.
    #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst"))]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/xdr:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrameLocks,
    >,
  >,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/xdr:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
    /// Defines the GroupShapeLocks Class.
    #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
    pub group_shape_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
        >,
    >,
    /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
    #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GroupShapeNonVisual/xdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/xdr:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/xdr:grpSpPr")]
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
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TwoCellAnchorChoice {
  #[sdk(child(qname = "xdr:CT_Shape/xdr:sp"))]
  XdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "xdr:CT_GroupShape/xdr:grpSp"))]
  XdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame"))]
  XdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "xdr:CT_Connector/xdr:cxnSp"))]
  XdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "xdr:CT_Picture/xdr:pic"))]
  XdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OneCellAnchorChoice {
  #[sdk(child(qname = "xdr:CT_Shape/xdr:sp"))]
  XdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "xdr:CT_GroupShape/xdr:grpSp"))]
  XdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame"))]
  XdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "xdr:CT_Connector/xdr:cxnSp"))]
  XdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "xdr:CT_Picture/xdr:pic"))]
  XdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AbsoluteAnchorChoice {
  #[sdk(child(qname = "xdr:CT_Shape/xdr:sp"))]
  XdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "xdr:CT_GroupShape/xdr:grpSp"))]
  XdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame"))]
  XdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "xdr:CT_Connector/xdr:cxnSp"))]
  XdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "xdr:CT_Picture/xdr:pic"))]
  XdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(qname = "xdr:CT_Shape/xdr:sp"))]
  XdrSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "xdr:CT_GroupShape/xdr:grpSp"))]
  XdrGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame"))]
  XdrGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "xdr:CT_Connector/xdr:cxnSp"))]
  XdrCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "xdr:CT_Picture/xdr:pic"))]
  XdrPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "xdr14:CT_ContentPart/xdr14:contentPart"))]
  Xdr14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ContentPart,
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
pub enum WorksheetDrawingChoice {
  /// Two Cell Anchor Shape Size.
  #[sdk(child(qname = "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor"))]
  XdrTwoCellAnchor(std::boxed::Box<TwoCellAnchor>),
  /// One Cell Anchor Shape Size.
  #[sdk(child(qname = "xdr:CT_OneCellAnchor/xdr:oneCellAnchor"))]
  XdrOneCellAnchor(std::boxed::Box<OneCellAnchor>),
  /// Absolute Anchor Shape Size.
  #[sdk(child(qname = "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor"))]
  XdrAbsoluteAnchor(std::boxed::Box<AbsoluteAnchor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
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
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
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
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
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
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
