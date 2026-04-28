//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AlternateContent Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is mc:AlternateContent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_AlternateContent/mc:AlternateContent")]
pub struct AlternateContent {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// mc:MustUnderstand
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// mc:ProcessContent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(choice(qname = "mc:CT_Choice/mc:Choice", qname = "mc:CT_Fallback/mc:Fallback"))]
  pub alternate_content_choice: Vec<AlternateContentChoice>,
}
/// Defines the Choice Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is mc:Choice.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Choice/mc:Choice")]
pub struct Choice {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// Requires
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: Requires
  #[sdk(attr(qname = "Requires"))]
  pub requires: Option<crate::simple_type::StringValue>,
  /// mc:MustUnderstand
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// mc:ProcessContent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the Fallback Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is mc:Fallback.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Fallback/mc:Fallback")]
pub struct Fallback {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// mc:MustUnderstand
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// mc:ProcessContent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AlternateContentChoice {
  #[sdk(child(qname = "mc:CT_Choice/mc:Choice"))]
  McChoice(std::boxed::Box<Choice>),
  #[sdk(child(qname = "mc:CT_Fallback/mc:Fallback"))]
  McFallback(std::boxed::Box<Fallback>),
}
#[cfg(feature = "mce")]
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_AlternateContent/mc:AlternateContent")]
pub struct AlternateContentOf<T: crate::sdk::SdkChoice> {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(choice(qname = "mc:CT_Choice/mc:Choice", qname = "mc:CT_Fallback/mc:Fallback"))]
  pub alternate_content_choice: Vec<AlternateContentChoiceOf<T>>,
}
#[cfg(feature = "mce")]
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Choice/mc:Choice")]
pub struct ChoiceOf<T: crate::sdk::SdkChoice> {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  #[sdk(attr(qname = "Requires"))]
  pub requires: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub children: Vec<T>,
}
#[cfg(feature = "mce")]
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Fallback/mc:Fallback")]
pub struct FallbackOf<T: crate::sdk::SdkChoice> {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub mc_ignorable: Option<String>,
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub children: Vec<T>,
}
#[cfg(feature = "mce")]
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AlternateContentChoiceOf<T: crate::sdk::SdkChoice> {
  #[sdk(child(qname = "mc:CT_Choice/mc:Choice"))]
  McChoice(std::boxed::Box<ChoiceOf<T>>),
  #[sdk(child(qname = "mc:CT_Fallback/mc:Fallback"))]
  McFallback(std::boxed::Box<FallbackOf<T>>),
}
