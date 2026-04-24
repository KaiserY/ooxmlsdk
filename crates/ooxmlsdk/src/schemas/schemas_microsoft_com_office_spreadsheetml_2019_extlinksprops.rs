//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalLinksPr Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxlnp:externalLinksPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxlnp:CT_ExternalLinksPr/xxlnp:externalLinksPr")]
pub struct ExternalLinksPr {
  /// autoRefresh
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :autoRefresh
  #[sdk(attr(qname = ":autoRefresh"))]
  pub auto_refresh: Option<crate::simple_type::BooleanValue>,
}
