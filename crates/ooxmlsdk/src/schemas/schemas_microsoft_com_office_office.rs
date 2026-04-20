//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum InsetMarginValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ColorModeValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionValues {
  #[sdk(rename = "perspective")]
  #[default]
  Perspective,
  #[sdk(rename = "parallel")]
  Parallel,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionRenderValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "wireFrame")]
  WireFrame,
  #[sdk(rename = "boundingCube")]
  BoundingCube,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ExtrusionPlaneValues {
  #[sdk(rename = "XY")]
  #[default]
  Xy,
  #[sdk(rename = "ZX")]
  Zx,
  #[sdk(rename = "YZ")]
  Yz,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalRuleAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "center")]
  Center,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OleLinkValues {
  #[sdk(rename = "Picture")]
  #[default]
  Picture,
  #[sdk(rename = "Bitmap")]
  Bitmap,
  #[sdk(rename = "EnhancedMetaFile")]
  EnhancedMetaFile,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OleValues {
  #[sdk(rename = "Embed")]
  #[default]
  Embed,
  #[sdk(rename = "Link")]
  Link,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OleDrawAspectValues {
  #[sdk(rename = "Content")]
  #[default]
  Content,
  #[sdk(rename = "Icon")]
  Icon,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OleUpdateModeValues {
  #[sdk(rename = "Always")]
  #[default]
  Always,
  #[sdk(rename = "OnCall")]
  OnCall,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum RuleValues {
  #[sdk(rename = "arc")]
  #[default]
  Arc,
  #[sdk(rename = "callout")]
  Callout,
  #[sdk(rename = "connector")]
  Connector,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:shapedefaults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ShapeDefaults/o:shapedefaults")]
pub struct ShapeDefaults {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Shape ID Optional Storage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spidmax
  #[sdk(attr(qname = ":spidmax"))]
  pub max_shape_id: Option<crate::simple_type::IntegerValue>,
  /// style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Shape Fill Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fill
  #[sdk(attr(qname = ":fill"))]
  pub be_filled: Option<crate::simple_type::TrueFalseValue>,
  /// Default Fill Color
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
  /// Represents the following attribute in the schema: :stroke
  #[sdk(attr(qname = ":stroke"))]
  pub is_stroke: Option<crate::simple_type::TrueFalseValue>,
  /// Shape Stroke Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Allow in Table Cell
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
  /// insetmode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:insetmode
  #[sdk(attr(qname = "o:insetmode"))]
  pub inset_mode: Option<InsetMarginValues>,
  /// _
  #[sdk(child(qname = "v:CT_Fill/v:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Fill>>,
  /// _
  #[sdk(child(qname = "v:CT_ImageData/v:imagedata"))]
  pub image_data: Option<crate::schemas::schemas_microsoft_com_vml::ImageData>,
  /// _
  #[sdk(child(qname = "v:CT_Stroke/v:stroke"))]
  pub stroke: Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Stroke>>,
  /// _
  #[sdk(child(qname = "v:CT_Textbox/v:textbox"))]
  pub text_box: Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::TextBox>>,
  /// _
  #[sdk(child(qname = "v:CT_Shadow/v:shadow"))]
  pub shadow: Option<crate::schemas::schemas_microsoft_com_vml::Shadow>,
  /// _
  #[sdk(child(qname = "o:CT_Skew/o:skew"))]
  pub skew: Option<Skew>,
  /// _
  #[sdk(child(qname = "o:CT_Extrusion/o:extrusion"))]
  pub extrusion: Option<Extrusion>,
  ///Callout
  #[sdk(child(qname = "o:CT_Callout/o:callout"))]
  pub callout: Option<Callout>,
  ///Shape Protections
  #[sdk(child(qname = "o:CT_Lock/o:lock"))]
  pub lock: Option<Lock>,
  ///Most Recently Used Colors
  #[sdk(child(qname = "o:CT_ColorMru/o:colormru"))]
  pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
  ///UI Default Colors
  #[sdk(child(qname = "o:CT_ColorMenu/o:colormenu"))]
  pub color_menu: Option<ColorMenu>,
}
/// Shape Layout Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:shapelayout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ShapeLayout/o:shapelayout")]
pub struct ShapeLayout {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  ///Shape ID Map
  #[sdk(child(qname = "o:CT_IdMap/o:idmap"))]
  pub shape_id_map: Option<ShapeIdMap>,
  ///Shape Grouping History
  #[sdk(child(qname = "o:CT_RegroupTable/o:regrouptable"))]
  pub regroup_table: Option<RegroupTable>,
  ///Rule Set
  #[sdk(child(qname = "o:CT_Rules/o:rules"))]
  pub rules: Option<Rules>,
}
/// Digital Signature Line.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:signatureline.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_SignatureLine/o:signatureline")]
pub struct SignatureLine {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Signature Line Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :issignatureline
  #[sdk(attr(qname = ":issignatureline"))]
  pub is_signature_line: Option<crate::simple_type::TrueFalseValue>,
  /// Unique ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Signature Provider ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :provid
  #[sdk(attr(qname = ":provid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// Use Signing Instructions Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :signinginstructionsset
  #[sdk(attr(qname = ":signinginstructionsset"))]
  pub signing_instructions_set: Option<crate::simple_type::TrueFalseValue>,
  /// User-specified Comments Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowcomments
  #[sdk(attr(qname = ":allowcomments"))]
  pub allow_comments: Option<crate::simple_type::TrueFalseValue>,
  /// Show Signed Date Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showsigndate
  #[sdk(attr(qname = ":showsigndate"))]
  pub show_sign_date: Option<crate::simple_type::TrueFalseValue>,
  /// Suggested Signer Line 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:suggestedsigner
  #[sdk(attr(qname = "o:suggestedsigner"))]
  pub suggested_signer: Option<crate::simple_type::StringValue>,
  /// Suggested Signer Line 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:suggestedsigner2
  #[sdk(attr(qname = "o:suggestedsigner2"))]
  pub suggested_signer2: Option<crate::simple_type::StringValue>,
  /// Suggested Signer E-mail Address
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:suggestedsigneremail
  #[sdk(attr(qname = "o:suggestedsigneremail"))]
  pub suggested_signer_email: Option<crate::simple_type::StringValue>,
  /// Instructions for Signing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :signinginstructions
  #[sdk(attr(qname = ":signinginstructions"))]
  pub signing_instructions: Option<crate::simple_type::StringValue>,
  /// Additional Signature Information
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :addlxml
  #[sdk(attr(qname = ":addlxml"))]
  pub additional_xml: Option<crate::simple_type::StringValue>,
  /// Signature Provider Download URL
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sigprovurl
  #[sdk(attr(qname = ":sigprovurl"))]
  pub signature_provider_url: Option<crate::simple_type::StringValue>,
}
/// Ink.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:ink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Ink/o:ink")]
pub struct Ink {
  /// Ink Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub ink_data: Option<crate::simple_type::Base64BinaryValue>,
  /// Annotation Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :annotation
  #[sdk(attr(qname = ":annotation"))]
  pub annotation_flag: Option<crate::simple_type::TrueFalseValue>,
}
/// VML Diagram.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:diagram.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Diagram/o:diagram")]
pub struct Diagram {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Diagram Style Options
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dgmstyle
  #[sdk(attr(qname = ":dgmstyle"))]
  pub style: Option<crate::simple_type::IntegerValue>,
  /// Diagram Automatic Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoformat
  #[sdk(attr(qname = ":autoformat"))]
  pub auto_format: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Reverse Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :reverse
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Automatic Layout
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autolayout
  #[sdk(attr(qname = ":autolayout"))]
  pub auto_layout: Option<crate::simple_type::TrueFalseValue>,
  /// Diagram Layout X Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dgmscalex
  #[sdk(attr(qname = ":dgmscalex"))]
  pub scale_x: Option<crate::simple_type::IntegerValue>,
  /// Diagram Layout Y Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dgmscaley
  #[sdk(attr(qname = ":dgmscaley"))]
  pub scale_y: Option<crate::simple_type::IntegerValue>,
  /// Diagram Font Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dgmfontsize
  #[sdk(attr(qname = ":dgmfontsize"))]
  pub font_size: Option<crate::simple_type::IntegerValue>,
  /// Diagram Layout Extents
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :constrainbounds
  #[sdk(attr(qname = ":constrainbounds"))]
  pub constrain_bounds: Option<crate::simple_type::StringValue>,
  /// Diagram Base Font Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dgmbasetextscale
  #[sdk(attr(qname = ":dgmbasetextscale"))]
  pub base_text_scale: Option<crate::simple_type::IntegerValue>,
  ///Diagram Relationship Table
  #[sdk(child(qname = "o:CT_RelationTable/o:relationtable"))]
  pub relation_table: Option<RelationTable>,
}
/// Skew Transform.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:skew.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Skew/o:skew")]
pub struct Skew {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Skew ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Skew Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Skew Offset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Skew Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :origin
  #[sdk(attr(qname = ":origin"))]
  pub origin: Option<crate::simple_type::StringValue>,
  /// Skew Perspective Matrix
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :matrix
  #[sdk(attr(qname = ":matrix"))]
  pub matrix: Option<crate::simple_type::StringValue>,
}
/// 3D Extrusion.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:extrusion.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Extrusion/o:extrusion")]
pub struct Extrusion {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Extrusion Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Extrusion Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ExtrusionValues>,
  /// Extrusion Render Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :render
  #[sdk(attr(qname = ":render"))]
  pub render: Option<ExtrusionRenderValues>,
  /// Extrusion Viewpoint Origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :viewpointorigin
  #[sdk(attr(qname = ":viewpointorigin"))]
  pub viewpoint_origin: Option<crate::simple_type::StringValue>,
  /// Extrusion Viewpoint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :viewpoint
  #[sdk(attr(qname = ":viewpoint"))]
  pub viewpoint: Option<crate::simple_type::StringValue>,
  /// Extrusion Skew Angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :skewangle
  #[sdk(attr(qname = ":skewangle"))]
  pub skew_angle: Option<crate::simple_type::SingleValue>,
  /// Extrusion Skew
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :skewamt
  #[sdk(attr(qname = ":skewamt"))]
  pub skew_amount: Option<crate::simple_type::StringValue>,
  /// Forward Extrusion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :foredepth
  #[sdk(attr(qname = ":foredepth"))]
  pub force_depth: Option<crate::simple_type::StringValue>,
  /// Backward Extrusion Depth
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :backdepth
  #[sdk(attr(qname = ":backdepth"))]
  pub back_depth: Option<crate::simple_type::StringValue>,
  /// Rotation Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<crate::simple_type::StringValue>,
  /// Rotation Around Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orientationangle
  #[sdk(attr(qname = ":orientationangle"))]
  pub orientation_angle: Option<crate::simple_type::SingleValue>,
  /// Rotation Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lockrotationcenter
  #[sdk(attr(qname = ":lockrotationcenter"))]
  pub lock_rotation_center: Option<crate::simple_type::TrueFalseValue>,
  /// Center of Rotation Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autorotationcenter
  #[sdk(attr(qname = ":autorotationcenter"))]
  pub auto_rotation_center: Option<crate::simple_type::TrueFalseValue>,
  /// Rotation Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotationcenter
  #[sdk(attr(qname = ":rotationcenter"))]
  pub rotation_center: Option<crate::simple_type::StringValue>,
  /// X-Y Rotation Angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotationangle
  #[sdk(attr(qname = ":rotationangle"))]
  pub rotation_angle: Option<crate::simple_type::StringValue>,
  /// Extrusion Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color
  #[sdk(attr(qname = ":color"))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Shininess
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shininess
  #[sdk(attr(qname = ":shininess"))]
  pub shininess: Option<crate::simple_type::SingleValue>,
  /// Specularity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :specularity
  #[sdk(attr(qname = ":specularity"))]
  pub specularity: Option<crate::simple_type::StringValue>,
  /// Diffuse Reflection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :diffusity
  #[sdk(attr(qname = ":diffusity"))]
  pub diffusity: Option<crate::simple_type::StringValue>,
  /// Metallic Surface Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :metal
  #[sdk(attr(qname = ":metal"))]
  pub metal: Option<crate::simple_type::TrueFalseValue>,
  /// Simulated Bevel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :edge
  #[sdk(attr(qname = ":edge"))]
  pub edge: Option<crate::simple_type::StringValue>,
  /// Faceting Quality
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :facet
  #[sdk(attr(qname = ":facet"))]
  pub facet: Option<crate::simple_type::StringValue>,
  /// Shape Face Lighting Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightface
  #[sdk(attr(qname = ":lightface"))]
  pub light_face: Option<crate::simple_type::TrueFalseValue>,
  /// Brightness
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :brightness
  #[sdk(attr(qname = ":brightness"))]
  pub brightness: Option<crate::simple_type::StringValue>,
  /// Primary Light Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightposition
  #[sdk(attr(qname = ":lightposition"))]
  pub light_position: Option<crate::simple_type::StringValue>,
  /// Primary Light Intensity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightlevel
  #[sdk(attr(qname = ":lightlevel"))]
  pub light_level: Option<crate::simple_type::StringValue>,
  /// Primary Light Harshness Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightharsh
  #[sdk(attr(qname = ":lightharsh"))]
  pub light_harsh: Option<crate::simple_type::TrueFalseValue>,
  /// Secondary Light Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightposition2
  #[sdk(attr(qname = ":lightposition2"))]
  pub light_position2: Option<crate::simple_type::StringValue>,
  /// Secondary Light Intensity
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightlevel2
  #[sdk(attr(qname = ":lightlevel2"))]
  pub light_level2: Option<crate::simple_type::StringValue>,
  /// Secondary Light Harshness Toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lightharsh2
  #[sdk(attr(qname = ":lightharsh2"))]
  pub light_harsh2: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Callout Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:callout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Callout/o:callout")]
pub struct Callout {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Callout toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :on
  #[sdk(attr(qname = ":on"))]
  pub on: Option<crate::simple_type::TrueFalseValue>,
  /// Callout type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Callout gap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gap
  #[sdk(attr(qname = ":gap"))]
  pub gap: Option<crate::simple_type::StringValue>,
  /// Callout angle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :angle
  #[sdk(attr(qname = ":angle"))]
  pub angle: Option<AngleValues>,
  /// Callout automatic drop toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dropauto
  #[sdk(attr(qname = ":dropauto"))]
  pub drop_auto: Option<crate::simple_type::TrueFalseValue>,
  /// Callout drop position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :drop
  #[sdk(attr(qname = ":drop"))]
  pub drop: Option<crate::simple_type::StringValue>,
  /// Callout drop distance
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :distance
  #[sdk(attr(qname = ":distance"))]
  pub distance: Option<crate::simple_type::StringValue>,
  /// Callout length toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lengthspecified
  #[sdk(attr(qname = ":lengthspecified"))]
  pub length_specified: Option<crate::simple_type::TrueFalseValue>,
  /// Callout length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::StringValue>,
  /// Callout accent bar toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accentbar
  #[sdk(attr(qname = ":accentbar"))]
  pub accent_bar: Option<crate::simple_type::TrueFalseValue>,
  /// Callout text border toggle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textborder
  #[sdk(attr(qname = ":textborder"))]
  pub text_border: Option<crate::simple_type::TrueFalseValue>,
  /// Callout flip x
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minusx
  #[sdk(attr(qname = ":minusx"))]
  pub minus_x: Option<crate::simple_type::TrueFalseValue>,
  /// Callout flip y
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minusy
  #[sdk(attr(qname = ":minusy"))]
  pub minus_y: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the Lock Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:lock.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Lock/o:lock")]
pub struct Lock {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Position Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::TrueFalseValue>,
  /// Selection Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :selection
  #[sdk(attr(qname = ":selection"))]
  pub selection: Option<crate::simple_type::TrueFalseValue>,
  /// Grouping Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grouping
  #[sdk(attr(qname = ":grouping"))]
  pub grouping: Option<crate::simple_type::TrueFalseValue>,
  /// Ungrouping Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ungrouping
  #[sdk(attr(qname = ":ungrouping"))]
  pub ungrouping: Option<crate::simple_type::TrueFalseValue>,
  /// Rotation Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotation
  #[sdk(attr(qname = ":rotation"))]
  pub rotation: Option<crate::simple_type::TrueFalseValue>,
  /// Cropping Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cropping
  #[sdk(attr(qname = ":cropping"))]
  pub cropping: Option<crate::simple_type::TrueFalseValue>,
  /// Vertices Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :verticies
  #[sdk(attr(qname = ":verticies"))]
  pub verticies: Option<crate::simple_type::TrueFalseValue>,
  /// Handles Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :adjusthandles
  #[sdk(attr(qname = ":adjusthandles"))]
  pub adjust_handles: Option<crate::simple_type::TrueFalseValue>,
  /// Text Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :text
  #[sdk(attr(qname = ":text"))]
  pub text_lock: Option<crate::simple_type::TrueFalseValue>,
  /// Aspect Ratio Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :aspectratio
  #[sdk(attr(qname = ":aspectratio"))]
  pub aspect_ratio: Option<crate::simple_type::TrueFalseValue>,
  /// AutoShape Type Lock
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shapetype
  #[sdk(attr(qname = ":shapetype"))]
  pub shape_type: Option<crate::simple_type::TrueFalseValue>,
}
/// Embedded OLE Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:OLEObject.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_OLEObject/o:OLEObject")]
pub struct OleObject {
  /// OLE Object Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :Type
  #[sdk(attr(qname = ":Type"))]
  pub r#type: Option<OleValues>,
  /// OLE Object Application
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ProgID
  #[sdk(attr(qname = ":ProgID"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// OLE Object Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ShapeID
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// OLE Object Representation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :DrawAspect
  #[sdk(attr(qname = ":DrawAspect"))]
  pub draw_aspect: Option<OleDrawAspectValues>,
  /// OLE Object Unique ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ObjectID
  #[sdk(attr(qname = ":ObjectID"))]
  pub object_id: Option<crate::simple_type::StringValue>,
  /// Relationship
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// OLE Update Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :UpdateMode
  #[sdk(attr(qname = ":UpdateMode"))]
  pub update_mode: Option<OleUpdateModeValues>,
  ///Embedded Object Alternate Image Request
  #[sdk(text_child(qname = "o:ST_OLELinkType/o:LinkType"))]
  pub link_type: Option<OleLinkValues>,
  ///Embedded Object Cannot Be Refreshed
  #[sdk(text_child(qname = "o:ST_TrueFalseBlank/o:LockedField"))]
  pub locked_field: Option<crate::simple_type::TrueFalseBlankValue>,
  ///WordprocessingML Field Switches
  #[sdk(text_child(qname = "xsd:string/o:FieldCodes"))]
  pub field_codes: Option<crate::simple_type::StringValue>,
}
/// Complex.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:complex.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Complex/o:complex")]
pub struct Complex {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
}
/// Text Box Left Stroke.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:left.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:left")]
pub struct LeftStroke {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Text Box Top Stroke.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:top.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:top")]
pub struct TopStroke {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Text Box Right Stroke.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:right.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:right")]
pub struct RightStroke {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Text Box Bottom Stroke.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:bottom.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:bottom")]
pub struct BottomStroke {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Text Box Interior Stroke.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:column.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/o:column")]
pub struct ColumnStroke {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Defines the StrokeChildType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_StrokeChild/")]
pub struct StrokeChildType {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
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
  /// Stroke Alternate Pattern Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :color2
  #[sdk(attr(qname = ":color2"))]
  pub color2: Option<crate::simple_type::StringValue>,
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
  pub line_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues>,
  /// Miter Joint Limit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :miterlimit
  #[sdk(attr(qname = ":miterlimit"))]
  pub miter_limit: Option<crate::simple_type::DecimalValue>,
  /// Line End Join Style)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :joinstyle
  #[sdk(attr(qname = ":joinstyle"))]
  pub join_style: Option<crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues>,
  /// Line End Cap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endcap
  #[sdk(attr(qname = ":endcap"))]
  pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
  /// Stroke Dash Pattern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dashstyle
  #[sdk(attr(qname = ":dashstyle"))]
  pub dash_style: Option<crate::simple_type::StringValue>,
  /// Inset Border From Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insetpen
  #[sdk(attr(qname = ":insetpen"))]
  pub inset_pen: Option<crate::simple_type::TrueFalseValue>,
  /// Stroke Image Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filltype
  #[sdk(attr(qname = ":filltype"))]
  pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
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
  pub image_aspect: Option<crate::schemas::schemas_microsoft_com_vml::ImageAspectValues>,
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
  /// Line Start Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrow
  #[sdk(attr(qname = ":startarrow"))]
  pub start_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line Start Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowwidth
  #[sdk(attr(qname = ":startarrowwidth"))]
  pub start_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line Start Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startarrowlength
  #[sdk(attr(qname = ":startarrowlength"))]
  pub start_arrow_length:
    Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
  /// Line End Arrowhead
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrow
  #[sdk(attr(qname = ":endarrow"))]
  pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
  /// Line End Arrowhead Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowwidth
  #[sdk(attr(qname = ":endarrowwidth"))]
  pub end_arrow_width: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues>,
  /// Line End Arrowhead Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endarrowlength
  #[sdk(attr(qname = ":endarrowlength"))]
  pub end_arrow_length: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues>,
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
}
/// Shape Clipping Path.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:clippath.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ClipPath/o:clippath")]
pub struct ClipPath {
  /// Path Definition
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: o:v
  #[sdk(attr(qname = "o:v"))]
  pub value: crate::simple_type::StringValue,
}
/// Shape Fill Extended Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:fill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Fill/o:fill")]
pub struct FillExtendedProperties {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Fill Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<FillValues>,
}
/// Shape ID Map.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:idmap.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_IdMap/o:idmap")]
pub struct ShapeIdMap {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Shape IDs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :data
  #[sdk(attr(qname = ":data"))]
  pub data: Option<crate::simple_type::StringValue>,
}
/// Shape Grouping History.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:regrouptable.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_RegroupTable/o:regrouptable")]
pub struct RegroupTable {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// _
  #[sdk(child(qname = "o:CT_Entry/o:entry"))]
  pub o_entry: Vec<Entry>,
}
/// Rule Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:rules.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Rules/o:rules")]
pub struct Rules {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// _
  #[sdk(child(qname = "o:CT_R/o:r"))]
  pub o_r: Vec<Rule>,
}
/// Regroup Entry.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:entry.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Entry/o:entry")]
pub struct Entry {
  /// New Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :new
  #[sdk(attr(qname = ":new"))]
  pub new: Option<crate::simple_type::Int32Value>,
  /// Old Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :old
  #[sdk(attr(qname = ":old"))]
  pub old: Option<crate::simple_type::Int32Value>,
}
/// Rule.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:r.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_R/o:r")]
pub struct Rule {
  /// Rule ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// Rule Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<RuleValues>,
  /// Alignment Rule Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :how
  #[sdk(attr(qname = ":how"))]
  pub how: Option<AlignmentValues>,
  /// Rule Shape Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idref
  #[sdk(attr(qname = ":idref"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub shape_reference: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "o:CT_Proxy/o:proxy"))]
  pub o_proxy: Vec<Proxy>,
}
/// Diagram Relationship Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:relationtable.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_RelationTable/o:relationtable")]
pub struct RelationTable {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// _
  #[sdk(child(qname = "o:CT_Relation/o:rel"))]
  pub o_rel: Vec<Relation>,
}
/// Diagram Relationship.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:rel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Relation/o:rel")]
pub struct Relation {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Diagram Relationship Source Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idsrc
  #[sdk(attr(qname = ":idsrc"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub source_id: Option<crate::simple_type::StringValue>,
  /// Diagram Relationship Destination Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iddest
  #[sdk(attr(qname = ":iddest"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub destination_id: Option<crate::simple_type::StringValue>,
  /// Diagram Relationship Center Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idcntr
  #[sdk(attr(qname = ":idcntr"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub center_shape_id: Option<crate::simple_type::StringValue>,
}
/// Embedded Object Alternate Image Request.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:LinkType.
pub type LinkType = OleLinkValues;
/// Embedded Object Cannot Be Refreshed.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:LockedField.
pub type LockedField = crate::simple_type::TrueFalseBlankValue;
/// WordprocessingML Field Switches.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:FieldCodes.
pub type FieldCodes = crate::simple_type::StringValue;
/// Shape Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:proxy.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_Proxy/o:proxy")]
pub struct Proxy {
  /// Start Point Connection Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::TrueFalseBlankValue>,
  /// End Point Connection Flag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::TrueFalseBlankValue>,
  /// Proxy Shape Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idref
  #[sdk(attr(qname = ":idref"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_reference: crate::simple_type::StringValue,
  /// Connection Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connectloc
  #[sdk(attr(qname = ":connectloc"))]
  pub connection_location: crate::simple_type::Int32Value,
}
/// Most Recently Used Colors.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:colormru.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ColorMru/o:colormru")]
pub struct ColorMostRecentlyUsed {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Recent colors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colors
  #[sdk(attr(qname = ":colors"))]
  pub colors: Option<crate::simple_type::StringValue>,
}
/// UI Default Colors.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is o:colormenu.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "o:CT_ColorMenu/o:colormenu")]
pub struct ColorMenu {
  /// VML Extension Handling Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: v:ext
  #[sdk(attr(qname = "v:ext"))]
  pub extension: Option<crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues>,
  /// Default stroke color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strokecolor
  #[sdk(attr(qname = ":strokecolor"))]
  pub stroke_color: Option<crate::simple_type::StringValue>,
  /// Default fill color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillcolor
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
  /// Default shadow color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadowcolor
  #[sdk(attr(qname = ":shadowcolor"))]
  pub shadow_color: Option<crate::simple_type::StringValue>,
  /// Default extrusion color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :extrusioncolor
  #[sdk(attr(qname = ":extrusioncolor"))]
  pub extrusion_color: Option<crate::simple_type::StringValue>,
}
