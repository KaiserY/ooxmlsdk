//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ScriptLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "asl:CT_ScriptLink/asl:scriptLink")]
pub struct ScriptLink {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(microsoft365, qname = "a:CT_OfficeArtExtensionList/asl:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "a:CT_OfficeArtExtensionList/asl:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
