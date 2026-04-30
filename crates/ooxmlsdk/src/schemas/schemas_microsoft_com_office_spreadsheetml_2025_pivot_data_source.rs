//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheDataSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlpds:CT_PivotCacheDataSource/xlpds:pivotCacheDataSource"
)]
pub struct PivotCacheDataSource {
  /// ref
  #[sdk(attr(microsoft365, qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
}
