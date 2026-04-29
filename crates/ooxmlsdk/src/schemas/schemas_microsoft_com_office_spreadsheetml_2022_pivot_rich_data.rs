//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheRichInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xprd:richInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xprd:CT_PivotCacheRichInfo/xprd:richInfo")]
pub struct PivotCacheRichInfo {
  /// pivotCacheGuid
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :pivotCacheGuid
  #[sdk(attr(qname = ":pivotCacheGuid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub pivot_cache_guid: crate::simple_type::StringValue,
  /// pivotIgnoreInvalidCache
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :pivotIgnoreInvalidCache
  #[sdk(attr(qname = ":pivotIgnoreInvalidCache"))]
  pub pivot_ignore_invalid_cache: Option<crate::simple_type::BooleanValue>,
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
}
