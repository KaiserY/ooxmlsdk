//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Legacy Drawing Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(canonical_namespace_prefix("com:comp"), qname = "comp:legacyDrawing")]
pub struct LegacyDrawing {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
