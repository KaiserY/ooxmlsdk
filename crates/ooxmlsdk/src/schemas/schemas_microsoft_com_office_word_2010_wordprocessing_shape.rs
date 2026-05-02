//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingShape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wps:CT_WordprocessingShape/wps:wsp")]
pub struct WordprocessingShape {
  /// normalEastAsianFlow
  #[sdk(attr(office2010, qname = ":normalEastAsianFlow"))]
  pub normal_east_asian_flow: Option<crate::simple_type::BooleanValue>,
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wps:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  #[sdk(choice(
    qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr",
    qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr"
  ))]
  pub wordprocessing_shape_choice1: Option<WordprocessingShapeChoice>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2010, qname = "a:CT_ShapeProperties/wps:spPr"))]
  pub wps_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ShapeStyle Class.
  #[sdk(child(office2010, qname = "a:CT_ShapeStyle/wps:style"))]
  pub wps_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub wps_ext_lst: Option<OfficeArtExtensionList>,
  #[sdk(choice(
    qname = "wps:CT_TextboxInfo/wps:txbx",
    qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx"
  ))]
  pub wordprocessing_shape_choice2: Option<WordprocessingShapeChoice2>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(office2010, qname = "a:CT_TextBodyProperties/wps:bodyPr"))]
  pub wps_body_pr: Option<std::boxed::Box<TextBodyProperties>>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/wps:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/wps:cNvPr")]
pub struct NonVisualDrawingProperties {
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
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Defines the NonVisualDrawingShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr")]
pub struct NonVisualDrawingShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<crate::schemas::a::ShapeLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the NonVisualConnectorProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr")]
pub struct NonVisualConnectorProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<crate::schemas::a::ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection: Option<crate::schemas::a::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection: Option<crate::schemas::a::EndConnection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_ShapeProperties/wps:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
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
  pub a_ln: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the ShapeStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_ShapeStyle/wps:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference: std::boxed::Box<crate::schemas::a::LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference: std::boxed::Box<crate::schemas::a::FillReference>,
  /// Effect Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<crate::schemas::a::EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference: std::boxed::Box<crate::schemas::a::FontReference>,
}
/// Defines the TextBoxInfo2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wps:CT_TextboxInfo/wps:txbx")]
pub struct TextBoxInfo2 {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  pub id: Option<crate::simple_type::UInt16Value>,
  /// Rich Text Box Content Container.
  #[sdk(child(qname = "w:CT_TxbxContent/w:txbxContent"))]
  pub text_box_content: Option<crate::schemas::w::TextBoxContent>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LinkedTextBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx")]
pub struct LinkedTextBox {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  pub id: crate::simple_type::UInt16Value,
  /// seq
  #[sdk(attr(office2010, qname = ":seq"))]
  pub sequence: crate::simple_type::UInt16Value,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_TextBodyProperties/wps:bodyPr")]
pub struct TextBodyProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(child(qname = "a:CT_PresetTextShape/a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<crate::schemas::a::PresetTextWarp>>,
  #[sdk(choice(
    qname = "a:CT_TextNoAutofit/a:noAutofit",
    qname = "a:CT_TextNormalAutofit/a:normAutofit",
    qname = "a:CT_TextShapeAutofit/a:spAutoFit"
  ))]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::ExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingShapeChoice {
  /// Defines the NonVisualDrawingShapeProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr"))]
  WpsCNvSpPr(std::boxed::Box<NonVisualDrawingShapeProperties>),
  /// Defines the NonVisualConnectorProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr"))]
  WpsCNvCnPr(std::boxed::Box<NonVisualConnectorProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingShapeChoice2 {
  /// Defines the TextBoxInfo2 Class.
  #[sdk(child(office2010, qname = "wps:CT_TextboxInfo/wps:txbx"))]
  WpsTxbx(std::boxed::Box<TextBoxInfo2>),
  /// Defines the LinkedTextBox Class.
  #[sdk(child(office2010, qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx"))]
  WpsLinkedTxbx(std::boxed::Box<LinkedTextBox>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit,
  /// Normal AutoFit.
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(std::boxed::Box<crate::schemas::a::NormalAutoFit>),
  /// Shape AutoFit.
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(std::boxed::Box<crate::schemas::a::ShapeAutoFit>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(std::boxed::Box<crate::schemas::a::Shape3DType>),
  /// No text in 3D scene.
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(std::boxed::Box<crate::schemas::a::FlatText>),
}
