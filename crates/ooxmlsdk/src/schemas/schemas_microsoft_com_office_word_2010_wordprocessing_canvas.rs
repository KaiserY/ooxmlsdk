//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WordprocessingCanvas Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wpc:wpc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpc:CT_WordprocessingCanvas/wpc:wpc")]
pub struct WordprocessingCanvas {
  /// _
  #[sdk(child(qname = "a:CT_BackgroundFormatting/wpc:bg"))]
  pub background_formatting: Option<std::boxed::Box<BackgroundFormatting>>,
  /// _
  #[sdk(child(qname = "a:CT_WholeE2oFormatting/wpc:whole"))]
  pub whole_formatting: Option<std::boxed::Box<WholeFormatting>>,
  #[sdk(choice(
    qname = "wps:CT_WordprocessingShape/wps:wsp",
    qname = "pic:CT_Picture/pic:pic",
    qname = "w14:CT_WordContentPart/w14:contentPart",
    qname = "wpg:CT_WordprocessingGroup/wpg:wgp",
    qname = "wpg:CT_GraphicFrame/wpc:graphicFrame"
  ))]
  pub wordprocessing_canvas_choice: Vec<WordprocessingCanvasChoice>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/wpc:extLst"))]
  pub wpc_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundFormatting Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wpc:bg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BackgroundFormatting/wpc:bg")]
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wpc:whole.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_WholeE2oFormatting/wpc:whole")]
pub struct WholeFormatting {
  ///Outline
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub whole_formatting_choice: Option<WholeFormattingChoice>,
}
/// Defines the GraphicFrameType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wpc:graphicFrame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wpg:CT_GraphicFrame/wpc:graphicFrame")]
pub struct GraphicFrameType {
    /// _
    #[sdk(child(qname = "a:CT_NonVisualDrawingProps/wpg:cNvPr"))]
    pub non_visual_drawing_properties: std::boxed::Box<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::NonVisualDrawingProperties,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr"))]
    pub non_visual_graphic_frame_properties: std::boxed::Box<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::NonVisualGraphicFrameProperties,
    >,
    /// _
    #[sdk(child(qname = "a:CT_Transform2D/wpg:xfrm"))]
    pub transform2_d: std::boxed::Box<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::Transform2D,
    >,
    /// _
    #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
    pub graphic: std::boxed::Box<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
    >,
    /// _
    #[sdk(child(qname = "a:CT_OfficeArtExtensionList/wpg:extLst"))]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::OfficeArtExtensionList,
    >,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wpc:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/wpc:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WordprocessingCanvasChoice {
  #[sdk(child(qname = "wps:CT_WordprocessingShape/wps:wsp"))]
    WpsWsp(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
        >,
    ),
    #[sdk(child(qname = "pic:CT_Picture/pic:pic"))]
    PicPic(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture,
        >,
    ),
    #[sdk(child(qname = "w14:CT_WordContentPart/w14:contentPart"))]
    W14ContentPart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
        >,
    ),
    #[sdk(child(qname = "wpg:CT_WordprocessingGroup/wpg:wgp"))]
    WpgWgp(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::WordprocessingGroup,
        >,
    ),
    #[sdk(child(qname = "wpg:CT_GraphicFrame/wpc:graphicFrame"))]
    WpcGraphicFrame(std::boxed::Box<GraphicFrameType>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFormattingChoice {
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
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFormattingChoice2 {
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
pub enum WholeFormattingChoice {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
