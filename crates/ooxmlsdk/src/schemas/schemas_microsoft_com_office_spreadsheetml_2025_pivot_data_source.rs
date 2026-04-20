//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheDataSource Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpds:pivotCacheDataSource.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpds:CT_PivotCacheDataSource/xlpds:pivotCacheDataSource")]
pub struct PivotCacheDataSource {
  /// ref
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
}
