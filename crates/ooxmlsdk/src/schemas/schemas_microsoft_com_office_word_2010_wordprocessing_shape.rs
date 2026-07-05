//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingShape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:wsp")]
pub struct WordprocessingShape {
  /// normalEastAsianFlow
  #[sdk(attr(qname = ":normalEastAsianFlow"))]
  pub normal_east_asian_flow: Option<crate::simple_type::BooleanValue>,
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "wps:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  #[sdk(
        choice(
            child(variant = NonVisualDrawingShapeProperties, qname = "wps:cNvSpPr"),
            child(variant = NonVisualConnectorProperties, qname = "wps:cNvCnPr")
        )
    )]
  pub wordprocessing_shape_choice1: Option<WordprocessingShapeChoice>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "wps:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(qname = "wps:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
  #[sdk(
        choice(
            child(variant = TextBoxInfo2, qname = "wps:txbx"),
            child(variant = LinkedTextBox, qname = "wps:linkedTxbx")
        )
    )]
  pub wordprocessing_shape_choice2: Option<WordprocessingShapeChoice2>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "wps:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:cNvPr")]
pub struct NonVisualDrawingProperties {
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
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Defines the NonVisualDrawingShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:cNvSpPr")]
pub struct NonVisualDrawingShapeProperties {
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
/// Defines the NonVisualConnectorProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:cNvCnPr")]
pub struct NonVisualConnectorProperties {
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
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:spPr")]
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
#[sdk(qname = "wps:style")]
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
/// Defines the TextBoxInfo2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:txbx")]
pub struct TextBoxInfo2 {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt16Value>,
  /// Rich Text Box Content Container.
  #[sdk(child(qname = "w:txbxContent"))]
  pub text_box_content: Option<crate::schemas::w::TextBoxContent>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LinkedTextBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:linkedTxbx")]
pub struct LinkedTextBox {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt16Value,
  /// seq
  #[sdk(attr(qname = ":seq"))]
  pub sequence: crate::simple_type::UInt16Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_overflow: Option<crate::schemas::a::TextVerticalOverflowValues>,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_overflow: Option<crate::schemas::a::TextHorizontalOverflowValues>,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical: Option<crate::schemas::a::TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(kind = "token"))]
  pub wrap: Option<crate::schemas::a::TextWrappingValues>,
  /// Left Inset
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(range = 1..= 16))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(range = 0..))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(kind = "token"))]
  pub anchor: Option<crate::schemas::a::TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  /// Preset Text Shape
  #[sdk(child(qname = "a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<crate::schemas::a::PresetTextWarp>>,
  #[sdk(
        choice(
            empty_child(variant = NoAutoFit, qname = "a:noAutofit"),
            child(variant = NormalAutoFit, qname = "a:normAutofit"),
            empty_child(variant = ShapeAutoFit, qname = "a:spAutoFit")
        )
    )]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  #[sdk(
        choice(
            child(variant = Shape3DType, qname = "a:sp3d"),
            child(variant = FlatText, qname = "a:flatTx")
        )
    )]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum WordprocessingShapeChoice {
  /// Defines the NonVisualDrawingShapeProperties Class.
  NonVisualDrawingShapeProperties(std::boxed::Box<NonVisualDrawingShapeProperties>),
  /// Defines the NonVisualConnectorProperties Class.
  NonVisualConnectorProperties(std::boxed::Box<NonVisualConnectorProperties>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum WordprocessingShapeChoice2 {
  /// Defines the TextBoxInfo2 Class.
  TextBoxInfo2(std::boxed::Box<TextBoxInfo2>),
  /// Defines the LinkedTextBox Class.
  LinkedTextBox(std::boxed::Box<LinkedTextBox>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  NoAutoFit,
  /// Normal AutoFit.
  NormalAutoFit(crate::schemas::a::NormalAutoFit),
  /// Shape AutoFit.
  ShapeAutoFit,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextBodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  Shape3DType(std::boxed::Box<crate::schemas::a::Shape3DType>),
  /// No text in 3D scene.
  FlatText(crate::schemas::a::FlatText),
}
