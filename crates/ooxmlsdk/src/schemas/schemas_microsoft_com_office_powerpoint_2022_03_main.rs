//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Reactions Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p223:reactions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p223:CT_Reactions/p223:reactions")]
pub struct Reactions {
  /// _
  #[sdk(child(qname = "p223:CT_Reaction/p223:rxn"))]
  pub p223_rxn: Vec<Reaction>,
}
/// Defines the ExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p223:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p223:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the ReactionInstance Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p223:instance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p223:CT_ReactionInstance/p223:instance")]
pub struct ReactionInstance {
  /// time
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p223:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Reaction Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p223:rxn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p223:CT_Reaction/p223:rxn")]
pub struct Reaction {
  /// type
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p223:CT_ReactionInstance/p223:instance"))]
  pub p223_instance: Vec<ReactionInstance>,
}
