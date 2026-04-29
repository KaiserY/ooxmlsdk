//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SizeRelativeHorizontallyValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "leftMargin")]
  LeftMargin,
  #[sdk(rename = "rightMargin")]
  RightMargin,
  #[sdk(rename = "insideMargin")]
  InsideMargin,
  #[sdk(rename = "outsideMargin")]
  OutsideMargin,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SizeRelativeVerticallyValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "page")]
  Page,
  #[sdk(rename = "topMargin")]
  TopMargin,
  #[sdk(rename = "bottomMargin")]
  BottomMargin,
  #[sdk(rename = "insideMargin")]
  InsideMargin,
  #[sdk(rename = "outsideMargin")]
  OutsideMargin,
}
/// Defines the PercentagePositionHeightOffset Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:pctPosHOffset.
pub type PercentagePositionHeightOffset = crate::simple_type::StringValue;
/// Defines the PercentagePositionVerticalOffset Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:pctPosVOffset.
pub type PercentagePositionVerticalOffset = crate::simple_type::StringValue;
/// Defines the RelativeWidth Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:sizeRelH.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp14:CT_SizeRelH/wp14:sizeRelH")]
pub struct RelativeWidth {
  /// relativeFrom
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :relativeFrom
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub object_id: SizeRelativeHorizontallyValues,
  /// _
  #[sdk(text_child(qname = "a:ST_PositivePercentage/wp14:pctWidth"))]
  pub percentage_width: crate::simple_type::StringValue,
}
/// Defines the RelativeHeight Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:sizeRelV.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp14:CT_SizeRelV/wp14:sizeRelV")]
pub struct RelativeHeight {
  /// relativeFrom
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :relativeFrom
  #[sdk(attr(qname = ":relativeFrom"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub relative_from: SizeRelativeVerticallyValues,
  /// _
  #[sdk(text_child(qname = "a:ST_PositivePercentage/wp14:pctHeight"))]
  pub percentage_height: crate::simple_type::StringValue,
}
/// Defines the PercentageWidth Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:pctWidth.
pub type PercentageWidth = crate::simple_type::StringValue;
/// Defines the PercentageHeight Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp14:pctHeight.
pub type PercentageHeight = crate::simple_type::StringValue;
