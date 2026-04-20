//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:twoCellAnchor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor")]
pub struct TwoCellAnchor {
  /// Positioning and Resizing Behaviors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :editAs
  #[sdk(attr(qname = ":editAs"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub edit_as: Option<EditAsValues>,
  ///Starting Anchor Point
  #[sdk(child(qname = "xdr:CT_Marker/xdr:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  ///Ending Anchor Point
  #[sdk(child(qname = "xdr:CT_Marker/xdr:to"))]
  pub to_marker: std::boxed::Box<ToMarker>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice)]
  pub two_cell_anchor_choice: Option<TwoCellAnchorChoice>,
  /// _
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: std::boxed::Box<ClientData>,
}
/// One Cell Anchor Shape Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:oneCellAnchor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_OneCellAnchor/xdr:oneCellAnchor")]
pub struct OneCellAnchor {
  /// _
  #[sdk(child(qname = "xdr:CT_Marker/xdr:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  /// _
  #[sdk(child(qname = "a:CT_PositiveSize2D/xdr:ext"))]
  pub extent: std::boxed::Box<Extent>,
  #[sdk(choice)]
  pub one_cell_anchor_choice: Option<OneCellAnchorChoice>,
  /// _
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: std::boxed::Box<ClientData>,
}
/// Absolute Anchor Shape Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:absoluteAnchor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor")]
pub struct AbsoluteAnchor {
  ///Position
  #[sdk(child(qname = "a:CT_Point2D/xdr:pos"))]
  pub position: std::boxed::Box<Position>,
  ///Shape Extent
  #[sdk(child(qname = "a:CT_PositiveSize2D/xdr:ext"))]
  pub extent: std::boxed::Box<Extent>,
  #[sdk(choice)]
  pub absolute_anchor_choice: Option<AbsoluteAnchorChoice>,
  /// _
  #[sdk(child(qname = "xdr:CT_AnchorClientData/xdr:clientData"))]
  pub xdr_client_data: std::boxed::Box<ClientData>,
}
/// Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:sp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Shape/xdr:sp")]
pub struct Shape {
  /// Reference to Custom Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Text Link
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textlink
  #[sdk(attr(qname = ":textlink"))]
  pub text_link: Option<crate::simple_type::StringValue>,
  /// Lock Text Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fLocksText
  #[sdk(attr(qname = ":fLocksText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// Publish to Server Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  ///Non-Visual Properties for a Shape
  #[sdk(child(qname = "xdr:CT_ShapeNonVisual/xdr:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  ///Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  ///Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/xdr:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
}
/// Group Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:grpSp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GroupShape/xdr:grpSp")]
pub struct GroupShape {
  ///Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "xdr:CT_GroupShapeNonVisual/xdr:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  ///Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/xdr:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice)]
  pub group_shape_choice: Vec<GroupShapeChoice>,
}
/// Graphic Frame.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:graphicFrame.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame")]
pub struct GraphicFrame {
  /// Reference To Custom Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  ///Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "xdr:CT_GraphicalObjectFrameNonVisual/xdr:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  ///2D Transform for Graphic Frames
  #[sdk(child(qname = "a:CT_Transform2D/xdr:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
}
/// Connection Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cxnSp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Connector/xdr:cxnSp")]
pub struct ConnectionShape {
  /// Reference to Custom Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  ///Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "xdr:CT_ConnectorNonVisual/xdr:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  ///Connector Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
/// Defines the Picture Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:pic.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Picture/xdr:pic")]
pub struct Picture {
  /// Reference To Custom Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fPublished
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  ///Non-Visual Properties for a Picture
  #[sdk(child(qname = "xdr:CT_PictureNonVisual/xdr:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  ///Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties2/xdr:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  ///Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
#[cfg(feature = "microsoft365")]
/// Defines the ContentPart Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:contentPart.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr14:CT_ContentPart/xdr:contentPart")]
pub struct ContentPart {
    /// id
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: r:id
    #[sdk(attr(qname = "r:id"))]
    pub relationship_id: crate::simple_type::StringValue,
    /// bwMode
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :bwMode
    #[sdk(attr(qname = ":bwMode"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// _
    #[sdk(child(qname = "xdr14:CT_ContentPartNonVisual/xdr14:nvContentPartPr"))]
    pub excel_non_visual_content_part_shape_properties: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ExcelNonVisualContentPartShapeProperties,
        >,
    >,
    /// _
    #[sdk(child(qname = "xdr14:CT_ApplicationNonVisualDrawingProps/xdr14:nvPr"))]
    pub application_non_visual_drawing_properties: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ApplicationNonVisualDrawingProperties,
    >,
    /// _
    #[sdk(child(qname = "a:CT_Transform2D/xdr14:xfrm"))]
    pub transform2_d: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::Transform2D,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_OfficeArtExtensionList/xdr14:extLst"))]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::OfficeArtExtensionList,
    >,
}
/// Worksheet Drawing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:wsDr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Drawing/xdr:wsDr")]
pub struct WorksheetDrawing {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  #[sdk(choice)]
  pub worksheet_drawing_choice: Vec<WorksheetDrawingChoice>,
}
/// Non-Visual Properties for a Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:nvSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_ShapeNonVisual/xdr:nvSpPr")]
pub struct NonVisualShapeProperties {
  ///Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Connection Non-Visual Shape Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/xdr:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Shape Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:spPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/xdr:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice)]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice)]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice)]
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
/// Defines the ShapeStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:style.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/xdr:style")]
pub struct ShapeStyle {
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
  ///Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Shape Text Body.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:txBody.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/xdr:txBody")]
pub struct TextBody {
  ///Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ///Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Non-Visual Properties for a Connection Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:nvCxnSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_ConnectorNonVisual/xdr:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  ///Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/xdr:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
}
/// Non-Visual Properties for a Picture.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:nvPicPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_PictureNonVisual/xdr:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// _
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/xdr:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Picture Fill.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:blipFill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties2/xdr:blipFill")]
pub struct BlipFill {
  /// Rotate With Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotWithShape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip:
    Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip>>,
  ///Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle>,
  #[sdk(choice)]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Properties for a Graphic Frame.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:nvGraphicFramePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GraphicalObjectFrameNonVisual/xdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  ///Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/xdr:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// 2D Transform for Graphic Frames.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:xfrm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/xdr:xfrm")]
pub struct Transform {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipH
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipV
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  ///Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  ///Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Column).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:col.
pub type ColumnId = crate::simple_type::Int32Value;
/// Column Offset.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:colOff.
pub type ColumnOffset = crate::simple_type::Int64Value;
/// Row Offset.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:rowOff.
pub type RowOffset = crate::simple_type::Int64Value;
/// Row.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:row.
pub type RowId = crate::simple_type::Int32Value;
/// Starting Anchor Point.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:from.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/xdr:from")]
pub struct FromMarker {
  ///Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  ///Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  ///Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  ///Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Ending Anchor Point.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:to.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/xdr:to")]
pub struct ToMarker {
  ///Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  ///Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  ///Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  ///Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Defines the MarkerType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/")]
pub struct MarkerType {
  #[sdk(choice)]
  pub xml_children: Vec<MarkerTypeChoice>,
}
/// Client Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:clientData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_AnchorClientData/xdr:clientData")]
pub struct ClientData {
  /// Locks With Sheet Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fLocksWithSheet
  #[sdk(attr(qname = ":fLocksWithSheet"))]
  pub lock_with_sheet: Option<crate::simple_type::BooleanValue>,
  /// Prints With Sheet Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fPrintsWithSheet
  #[sdk(attr(qname = ":fPrintsWithSheet"))]
  pub print_with_sheet: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Extent Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/xdr:ext")]
pub struct Extent {
  /// Extent Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cx
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cy
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:pos.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/xdr:pos")]
pub struct Position {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// id
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :id
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// name
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// descr
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :descr
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// hidden
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :hidden
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// title
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :title
    #[sdk(attr(qname = ":title"))]
    pub title: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Connection Non-Visual Shape Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/xdr:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  /// Text Box
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :txBox
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  ///Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Connector Shape Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvCxnSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/xdr:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  ///Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
  >,
  ///Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection>,
  ///Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Picture Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvPicPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/xdr:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :preferRelativeResize
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
/// Non-Visual Graphic Frame Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/xdr:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  ///Graphic Frame Locks
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
/// Non-Visual Group Shape Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:cNvGrpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/xdr:cNvGrpSpPr")]
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
/// Non-Visual Properties for a Group Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:nvGrpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_GroupShapeNonVisual/xdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  ///Connection Non-Visual Properties
  #[sdk(child(qname = "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/xdr:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Group Shape Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xdr:grpSpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/xdr:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup>,
  >,
  #[sdk(choice)]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(choice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "xdr14:CT_ContentPart/xdr:contentPart"))]
  XdrContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "xdr14:CT_ContentPart/xdr14:contentPart"))]
  Xdr14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ContentPart,
    >,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum WorksheetDrawingChoice {
  #[sdk(child(qname = "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor"))]
  XdrTwoCellAnchor(std::boxed::Box<TwoCellAnchor>),
  #[sdk(child(qname = "xdr:CT_OneCellAnchor/xdr:oneCellAnchor"))]
  XdrOneCellAnchor(std::boxed::Box<OneCellAnchor>),
  #[sdk(child(qname = "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor"))]
  XdrAbsoluteAnchor(std::boxed::Box<AbsoluteAnchor>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MarkerTypeChoice {
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  XdrCol(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  XdrColOff(crate::simple_type::Int64Value),
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  XdrRow(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  XdrRowOff(crate::simple_type::Int64Value),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
