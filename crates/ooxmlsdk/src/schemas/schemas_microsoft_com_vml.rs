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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Path/v:path")]
pub struct Path {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Path Definition
  #[sdk(attr(qname = ":v"))]
  pub value: Option<crate::simple_type::StringValue>,
  /// Limo Stretch Point
  #[sdk(attr(qname = ":limo"))]
  pub limo: Option<crate::simple_type::StringValue>,
  /// Text Box Bounding Box
  #[sdk(attr(qname = ":textboxrect"))]
  pub textbox_rectangle: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":fillok"))]
  pub allow_fill: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":strokeok"))]
  pub allow_stroke: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Toggle
  #[sdk(attr(qname = ":shadowok"))]
  pub allow_shading: Option<crate::simple_type::TrueFalseValue>,
  /// Arrowhead Display Toggle
  #[sdk(attr(qname = ":arrowok"))]
  pub show_arrowhead: Option<crate::simple_type::TrueFalseValue>,
  /// Gradient Shape Toggle
  #[sdk(attr(qname = ":gradientshapeok"))]
  pub allow_gradient_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Toggle
  #[sdk(attr(qname = ":textpathok"))]
  pub allow_text_path: Option<crate::simple_type::TrueFalseValue>,
  /// Inset Stroke From Path Flag
  #[sdk(attr(qname = ":insetpenok"))]
  pub allow_inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Connection Point Type
  #[sdk(attr(qname = "o:connecttype"))]
  pub connection_point_type:
    Option<crate::schemas::schemas_microsoft_com_office_office::ConnectValues>,
  /// Connection Points
  #[sdk(attr(qname = "o:connectlocs"))]
  pub connection_points: Option<crate::simple_type::StringValue>,
  /// Connection Point Connect Angles
  #[sdk(attr(qname = "o:connectangles"))]
  pub connect_angles: Option<crate::simple_type::StringValue>,
  /// Extrusion Toggle
  #[sdk(attr(qname = "o:extrusionok"))]
  pub allow_extrusion: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Formulas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Formulas/v:formulas")]
pub struct Formulas {
  /// Single Formula.
  #[sdk(child(qname = "v:CT_F/v:f"))]
  pub v_f: Vec<Formula>,
}
/// Defines the ShapeHandles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Handles/v:handles")]
pub struct ShapeHandles {
  /// Shape Handle.
  #[sdk(child(qname = "v:CT_H/v:h"))]
  pub v_h: Vec<ShapeHandle>,
}
/// Defines the Fill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Fill/v:fill")]
pub struct Fill {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Fill Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<FillTypeValues>,
  /// Fill Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Primary Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Primary Color Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Secondary Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Fill Image Source
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = "o:href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Alternate Image Reference Location
  #[sdk(attr(qname = "o:althref"))]
  pub alternate_image_reference: Option<crate::simple_type::StringValue>,
  /// Fill Image Size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::StringValue>,
  /// Fill Image Origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Fill Image Position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::StringValue>,
  /// Image Aspect Ratio
  #[sdk(attr(qname = ":aspect"))]
  pub aspect: Option<ImageAspectValues>,
  /// Intermediate Colors
  #[sdk(attr(qname = ":colors"))]
  pub colors: Option<crate::simple_type::StringValue>,
  /// Gradient Angle
  #[sdk(attr(qname = ":angle"))]
  pub angle: Option<crate::simple_type::DecimalValue>,
  /// Align Image With Shape
  #[sdk(attr(qname = ":alignshape"))]
  pub align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Gradient Center
  #[sdk(attr(qname = ":focus"))]
  pub focus: Option<crate::simple_type::StringValue>,
  /// Radial Gradient Size
  #[sdk(attr(qname = ":focussize"))]
  pub focus_size: Option<crate::simple_type::StringValue>,
  /// Radial Gradient Center
  #[sdk(attr(qname = ":focusposition"))]
  pub focus_position: Option<crate::simple_type::StringValue>,
  /// Gradient Fill Method
  #[sdk(attr(qname = ":method"))]
  pub method: Option<FillMethodValues>,
  /// Detect Mouse Click
  #[sdk(attr(qname = "o:detectmouseclick"))]
  pub detect_mouse_click: Option<crate::simple_type::TrueFalseValue>,
  /// Title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Secondary Color Opacity
  #[sdk(attr(qname = "o:opacity2"))]
  pub opacity2: Option<crate::simple_type::StringValue>,
  /// Recolor Fill as Picture
  #[sdk(attr(qname = ":recolor"))]
  pub recolor: Option<crate::simple_type::TrueFalseValue>,
  /// Rotate Fill with Shape
  #[sdk(attr(qname = ":rotate"))]
  pub rotate: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// Shape Fill Extended Properties.
  #[sdk(child(qname = "o:CT_Fill/o:fill"))]
  pub fill_extended_properties:
    Option<crate::schemas::schemas_microsoft_com_office_office::FillExtendedProperties>,
}
/// Defines the Stroke Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Stroke/v:stroke")]
pub struct Stroke {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miterlimit: Option<crate::simple_type::StringValue>,
  /// Line End Join Style
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<StrokeFillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<StrokeArrowLengthValues>,
  /// Original Image Reference
  #[sdk(attr(qname = "o:href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Alternate Image Reference
  #[sdk(attr(qname = "o:althref"))]
  pub alternate_image_reference: Option<crate::simple_type::StringValue>,
  /// Stroke Title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub insetpen: Option<crate::simple_type::TrueFalseValue>,
  /// Text Box Left Stroke.
  #[sdk(child(qname = "o:CT_StrokeChild/o:left"))]
  pub left_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::LeftStroke>,
  /// Text Box Top Stroke.
  #[sdk(child(qname = "o:CT_StrokeChild/o:top"))]
  pub top_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::TopStroke>,
  /// Text Box Right Stroke.
  #[sdk(child(qname = "o:CT_StrokeChild/o:right"))]
  pub right_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::RightStroke>,
  /// Text Box Bottom Stroke.
  #[sdk(child(qname = "o:CT_StrokeChild/o:bottom"))]
  pub bottom_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::BottomStroke>,
  /// Text Box Interior Stroke.
  #[sdk(child(qname = "o:CT_StrokeChild/o:column"))]
  pub column_stroke: Option<crate::schemas::schemas_microsoft_com_office_office::ColumnStroke>,
}
/// Defines the Shadow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shadow/v:shadow")]
pub struct Shadow {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shadow Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ShadowValues>,
  /// Shadow Transparency
  #[sdk(attr(qname = ":obscured"))]
  pub obscured: Option<crate::simple_type::TrueFalseValue>,
  /// Shadow Primary Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Shadow Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Shadow Primary Offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Shadow Secondary Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Shadow Secondary Offset
  #[sdk(attr(qname = ":offset2"))]
  pub offset2: Option<crate::simple_type::StringValue>,
  /// Shadow Origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Shadow Perspective Matrix
  #[sdk(attr(qname = ":matrix"))]
  pub matrix: Option<crate::simple_type::StringValue>,
}
/// Defines the TextBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Textbox/v:textbox")]
pub struct TextBox {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Text Box Inset
  #[sdk(attr(qname = ":inset"))]
  pub inset: Option<crate::simple_type::StringValue>,
  /// Text Box Single-Click Selection Toggle
  #[sdk(attr(qname = "o:singleclick"))]
  pub single_click: Option<crate::simple_type::TrueFalseValue>,
  #[sdk(choice(qname = "w:CT_TxbxContent/w:txbxContent", any))]
  pub text_box_choice: Option<TextBoxChoice>,
}
/// Defines the TextPath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_TextPath/v:textpath")]
pub struct TextPath {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Text Path Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Fit Toggle
  #[sdk(attr(qname = ":fitshape"))]
  pub fit_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Path Fit Toggle
  #[sdk(attr(qname = ":fitpath"))]
  pub fit_path: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Trim Toggle
  #[sdk(attr(qname = ":trim"))]
  pub trim: Option<crate::simple_type::TrueFalseValue>,
  /// Text X-Scaling
  #[sdk(attr(qname = ":xscale"))]
  pub x_scale: Option<crate::simple_type::TrueFalseValue>,
  /// Text Path Text
  #[sdk(attr(qname = ":string"))]
  pub string: Option<crate::simple_type::StringValue>,
}
/// Defines the ImageData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_ImageData/v:imagedata")]
pub struct ImageData {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Image Transparency Color
  #[sdk(attr(qname = ":chromakey"))]
  pub chrom_a_key: Option<crate::simple_type::StringValue>,
  /// Image Left Crop
  #[sdk(attr(qname = ":cropleft"))]
  pub crop_left: Option<crate::simple_type::StringValue>,
  /// Image Top Crop
  #[sdk(attr(qname = ":croptop"))]
  pub crop_top: Option<crate::simple_type::StringValue>,
  /// Image Right Crop
  #[sdk(attr(qname = ":cropright"))]
  pub crop_right: Option<crate::simple_type::StringValue>,
  /// Image Bottom Crop
  #[sdk(attr(qname = ":cropbottom"))]
  pub crop_bottom: Option<crate::simple_type::StringValue>,
  /// Image Intensity
  #[sdk(attr(qname = ":gain"))]
  pub gain: Option<crate::simple_type::StringValue>,
  /// Image Brightness
  #[sdk(attr(qname = ":blacklevel"))]
  pub black_level: Option<crate::simple_type::StringValue>,
  /// Image Gamma Correction
  #[sdk(attr(qname = ":gamma"))]
  pub gamma: Option<crate::simple_type::StringValue>,
  /// Image Grayscale Toggle
  #[sdk(attr(qname = ":grayscale"))]
  pub grayscale: Option<crate::simple_type::TrueFalseValue>,
  /// Image Bilevel Toggle
  #[sdk(attr(qname = ":bilevel"))]
  pub bi_level: Option<crate::simple_type::TrueFalseValue>,
  /// Embossed Color
  #[sdk(attr(qname = ":embosscolor"))]
  pub emboss_color: Option<crate::simple_type::StringValue>,
  /// Black Recoloring Color
  #[sdk(attr(qname = ":recolortarget"))]
  pub recolor_target: Option<crate::simple_type::StringValue>,
  /// Image Data Title
  #[sdk(attr(qname = "o:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Detect Mouse Click
  #[sdk(attr(qname = "o:detectmouseclick"))]
  pub detect_mouse_click: Option<crate::simple_type::TrueFalseValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "o:relid"))]
  pub rel_id: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Image Data
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Alternate Image Data
  #[sdk(attr(qname = "r:pict"))]
  pub picture: Option<crate::simple_type::StringValue>,
  /// Explicit Relationship to Hyperlink Target
  #[sdk(attr(qname = "r:href"))]
  pub rel_href: Option<crate::simple_type::StringValue>,
}
/// Shape Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shape/v:shape")]
pub struct Shape {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Type Reference
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Adjustment Parameters
  #[sdk(attr(qname = ":adj"))]
  pub adjustment: Option<crate::simple_type::StringValue>,
  /// Edge Path
  #[sdk(attr(qname = ":path"))]
  pub edge_path: Option<crate::simple_type::StringValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub encoded_package: Option<crate::simple_type::Base64BinaryValue>,
  /// Storage for Alternate Math Content
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
  pub shape_choice: Vec<ShapeChoice>,
}
/// Shape Template.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Shapetype/v:shapetype")]
pub struct Shapetype {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Adjustment Parameters
  #[sdk(attr(qname = ":adj"))]
  pub adjustment: Option<crate::simple_type::StringValue>,
  /// Edge Path
  #[sdk(attr(qname = ":path"))]
  pub edge_path: Option<crate::simple_type::StringValue>,
  /// Master Element Toggle
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
  /// Complex.
  #[sdk(child(qname = "o:CT_Complex/o:complex"))]
  pub o_complex: Option<crate::schemas::schemas_microsoft_com_office_office::Complex>,
}
/// Shape Group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Group/v:group")]
pub struct Group {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// spid
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// oned
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// regroupid
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// doubleclicknotify
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// button
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// userhidden
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// hr
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// hrstd
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// hrnoshade
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// hrpct
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// hralign
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// allowincell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// userdrawn
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// dgmlayout
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// dgmnodekind
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// dgmlayoutmru
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Group Diagram Type
  #[sdk(attr(qname = ":editas"))]
  pub edit_as: Option<EditAsValues>,
  /// Table Properties
  #[sdk(attr(qname = "o:tableproperties"))]
  pub table_properties: Option<crate::simple_type::StringValue>,
  /// Table Row Height Limits
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
  pub group_choice: Vec<GroupChoice>,
}
/// Document Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Background/v:background")]
pub struct Background {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(max = 255u32))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":fill"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fillcolor: Option<crate::simple_type::StringValue>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Target Screen Size
  #[sdk(attr(qname = "o:targetscreensize"))]
  pub target_screen_size:
    Option<crate::schemas::schemas_microsoft_com_office_office::ScreenSizeValues>,
  /// Defines the Fill Class.
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
}
/// Arc Segment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Arc/v:arc")]
pub struct Arc {
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(max = 255u32))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrapcoords: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Starting Angle
  #[sdk(attr(qname = ":startangle"))]
  pub start_angle: Option<crate::simple_type::DecimalValue>,
  /// Ending Angle
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Curve/v:curve")]
pub struct Curve {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Curve Starting Point
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// First Curve Control Point
  #[sdk(attr(qname = ":control1"))]
  pub control1: Option<crate::simple_type::StringValue>,
  /// Second Curve Control Point
  #[sdk(attr(qname = ":control2"))]
  pub control2: Option<crate::simple_type::StringValue>,
  /// Curve Ending Point
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Image/v:image")]
pub struct ImageFile {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Image Source
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Image Left Crop
  #[sdk(attr(qname = ":cropleft"))]
  pub crop_left: Option<crate::simple_type::StringValue>,
  /// Image Top Crop
  #[sdk(attr(qname = ":croptop"))]
  pub crop_top: Option<crate::simple_type::StringValue>,
  /// Image Right Crop
  #[sdk(attr(qname = ":cropright"))]
  pub crop_right: Option<crate::simple_type::StringValue>,
  /// Image Bottom Crop
  #[sdk(attr(qname = ":cropbottom"))]
  pub crop_bottom: Option<crate::simple_type::StringValue>,
  /// Image Intensity
  #[sdk(attr(qname = ":gain"))]
  pub gain: Option<crate::simple_type::StringValue>,
  /// Image Brightness
  #[sdk(attr(qname = ":blacklevel"))]
  pub black_level: Option<crate::simple_type::StringValue>,
  /// Image Gamma Correction
  #[sdk(attr(qname = ":gamma"))]
  pub gamma: Option<crate::simple_type::StringValue>,
  /// Image Grayscale Toggle
  #[sdk(attr(qname = ":grayscale"))]
  pub gray_scale: Option<crate::simple_type::TrueFalseValue>,
  /// Image Bilevel Toggle
  #[sdk(attr(qname = ":bilevel"))]
  pub bi_level: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Line/v:line")]
pub struct Line {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Line Start
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// Line End Point
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Oval/v:oval")]
pub struct Oval {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
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
  pub oval_choice: Vec<OvalChoice>,
}
/// Multiple Path Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_PolyLine/v:polyline")]
pub struct PolyLine {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Points for Compound Line
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
  pub poly_line_choice: Vec<PolyLineChoice>,
}
/// Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_Rect/v:rect")]
pub struct Rectangle {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Shape Styling Properties
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// Hyperlink Display Target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// CSS Reference
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// Shape Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Alternate Text
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Size
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// Coordinate Space Origin
  #[sdk(attr(qname = ":coordorigin"))]
  pub coordinate_origin: Option<crate::simple_type::StringValue>,
  /// Shape Bounding Polygon
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// Print Toggle
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
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
  pub rectangle_choice: Vec<RectangleChoice>,
}
/// Rounded Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_RoundRect/v:roundrect")]
pub struct RoundRectangle {
  /// Unique Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// href
  #[sdk(attr(qname = ":href"))]
  pub href: Option<crate::simple_type::StringValue>,
  /// target
  #[sdk(attr(qname = ":target"))]
  pub target: Option<crate::simple_type::StringValue>,
  /// class
  #[sdk(attr(qname = ":class"))]
  pub class: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// alt
  #[sdk(attr(qname = ":alt"))]
  pub alternate: Option<crate::simple_type::StringValue>,
  /// coordsize
  #[sdk(attr(qname = ":coordsize"))]
  pub coordinate_size: Option<crate::simple_type::StringValue>,
  /// wrapcoords
  #[sdk(attr(qname = ":wrapcoords"))]
  pub wrap_coordinates: Option<crate::simple_type::StringValue>,
  /// print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::TrueFalseValue>,
  /// Optional String
  #[sdk(attr(qname = "o:spid"))]
  pub optional_string: Option<crate::simple_type::StringValue>,
  /// Shape Handle Toggle
  #[sdk(attr(qname = "o:oned"))]
  pub oned: Option<crate::simple_type::TrueFalseValue>,
  /// Regroup ID
  #[sdk(attr(qname = "o:regroupid"))]
  pub regroup_id: Option<crate::simple_type::IntegerValue>,
  /// Double-click Notification Toggle
  #[sdk(attr(qname = "o:doubleclicknotify"))]
  pub double_click_notify: Option<crate::simple_type::TrueFalseValue>,
  /// Button Behavior Toggle
  #[sdk(attr(qname = "o:button"))]
  pub button: Option<crate::simple_type::TrueFalseValue>,
  /// Hide Script Anchors
  #[sdk(attr(qname = "o:userhidden"))]
  pub user_hidden: Option<crate::simple_type::TrueFalseValue>,
  /// Graphical Bullet
  #[sdk(attr(qname = "o:bullet"))]
  pub bullet: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Toggle
  #[sdk(attr(qname = "o:hr"))]
  pub horizontal: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Standard Display Toggle
  #[sdk(attr(qname = "o:hrstd"))]
  pub horizontal_standard: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule 3D Shading Toggle
  #[sdk(attr(qname = "o:hrnoshade"))]
  pub horizontal_no_shade: Option<crate::simple_type::TrueFalseValue>,
  /// Horizontal Rule Length Percentage
  #[sdk(attr(qname = "o:hrpct"))]
  pub horizontal_percentage: Option<crate::simple_type::SingleValue>,
  /// Horizontal Rule Alignment
  #[sdk(attr(qname = "o:hralign"))]
  pub horizontal_alignment:
    Option<crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// Allow Shape Overlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// Exists In Master Slide
  #[sdk(attr(qname = "o:userdrawn"))]
  pub user_drawn: Option<crate::simple_type::TrueFalseValue>,
  /// Border Top Color
  #[sdk(attr(qname = "o:bordertopcolor"))]
  pub border_top_color: Option<crate::simple_type::StringValue>,
  /// Border Left Color
  #[sdk(attr(qname = "o:borderleftcolor"))]
  pub border_left_color: Option<crate::simple_type::StringValue>,
  /// Bottom Border Color
  #[sdk(attr(qname = "o:borderbottomcolor"))]
  pub border_bottom_color: Option<crate::simple_type::StringValue>,
  /// Border Right Color
  #[sdk(attr(qname = "o:borderrightcolor"))]
  pub border_right_color: Option<crate::simple_type::StringValue>,
  /// Diagram Node Layout Identifier
  #[sdk(attr(qname = "o:dgmlayout"))]
  pub diagram_layout: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Identifier
  #[sdk(attr(qname = "o:dgmnodekind"))]
  pub diagram_node_kind: Option<crate::simple_type::IntegerValue>,
  /// Diagram Node Recent Layout Identifier
  #[sdk(attr(qname = "o:dgmlayoutmru"))]
  pub diagram_layout_most_recent_used: Option<crate::simple_type::IntegerValue>,
  /// Text Inset Mode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":filled"))]
  pub filled: Option<crate::simple_type::TrueFalseValue>,
  /// Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroked"))]
  pub stroked: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Weight
  #[sdk(attr(qname = ":strokeweight"))]
  pub stroke_weight: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Optional Number
  #[sdk(attr(qname = "o:spt"))]
  #[sdk(number_range(range = 0..= 202))]
  pub optional_number: Option<crate::simple_type::Int32Value>,
  /// Shape Connector Type
  #[sdk(attr(qname = "o:connectortype"))]
  pub connector_type: Option<crate::schemas::schemas_microsoft_com_office_office::ConnectorValues>,
  /// Black-and-White Mode
  #[sdk(attr(qname = "o:bwmode"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Pure Black-and-White Mode
  #[sdk(attr(qname = "o:bwpure"))]
  pub pure_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Normal Black-and-White Mode
  #[sdk(attr(qname = "o:bwnormal"))]
  pub normal_black_white_mode:
    Option<crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues>,
  /// Force Dashed Outline
  #[sdk(attr(qname = "o:forcedash"))]
  pub force_dash: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Icon Toggle
  #[sdk(attr(qname = "o:oleicon"))]
  pub ole_icon: Option<crate::simple_type::TrueFalseValue>,
  /// Embedded Object Toggle
  #[sdk(attr(qname = "o:ole"))]
  pub ole: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Relative Resize Toggle
  #[sdk(attr(qname = "o:preferrelative"))]
  pub prefer_relative: Option<crate::simple_type::TrueFalseValue>,
  /// Clip to Wrapping Polygon
  #[sdk(attr(qname = "o:cliptowrap"))]
  pub clip_to_wrap: Option<crate::simple_type::TrueFalseValue>,
  /// Clipping Toggle
  #[sdk(attr(qname = "o:clip"))]
  pub clip: Option<crate::simple_type::TrueFalseValue>,
  /// Encoded Package
  #[sdk(attr(qname = "o:gfxdata"))]
  pub o_gfxdata: Option<crate::simple_type::Base64BinaryValue>,
  /// Rounded Corner Arc Size
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
  pub round_rectangle_choice: Vec<RoundRectangleChoice>,
}
/// Shape Handle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_H/v:h")]
pub struct ShapeHandle {
  /// Handle Position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::StringValue>,
  /// Handle Polar Center
  #[sdk(attr(qname = ":polar"))]
  pub polar: Option<crate::simple_type::StringValue>,
  /// Handle Coordinate Mapping
  #[sdk(attr(qname = ":map"))]
  pub map: Option<crate::simple_type::StringValue>,
  /// Invert Handle's X Position
  #[sdk(attr(qname = ":invx"))]
  pub invert_x: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Invert Handle's Y Position
  #[sdk(attr(qname = ":invy"))]
  pub invert_y: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Handle Inversion Toggle
  #[sdk(attr(qname = ":switch"))]
  pub switch: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Handle X Position Range
  #[sdk(attr(qname = ":xrange"))]
  pub x_range: Option<crate::simple_type::StringValue>,
  /// Handle Y Position Range
  #[sdk(attr(qname = ":yrange"))]
  pub y_range: Option<crate::simple_type::StringValue>,
  /// Handle Polar Radius Range
  #[sdk(attr(qname = ":radiusrange"))]
  pub radius_range: Option<crate::simple_type::StringValue>,
}
/// Single Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "v:CT_F/v:f")]
pub struct Formula {
  /// Equation
  #[sdk(attr(qname = ":eqn"))]
  pub equation: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBoxChoice {
  /// Rich Text Box Content Container.
  #[sdk(child(qname = "w:CT_TxbxContent/w:txbxContent"))]
  WTxbxContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
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
  /// Defines the Path Class.
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  /// Defines the Formulas Class.
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  /// Defines the ShapeHandles Class.
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  /// Defines the Fill Class.
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  /// Defines the Stroke Class.
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  /// Defines the Shadow Class.
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  /// Defines the TextBox Class.
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  /// Defines the TextPath Class.
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  /// Defines the ImageData Class.
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  /// Skew Transform.
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  /// 3D Extrusion.
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  /// Defines the Callout Class.
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  /// Defines the Lock Class.
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  /// Shape Clipping Path.
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  /// Digital Signature Line.
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  /// Text Wrapping.
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  /// Top Border.
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  /// Bottom Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  /// Left Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  /// Right Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  /// Attached Object Data.
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  /// VML Diagram Text.
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CurveChoice {
  /// Defines the Path Class.
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  /// Defines the Formulas Class.
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  /// Defines the ShapeHandles Class.
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  /// Defines the Fill Class.
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  /// Defines the Stroke Class.
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  /// Defines the Shadow Class.
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  /// Defines the TextBox Class.
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  /// Defines the TextPath Class.
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  /// Defines the ImageData Class.
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  /// Skew Transform.
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  /// 3D Extrusion.
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  /// Defines the Callout Class.
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  /// Defines the Lock Class.
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  /// Shape Clipping Path.
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  /// Digital Signature Line.
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  /// Text Wrapping.
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  /// Top Border.
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  /// Bottom Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  /// Left Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  /// Right Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  /// Attached Object Data.
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  /// VML Diagram Text.
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ImageFileChoice {
  /// Defines the Path Class.
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  /// Defines the Formulas Class.
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  /// Defines the ShapeHandles Class.
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  /// Defines the Fill Class.
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  /// Defines the Stroke Class.
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  /// Defines the Shadow Class.
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  /// Defines the TextBox Class.
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  /// Defines the TextPath Class.
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  /// Defines the ImageData Class.
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  /// Skew Transform.
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  /// 3D Extrusion.
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  /// Defines the Callout Class.
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  /// Defines the Lock Class.
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  /// Shape Clipping Path.
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  /// Digital Signature Line.
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  /// Text Wrapping.
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  /// Top Border.
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  /// Bottom Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  /// Left Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  /// Right Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  /// Attached Object Data.
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  /// VML Diagram Text.
  #[sdk(child(qname = "pvml:CT_Rel/pvml:textdata"))]
  PvmlTextdata(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint::TextData>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineChoice {
  /// Defines the Path Class.
  #[sdk(child(qname = "v:CT_Path/v:path"))]
  VPath(std::boxed::Box<Path>),
  /// Defines the Formulas Class.
  #[sdk(child(qname = "v:CT_Formulas/v:formulas"))]
  VFormulas(std::boxed::Box<Formulas>),
  /// Defines the ShapeHandles Class.
  #[sdk(child(qname = "v:CT_Handles/v:handles"))]
  VHandles(std::boxed::Box<ShapeHandles>),
  /// Defines the Fill Class.
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  VFill(std::boxed::Box<Fill>),
  /// Defines the Stroke Class.
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  VStroke(std::boxed::Box<Stroke>),
  /// Defines the Shadow Class.
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  VShadow(std::boxed::Box<Shadow>),
  /// Defines the TextBox Class.
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  VTextbox(std::boxed::Box<TextBox>),
  /// Defines the TextPath Class.
  #[sdk(child(qname = "v:CT_TextPath/v:textpath"))]
  VTextpath(std::boxed::Box<TextPath>),
  /// Defines the ImageData Class.
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  VImagedata(std::boxed::Box<ImageData>),
  /// Skew Transform.
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  OSkew(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Skew>),
  /// 3D Extrusion.
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  OExtrusion(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Extrusion>),
  /// Defines the Callout Class.
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  OCallout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Callout>),
  /// Defines the Lock Class.
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  OLock(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::Lock>),
  /// Shape Clipping Path.
  #[sdk(child(qname = "o:CT_ClipPath/o:clippath"))]
  OClippath(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ClipPath>),
  /// Digital Signature Line.
  #[sdk(child(qname = "o:CT_SignatureLine/o:signatureline"))]
  OSignatureline(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::SignatureLine>,
  ),
  /// Text Wrapping.
  #[sdk(child(qname = "w10:CT_Wrap/w10:wrap"))]
  W10Wrap(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TextWrap>),
  /// Anchor Location Is Locked.
  #[sdk(empty_child(qname = "w10:CT_AnchorLock/w10:anchorlock"))]
  W10Anchorlock,
  /// Top Border.
  #[sdk(child(qname = "w10:CT_Border/w10:bordertop"))]
  W10Bordertop(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::TopBorder>),
  /// Bottom Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderbottom"))]
  W10Borderbottom(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::BottomBorder>),
  /// Left Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderleft"))]
  W10Borderleft(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::LeftBorder>),
  /// Right Border.
  #[sdk(child(qname = "w10:CT_Border/w10:borderright"))]
  W10Borderright(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word::RightBorder>),
  /// Attached Object Data.
  #[sdk(child(qname = "xvml:CT_ClientData/xvml:ClientData"))]
  XvmlClientData(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_excel::ClientData>),
  /// VML Diagram Text.
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
