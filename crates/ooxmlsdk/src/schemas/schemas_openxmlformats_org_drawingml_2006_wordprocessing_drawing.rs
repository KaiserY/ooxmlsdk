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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:wrapSquare.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapSquare/wp:wrapSquare")]
pub struct WrapSquare {
  /// Text Wrapping Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapText
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Text (Top)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distT
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distB
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distL
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distR
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Object Extents Including Effects
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
}
/// Tight Wrapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:wrapTight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapTight/wp:wrapTight")]
pub struct WrapTight {
  /// Text Wrapping Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapText
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Test on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distL
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distR
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Tight Wrapping Extents Polygon
  #[sdk(child(qname = "wp:CT_WrapPath/wp:wrapPolygon"))]
  pub wrap_polygon: std::boxed::Box<WrapPolygon>,
}
/// Through Wrapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:wrapThrough.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapThrough/wp:wrapThrough")]
pub struct WrapThrough {
  /// Text Wrapping Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapText
  #[sdk(attr(qname = ":wrapText"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub wrap_text: WrapTextValues,
  /// Distance From Text on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distL
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distR
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Wrapping Polygon
  #[sdk(child(qname = "wp:CT_WrapPath/wp:wrapPolygon"))]
  pub wrap_polygon: std::boxed::Box<WrapPolygon>,
}
/// Top and Bottom Wrapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:wrapTopAndBottom.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapTopBottom/wp:wrapTopAndBottom")]
pub struct WrapTopBottom {
  /// Distance From Text on Top Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distT
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distB
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Wrapping Boundaries
  #[sdk(child(qname = "wp:CT_EffectExtent/wp:effectExtent"))]
  pub effect_extent: Option<EffectExtent>,
}
/// Inline DrawingML Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:inline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_Inline/wp:inline")]
pub struct Inline {
  /// Distance From Text on Top Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distT
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distB
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distL
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distR
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  #[cfg(feature = "microsoft365")]
  /// anchorId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: wp14:anchorId
  #[sdk(attr(qname = "wp14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub wp14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  #[cfg(feature = "microsoft365")]
  /// editId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: wp14:editId
  #[sdk(attr(qname = "wp14:editId"))]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:anchor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_Anchor/wp:anchor")]
pub struct Anchor {
  pub xml_other_children: Vec<(usize, String)>,
  /// Distance From Text on Top Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distT
  #[sdk(attr(qname = ":distT"))]
  pub distance_from_top: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Bottom Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distB
  #[sdk(attr(qname = ":distB"))]
  pub distance_from_bottom: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distL
  #[sdk(attr(qname = ":distL"))]
  pub distance_from_left: Option<crate::simple_type::UInt32Value>,
  /// Distance From Text on Right Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distR
  #[sdk(attr(qname = ":distR"))]
  pub distance_from_right: Option<crate::simple_type::UInt32Value>,
  /// Page Positioning
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :simplePos
  #[sdk(attr(qname = ":simplePos"))]
  pub simple_pos: Option<crate::simple_type::BooleanValue>,
  /// Relative Z-Ordering Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relativeHeight
  #[sdk(attr(qname = ":relativeHeight"))]
  pub relative_height: crate::simple_type::UInt32Value,
  /// Display Behind Document Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :behindDoc
  #[sdk(attr(qname = ":behindDoc"))]
  pub behind_doc: crate::simple_type::BooleanValue,
  /// Lock Anchor
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: crate::simple_type::BooleanValue,
  /// Layout In Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :layoutInCell
  #[sdk(attr(qname = ":layoutInCell"))]
  pub layout_in_cell: crate::simple_type::BooleanValue,
  /// Hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Allow Objects to Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowOverlap
  #[sdk(attr(qname = ":allowOverlap"))]
  pub allow_overlap: crate::simple_type::BooleanValue,
  #[cfg(feature = "microsoft365")]
  /// editId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: wp14:editId
  #[sdk(attr(qname = "wp14:editId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub edit_id: Option<crate::simple_type::HexBinaryValue>,
  #[cfg(feature = "microsoft365")]
  /// anchorId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: wp14:anchorId
  #[sdk(attr(qname = "wp14:anchorId"))]
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
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "wp14:CT_SizeRelH/wp14:sizeRelH"))]
  pub wp14_size_rel_h: Option<
    crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeWidth,
  >,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "wp14:CT_SizeRelV/wp14:sizeRelV"))]
  pub wp14_size_rel_v: Option<
    crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeHeight,
  >,
}
/// Wrapping Polygon Start.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:start.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:start")]
pub struct StartPoint {
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
/// Wrapping Polygon Line End Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:lineTo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:lineTo")]
pub struct LineTo {
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
/// Simple Positioning Coordinates.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:simplePos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/wp:simplePos")]
pub struct SimplePosition {
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
/// Defines the Point2DType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/")]
pub struct Point2DType {
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
/// Object Extents Including Effects.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:effectExtent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_EffectExtent/wp:effectExtent")]
pub struct EffectExtent {
  /// Additional Extent on Left Edge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :l
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:wrapPolygon.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_WrapPath/wp:wrapPolygon")]
pub struct WrapPolygon {
  /// Wrapping Points Modified
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :edited
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:positionH.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_PosH/wp:positionH")]
pub struct HorizontalPosition {
  /// Horizontal Position Relative Base
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relativeFrom
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub relative_from: HorizontalRelativePositionValues,
  #[sdk(choice(
    qname = "wp:ST_AlignH/wp:align",
    qname = "wp:ST_PositionOffset/wp:posOffset"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "a:ST_Percentage/wp14:pctPosHOffset"))
  )]
  pub xml_children: Option<HorizontalPositionChoice>,
}
/// Vertical Positioning.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:positionV.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp:CT_PosV/wp:positionV")]
pub struct VerticalPosition {
  /// Vertical Position Relative Base
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relativeFrom
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub relative_from: VerticalRelativePositionValues,
  #[sdk(choice(
    qname = "wp:ST_AlignV/wp:align",
    qname = "wp:ST_PositionOffset/wp:posOffset"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "a:ST_Percentage/wp14:pctPosVOffset"))
  )]
  pub xml_children: Option<VerticalPositionChoice>,
}
/// Inline Drawing Object Extents.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:extent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/wp:extent")]
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
/// Drawing Object Non-Visual Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:docPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/wp:docPr")]
pub struct DocProperties {
    /// Application defined unique identifier.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :id
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// Name compatible with Object Model (non-unique).
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// Description of the drawing element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :descr
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// Flag determining to show or hide this element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :hidden
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// Title
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :title
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:cNvGraphicFramePr.
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:align.
pub type VerticalAlignment = VerticalAlignmentValues;
/// Defines the PositionOffset Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:posOffset.
pub type PositionOffset = crate::simple_type::Int32Value;
/// Relative Horizontal Alignment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp:align.
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
  #[cfg(feature = "microsoft365")]
  /// Defines the PercentagePositionHeightOffset Class.
  #[sdk(text_child(qname = "a:ST_Percentage/wp14:pctPosHOffset"))]
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
  #[cfg(feature = "microsoft365")]
  /// Defines the PercentagePositionVerticalOffset Class.
  #[sdk(text_child(qname = "a:ST_Percentage/wp14:pctPosVOffset"))]
  Wp14PctPosVOffset(crate::simple_type::Int32Value),
}
