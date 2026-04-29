//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CompatibilityShape Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is com14:compatSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "com14:CT_CompatShape/com14:compatSp")]
pub struct CompatibilityShape {
  /// spid
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
