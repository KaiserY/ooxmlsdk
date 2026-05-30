//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Extension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cur:ext")]
pub struct Extension {
  /// uri
  #[sdk(attr(qname = "w16cur:uri"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub w16cur_uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
