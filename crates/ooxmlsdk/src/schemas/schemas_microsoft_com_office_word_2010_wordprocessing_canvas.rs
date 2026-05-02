//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingCanvas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpc:CT_WordprocessingCanvas/wpc:wpc")]
pub struct WordprocessingCanvas {
  /// Defines the BackgroundFormatting Class.
  #[sdk(child(office2010, qname = "a:CT_BackgroundFormatting/wpc:bg"))]
  pub background_formatting: Option<std::boxed::Box<BackgroundFormatting>>,
  /// Defines the WholeFormatting Class.
  #[sdk(child(office2010, qname = "a:CT_WholeE2oFormatting/wpc:whole"))]
  pub whole_formatting: Option<std::boxed::Box<WholeFormatting>>,
  #[sdk(choice(
    qname = "wps:CT_WordprocessingShape/wps:wsp",
    qname = "pic:CT_Picture/pic:pic",
    qname = "w14:CT_WordContentPart/w14:contentPart",
    qname = "wpg:CT_WordprocessingGroup/wpg:wgp",
    qname = "wpg:CT_GraphicFrame/wpc:graphicFrame"
  ))]
  pub wordprocessing_canvas_choice: Vec<WordprocessingCanvasChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpc:extLst"))]
  pub wpc_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_BackgroundFormatting/wpc:bg")]
pub struct BackgroundFormatting {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub background_formatting_choice1: Option<BackgroundFormattingChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub background_formatting_choice2: Option<BackgroundFormattingChoice2>,
}
/// Defines the WholeFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_WholeE2oFormatting/wpc:whole")]
pub struct WholeFormatting {
  /// Outline
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub whole_formatting_choice: Option<WholeFormattingChoice>,
}
/// Defines the GraphicFrameType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wpg:CT_GraphicFrame/wpc:graphicFrame")]
pub struct GraphicFrameType {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
  pub non_visual_drawing_properties:
    std::boxed::Box<crate::schemas::wpg::NonVisualDrawingProperties>,
  /// Defines the NonVisualGraphicFrameProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr"))]
  pub non_visual_graphic_frame_properties:
    std::boxed::Box<crate::schemas::wpg::NonVisualGraphicFrameProperties>,
  /// Defines the Transform2D Class.
  #[sdk(child(office2010, qname = "a:CT_Transform2D/wpg:xfrm"))]
  pub transform2_d: std::boxed::Box<crate::schemas::wpg::Transform2D>,
  /// Graphic Object.
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::wpg::OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/wpc:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingCanvasChoice {
  #[sdk(child(office2010, qname = "wps:CT_WordprocessingShape/wps:wsp"))]
  WpsWsp(std::boxed::Box<crate::schemas::wps::WordprocessingShape>),
  #[sdk(child(qname = "pic:CT_Picture/pic:pic"))]
  PicPic(std::boxed::Box<crate::schemas::pic::Picture>),
  #[sdk(child(office2010, qname = "w14:CT_WordContentPart/w14:contentPart"))]
  W14ContentPart(std::boxed::Box<crate::schemas::w14::ContentPart>),
  #[sdk(child(office2010, qname = "wpg:CT_WordprocessingGroup/wpg:wgp"))]
  WpgWgp(std::boxed::Box<crate::schemas::wpg::WordprocessingGroup>),
  #[sdk(child(office2010, qname = "wpg:CT_GraphicFrame/wpc:graphicFrame"))]
  WpcGraphicFrame(std::boxed::Box<GraphicFrameType>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFormattingChoice {
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
pub enum BackgroundFormattingChoice2 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WholeFormattingChoice {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
