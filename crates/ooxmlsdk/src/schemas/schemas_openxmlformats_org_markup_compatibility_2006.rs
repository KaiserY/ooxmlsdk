//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AlternateContent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_AlternateContent/mc:AlternateContent")]
pub struct AlternateContent {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  #[sdk(choice(qname = "mc:CT_Choice/mc:Choice", qname = "mc:CT_Fallback/mc:Fallback"))]
  pub alternate_content_choice: Vec<AlternateContentChoice>,
}
/// Defines the Choice Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Choice/mc:Choice")]
pub struct Choice {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<str>>,
}
/// Defines the Fallback Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Fallback/mc:Fallback")]
pub struct Fallback {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<str>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AlternateContentChoice {
  #[sdk(child(qname = "mc:CT_Choice/mc:Choice"))]
  McChoice(std::boxed::Box<Choice>),
  #[sdk(child(qname = "mc:CT_Fallback/mc:Fallback"))]
  McFallback(std::boxed::Box<Fallback>),
}
