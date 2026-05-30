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
#[sdk(qname = "xdr:twoCellAnchor")]
pub struct TwoCellAnchor {
  /// Positioning and Resizing Behaviors
  #[sdk(attr(qname = ":editAs"))]
  #[sdk(string_format(kind = "token"))]
  pub edit_as: Option<EditAsValues>,
  /// Starting Anchor Point
  #[sdk(child(qname = "xdr:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  /// Ending Anchor Point
  #[sdk(child(qname = "xdr:to"))]
  pub to_marker: std::boxed::Box<ToMarker>,
  #[sdk(
        choice(
            child(variant = Shape, qname = "xdr:sp"),
            child(variant = GroupShape, qname = "xdr:grpSp"),
            child(variant = GraphicFrame, qname = "xdr:graphicFrame"),
            child(variant = ConnectionShape, qname = "xdr:cxnSp"),
            child(variant = Picture, qname = "xdr:pic"),
            child(variant = ContentPart, qname = "xdr:contentPart"),
            any
        )
    )]
  pub two_cell_anchor_choice: Option<TwoCellAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:clientData"))]
  pub client_data: std::boxed::Box<ClientData>,
}
/// One Cell Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:oneCellAnchor")]
pub struct OneCellAnchor {
  /// Starting Anchor Point.
  #[sdk(child(qname = "xdr:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  /// Defines the Extent Class.
  #[sdk(child(qname = "xdr:ext"))]
  pub extent: std::boxed::Box<Extent>,
  #[sdk(
        choice(
            child(variant = Shape, qname = "xdr:sp"),
            child(variant = GroupShape, qname = "xdr:grpSp"),
            child(variant = GraphicFrame, qname = "xdr:graphicFrame"),
            child(variant = ConnectionShape, qname = "xdr:cxnSp"),
            child(variant = Picture, qname = "xdr:pic"),
            child(variant = ContentPart, qname = "xdr:contentPart")
        )
    )]
  pub one_cell_anchor_choice: Option<OneCellAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:clientData"))]
  pub client_data: std::boxed::Box<ClientData>,
}
/// Absolute Anchor Shape Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:absoluteAnchor")]
pub struct AbsoluteAnchor {
  /// Position
  #[sdk(child(qname = "xdr:pos"))]
  pub position: std::boxed::Box<Position>,
  /// Shape Extent
  #[sdk(child(qname = "xdr:ext"))]
  pub extent: std::boxed::Box<Extent>,
  #[sdk(
        choice(
            child(variant = Shape, qname = "xdr:sp"),
            child(variant = GroupShape, qname = "xdr:grpSp"),
            child(variant = GraphicFrame, qname = "xdr:graphicFrame"),
            child(variant = ConnectionShape, qname = "xdr:cxnSp"),
            child(variant = Picture, qname = "xdr:pic"),
            child(variant = ContentPart, qname = "xdr:contentPart")
        )
    )]
  pub absolute_anchor_choice: Option<AbsoluteAnchorChoice>,
  /// Client Data.
  #[sdk(child(qname = "xdr:clientData"))]
  pub client_data: std::boxed::Box<ClientData>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:sp")]
pub struct Shape {
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
  #[sdk(child(qname = "xdr:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Shape Properties
  #[sdk(child(qname = "xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Shape Text Body
  #[sdk(child(qname = "xdr:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
}
/// Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:grpSp")]
pub struct GroupShape {
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "xdr:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Group Shape Properties
  #[sdk(child(qname = "xdr:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = Shape, qname = "xdr:sp"),
            child(variant = GroupShape, qname = "xdr:grpSp"),
            child(variant = GraphicFrame, qname = "xdr:graphicFrame"),
            child(variant = ConnectionShape, qname = "xdr:cxnSp"),
            child(variant = Picture, qname = "xdr:pic"),
            child(variant = ContentPart, qname = "xdr14:contentPart")
        )
    )]
  pub group_shape_choice: Vec<GroupShapeChoice>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:graphicFrame")]
pub struct GraphicFrame {
  /// Reference To Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "xdr:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// 2D Transform for Graphic Frames
  #[sdk(child(qname = "xdr:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// Graphic Object.
  #[sdk(child(qname = "a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cxnSp")]
pub struct ConnectionShape {
  /// Reference to Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "xdr:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Connector Shape Properties
  #[sdk(child(qname = "xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:pic")]
pub struct Picture {
  /// Reference To Custom Function
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// Publish to Server Flag
  #[sdk(attr(qname = ":fPublished"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "xdr:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "xdr:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// Shape Properties.
  #[sdk(child(qname = "xdr:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "xdr:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:contentPart")]
pub struct ContentPart {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// Defines the ExcelNonVisualContentPartShapeProperties Class.
  #[sdk(child(qname = "xdr14:nvContentPartPr"))]
  pub excel_non_visual_content_part_shape_properties:
    Option<std::boxed::Box<crate::schemas::xdr14::ExcelNonVisualContentPartShapeProperties>>,
  /// Defines the ApplicationNonVisualDrawingProperties Class.
  #[sdk(child(qname = "xdr14:nvPr"))]
  pub application_non_visual_drawing_properties:
    Option<crate::schemas::xdr14::ApplicationNonVisualDrawingProperties>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "xdr14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::xdr14::Transform2D>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "xdr14:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::xdr14::OfficeArtExtensionList>,
}
/// Worksheet Drawing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:wsDr")]
pub struct WorksheetDrawing {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  #[sdk(
        choice(
            child(variant = TwoCellAnchor, qname = "xdr:twoCellAnchor"),
            child(variant = OneCellAnchor, qname = "xdr:oneCellAnchor"),
            child(variant = AbsoluteAnchor, qname = "xdr:absoluteAnchor"),
            any
        )
    )]
  pub worksheet_drawing_choice: Vec<WorksheetDrawingChoice>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:nvSpPr")]
pub struct NonVisualShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Connection Non-Visual Shape Properties
  #[sdk(child(qname = "xdr:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, qname = "a:custGeom"),
            child(variant = PresetGeometry, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
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
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the ShapeStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:lnRef"))]
  pub line_reference: std::boxed::Box<crate::schemas::a::LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:fillRef"))]
  pub fill_reference: std::boxed::Box<crate::schemas::a::FillReference>,
  /// Effect Reference.
  #[sdk(child(qname = "a:effectRef"))]
  pub effect_reference: std::boxed::Box<crate::schemas::a::EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:fontRef"))]
  pub font_reference: std::boxed::Box<crate::schemas::a::FontReference>,
}
/// Shape Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:txBody")]
pub struct TextBody {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<crate::schemas::a::Paragraph>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "xdr:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Non-Visual Drawing Properties.
  #[sdk(child(qname = "xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "xdr:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:blipFill")]
pub struct BlipFill {
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:blip"))]
  pub blip: Option<std::boxed::Box<crate::schemas::a::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:srcRect"))]
  pub source_rectangle: Option<crate::schemas::a::SourceRectangle>,
  #[sdk(
        choice(
            child(variant = Tile, qname = "a:tile"),
            child(variant = Stretch, qname = "a:stretch")
        )
    )]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "xdr:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// 2D Transform for Graphic Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:xfrm")]
pub struct Transform {
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
  #[sdk(child(qname = "a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
/// Column).
pub type ColumnId = crate::simple_type::Int32Value;
/// Column Offset.
pub type ColumnOffset = crate::simple_type::CoordinateValue;
/// Row Offset.
pub type RowOffset = crate::simple_type::CoordinateValue;
/// Row.
pub type RowId = crate::simple_type::Int32Value;
/// Starting Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:from")]
pub struct FromMarker {
  /// Column)
  #[sdk(text_child(simple_type = "Int32Value", qname = "xdr:col"))]
  pub column_id: ColumnId,
  /// Column Offset
  #[sdk(text_child(simple_type = "CoordinateValue", qname = "xdr:colOff"))]
  pub column_offset: ColumnOffset,
  /// Row
  #[sdk(text_child(simple_type = "Int32Value", qname = "xdr:row"))]
  pub row_id: RowId,
  /// Row Offset
  #[sdk(text_child(simple_type = "CoordinateValue", qname = "xdr:rowOff"))]
  pub row_offset: RowOffset,
}
/// Ending Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:to")]
pub struct ToMarker {
  /// Column)
  #[sdk(text_child(simple_type = "Int32Value", qname = "xdr:col"))]
  pub column_id: ColumnId,
  /// Column Offset
  #[sdk(text_child(simple_type = "CoordinateValue", qname = "xdr:colOff"))]
  pub column_offset: ColumnOffset,
  /// Row
  #[sdk(text_child(simple_type = "Int32Value", qname = "xdr:row"))]
  pub row_id: RowId,
  /// Row Offset
  #[sdk(text_child(simple_type = "CoordinateValue", qname = "xdr:rowOff"))]
  pub row_offset: RowOffset,
}
/// Client Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:clientData")]
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
#[sdk(qname = "xdr:ext")]
pub struct Extent {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: crate::simple_type::Int64Value,
}
/// Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:pos")]
pub struct Position {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
}
/// Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvPr")]
pub struct NonVisualDrawingProperties {
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
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Defines the HyperlinkOnHover Class.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Defines the NonVisualDrawingPropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Connection Non-Visual Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<crate::schemas::a::ShapeLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<crate::schemas::a::ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:stCxn"))]
  pub start_connection: Option<crate::schemas::a::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:endCxn"))]
  pub end_connection: Option<crate::schemas::a::EndConnection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the PictureLocks Class.
  #[sdk(child(qname = "a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<crate::schemas::a::PictureLocks>>,
  /// Defines the NonVisualPicturePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_picture_properties_extension_list:
    Option<crate::schemas::a::NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<crate::schemas::a::GraphicFrameLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<crate::schemas::a::GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<crate::schemas::a::NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Connection Non-Visual Properties
  #[sdk(child(qname = "xdr:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "xdr:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:xfrm"))]
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
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TwoCellAnchorChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OneCellAnchorChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AbsoluteAnchorChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  ContentPart(std::boxed::Box<crate::schemas::xdr14::ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WorksheetDrawingChoice {
  /// Two Cell Anchor Shape Size.
  TwoCellAnchor(std::boxed::Box<TwoCellAnchor>),
  /// One Cell Anchor Shape Size.
  OneCellAnchor(std::boxed::Box<OneCellAnchor>),
  /// Absolute Anchor Shape Size.
  AbsoluteAnchor(std::boxed::Box<AbsoluteAnchor>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
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
  #[sdk(empty_child(qname = "a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  Tile(std::boxed::Box<crate::schemas::a::Tile>),
  Stretch(std::boxed::Box<crate::schemas::a::Stretch>),
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
  #[sdk(empty_child(qname = "a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
