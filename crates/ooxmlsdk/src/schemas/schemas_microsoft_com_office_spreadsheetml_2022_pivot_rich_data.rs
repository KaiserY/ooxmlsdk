//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheRichInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xprd:richInfo")]
pub struct PivotCacheRichInfo {
  /// pivotCacheGuid
  #[sdk(attr(qname = ":pivotCacheGuid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub pivot_cache_guid: crate::simple_type::StringValue,
  /// pivotIgnoreInvalidCache
  #[sdk(attr(qname = ":pivotIgnoreInvalidCache"))]
  pub pivot_ignore_invalid_cache: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
}
