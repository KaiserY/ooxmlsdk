//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HyperlinkColorEnum {
  #[sdk(rename = "hlink")]
  #[default]
  HLink,
  #[sdk(rename = "tx")]
  Tx,
}
/// Defines the HyperlinkColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "ahyp:CT_HyperlinkColor/ahyp:hlinkClr")]
pub struct HyperlinkColor {
  /// val
  #[sdk(attr(office2019, qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: HyperlinkColorEnum,
}
