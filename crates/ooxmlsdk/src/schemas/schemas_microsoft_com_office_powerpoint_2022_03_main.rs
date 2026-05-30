//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Reactions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:reactions")]
pub struct Reactions {
  /// Defines the Reaction Class.
  #[sdk(child(microsoft365, qname = "p223:rxn"))]
  pub reaction: Vec<Reaction>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the ReactionInstance Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:instance")]
pub struct ReactionInstance {
  /// time
  #[sdk(attr(microsoft365, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p223:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Reaction Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:rxn")]
pub struct Reaction {
  /// type
  #[sdk(attr(microsoft365, qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// Defines the ReactionInstance Class.
  #[sdk(child(microsoft365, qname = "p223:instance"))]
  pub reaction_instance: Vec<ReactionInstance>,
}
