//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum WrapTextValues {
  #[sdk(rename = "bothSides")]
  #[default]
  BothSides,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "largest")]
  Largest,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "inside")]
  Inside,
  #[sdk(rename = "outside")]
  Outside,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalRelativePositionValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "column")]
  Column,
  #[sdk(rename = "character")]
  Character,
  #[sdk(rename = "leftMargin")]
  LeftMargin,
  #[sdk(rename = "rightMargin")]
  RightMargin,
  #[sdk(rename = "insideMargin")]
  InsideMargin,
  #[sdk(rename = "outsideMargin")]
  OutsideMargin,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "inside")]
  Inside,
  #[sdk(rename = "outside")]
  Outside,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalRelativePositionValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "paragraph")]
  Paragraph,
  #[sdk(rename = "line")]
  Line,
  #[sdk(rename = "topMargin")]
  TopMargin,
  #[sdk(rename = "bottomMargin")]
  BottomMargin,
  #[sdk(rename = "insideMargin")]
  InsideMargin,
  #[sdk(rename = "outsideMargin")]
  OutsideMargin,
}
/// Square Wrapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapSquare/wp:wrapSquare")]
pub struct WrapSquare {
  /// Text Wrapping Location
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Text (Top)
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Object Extents Including Effects
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
}
/// Tight Wrapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapTight/wp:wrapTight")]
pub struct WrapTight {
  /// Text Wrapping Location
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Test on Left Edge
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Tight Wrapping Extents Polygon
  #[sdk(child(qname = "wp:CT_WrapPath/wp:wrapPolygon"))]
  pub wrap_polygon: std::boxed::Box<WrapPolygon>,
}
/// Through Wrapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapThrough/wp:wrapThrough")]
pub struct WrapThrough {
  /// Text Wrapping Location
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Text on Left Edge
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Wrapping Polygon
  #[sdk(child(qname = "wp:CT_WrapPath/wp:wrapPolygon"))]
  pub wrap_polygon: std::boxed::Box<WrapPolygon>,
}
/// Top and Bottom Wrapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapTopBottom/wp:wrapTopAndBottom")]
pub struct WrapTopBottom {
  /// Distance From Text on Top Edge
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Wrapping Boundaries
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
}
/// Inline DrawingML Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_Inline/wp:inline")]
pub struct Inline {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Distance From Text on Top Edge
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// anchorId
  #[sdk(attr(office2010, qname = "wp14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub wp14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  /// editId
  #[sdk(attr(office2010, qname = "wp14:editId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub edit_id: Option<crate::simple_type::HexBinaryValue>,
  /// Drawing Object Size
  #[sdk(child(qname = "a:CT_PositiveSize2D/wp:extent"))]
  pub extent: std::boxed::Box<Extent>,
  /// Inline Wrapping Extent
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
  /// Drawing Object Non-Visual Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/wp:docPr"))]
  pub doc_properties: std::boxed::Box<DocProperties>,
  /// Common DrawingML Non-Visual Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/wp:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    Option<std::boxed::Box<NonVisualGraphicFrameDrawingProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
}
/// Anchor for Floating DrawingML Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_Anchor/wp:anchor")]
pub struct Anchor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Distance From Text on Top Edge
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Page Positioning
  #[sdk(attr(qname = ":simplePos"))]
  pub simple_pos: Option<crate::simple_type::BooleanValue>,
  /// Relative Z-Ordering Position
  #[sdk(attr(qname = ":relativeHeight"))]
  pub relative_height: crate::simple_type::UInt32Value,
  /// Display Behind Document Text
  #[sdk(attr(qname = ":behindDoc"))]
  pub behind_doc: crate::simple_type::BooleanValue,
  /// Lock Anchor
  #[sdk(attr(qname = ":locked"))]
  pub locked: crate::simple_type::BooleanValue,
  /// Layout In Table Cell
  #[sdk(attr(qname = ":layoutInCell"))]
  pub layout_in_cell: crate::simple_type::BooleanValue,
  /// Hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Allow Objects to Overlap
  #[sdk(attr(qname = ":allowOverlap"))]
  pub allow_overlap: crate::simple_type::BooleanValue,
  /// editId
  #[sdk(attr(office2010, qname = "wp14:editId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub edit_id: Option<crate::simple_type::HexBinaryValue>,
  /// anchorId
  #[sdk(attr(office2010, qname = "wp14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub wp14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  /// Simple Positioning Coordinates
  #[sdk(child(qname = "a:CT_Point2D/wp:simplePos"))]
  pub simple_position: std::boxed::Box<SimplePosition>,
  /// Horizontal Positioning
  #[sdk(child(qname = "wp:CT_PosH/wp:positionH"))]
  pub horizontal_position: std::boxed::Box<HorizontalPosition>,
  /// Vertical Positioning
  #[sdk(child(qname = "wp:CT_PosV/wp:positionV"))]
  pub vertical_position: std::boxed::Box<VerticalPosition>,
  /// Inline Drawing Object Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/wp:extent"))]
  pub extent: std::boxed::Box<Extent>,
  /// _
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
  #[sdk(choice(
    qname = "wp:CT_WrapNone/wp:wrapNone",
    qname = "wp:CT_WrapSquare/wp:wrapSquare",
    qname = "wp:CT_WrapTight/wp:wrapTight",
    qname = "wp:CT_WrapThrough/wp:wrapThrough",
    qname = "wp:CT_WrapTopBottom/wp:wrapTopAndBottom"
  ))]
  pub anchor_choice: Option<AnchorChoice>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/wp:docPr"))]
  pub wp_doc_pr: std::boxed::Box<DocProperties>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/wp:cNvGraphicFramePr"))]
  pub wp_c_nv_graphic_frame_pr: Option<std::boxed::Box<NonVisualGraphicFrameDrawingProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub a_graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
  /// _
  #[sdk(child(office2010, qname = "wp14:CT_SizeRelH/wp14:sizeRelH"))]
  pub wp14_size_rel_h: Option<
    crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeWidth,
  >,
  /// _
  #[sdk(child(office2010, qname = "wp14:CT_SizeRelV/wp14:sizeRelV"))]
  pub wp14_size_rel_v: Option<
    crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeHeight,
  >,
}
/// Wrapping Polygon Start.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:start")]
pub struct StartPoint {
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
/// Wrapping Polygon Line End Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:lineTo")]
pub struct LineTo {
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
/// Simple Positioning Coordinates.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:simplePos")]
pub struct SimplePosition {
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
/// Object Extents Including Effects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_EffectExtent/wp:effectExtent")]
pub struct EffectExtent {
  /// Additional Extent on Left Edge
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_edge: crate::simple_type::Int64Value,
  /// Additional Extent on Top Edge
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub top_edge: crate::simple_type::Int64Value,
  /// Additional Extent on Right Edge
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_edge: crate::simple_type::Int64Value,
  /// Additional Extent on Bottom Edge
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bottom_edge: crate::simple_type::Int64Value,
}
/// Tight Wrapping Extents Polygon.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapPath/wp:wrapPolygon")]
pub struct WrapPolygon {
  /// Wrapping Points Modified
  #[sdk(attr(qname = ":edited"))]
  pub edited: Option<crate::simple_type::BooleanValue>,
  /// Wrapping Polygon Start
  #[sdk(child(qname = "a:CT_Point2D/wp:start"))]
  pub start_point: std::boxed::Box<StartPoint>,
  /// _
  #[sdk(child(qname = "a:CT_Point2D/wp:lineTo"))]
  pub wp_line_to: Vec<LineTo>,
}
/// Horizontal Positioning.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_PosH/wp:positionH")]
pub struct HorizontalPosition {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Horizontal Position Relative Base
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub relative_from: HorizontalRelativePositionValues,
  #[sdk(choice(
    qname = "wp:ST_AlignH/wp:align",
    qname = "wp:ST_PositionOffset/wp:posOffset",
    qname = "a:ST_Percentage/wp14:pctPosHOffset"
  ))]
  pub xml_children: Option<HorizontalPositionChoice>,
}
/// Vertical Positioning.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_PosV/wp:positionV")]
pub struct VerticalPosition {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Vertical Position Relative Base
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub relative_from: VerticalRelativePositionValues,
  #[sdk(choice(
    qname = "wp:ST_AlignV/wp:align",
    qname = "wp:ST_PositionOffset/wp:posOffset",
    qname = "a:ST_Percentage/wp14:pctPosVOffset"
  ))]
  pub xml_children: Option<VerticalPositionChoice>,
}
/// Inline Drawing Object Extents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/wp:extent")]
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
/// Drawing Object Non-Visual Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/wp:docPr")]
pub struct DocProperties {
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
/// Defines the NonVisualGraphicFrameDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/wp:cNvGraphicFramePr")]
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
/// Relative Vertical Alignment.
pub type VerticalAlignment = VerticalAlignmentValues;
/// Defines the PositionOffset Class.
pub type PositionOffset = crate::simple_type::Int32Value;
/// Relative Horizontal Alignment.
pub type HorizontalAlignment = HorizontalAlignmentValues;
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AnchorChoice {
  /// No Text Wrapping.
  #[sdk(empty_child(qname = "wp:CT_WrapNone/wp:wrapNone"))]
  WpWrapNone,
  #[sdk(child(qname = "wp:CT_WrapSquare/wp:wrapSquare"))]
  WpWrapSquare(std::boxed::Box<WrapSquare>),
  #[sdk(child(qname = "wp:CT_WrapTight/wp:wrapTight"))]
  WpWrapTight(std::boxed::Box<WrapTight>),
  #[sdk(child(qname = "wp:CT_WrapThrough/wp:wrapThrough"))]
  WpWrapThrough(std::boxed::Box<WrapThrough>),
  #[sdk(child(qname = "wp:CT_WrapTopBottom/wp:wrapTopAndBottom"))]
  WpWrapTopAndBottom(std::boxed::Box<WrapTopBottom>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HorizontalPositionChoice {
  /// Relative Horizontal Alignment.
  #[sdk(text_child(qname = "wp:ST_AlignH/wp:align"))]
  WpAlign(HorizontalAlignmentValues),
  /// Defines the PositionOffset Class.
  #[sdk(text_child(qname = "wp:ST_PositionOffset/wp:posOffset"))]
  WpPosOffset(crate::simple_type::Int32Value),
  /// Defines the PercentagePositionHeightOffset Class.
  #[sdk(text_child(office2010, qname = "a:ST_Percentage/wp14:pctPosHOffset"))]
  Wp14PctPosHOffset(crate::simple_type::Int32Value),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VerticalPositionChoice {
  /// Relative Vertical Alignment.
  #[sdk(text_child(qname = "wp:ST_AlignV/wp:align"))]
  WpAlign(VerticalAlignmentValues),
  /// Defines the PositionOffset Class.
  #[sdk(text_child(qname = "wp:ST_PositionOffset/wp:posOffset"))]
  WpPosOffset(crate::simple_type::Int32Value),
  /// Defines the PercentagePositionVerticalOffset Class.
  #[sdk(text_child(office2010, qname = "a:ST_Percentage/wp14:pctPosVOffset"))]
  Wp14PctPosVOffset(crate::simple_type::Int32Value),
}
