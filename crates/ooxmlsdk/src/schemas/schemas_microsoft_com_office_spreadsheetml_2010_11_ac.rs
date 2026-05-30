//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AbsolutePath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15ac:absPath")]
pub struct AbsolutePath {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// url
  #[sdk(attr(qname = ":url"))]
  pub url: crate::simple_type::StringValue,
}
