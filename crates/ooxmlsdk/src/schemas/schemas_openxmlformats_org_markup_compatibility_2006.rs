//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AlternateContent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_AlternateContent/mc:AlternateContent")]
pub struct AlternateContent {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(choice(qname = "mc:CT_Choice/mc:Choice", qname = "mc:CT_Fallback/mc:Fallback"))]
  pub alternate_content_choice: Vec<AlternateContentChoice>,
}
/// Defines the Choice Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Choice/mc:Choice")]
pub struct Choice {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
/// Defines the Fallback Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Fallback/mc:Fallback")]
pub struct Fallback {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AlternateContentChoice {
  /// Defines the Choice Class.
  Choice(std::boxed::Box<Choice>),
  /// Defines the Fallback Class.
  Fallback(std::boxed::Box<Fallback>),
}
