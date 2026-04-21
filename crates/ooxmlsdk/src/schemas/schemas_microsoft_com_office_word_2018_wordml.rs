//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Extension Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cur:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cur:CT_Extension/w16cur:ext")]
pub struct Extension {
  /// uri
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: w16cur:uri
  #[sdk(attr(qname = "w16cur:uri"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub w16cur_uri: Option<crate::simple_type::StringValue>,
  #[sdk(choice(any))]
  pub xml_children: Vec<ExtensionChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ExtensionChoice {
  #[sdk(any)]
  UnknownXml(String),
}
