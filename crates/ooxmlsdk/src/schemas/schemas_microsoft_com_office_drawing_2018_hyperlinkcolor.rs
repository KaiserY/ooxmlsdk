//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum HyperlinkColorEnum {
  #[sdk(rename = "hlink")]
  #[default]
  HLink,
  #[sdk(rename = "tx")]
  Tx,
}
/// Defines the HyperlinkColor Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is ahyp:hlinkClr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ahyp:CT_HyperlinkColor/ahyp:hlinkClr")]
pub struct HyperlinkColor {
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub val: HyperlinkColorEnum,
}
