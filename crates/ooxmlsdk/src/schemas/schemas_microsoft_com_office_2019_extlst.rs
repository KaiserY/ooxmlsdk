//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Extension Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is oel:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oel:CT_Extension/oel:ext")]
pub struct Extension {
  /// uri
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
