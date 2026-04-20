//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Legacy Drawing Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is comp:legacyDrawing.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "comp:CT_Compat/comp:legacyDrawing")]
pub struct LegacyDrawing {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
