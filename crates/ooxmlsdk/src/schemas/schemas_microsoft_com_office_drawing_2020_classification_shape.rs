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
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is aclsh:classification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aclsh:CT_ClassificationOutcome/aclsh:classification")]
pub struct ClassificationOutcome {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// classificationOutcomeType
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :classificationOutcomeType
  #[sdk(attr(qname = ":classificationOutcomeType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub classification_outcome_type: Option<ClassificationOutcomeType>,
}
