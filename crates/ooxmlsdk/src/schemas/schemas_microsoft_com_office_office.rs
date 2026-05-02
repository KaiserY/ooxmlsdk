//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "middle")]
  Middle,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ScreenSizeValues {
  #[sdk(rename = "544,376")]
  #[default]
  Sz544x376,
  #[sdk(rename = "640,480")]
  Sz640x480,
  #[sdk(rename = "720,512")]
  Sz720x512,
  #[sdk(rename = "800,600")]
  Sz800x600,
  #[sdk(rename = "1024,768")]
  Sz1024x768,
  #[sdk(rename = "1152,862")]
  Sz1152x862,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum InsetMarginValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ColorModeValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionValues {
  #[sdk(rename = "perspective")]
  #[default]
  Perspective,
  #[sdk(rename = "parallel")]
  Parallel,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionRenderValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "wireFrame")]
  WireFrame,
  #[sdk(rename = "boundingCube")]
  BoundingCube,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionPlaneValues {
  #[sdk(rename = "XY")]
  #[default]
  Xy,
  #[sdk(rename = "ZX")]
  Zx,
  #[sdk(rename = "YZ")]
  Yz,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AngleValues {
  #[sdk(rename = "any")]
  #[default]
  Any,
  #[sdk(rename = "30")]
  Degree30,
  #[sdk(rename = "45")]
  Degree45,
  #[sdk(rename = "60")]
  Degree60,
  #[sdk(rename = "90")]
  Degree90,
  #[sdk(rename = "auto")]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CalloutPlacementValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "user")]
  User,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectorValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "straight")]
  Straight,
  #[sdk(rename = "elbow")]
  Elbow,
  #[sdk(rename = "curved")]
  Curved,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalRuleAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "center")]
  Center,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "rect")]
  Rectangle,
  #[sdk(rename = "segments")]
  Segments,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleLinkValues {
  #[sdk(rename = "Picture")]
  #[default]
  Picture,
  #[sdk(rename = "Bitmap")]
  Bitmap,
  #[sdk(rename = "EnhancedMetaFile")]
  EnhancedMetaFile,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleValues {
  #[sdk(rename = "Embed")]
  #[default]
  Embed,
  #[sdk(rename = "Link")]
  Link,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleDrawAspectValues {
  #[sdk(rename = "Content")]
  #[default]
  Content,
  #[sdk(rename = "Icon")]
  Icon,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleUpdateModeValues {
  #[sdk(rename = "Always")]
  #[default]
  Always,
  #[sdk(rename = "OnCall")]
  OnCall,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FillValues {
  #[sdk(rename = "gradientCenter")]
  #[default]
  GradientCenter,
  #[sdk(rename = "solid")]
  Solid,
  #[sdk(rename = "pattern")]
  Pattern,
  #[sdk(rename = "tile")]
  Tile,
  #[sdk(rename = "frame")]
  Frame,
  #[sdk(rename = "gradientUnscaled")]
  GradientUnscaled,
  #[sdk(rename = "gradientRadial")]
  GradientRadial,
  #[sdk(rename = "gradient")]
  Gradient,
  #[sdk(rename = "background")]
  Background,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RuleValues {
  #[sdk(rename = "arc")]
  #[default]
  Arc,
  #[sdk(rename = "callout")]
  Callout,
  #[sdk(rename = "connector")]
  Connector,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BlackAndWhiteModeValues {
  #[sdk(rename = "color")]
  #[default]
  Color,
  #[sdk(rename = "auto")]
  Auto,
  #[sdk(rename = "grayScale")]
  GrayScale,
  #[sdk(rename = "lightGrayScale")]
  LightGrayScale,
  #[sdk(rename = "inverseGray")]
  InverseGray,
  #[sdk(rename = "grayOutline")]
  GrayOutline,
  #[sdk(rename = "highContrast")]
  HighContrast,
  #[sdk(rename = "black")]
  Black,
  #[sdk(rename = "white")]
  White,
  #[sdk(rename = "undrawn")]
  Undrawn,
  #[sdk(rename = "blackTextAndLines")]
  BlackTextAndLines,
}
/// New Shape Defaults.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ShapeDefaults/o:shapedefaults")]
pub struct ShapeDefaults {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Shape ID Optional Storage
  #[sdk(attr(qname = ":spidmax"))]
  pub max_shape_id: Option<crate::simple_type::IntegerValue>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  #[sdk(attr(qname = ":fill"))]
  pub be_filled: Option<crate::simple_type::TrueFalseValue>,
  /// Default Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Shape Stroke Toggle
  #[sdk(attr(qname = ":stroke"))]
  pub is_stroke: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Allow in Table Cell
  #[sdk(attr(qname = "o:allowincell"))]
  pub allow_in_cell: Option<crate::simple_type::TrueFalseValue>,
  /// allowoverlap
  #[sdk(attr(qname = "o:allowoverlap"))]
  pub allow_overlap: Option<crate::simple_type::TrueFalseValue>,
  /// insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<InsetMarginValues>,
  /// _
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::v::Fill>>,
  /// _
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  pub image_data: Option<crate::schemas::v::ImageData>,
  /// _
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  pub stroke: Option<std::boxed::Box<crate::schemas::v::Stroke>>,
  /// _
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  pub text_box: Option<std::boxed::Box<crate::schemas::v::TextBox>>,
  /// _
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  pub shadow: Option<crate::schemas::v::Shadow>,
  /// _
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  pub skew: Option<Skew>,
  /// _
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  pub extrusion: Option<Extrusion>,
  /// Callout
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  pub callout: Option<Callout>,
  /// Shape Protections
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  pub lock: Option<Lock>,
  /// Most Recently Used Colors
  #[sdk(child(qname = "o:CT_ColorMru/o:colormru"))]
  pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
  /// UI Default Colors
  #[sdk(child(qname = "o:CT_ColorMenu/o:colormenu"))]
  pub color_menu: Option<ColorMenu>,
}
/// Shape Layout Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ShapeLayout/o:shapelayout")]
pub struct ShapeLayout {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Shape ID Map
  #[sdk(child(qname = "o:CT_IdMap/o:idmap"))]
  pub shape_id_map: Option<ShapeIdMap>,
  /// Shape Grouping History
  #[sdk(child(qname = "o:CT_RegroupTable/o:regrouptable"))]
  pub regroup_table: Option<RegroupTable>,
  /// Rule Set
  #[sdk(child(qname = "o:CT_Rules/o:rules"))]
  pub rules: Option<Rules>,
}
/// Digital Signature Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_SignatureLine/o:signatureline")]
pub struct SignatureLine {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Signature Line Flag
  #[sdk(attr(qname = ":issignatureline"))]
  pub is_signature_line: Option<crate::simple_type::TrueFalseValue>,
  /// Unique ID
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Signature Provider ID
  #[sdk(attr(qname = ":provid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// Use Signing Instructions Flag
  #[sdk(attr(qname = ":signinginstructionsset"))]
  pub signing_instructions_set: Option<crate::simple_type::TrueFalseValue>,
  /// User-specified Comments Flag
  #[sdk(attr(qname = ":allowcomments"))]
  pub allow_comments: Option<crate::simple_type::TrueFalseValue>,
  /// Show Signed Date Flag
  #[sdk(attr(qname = ":showsigndate"))]
  pub show_sign_date: Option<crate::simple_type::TrueFalseValue>,
  /// Suggested Signer Line 1
  #[sdk(attr(qname = "o:suggestedsigner"))]
  pub suggested_signer: Option<crate::simple_type::StringValue>,
  /// Suggested Signer Line 2
  #[sdk(attr(qname = "o:suggestedsigner2"))]
  pub suggested_signer2: Option<crate::simple_type::StringValue>,
  /// Suggested Signer E-mail Address
  #[sdk(attr(qname = "o:suggestedsigneremail"))]
  pub suggested_signer_email: Option<crate::simple_type::StringValue>,
  /// Instructions for Signing
  #[sdk(attr(qname = ":signinginstructions"))]
  pub signing_instructions: Option<crate::simple_type::StringValue>,
  /// Additional Signature Information
  #[sdk(attr(qname = ":addlxml"))]
  pub additional_xml: Option<crate::simple_type::StringValue>,
  /// Signature Provider Download URL
  #[sdk(attr(qname = ":sigprovurl"))]
  pub signature_provider_url: Option<crate::simple_type::StringValue>,
}
/// Ink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Ink/o:ink")]
pub struct Ink {
  /// Ink Data
  #[sdk(attr(qname = ":i"))]
  pub ink_data: Option<crate::simple_type::Base64BinaryValue>,
  /// Annotation Flag
  #[sdk(attr(qname = ":annotation"))]
  pub annotation_flag: Option<crate::simple_type::TrueFalseValue>,
}
/// VML Diagram.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Diagram/o:diagram")]
pub struct Diagram {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Diagram Style Options
  #[sdk(attr(qname = ":dgmstyle"))]
  pub style: Option<crate::simple_type::IntegerValue>,
  /// Diagram Automatic Format
  #[sdk(attr(qname = ":autoformat"))]
  pub auto_format: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Reverse Direction
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Automatic Layout
  #[sdk(attr(qname = ":autolayout"))]
  pub auto_layout: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Layout X Scale
  #[sdk(attr(qname = ":dgmscalex"))]
  pub scale_x: Option<crate::simple_type::IntegerValue>,
  /// Diagram Layout Y Scale
  #[sdk(attr(qname = ":dgmscaley"))]
  pub scale_y: Option<crate::simple_type::IntegerValue>,
  /// Diagram Font Size
  #[sdk(attr(qname = ":dgmfontsize"))]
  pub font_size: Option<crate::simple_type::IntegerValue>,
  /// Diagram Layout Extents
  #[sdk(attr(qname = ":constrainbounds"))]
  pub constrain_bounds: Option<crate::simple_type::StringValue>,
  /// Diagram Base Font Size
  #[sdk(attr(qname = ":dgmbasetextscale"))]
  pub base_text_scale: Option<crate::simple_type::IntegerValue>,
  /// Diagram Relationship Table
  #[sdk(child(qname = "o:CT_RelationTable/o:relationtable"))]
  pub relation_table: Option<RelationTable>,
}
/// Skew Transform.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Skew/o:skew")]
pub struct Skew {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Skew ID
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Skew Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Skew Offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Skew Origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Skew Perspective Matrix
  #[sdk(attr(qname = ":matrix"))]
  pub matrix: Option<crate::simple_type::StringValue>,
}
/// 3D Extrusion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Extrusion/o:extrusion")]
pub struct Extrusion {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Extrusion Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Extrusion Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ExtrusionValues>,
  /// Extrusion Render Mode
  #[sdk(attr(qname = ":render"))]
  pub render: Option<ExtrusionRenderValues>,
  /// Extrusion Viewpoint Origin
  #[sdk(attr(qname = ":viewpointorigin"))]
  pub viewpoint_origin: Option<crate::simple_type::StringValue>,
  /// Extrusion Viewpoint
  #[sdk(attr(qname = ":viewpoint"))]
  pub viewpoint: Option<crate::simple_type::StringValue>,
  /// Extrusion Skew Angle
  #[sdk(attr(qname = ":skewangle"))]
  pub skew_angle: Option<crate::simple_type::SingleValue>,
  /// Extrusion Skew
  #[sdk(attr(qname = ":skewamt"))]
  pub skew_amount: Option<crate::simple_type::StringValue>,
  /// Forward Extrusion
  #[sdk(attr(qname = ":foredepth"))]
  pub force_depth: Option<crate::simple_type::StringValue>,
  /// Backward Extrusion Depth
  #[sdk(attr(qname = ":backdepth"))]
  pub back_depth: Option<crate::simple_type::StringValue>,
  /// Rotation Axis
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<crate::simple_type::StringValue>,
  /// Rotation Around Axis
  #[sdk(attr(qname = ":orientationangle"))]
  pub orientation_angle: Option<crate::simple_type::SingleValue>,
  /// Rotation Toggle
  #[sdk(attr(qname = ":lockrotationcenter"))]
  pub lock_rotation_center: Option<crate::simple_type::TrueFalseValue>,
  /// Center of Rotation Toggle
  #[sdk(attr(qname = ":autorotationcenter"))]
  pub auto_rotation_center: Option<crate::simple_type::TrueFalseValue>,
  /// Rotation Center
  #[sdk(attr(qname = ":rotationcenter"))]
  pub rotation_center: Option<crate::simple_type::StringValue>,
  /// X-Y Rotation Angle
  #[sdk(attr(qname = ":rotationangle"))]
  pub rotation_angle: Option<crate::simple_type::StringValue>,
  /// Extrusion Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Shininess
  #[sdk(attr(qname = ":shininess"))]
  pub shininess: Option<crate::simple_type::SingleValue>,
  /// Specularity
  #[sdk(attr(qname = ":specularity"))]
  pub specularity: Option<crate::simple_type::StringValue>,
  /// Diffuse Reflection
  #[sdk(attr(qname = ":diffusity"))]
  pub diffusity: Option<crate::simple_type::StringValue>,
  /// Metallic Surface Toggle
  #[sdk(attr(qname = ":metal"))]
  pub metal: Option<crate::simple_type::TrueFalseValue>,
  /// Simulated Bevel
  #[sdk(attr(qname = ":edge"))]
  pub edge: Option<crate::simple_type::StringValue>,
  /// Faceting Quality
  #[sdk(attr(qname = ":facet"))]
  pub facet: Option<crate::simple_type::StringValue>,
  /// Shape Face Lighting Toggle
  #[sdk(attr(qname = ":lightface"))]
  pub light_face: Option<crate::simple_type::TrueFalseValue>,
  /// Brightness
  #[sdk(attr(qname = ":brightness"))]
  pub brightness: Option<crate::simple_type::StringValue>,
  /// Primary Light Position
  #[sdk(attr(qname = ":lightposition"))]
  pub light_position: Option<crate::simple_type::StringValue>,
  /// Primary Light Intensity
  #[sdk(attr(qname = ":lightlevel"))]
  pub light_level: Option<crate::simple_type::StringValue>,
  /// Primary Light Harshness Toggle
  #[sdk(attr(qname = ":lightharsh"))]
  pub light_harsh: Option<crate::simple_type::TrueFalseValue>,
  /// Secondary Light Position
  #[sdk(attr(qname = ":lightposition2"))]
  pub light_position2: Option<crate::simple_type::StringValue>,
  /// Secondary Light Intensity
  #[sdk(attr(qname = ":lightlevel2"))]
  pub light_level2: Option<crate::simple_type::StringValue>,
  /// Secondary Light Harshness Toggle
  #[sdk(attr(qname = ":lightharsh2"))]
  pub light_harsh2: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Callout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Callout/o:callout")]
pub struct Callout {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Callout toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Callout type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Callout gap
  #[sdk(attr(qname = ":gap"))]
  pub gap: Option<crate::simple_type::StringValue>,
  /// Callout angle
  #[sdk(attr(qname = ":angle"))]
  pub angle: Option<AngleValues>,
  /// Callout automatic drop toggle
  #[sdk(attr(qname = ":dropauto"))]
  pub drop_auto: Option<crate::simple_type::TrueFalseValue>,
  /// Callout drop position
  #[sdk(attr(qname = ":drop"))]
  pub drop: Option<crate::simple_type::StringValue>,
  /// Callout drop distance
  #[sdk(attr(qname = ":distance"))]
  pub distance: Option<crate::simple_type::StringValue>,
  /// Callout length toggle
  #[sdk(attr(qname = ":lengthspecified"))]
  pub length_specified: Option<crate::simple_type::TrueFalseValue>,
  /// Callout length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::StringValue>,
  /// Callout accent bar toggle
  #[sdk(attr(qname = ":accentbar"))]
  pub accent_bar: Option<crate::simple_type::TrueFalseValue>,
  /// Callout text border toggle
  #[sdk(attr(qname = ":textborder"))]
  pub text_border: Option<crate::simple_type::TrueFalseValue>,
  /// Callout flip x
  #[sdk(attr(qname = ":minusx"))]
  pub minus_x: Option<crate::simple_type::TrueFalseValue>,
  /// Callout flip y
  #[sdk(attr(qname = ":minusy"))]
  pub minus_y: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Lock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Lock/o:lock")]
pub struct Lock {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Position Lock
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::TrueFalseValue>,
  /// Selection Lock
  #[sdk(attr(qname = ":selection"))]
  pub selection: Option<crate::simple_type::TrueFalseValue>,
  /// Grouping Lock
  #[sdk(attr(qname = ":grouping"))]
  pub grouping: Option<crate::simple_type::TrueFalseValue>,
  /// Ungrouping Lock
  #[sdk(attr(qname = ":ungrouping"))]
  pub ungrouping: Option<crate::simple_type::TrueFalseValue>,
  /// Rotation Lock
  #[sdk(attr(qname = ":rotation"))]
  pub rotation: Option<crate::simple_type::TrueFalseValue>,
  /// Cropping Lock
  #[sdk(attr(qname = ":cropping"))]
  pub cropping: Option<crate::simple_type::TrueFalseValue>,
  /// Vertices Lock
  #[sdk(attr(qname = ":verticies"))]
  pub verticies: Option<crate::simple_type::TrueFalseValue>,
  /// Handles Lock
  #[sdk(attr(qname = ":adjusthandles"))]
  pub adjust_handles: Option<crate::simple_type::TrueFalseValue>,
  /// Text Lock
  #[sdk(attr(qname = ":text"))]
  pub text_lock: Option<crate::simple_type::TrueFalseValue>,
  /// Aspect Ratio Lock
  #[sdk(attr(qname = ":aspectratio"))]
  pub aspect_ratio: Option<crate::simple_type::TrueFalseValue>,
  /// AutoShape Type Lock
  #[sdk(attr(qname = ":shapetype"))]
  pub shape_type: Option<crate::simple_type::TrueFalseValue>,
}
/// Embedded OLE Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_OLEObject/o:OLEObject")]
pub struct OleObject {
  /// OLE Object Type
  #[sdk(attr(qname = ":Type"))]
  pub r#type: Option<OleValues>,
  /// OLE Object Application
  #[sdk(attr(qname = ":ProgID"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// OLE Object Shape
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// OLE Object Representation
  #[sdk(attr(qname = ":DrawAspect"))]
  pub draw_aspect: Option<OleDrawAspectValues>,
  /// OLE Object Unique ID
  #[sdk(attr(qname = ":ObjectID"))]
  pub object_id: Option<crate::simple_type::StringValue>,
  /// Relationship
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// OLE Update Mode
  #[sdk(attr(qname = ":UpdateMode"))]
  pub update_mode: Option<OleUpdateModeValues>,
  /// Embedded Object Alternate Image Request
  #[sdk(text_child(qname = "o:ST_OLELinkType/o:LinkType"))]
  pub link_type: Option<OleLinkValues>,
  /// Embedded Object Cannot Be Refreshed
  #[sdk(text_child(qname = "o:ST_TrueFalseBlank/o:LockedField"))]
  pub locked_field: Option<crate::simple_type::TrueFalseBlankValue>,
  /// WordprocessingML Field Switches
  #[sdk(text_child(qname = "xsd:string/o:FieldCodes"))]
  pub field_codes: Option<crate::simple_type::StringValue>,
}
/// Complex.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Complex/o:complex")]
pub struct Complex {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
}
/// Text Box Left Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:left")]
pub struct LeftStroke {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<crate::schemas::v::StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::v::StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::v::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::v::FillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<crate::schemas::v::ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
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
}
/// Text Box Top Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:top")]
pub struct TopStroke {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<crate::schemas::v::StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::v::StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::v::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::v::FillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<crate::schemas::v::ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
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
}
/// Text Box Right Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:right")]
pub struct RightStroke {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<crate::schemas::v::StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::v::StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::v::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::v::FillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<crate::schemas::v::ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
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
}
/// Text Box Bottom Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:bottom")]
pub struct BottomStroke {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<crate::schemas::v::StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::v::StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::v::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::v::FillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<crate::schemas::v::ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
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
}
/// Text Box Interior Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:column")]
pub struct ColumnStroke {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Stroke Toggle
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
  /// Stroke Color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Stroke Alternate Pattern Color
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
  /// Stroke Opacity
  #[sdk(attr(qname = ":opacity"))]
  pub opacity: Option<crate::simple_type::StringValue>,
  /// Stroke Line Style
  #[sdk(attr(qname = ":linestyle"))]
  pub line_style: Option<crate::schemas::v::StrokeLineStyleValues>,
  /// Miter Joint Limit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::v::StrokeJoinStyleValues>,
  /// Line End Cap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::v::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::v::FillTypeValues>,
  /// Stroke Image Location
  #[sdk(attr(qname = ":src"))]
  pub source: Option<crate::simple_type::StringValue>,
  /// Stroke Image Aspect Ratio
  #[sdk(attr(qname = ":imageaspect"))]
  pub image_aspect: Option<crate::schemas::v::ImageAspectValues>,
  /// Stroke Image Size
  #[sdk(attr(qname = ":imagesize"))]
  pub image_size: Option<crate::simple_type::StringValue>,
  /// Stoke Image Alignment
  #[sdk(attr(qname = ":imagealignshape"))]
  pub image_align_shape: Option<crate::simple_type::TrueFalseValue>,
  /// Line Start Arrowhead
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::v::StrokeArrowValues>,
  /// Line End Arrowhead Width
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::v::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::v::StrokeArrowLengthValues>,
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
}
/// Shape Clipping Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ClipPath/o:clippath")]
pub struct ClipPath {
  /// Path Definition
  #[sdk(attr(qname = "o:v"))]
  pub value: crate::simple_type::StringValue,
}
/// Shape Fill Extended Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Fill/o:fill")]
pub struct FillExtendedProperties {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Fill Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<FillValues>,
}
/// Shape ID Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_IdMap/o:idmap")]
pub struct ShapeIdMap {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Shape IDs
  #[sdk(attr(qname = ":data"))]
  pub data: Option<crate::simple_type::StringValue>,
}
/// Shape Grouping History.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_RegroupTable/o:regrouptable")]
pub struct RegroupTable {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Regroup Entry.
  #[sdk(child(qname = "o:CT_Entry/o:entry"))]
  pub o_entry: Vec<Entry>,
}
/// Rule Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Rules/o:rules")]
pub struct Rules {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Rule.
  #[sdk(child(qname = "o:CT_R/o:r"))]
  pub o_r: Vec<Rule>,
}
/// Regroup Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Entry/o:entry")]
pub struct Entry {
  /// New Group ID
  #[sdk(attr(qname = ":new"))]
  pub new: Option<crate::simple_type::Int32Value>,
  /// Old Group ID
  #[sdk(attr(qname = ":old"))]
  pub old: Option<crate::simple_type::Int32Value>,
}
/// Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_R/o:r")]
pub struct Rule {
  /// Rule ID
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// Rule Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<RuleValues>,
  /// Alignment Rule Type
  #[sdk(attr(qname = ":how"))]
  pub how: Option<AlignmentValues>,
  /// Rule Shape Reference
  #[sdk(attr(qname = ":idref"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_reference: Option<crate::simple_type::StringValue>,
  /// Shape Reference.
  #[sdk(child(qname = "o:CT_Proxy/o:proxy"))]
  pub o_proxy: Vec<Proxy>,
}
/// Diagram Relationship Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_RelationTable/o:relationtable")]
pub struct RelationTable {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Diagram Relationship.
  #[sdk(child(qname = "o:CT_Relation/o:rel"))]
  pub o_rel: Vec<Relation>,
}
/// Diagram Relationship.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Relation/o:rel")]
pub struct Relation {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Diagram Relationship Source Shape
  #[sdk(attr(qname = ":idsrc"))]
  #[sdk(string_format(kind = "token"))]
  pub source_id: Option<crate::simple_type::StringValue>,
  /// Diagram Relationship Destination Shape
  #[sdk(attr(qname = ":iddest"))]
  #[sdk(string_format(kind = "token"))]
  pub destination_id: Option<crate::simple_type::StringValue>,
  /// Diagram Relationship Center Shape
  #[sdk(attr(qname = ":idcntr"))]
  #[sdk(string_format(kind = "token"))]
  pub center_shape_id: Option<crate::simple_type::StringValue>,
}
/// Embedded Object Alternate Image Request.
pub type LinkType = OleLinkValues;
/// Embedded Object Cannot Be Refreshed.
pub type LockedField = crate::simple_type::TrueFalseBlankValue;
/// WordprocessingML Field Switches.
pub type FieldCodes = crate::simple_type::StringValue;
/// Shape Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Proxy/o:proxy")]
pub struct Proxy {
  /// Start Point Connection Flag
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::TrueFalseBlankValue>,
  /// End Point Connection Flag
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Proxy Shape Reference
  #[sdk(attr(qname = ":idref"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_reference: crate::simple_type::StringValue,
  /// Connection Location
  #[sdk(attr(qname = ":connectloc"))]
  pub connection_location: crate::simple_type::Int32Value,
}
/// Most Recently Used Colors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ColorMru/o:colormru")]
pub struct ColorMostRecentlyUsed {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Recent colors
  #[sdk(attr(qname = ":colors"))]
  pub colors: Option<crate::simple_type::StringValue>,
}
/// UI Default Colors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ColorMenu/o:colormenu")]
pub struct ColorMenu {
  /// VML Extension Handling Behavior
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::v::ExtensionHandlingBehaviorValues>,
  /// Default stroke color
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Default fill color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Default shadow color
  #[sdk(attr(qname = ":shadowcolor"))]
  pub shadow_color: Option<crate::simple_type::StringValue>,
  /// Default extrusion color
  #[sdk(attr(qname = ":extrusioncolor"))]
  pub extrusion_color: Option<crate::simple_type::StringValue>,
}
