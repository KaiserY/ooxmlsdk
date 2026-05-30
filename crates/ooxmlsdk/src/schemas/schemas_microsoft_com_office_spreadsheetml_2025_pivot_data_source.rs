//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheDataSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpds:pivotCacheDataSource")]
pub struct PivotCacheDataSource {
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// Defines the Formula Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "xne:f"))]
  pub formula: Option<crate::schemas::xne::Formula>,
}
