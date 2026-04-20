//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum BorderValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "single")]
  Single,
  #[sdk(rename = "thick")]
  Thick,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "hairline")]
  Hairline,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "dotDash")]
  DotDash,
  #[sdk(rename = "dashDotDot")]
  DashDotDot,
  #[sdk(rename = "triple")]
  Triple,
  #[sdk(rename = "thinThickSmall")]
  ThinThickSmall,
  #[sdk(rename = "thickThinSmall")]
  ThickThinSmall,
  #[sdk(rename = "thickBetweenThinSmall")]
  ThickBetweenThinSmall,
  #[sdk(rename = "thinThick")]
  ThinThick,
  #[sdk(rename = "thickThin")]
  ThickThin,
  #[sdk(rename = "thickBetweenThin")]
  ThickBetweenThin,
  #[sdk(rename = "thinThickLarge")]
  ThinThickLarge,
  #[sdk(rename = "thickThinLarge")]
  ThickThinLarge,
  #[sdk(rename = "thickBetweenThinLarge")]
  ThickBetweenThinLarge,
  #[sdk(rename = "wave")]
  Wave,
  #[sdk(rename = "doubleWave")]
  DoubleWave,
  #[sdk(rename = "dashedSmall")]
  DashedSmall,
  #[sdk(rename = "dashDotStroked")]
  DashDotStroked,
  #[sdk(rename = "threeDEmboss")]
  ThreeDEmboss,
  #[sdk(rename = "threeDEngrave")]
  ThreeDEngrave,
  #[sdk(rename = "HTMLOutset")]
  HtmlOutset,
  #[sdk(rename = "HTMLInset")]
  HtmlInset,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum WrapValues {
  #[sdk(rename = "topAndBottom")]
  #[default]
  TopAndBottom,
  #[sdk(rename = "square")]
  Square,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "tight")]
  Tight,
  #[sdk(rename = "through")]
  Through,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum WrapSideValues {
  #[sdk(rename = "both")]
  #[default]
  Both,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "largest")]
  Largest,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAnchorValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "text")]
  Text,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAnchorValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "text")]
  Text,
}
/// Top Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:bordertop.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Border/w10:bordertop")]
pub struct TopBorder {
  /// Border Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<BorderValues>,
  /// Border Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub width: Option<crate::simple_type::IntegerValue>,
  /// Border shadow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadow
  #[sdk(attr(qname = ":shadow"))]
  pub shadow: Option<crate::simple_type::TrueFalseValue>,
}
/// Left Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:borderleft.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Border/w10:borderleft")]
pub struct LeftBorder {
  /// Border Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<BorderValues>,
  /// Border Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub width: Option<crate::simple_type::IntegerValue>,
  /// Border shadow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadow
  #[sdk(attr(qname = ":shadow"))]
  pub shadow: Option<crate::simple_type::TrueFalseValue>,
}
/// Right Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:borderright.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Border/w10:borderright")]
pub struct RightBorder {
  /// Border Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<BorderValues>,
  /// Border Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub width: Option<crate::simple_type::IntegerValue>,
  /// Border shadow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadow
  #[sdk(attr(qname = ":shadow"))]
  pub shadow: Option<crate::simple_type::TrueFalseValue>,
}
/// Bottom Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:borderbottom.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Border/w10:borderbottom")]
pub struct BottomBorder {
  /// Border Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<BorderValues>,
  /// Border Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub width: Option<crate::simple_type::IntegerValue>,
  /// Border shadow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadow
  #[sdk(attr(qname = ":shadow"))]
  pub shadow: Option<crate::simple_type::TrueFalseValue>,
}
/// Defines the BorderType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Border/")]
pub struct BorderType {
  /// Border Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<BorderValues>,
  /// Border Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub width: Option<crate::simple_type::IntegerValue>,
  /// Border shadow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadow
  #[sdk(attr(qname = ":shadow"))]
  pub shadow: Option<crate::simple_type::TrueFalseValue>,
}
/// Text Wrapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:wrap.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_Wrap/w10:wrap")]
pub struct TextWrap {
  /// Wrapping type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<WrapValues>,
  /// Wrapping side
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :side
  #[sdk(attr(qname = ":side"))]
  pub side: Option<WrapSideValues>,
  /// Horizontal Positioning Base
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchorx
  #[sdk(attr(qname = ":anchorx"))]
  pub anchor_x: Option<HorizontalAnchorValues>,
  /// Vertical Positioning Base
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchory
  #[sdk(attr(qname = ":anchory"))]
  pub anchor_y: Option<VerticalAnchorValues>,
}
/// Anchor Location Is Locked.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w10:anchorlock.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w10:CT_AnchorLock/w10:anchorlock")]
pub struct AnchorLock {}
