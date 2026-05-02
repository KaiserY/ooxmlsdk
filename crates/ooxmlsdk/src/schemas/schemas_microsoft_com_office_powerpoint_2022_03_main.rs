//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Reactions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:CT_Reactions/p223:reactions")]
pub struct Reactions {
  /// Defines the Reaction Class.
  #[sdk(child(microsoft365, qname = "p223:CT_Reaction/p223:rxn"))]
  pub p223_rxn: Vec<Reaction>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p:CT_ExtensionList/p223:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<crate::schemas::p::Extension>,
}
/// Defines the ReactionInstance Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:CT_ReactionInstance/p223:instance")]
pub struct ReactionInstance {
  /// time
  #[sdk(attr(microsoft365, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p223:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Reaction Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p223:CT_Reaction/p223:rxn")]
pub struct Reaction {
  /// type
  #[sdk(attr(microsoft365, qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// Defines the ReactionInstance Class.
  #[sdk(child(microsoft365, qname = "p223:CT_ReactionInstance/p223:instance"))]
  pub p223_instance: Vec<ReactionInstance>,
}
