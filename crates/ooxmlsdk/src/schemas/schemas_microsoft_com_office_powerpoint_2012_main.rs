//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PresetTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_PresetTransition/p15:prstTrans")]
pub struct PresetTransition {
  /// prst
  #[sdk(attr(office2013, qname = ":prst"))]
  pub preset: Option<crate::simple_type::StringValue>,
  /// invX
  #[sdk(attr(office2013, qname = ":invX"))]
  pub inv_x: Option<crate::simple_type::BooleanValue>,
  /// invY
  #[sdk(attr(office2013, qname = ":invY"))]
  pub inv_y: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PresenceInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_PresenceInfo/p15:presenceInfo")]
pub struct PresenceInfo {
  /// userId
  #[sdk(attr(office2013, qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  #[sdk(attr(office2013, qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
}
/// Defines the ThreadingInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_CommentThreading/p15:threadingInfo")]
pub struct ThreadingInfo {
  /// timeZoneBias
  #[sdk(attr(office2013, qname = ":timeZoneBias"))]
  pub time_zone_bias: Option<crate::simple_type::Int32Value>,
  /// Defines the ParentCommentIdentifier Class.
  #[sdk(child(office2013, qname = "p15:CT_ParentCommentIdentifier/p15:parentCm"))]
  pub parent_comment_identifier: Option<ParentCommentIdentifier>,
}
/// Defines the SlideGuideList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst")]
pub struct SlideGuideList {
  /// Defines the ExtendedGuide Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuide/p15:guide"))]
  pub p15_guide: Vec<ExtendedGuide>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "p:CT_ExtensionList/p15:extLst"))]
  pub p15_ext_lst: Option<ExtensionList>,
}
/// Defines the NotesGuideList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst")]
pub struct NotesGuideList {
  /// Defines the ExtendedGuide Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuide/p15:guide"))]
  pub p15_guide: Vec<ExtendedGuide>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "p:CT_ExtensionList/p15:extLst"))]
  pub p15_ext_lst: Option<ExtensionList>,
}
/// Defines the ChartTrackingReferenceBased Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased"
)]
pub struct ChartTrackingReferenceBased {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2013, qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ParentCommentIdentifier Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_ParentCommentIdentifier/p15:parentCm")]
pub struct ParentCommentIdentifier {
  /// authorId
  #[sdk(attr(office2013, qname = ":authorId"))]
  pub author_id: Option<crate::simple_type::UInt32Value>,
  /// idx
  #[sdk(attr(office2013, qname = ":idx"))]
  pub index: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ColorType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_Color/p15:clr")]
pub struct ColorType {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub color_type_choice: Option<ColorTypeChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p:CT_ExtensionList/p15:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the ExtendedGuide Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "p15:CT_ExtendedGuide/p15:guide")]
pub struct ExtendedGuide {
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// orient
  #[sdk(attr(office2013, qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
  /// pos
  #[sdk(attr(office2013, qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
  /// userDrawn
  #[sdk(attr(office2013, qname = ":userDrawn"))]
  pub is_user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Defines the ColorType Class.
  #[sdk(child(office2013, qname = "a:CT_Color/p15:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "p:CT_ExtensionList/p15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorTypeChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
