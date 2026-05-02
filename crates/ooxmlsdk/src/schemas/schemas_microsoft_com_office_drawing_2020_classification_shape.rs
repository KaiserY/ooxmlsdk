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
  office2021,
  qname = "aclsh:CT_ClassificationOutcome/aclsh:classification"
)]
pub struct ClassificationOutcome {
  /// classificationOutcomeType
  #[sdk(attr(office2021, qname = ":classificationOutcomeType"))]
  #[sdk(string_format(kind = "token"))]
  pub classification_outcome_type: Option<ClassificationOutcomeType>,
}
