//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheRichInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xprd:CT_PivotCacheRichInfo/xprd:richInfo")]
pub struct PivotCacheRichInfo {
  /// pivotCacheGuid
  #[sdk(attr(microsoft365, qname = ":pivotCacheGuid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub pivot_cache_guid: crate::simple_type::StringValue,
  /// pivotIgnoreInvalidCache
  #[sdk(attr(microsoft365, qname = ":pivotIgnoreInvalidCache"))]
  pub pivot_ignore_invalid_cache: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(microsoft365, qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
}
