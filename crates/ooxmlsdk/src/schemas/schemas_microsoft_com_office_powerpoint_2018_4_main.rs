//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ClassificationOutcomeType {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "hdr")]
  Hdr,
  #[sdk(rename = "ftr")]
  Ftr,
  #[sdk(rename = "watermark")]
  Watermark,
}
/// Defines the ClassificationOutcome Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "p184:CT_ClassificationOutcome/p184:classification"
)]
pub struct ClassificationOutcome {
  /// val
  #[sdk(attr(office2019, qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<ClassificationOutcomeType>,
}
