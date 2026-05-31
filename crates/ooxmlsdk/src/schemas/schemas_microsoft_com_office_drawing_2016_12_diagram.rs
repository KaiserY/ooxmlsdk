//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1612:spPr")]
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
/// Defines the TextListStyleType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm1612:lstStyle")]
pub struct TextListStyleType {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
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
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
