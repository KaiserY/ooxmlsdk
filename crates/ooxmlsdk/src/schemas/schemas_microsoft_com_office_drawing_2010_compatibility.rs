//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CompatibilityShape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "com14:CT_CompatShape/com14:compatSp")]
pub struct CompatibilityShape {
  /// spid
  #[sdk(attr(office2010, qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
