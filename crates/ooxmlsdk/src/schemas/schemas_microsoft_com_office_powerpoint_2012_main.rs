//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PresetTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:prstTrans")]
pub struct PresetTransition {
  /// prst
  #[sdk(attr(qname = ":prst"))]
  pub preset: Option<crate::simple_type::StringValue>,
  /// invX
  #[sdk(attr(qname = ":invX"))]
  pub inv_x: Option<crate::simple_type::BooleanValue>,
  /// invY
  #[sdk(attr(qname = ":invY"))]
  pub inv_y: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PresenceInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:presenceInfo")]
pub struct PresenceInfo {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
}
/// Defines the ThreadingInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:threadingInfo")]
pub struct ThreadingInfo {
  /// timeZoneBias
  #[sdk(attr(qname = ":timeZoneBias"))]
  pub time_zone_bias: Option<crate::simple_type::Int32Value>,
  /// Defines the ParentCommentIdentifier Class.
  #[sdk(child(qname = "p15:parentCm"))]
  pub parent_comment_identifier: Option<ParentCommentIdentifier>,
}
/// Defines the SlideGuideList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:sldGuideLst")]
pub struct SlideGuideList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ExtendedGuide Class.
  #[sdk(child(qname = "p15:guide"))]
  pub extended_guide: Vec<ExtendedGuide>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NotesGuideList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:notesGuideLst")]
pub struct NotesGuideList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ExtendedGuide Class.
  #[sdk(child(qname = "p15:guide"))]
  pub extended_guide: Vec<ExtendedGuide>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ChartTrackingReferenceBased Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:chartTrackingRefBased")]
pub struct ChartTrackingReferenceBased {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ParentCommentIdentifier Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:parentCm")]
pub struct ParentCommentIdentifier {
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: Option<crate::simple_type::UInt32Value>,
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ColorType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:clr")]
pub struct ColorType {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_type_choice: Option<ColorTypeChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the ExtendedGuide Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:guide")]
pub struct ExtendedGuide {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation: Option<crate::schemas::p::DirectionValues>,
  /// pos
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
  /// userDrawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub is_user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Defines the ColorType Class.
  #[sdk(child(qname = "p15:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorTypeChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
