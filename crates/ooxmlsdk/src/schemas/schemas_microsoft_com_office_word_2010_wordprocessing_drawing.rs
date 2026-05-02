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
pub type PercentagePositionHeightOffset = crate::simple_type::Int32Value;
/// Defines the PercentagePositionVerticalOffset Class.
pub type PercentagePositionVerticalOffset = crate::simple_type::Int32Value;
/// Defines the RelativeWidth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wp14:CT_SizeRelH/wp14:sizeRelH")]
pub struct RelativeWidth {
  /// relativeFrom
  #[sdk(attr(office2010, qname = ":relativeFrom"))]
  #[sdk(string_format(kind = "token"))]
  pub object_id: SizeRelativeHorizontallyValues,
  /// Defines the PercentageWidth Class.
  #[sdk(text_child(office2010, qname = "a:ST_PositivePercentage/wp14:pctWidth"))]
  pub percentage_width: crate::simple_type::Int32Value,
}
/// Defines the RelativeHeight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "wp14:CT_SizeRelV/wp14:sizeRelV")]
pub struct RelativeHeight {
  /// relativeFrom
  #[sdk(attr(office2010, qname = ":relativeFrom"))]
  #[sdk(string_format(kind = "token"))]
  pub relative_from: SizeRelativeVerticallyValues,
  /// Defines the PercentageHeight Class.
  #[sdk(text_child(office2010, qname = "a:ST_PositivePercentage/wp14:pctHeight"))]
  pub percentage_height: crate::simple_type::Int32Value,
}
/// Defines the PercentageWidth Class.
pub type PercentageWidth = crate::simple_type::Int32Value;
/// Defines the PercentageHeight Class.
pub type PercentageHeight = crate::simple_type::Int32Value;
