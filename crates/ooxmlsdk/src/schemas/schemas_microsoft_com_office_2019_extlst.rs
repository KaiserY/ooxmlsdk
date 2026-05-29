//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Extension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "oel:CT_Extension/oel:ext")]
pub struct Extension {
  /// uri
  #[sdk(attr(office2021, qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<str>>,
}
