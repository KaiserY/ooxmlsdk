//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is p184:classification.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p184:CT_ClassificationOutcome/p184:classification")]
pub struct ClassificationOutcome {
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<ClassificationOutcomeType>,
}
