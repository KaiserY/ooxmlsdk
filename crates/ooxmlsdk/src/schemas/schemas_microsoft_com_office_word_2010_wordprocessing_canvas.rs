//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingCanvas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:wpc")]
pub struct WordprocessingCanvas {
  /// Defines the BackgroundFormatting Class.
  #[sdk(child(qname = "wpc:bg"))]
  pub background_formatting: Option<std::boxed::Box<BackgroundFormatting>>,
  /// Defines the WholeFormatting Class.
  #[sdk(child(qname = "wpc:whole"))]
  pub whole_formatting: Option<std::boxed::Box<WholeFormatting>>,
  #[sdk(
        choice(
            child(variant = WordprocessingShape, qname = "wps:wsp"),
            child(variant = Picture, qname = "pic:pic"),
            child(variant = ContentPart, qname = "w14:contentPart"),
            child(variant = WordprocessingGroup, qname = "wpg:wgp"),
            child(variant = GraphicFrameType, qname = "wpc:graphicFrame")
        )
    )]
  pub wordprocessing_canvas_choice: Vec<WordprocessingCanvasChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wpc:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:bg")]
pub struct BackgroundFormatting {
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
  pub background_formatting_choice1: Option<BackgroundFormattingChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub background_formatting_choice2: Option<BackgroundFormattingChoice2>,
}
/// Defines the WholeFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:whole")]
pub struct WholeFormatting {
  /// Outline
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub whole_formatting_choice: Option<WholeFormattingChoice>,
}
/// Defines the GraphicFrameType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:graphicFrame")]
pub struct GraphicFrameType {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "wpg:cNvPr"))]
  pub non_visual_drawing_properties:
    std::boxed::Box<crate::schemas::wpg::NonVisualDrawingProperties>,
  /// Defines the NonVisualGraphicFrameProperties Class.
  #[sdk(child(qname = "wpg:cNvFrPr"))]
  pub non_visual_graphic_frame_properties:
    std::boxed::Box<crate::schemas::wpg::NonVisualGraphicFrameProperties>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "wpg:xfrm"))]
  pub transform2_d: std::boxed::Box<crate::schemas::wpg::Transform2D>,
  /// Graphic Object.
  #[sdk(child(qname = "a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wpg:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::wpg::OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingCanvasChoice {
  WordprocessingShape(std::boxed::Box<crate::schemas::wps::WordprocessingShape>),
  Picture(std::boxed::Box<crate::schemas::pic::Picture>),
  ContentPart(std::boxed::Box<crate::schemas::w14::ContentPart>),
  WordprocessingGroup(std::boxed::Box<crate::schemas::wpg::WordprocessingGroup>),
  /// Defines the GraphicFrameType Class.
  GraphicFrameType(std::boxed::Box<GraphicFrameType>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFormattingChoice {
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
  #[sdk(empty_child(qname = "a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFormattingChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WholeFormattingChoice {
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
