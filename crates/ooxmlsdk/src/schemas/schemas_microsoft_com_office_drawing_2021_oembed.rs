//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the OEmbedShared Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is aoe:oembedShared.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aoe:CT_OEmbedShared/aoe:oembedShared")]
pub struct OEmbedShared {
  /// srcUrl
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :srcUrl
  #[sdk(attr(qname = ":srcUrl"))]
  pub src_url: crate::simple_type::StringValue,
  /// type
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/aoe:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is aoe:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/aoe:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
