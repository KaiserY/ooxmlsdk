//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Extension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oel:ext")]
pub struct Extension {
  /// uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
