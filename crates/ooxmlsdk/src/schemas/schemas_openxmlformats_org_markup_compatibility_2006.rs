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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_AlternateContent/mc:AlternateContent")]
pub struct AlternateContent {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Choice/mc:Choice")]
pub struct Choice {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mc:CT_Fallback/mc:Fallback")]
pub struct Fallback {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum AlternateContentChoice {
  #[sdk(child(qname = "mc:CT_Choice/mc:Choice"))]
  McChoice(std::boxed::Box<Choice>),
  #[sdk(child(qname = "mc:CT_Fallback/mc:Fallback"))]
  McFallback(std::boxed::Box<Fallback>),
}
