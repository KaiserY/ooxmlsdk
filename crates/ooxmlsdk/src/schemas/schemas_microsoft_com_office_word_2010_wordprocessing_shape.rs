//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingShape Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:wsp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:CT_WordprocessingShape/wps:wsp")]
pub struct WordprocessingShape {
  /// normalEastAsianFlow
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :normalEastAsianFlow
  #[sdk(attr(qname = ":normalEastAsianFlow"))]
  pub normal_east_asian_flow: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/wps:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  #[sdk(choice(
    qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr",
    qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr"
  ))]
  pub wordprocessing_shape_choice1: Option<WordprocessingShapeChoice>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/wps:spPr"))]
  pub wps_sp_pr: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/wps:style"))]
  pub wps_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub wps_ext_lst: Option<OfficeArtExtensionList>,
  #[sdk(choice(
    qname = "wps:CT_TextboxInfo/wps:txbx",
    qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx"
  ))]
  pub wordprocessing_shape_choice2: Option<WordprocessingShapeChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/wps:bodyPr"))]
  pub wps_body_pr: std::boxed::Box<TextBodyProperties>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/wps:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the NonVisualDrawingProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:cNvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/wps:cNvPr")]
pub struct NonVisualDrawingProperties {
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
/// Defines the NonVisualDrawingShapeProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:cNvSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr")]
pub struct NonVisualDrawingShapeProperties {
  /// Text Box
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :txBox
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the NonVisualConnectorProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:cNvCnPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr")]
pub struct NonVisualConnectorProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
  >,
  /// Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/wps:spPr")]
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
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
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
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/wps:style")]
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
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Defines the TextBoxInfo2 Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:txbx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:CT_TextboxInfo/wps:txbx")]
pub struct TextBoxInfo2 {
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt16Value>,
  /// _
  #[sdk(child(qname = "w:CT_TxbxContent/w:txbxContent"))]
  pub text_box_content:
    Option<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LinkedTextBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:linkedTxbx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx")]
pub struct LinkedTextBox {
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt16Value,
  /// seq
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :seq
  #[sdk(attr(qname = ":seq"))]
  pub sequence: crate::simple_type::UInt16Value,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/wps:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TextBodyProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wps:bodyPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBodyProperties/wps:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcFirstLastPara
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vertOverflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
  >,
  /// Text Horizontal Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horzOverflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
  >,
  /// Vertical Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vert
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues>,
  /// Text Wrapping Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrap
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues>,
  /// Left Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lIns
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tIns
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rIns
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bIns
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numCol
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "16",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcCol
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rtlCol
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fromWordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues>,
  /// Anchor Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchorCtr
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :forceAA
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compatLnSpc
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  /// Preset Text Shape
  #[sdk(child(qname = "a:CT_PresetTextShape/a:prstTxWarp"))]
  pub preset_text_warp: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp>,
  >,
  #[sdk(choice(
    qname = "a:CT_TextNoAutofit/a:noAutofit",
    qname = "a:CT_TextNormalAutofit/a:normAutofit",
    qname = "a:CT_TextShapeAutofit/a:spAutoFit"
  ))]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingShapeChoice {
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr"))]
  WpsCNvSpPr(std::boxed::Box<NonVisualDrawingShapeProperties>),
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/wps:cNvCnPr"))]
  WpsCNvCnPr(std::boxed::Box<NonVisualConnectorProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingShapeChoice2 {
  #[sdk(child(qname = "wps:CT_TextboxInfo/wps:txbx"))]
  WpsTxbx(std::boxed::Box<TextBoxInfo2>),
  #[sdk(child(qname = "wps:CT_LinkedTextboxInformation/wps:linkedTxbx"))]
  WpsLinkedTxbx(std::boxed::Box<LinkedTextBox>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit,
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  ),
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText>,
  ),
}
