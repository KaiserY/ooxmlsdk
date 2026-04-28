//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtensionHandlingBehaviorValues {
  #[sdk(rename = "view")]
  #[default]
  View,
  #[sdk(rename = "edit")]
  Edit,
  #[sdk(rename = "backwardCompatible")]
  BackwardCompatible,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FillTypeValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "gradient")]
  Gradient,
  #[sdk(rename = "gradientRadial")]
  GradientRadial,
  #[sdk(rename = "tile")]
  Tile,
  #[sdk(rename = "pattern")]
  Pattern,
  #[sdk(rename = "frame")]
  Frame,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FillMethodValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "linear")]
  Linear,
  #[sdk(rename = "sigma")]
  Sigma,
  #[sdk(rename = "any")]
  Any,
  #[sdk(rename = "linear sigma")]
  Linearsigma,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeLineStyleValues {
  #[sdk(rename = "single")]
  #[default]
  Single,
  #[sdk(rename = "thinThin")]
  ThinThin,
  #[sdk(rename = "thinThick")]
  ThinThick,
  #[sdk(rename = "thickThin")]
  ThickThin,
  #[sdk(rename = "thickBetweenThin")]
  ThickBetweenThin,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeJoinStyleValues {
  #[sdk(rename = "round")]
  #[default]
  Round,
  #[sdk(rename = "bevel")]
  Bevel,
  #[sdk(rename = "miter")]
  Miter,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeEndCapValues {
  #[sdk(rename = "flat")]
  #[default]
  Flat,
  #[sdk(rename = "square")]
  Square,
  #[sdk(rename = "round")]
  Round,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeArrowLengthValues {
  #[sdk(rename = "short")]
  #[default]
  Short,
  #[sdk(rename = "medium")]
  Medium,
  #[sdk(rename = "long")]
  Long,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeArrowWidthValues {
  #[sdk(rename = "narrow")]
  #[default]
  Narrow,
  #[sdk(rename = "medium")]
  Medium,
  #[sdk(rename = "wide")]
  Wide,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeArrowValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "block")]
  Block,
  #[sdk(rename = "classic")]
  Classic,
  #[sdk(rename = "oval")]
  Oval,
  #[sdk(rename = "diamond")]
  Diamond,
  #[sdk(rename = "open")]
  Open,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ImageAspectValues {
  #[sdk(rename = "ignore")]
  #[default]
  Ignore,
  #[sdk(rename = "atMost")]
  AtMost,
  #[sdk(rename = "atLeast")]
  AtLeast,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EditAsValues {
  #[sdk(rename = "canvas")]
  #[default]
  Canvas,
  #[sdk(rename = "orgchart")]
  OrganizationChart,
  #[sdk(rename = "radial")]
  Radial,
  #[sdk(rename = "cycle")]
  Cycle,
  #[sdk(rename = "stacked")]
  Stacked,
  #[sdk(rename = "venn")]
  Venn,
  #[sdk(rename = "bullseye")]
  Bullseye,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShadowValues {
  #[sdk(rename = "single")]
  #[default]
  Single,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "emboss")]
  Emboss,
  #[sdk(rename = "perspective")]
  Perspective,
  #[sdk(rename = "shapeRelative")]
  ShapeRelative,
  #[sdk(rename = "drawingRelative")]
  DrawingRelative,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrokeFillTypeValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "tile")]
  Tile,
  #[sdk(rename = "pattern")]
  Pattern,
  #[sdk(rename = "frame")]
  Frame,
}
/// Defines the Path Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Path/v:path")]
pub struct Path {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Path Definition
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub value: Option<crate::simple_type::StringValue>,
  /// Limo Stretch Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :limo
  #[sdk(attr(qname = ":limo"))]
  pub limo: Option<crate::simple_type::StringValue>,
  /// Text Box Bounding Box
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textboxrect
  #[sdk(attr(qname = ":textboxrect"))]
  pub textbox_rectangle: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillok
  #[sdk(attr(qname = ":fillok"))]
  pub allow_fill: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeok
  #[sdk(attr(qname = ":strokeok"))]
  pub allow_stroke: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadowok
  #[sdk(attr(qname = ":shadowok"))]
  pub allow_shading: Option<crate::simple_type::TrueFalseValue>,
  /// Arrowhead Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :arrowok
  #[sdk(attr(qname = ":arrowok"))]
  pub show_arrowhead: Option<crate::simple_type::TrueFalseValue>,
  /// Gradient Shape Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gradientshapeok
  #[sdk(attr(qname = ":gradientshapeok"))]
  pub allow_gradient_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textpathok
  #[sdk(attr(qname = ":textpathok"))]
  pub allow_text_path: Option<crate::simple_type::TrueFalseValue>,
  /// Inset Stroke From Path Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpenok
  #[sdk(attr(qname = ":insetpenok"))]
  pub allow_inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Connection Point Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connecttype
  #[sdk(attr(qname = "o:connecttype"))]
  pub connection_point_type:
    Option<crate::schemas::schemas_microsoft_com_office_office::ConnectValues>,
  /// Connection Points
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectlocs
  #[sdk(attr(qname = "o:connectlocs"))]
  pub connection_points: Option<crate::simple_type::StringValue>,
  /// Connection Point Connect Angles
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectangles
  #[sdk(attr(qname = "o:connectangles"))]
  pub connect_angles: Option<crate::simple_type::StringValue>,
  /// Extrusion Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:extrusionok
  #[sdk(attr(qname = "o:extrusionok"))]
  pub allow_extrusion: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Formulas Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:formulas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Formulas/v:formulas")]
pub struct Formulas {
  /// _
  #[sdk(child(qname = "v:CT_F/v:f"))]
  pub v_f: Vec<Formula>,
}
/// Defines the ShapeHandles Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:handles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Handles/v:handles")]
pub struct ShapeHandles {
  /// _
  #[sdk(child(qname = "v:CT_H/v:h"))]
  pub v_h: Vec<ShapeHandle>,
}
/// Defines the Fill Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Fill/v:fill")]
pub struct Fill {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Fill Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<FillTypeValues>,
  /// Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Primary Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Primary Color Opacity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Secondary Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Fill Image Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :src
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:href
  #[sdk(attr(qname = "o:href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Alternate Image Reference Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:althref
  #[sdk(attr(qname = "o:althref"))]
  pub alternate_image_reference: Option<crate::simple_type::StringValue>,
  /// Fill Image Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::StringValue>,
  /// Fill Image Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Fill Image Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::StringValue>,
  /// Image Aspect Ratio
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :aspect
  #[sdk(attr(qname = ":aspect"))]
  pub aspect: Option<ImageAspectValues>,
  /// Intermediate Colors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colors
  #[sdk(attr(qname = ":colors"))]
  pub colors: Option<crate::simple_type::StringValue>,
  /// Gradient Angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :angle
  #[sdk(attr(qname = ":angle"))]
  pub angle: Option<crate::simple_type::DecimalValue>,
  /// Align Image With Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alignshape
  #[sdk(attr(qname = ":alignshape"))]
  pub align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Gradient Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :focus
  #[sdk(attr(qname = ":focus"))]
  pub focus: Option<crate::simple_type::StringValue>,
  /// Radial Gradient Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :focussize
  #[sdk(attr(qname = ":focussize"))]
  pub focus_size: Option<crate::simple_type::StringValue>,
  /// Radial Gradient Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :focusposition
  #[sdk(attr(qname = ":focusposition"))]
  pub focus_position: Option<crate::simple_type::StringValue>,
  /// Gradient Fill Method
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :method
  #[sdk(attr(qname = ":method"))]
  pub method: Option<FillMethodValues>,
  /// Detect Mouse Click
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:detectmouseclick
  #[sdk(attr(qname = "o:detectmouseclick"))]
  pub detect_mouse_click: Option<crate::simple_type::TrueFalseValue>,
  /// Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Secondary Color Opacity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:opacity2
  #[sdk(attr(qname = "o:opacity2"))]
  pub opacity2: Option<crate::simple_type::StringValue>,
  /// Recolor Fill as Picture
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :recolor
  #[sdk(attr(qname = ":recolor"))]
  pub recolor: Option<crate::simple_type::TrueFalseValue>,
  /// Rotate Fill with Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotate
  #[sdk(attr(qname = ":rotate"))]
  pub rotate: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship to Part
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "o:CT_Fill/o:fill"))]
  pub fill_extended_properties:
    Option<crate::schemas::schemas_microsoft_com_office_office::FillExtendedProperties>,
}
/// Defines the Stroke Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Stroke/v:stroke")]
pub struct Stroke {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :linestyle
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miterlimit: Option<crate::simple_type::StringValue>,
  /// Line End Join Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<StrokeFillTypeValues>,
  /// Stroke Image Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :src
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imageaspect
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<ImageAspectValues>,
  /// Stroke Image Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imagesize
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imagealignshape
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<StrokeArrowLengthValues>,
  /// Original Image Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:href
  #[sdk(attr(qname = "o:href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Alternate Image Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:althref
  #[sdk(attr(qname = "o:althref"))]
  pub alternate_image_reference: Option<crate::simple_type::StringValue>,
  /// Stroke Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub insetpen: Option<crate::simple_type::TrueFalseValue>,
  /// _
  #[sdk(child(qname = "o:CT_StrokeChild/o:left"))]
  pub left_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::LeftStroke>,
  /// _
  #[sdk(child(qname = "o:CT_StrokeChild/o:top"))]
  pub top_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::TopStroke>,
  /// _
  #[sdk(child(qname = "o:CT_StrokeChild/o:right"))]
  pub right_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::RightStroke>,
  /// _
  #[sdk(child(qname = "o:CT_StrokeChild/o:bottom"))]
  pub bottom_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::BottomStroke>,
  /// _
  #[sdk(child(qname = "o:CT_StrokeChild/o:column"))]
  pub column_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::ColumnStroke>,
}
/// Defines the Shadow Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:shadow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shadow/v:shadow")]
pub struct Shadow {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shadow Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ShadowValues>,
  /// Shadow Transparency
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :obscured
  #[sdk(attr(qname = ":obscured"))]
  pub obscured: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Primary Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Shadow Opacity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Shadow Primary Offset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Shadow Secondary Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Shadow Secondary Offset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :offset2
  #[sdk(attr(qname = ":offset2"))]
  pub offset2: Option<crate::simple_type::StringValue>,
  /// Shadow Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Shadow Perspective Matrix
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :matrix
  #[sdk(attr(qname = ":matrix"))]
  pub matrix: Option<crate::simple_type::StringValue>,
}
/// Defines the TextBox Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:textbox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Textbox/v:textbox")]
pub struct TextBox {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Text Box Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :inset
  #[sdk(attr(qname = ":inset"))]
  pub inset: Option<crate::simple_type::StringValue>,
  /// Text Box Single-Click Selection Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:singleclick
  #[sdk(attr(qname = "o:singleclick"))]
  pub single_click: Option<crate::simple_type::TrueFalseValue>,
  #[sdk(choice(qname = "w:CT_TxbxContent/w:txbxContent", any))]
  pub xml_children: Option<TextBoxChoice>,
}
/// Defines the TextPath Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:textpath.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_TextPath/v:textpath")]
pub struct TextPath {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Text Path Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Fit Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitshape
  #[sdk(attr(qname = ":fitshape"))]
  pub fit_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Path Fit Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitpath
  #[sdk(attr(qname = ":fitpath"))]
  pub fit_path: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Trim Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :trim
  #[sdk(attr(qname = ":trim"))]
  pub trim: Option<crate::simple_type::TrueFalseValue>,
  /// Text X-Scaling
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xscale
  #[sdk(attr(qname = ":xscale"))]
  pub x_scale: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :string
  #[sdk(attr(qname = ":string"))]
  pub string: Option<crate::simple_type::StringValue>,
}
/// Defines the ImageData Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:imagedata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_ImageData/v:imagedata")]
pub struct ImageData {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Image Transparency Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :chromakey
  #[sdk(attr(qname = ":chromakey"))]
  pub chrom_a_key: Option<crate::simple_type::StringValue>,
  /// Image Left Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropleft
  #[sdk(attr(qname = ":cropleft"))]
  pub crop_left: Option<crate::simple_type::StringValue>,
  /// Image Top Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :croptop
  #[sdk(attr(qname = ":croptop"))]
  pub crop_top: Option<crate::simple_type::StringValue>,
  /// Image Right Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropright
  #[sdk(attr(qname = ":cropright"))]
  pub crop_right: Option<crate::simple_type::StringValue>,
  /// Image Bottom Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropbottom
  #[sdk(attr(qname = ":cropbottom"))]
  pub crop_bottom: Option<crate::simple_type::StringValue>,
  /// Image Intensity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gain
  #[sdk(attr(qname = ":gain"))]
  pub gain: Option<crate::simple_type::StringValue>,
  /// Image Brightness
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blacklevel
  #[sdk(attr(qname = ":blacklevel"))]
  pub black_level: Option<crate::simple_type::StringValue>,
  /// Image Gamma Correction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gamma
  #[sdk(attr(qname = ":gamma"))]
  pub gamma: Option<crate::simple_type::StringValue>,
  /// Image Grayscale Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grayscale
  #[sdk(attr(qname = ":grayscale"))]
  pub grayscale: Option<crate::simple_type::TrueFalseValue>,
  /// Image Bilevel Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bilevel
  #[sdk(attr(qname = ":bilevel"))]
  pub bi_level: Option<crate::simple_type::TrueFalseValue>,
  /// Embossed Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :embosscolor
  #[sdk(attr(qname = ":embosscolor"))]
  pub emboss_color: Option<crate::simple_type::StringValue>,
  /// Black Recoloring Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :recolortarget
  #[sdk(attr(qname = ":recolortarget"))]
  pub recolor_target: Option<crate::simple_type::StringValue>,
  /// Image Data Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Detect Mouse Click
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:detectmouseclick
  #[sdk(attr(qname = "o:detectmouseclick"))]
  pub detect_mouse_click: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship to Part
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:relid
  #[sdk(attr(qname = "o:relid"))]
  pub rel_id: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Image Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Alternate Image Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:pict
  #[sdk(attr(qname = "r:pict"))]
  pub picture: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:href
  #[sdk(attr(qname = "r:href"))]
  pub rel_href: Option<crate::simple_type::StringValue>,
}
/// Shape Definition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shape/v:shape")]
pub struct Shape {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Type Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Adjustment Parameters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :adj
  #[sdk(attr(qname = ":adj"))]
  pub adjustment: Option<crate::simple_type::StringValue>,
  /// Edge Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :path
  #[sdk(attr(qname = ":path"))]
  pub edge_path: Option<crate::simple_type::StringValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub encoded_package: Option<crate::simple_type::Base64BinaryValue>,
  /// Storage for Alternate Math Content
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :equationxml
  #[sdk(attr(qname = ":equationxml"))]
  pub equation_xml: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata",
    qname = "o:CT_Ink/o:ink",
    qname = "pvml:CT_Empty/pvml:iscomment"
  ))]
  pub xml_children: Vec<ShapeChoice>,
}
/// Shape Template.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:shapetype.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shapetype/v:shapetype")]
pub struct Shapetype {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Adjustment Parameters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :adj
  #[sdk(attr(qname = ":adj"))]
  pub adjustment: Option<crate::simple_type::StringValue>,
  /// Edge Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :path
  #[sdk(attr(qname = ":path"))]
  pub edge_path: Option<crate::simple_type::StringValue>,
  /// Master Element Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:master
  #[sdk(attr(qname = "o:master"))]
  pub master: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub shapetype_choice: Vec<ShapetypeChoice>,
  /// _
  #[sdk(child(qname = "o:CT_Complex/o:complex"))]
  pub o_complex: Option<crate::schemas::schemas_microsoft_com_office_office::Complex>,
}
/// Shape Group.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Group/v:group")]
pub struct Group {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// spid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// oned
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// regroupid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// doubleclicknotify
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// userhidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// hr
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// hrstd
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// hrnoshade
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// hrpct
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// hralign
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// allowincell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// allowoverlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// userdrawn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// dgmlayout
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// dgmnodekind
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// dgmlayoutmru
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// insetmode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Group Diagram Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :editas
  #[sdk(attr(qname = ":editas"))]
  pub edit_as: Option<EditAsValues>,
  /// Table Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:tableproperties
  #[sdk(attr(qname = "o:tableproperties"))]
  pub table_properties: Option<crate::simple_type::StringValue>,
  /// Table Row Height Limits
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:tablelimits
  #[sdk(attr(qname = "o:tablelimits"))]
  pub table_limits: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Group/v:group",
    qname = "v:CT_Shape/v:shape",
    qname = "v:CT_Shapetype/v:shapetype",
    qname = "v:CT_Arc/v:arc",
    qname = "v:CT_Curve/v:curve",
    qname = "v:CT_Image/v:image",
    qname = "v:CT_Line/v:line",
    qname = "v:CT_Oval/v:oval",
    qname = "v:CT_PolyLine/v:polyline",
    qname = "v:CT_Rect/v:rect",
    qname = "v:CT_RoundRect/v:roundrect",
    qname = "o:CT_Diagram/o:diagram",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "xvml:CT_ClientData/xvml:ClientData"
  ))]
  pub xml_children: Vec<GroupChoice>,
}
/// Document Background.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Background/v:background")]
pub struct Background {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, max = 255u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fill
  #[sdk(attr(qname = ":fill"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fillcolor: Option<crate::simple_type::StringValue>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Target Screen Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:targetscreensize
  #[sdk(attr(qname = "o:targetscreensize"))]
  pub target_screen_size:
    Option<crate::schemas::schemas_microsoft_com_office_office::ScreenSizeValues>,
  /// _
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
}
/// Arc Segment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:arc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Arc/v:arc")]
pub struct Arc {
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, max = 255u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrapcoords: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Starting Angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startangle
  #[sdk(attr(qname = ":startangle"))]
  pub start_angle: Option<crate::simple_type::DecimalValue>,
  /// Ending Angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endangle
  #[sdk(attr(qname = ":endangle"))]
  pub end_angle: Option<crate::simple_type::DecimalValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub arc_choice: Vec<ArcChoice>,
}
/// Bezier Curve.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:curve.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Curve/v:curve")]
pub struct Curve {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Curve Starting Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// First Curve Control Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :control1
  #[sdk(attr(qname = ":control1"))]
  pub control1: Option<crate::simple_type::StringValue>,
  /// Second Curve Control Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :control2
  #[sdk(attr(qname = ":control2"))]
  pub control2: Option<crate::simple_type::StringValue>,
  /// Curve Ending Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub curve_choice: Vec<CurveChoice>,
}
/// Image File.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:image.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Image/v:image")]
pub struct ImageFile {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// href
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// class
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// alt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// coordsize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// wrapcoords
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// print
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Image Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :src
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Image Left Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropleft
  #[sdk(attr(qname = ":cropleft"))]
  pub crop_left: Option<crate::simple_type::StringValue>,
  /// Image Top Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :croptop
  #[sdk(attr(qname = ":croptop"))]
  pub crop_top: Option<crate::simple_type::StringValue>,
  /// Image Right Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropright
  #[sdk(attr(qname = ":cropright"))]
  pub crop_right: Option<crate::simple_type::StringValue>,
  /// Image Bottom Crop
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropbottom
  #[sdk(attr(qname = ":cropbottom"))]
  pub crop_bottom: Option<crate::simple_type::StringValue>,
  /// Image Intensity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gain
  #[sdk(attr(qname = ":gain"))]
  pub gain: Option<crate::simple_type::StringValue>,
  /// Image Brightness
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blacklevel
  #[sdk(attr(qname = ":blacklevel"))]
  pub black_level: Option<crate::simple_type::StringValue>,
  /// Image Gamma Correction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gamma
  #[sdk(attr(qname = ":gamma"))]
  pub gamma: Option<crate::simple_type::StringValue>,
  /// Image Grayscale Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grayscale
  #[sdk(attr(qname = ":grayscale"))]
  pub gray_scale: Option<crate::simple_type::TrueFalseValue>,
  /// Image Bilevel Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bilevel
  #[sdk(attr(qname = ":bilevel"))]
  pub bi_level: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub image_file_choice: Vec<ImageFileChoice>,
}
/// Line.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Line/v:line")]
pub struct Line {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Line Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// Line End Point
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub line_choice: Vec<LineChoice>,
}
/// Oval.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:oval.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Oval/v:oval")]
pub struct Oval {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub xml_children: Vec<OvalChoice>,
}
/// Multiple Path Line.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:polyline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_PolyLine/v:polyline")]
pub struct PolyLine {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Points for Compound Line
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :points
  #[sdk(attr(qname = ":points"))]
  pub points: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata",
    qname = "o:CT_Ink/o:ink"
  ))]
  pub xml_children: Vec<PolyLineChoice>,
}
/// Rectangle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:rect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Rect/v:rect")]
pub struct Rectangle {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordorigin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub xml_children: Vec<RectangleChoice>,
}
/// Rounded Rectangle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:roundrect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_RoundRect/v:roundrect")]
pub struct RoundRectangle {
  /// Unique Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// href
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// class
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// alt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// coordsize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// wrapcoords
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// print
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bordertopcolor
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderleftcolor
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderbottomcolor
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:borderrightcolor
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filled
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stroked
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokeweight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:spt
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "202",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:connectortype
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwmode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwpure
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:bwnormal
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:forcedash
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:oleicon
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:ole
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:preferrelative
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:cliptowrap
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:clip
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:gfxdata
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Rounded Corner Arc Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :arcsize
  #[sdk(attr(qname = ":arcsize"))]
  pub arc_size: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "v:CT_Path/v:path",
    qname = "v:CT_Formulas/v:formulas",
    qname = "v:CT_Handles/v:handles",
    qname = "v:CT_Fill/v:fill",
    qname = "v:CT_Stroke/v:stroke",
    qname = "v:CT_Shadow/v:shadow",
    qname = "v:CT_Textbox/v:textbox",
    qname = "v:CT_TextPath/v:textpath",
    qname = "v:CT_ImageData/v:imagedata",
    qname = "o:CT_Skew/o:skew",
    qname = "o:CT_Extrusion/o:extrusion",
    qname = "o:CT_Callout/o:callout",
    qname = "o:CT_Lock/o:lock",
    qname = "o:CT_ClipPath/o:clippath",
    qname = "o:CT_SignatureLine/o:signatureline",
    qname = "w10:CT_Wrap/w10:wrap",
    qname = "w10:CT_AnchorLock/w10:anchorlock",
    qname = "w10:CT_Border/w10:bordertop",
    qname = "w10:CT_Border/w10:borderbottom",
    qname = "w10:CT_Border/w10:borderleft",
    qname = "w10:CT_Border/w10:borderright",
    qname = "xvml:CT_ClientData/xvml:ClientData",
    qname = "pvml:CT_Rel/pvml:textdata"
  ))]
  pub xml_children: Vec<RoundRectangleChoice>,
}
/// Shape Handle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:h.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_H/v:h")]
pub struct ShapeHandle {
  /// Handle Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::StringValue>,
  /// Handle Polar Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :polar
  #[sdk(attr(qname = ":polar"))]
  pub polar: Option<crate::simple_type::StringValue>,
  /// Handle Coordinate Mapping
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :map
  #[sdk(attr(qname = ":map"))]
  pub map: Option<crate::simple_type::StringValue>,
  /// Invert Handle's X Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :invx
  #[sdk(attr(qname = ":invx"))]
  pub invert_x: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Invert Handle's Y Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :invy
  #[sdk(attr(qname = ":invy"))]
  pub invert_y: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Handle Inversion Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :switch
  #[sdk(attr(qname = ":switch"))]
  pub switch: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Handle X Position Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xrange
  #[sdk(attr(qname = ":xrange"))]
  pub x_range: Option<crate::simple_type::StringValue>,
  /// Handle Y Position Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :yrange
  #[sdk(attr(qname = ":yrange"))]
  pub y_range: Option<crate::simple_type::StringValue>,
  /// Handle Polar Radius Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :radiusrange
  #[sdk(attr(qname = ":radiusrange"))]
  pub radius_range: Option<crate::simple_type::StringValue>,
}
/// Single Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is v:f.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_F/v:f")]
pub struct Formula {
  /// Equation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :eqn
  #[sdk(attr(qname = ":eqn"))]
  pub equation: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBoxChoice {
  #[sdk(child(qname = "w:CT_TxbxContent/w:txbxContent"))]
  WTxbxContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
  #[sdk(child(qname = "o:CT_Ink/o:ink"))]
  OInk(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Ink>),
  /// Ink Annotation Flag.
  #[sdk(empty_child(qname = "pvml:CT_Empty/pvml:iscomment"))]
  PvmlIscomment,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapetypeChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupChoice {
  /// Shape Group.
  #[sdk(child(qname = "v:CT_Group/v:group"))]
  VGroup(std::boxed::Box<Group>),
  /// Shape Definition.
  #[sdk(child(qname = "v:CT_Shape/v:shape"))]
  VShape(std::boxed::Box<Shape>),
  /// Shape Template.
  #[sdk(child(qname = "v:CT_Shapetype/v:shapetype"))]
  VShapetype(std::boxed::Box<Shapetype>),
  /// Arc Segment.
  #[sdk(child(qname = "v:CT_Arc/v:arc"))]
  VArc(std::boxed::Box<Arc>),
  /// Bezier Curve.
  #[sdk(child(qname = "v:CT_Curve/v:curve"))]
  VCurve(std::boxed::Box<Curve>),
  /// Image File.
  #[sdk(child(qname = "v:CT_Image/v:image"))]
  VImage(std::boxed::Box<ImageFile>),
  /// Line.
  #[sdk(child(qname = "v:CT_Line/v:line"))]
  VLine(std::boxed::Box<Line>),
  /// Oval.
  #[sdk(child(qname = "v:CT_Oval/v:oval"))]
  VOval(std::boxed::Box<Oval>),
  /// Multiple Path Line.
  #[sdk(child(qname = "v:CT_PolyLine/v:polyline"))]
  VPolyline(std::boxed::Box<PolyLine>),
  /// Rectangle.
  #[sdk(child(qname = "v:CT_Rect/v:rect"))]
  VRect(std::boxed::Box<Rectangle>),
  /// Rounded Rectangle.
  #[sdk(child(qname = "v:CT_RoundRect/v:roundrect"))]
  VRoundrect(std::boxed::Box<RoundRectangle>),
  /// VML Diagram.
  #[sdk(child(qname = "o:CT_Diagram/o:diagram"))]
  ODiagram(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Diagram>),
  /// Defines the Lock Class.
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  /// Shape Clipping Path.
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  /// Text Wrapping.
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  /// Attached Object Data.
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ArcChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CurveChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ImageFileChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OvalChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PolyLineChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
  #[sdk(child(qname = "o:CT_Ink/o:ink"))]
  OInk(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Ink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RectangleChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RoundRectangleChoice {
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
