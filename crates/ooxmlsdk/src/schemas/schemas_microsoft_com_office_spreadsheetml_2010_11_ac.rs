//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AbsolutePath Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15ac:absPath.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15ac:CT_AbsolutePath/x15ac:absPath")]
pub struct AbsolutePath {
  /// url
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :url
  #[sdk(attr(qname = ":url"))]
  pub url: crate::simple_type::StringValue,
}
