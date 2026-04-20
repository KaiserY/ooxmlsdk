//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRefreshIntervals Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshIntervals.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvr:CT_RichValueRefreshIntervals/xlrvr:refreshIntervals")]
pub struct RichValueRefreshIntervals {
  /// _
  #[sdk(child(qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval"))]
  pub xlrvr_refresh_interval: Vec<RichValueRefreshInterval>,
}
/// Defines the RichValueRefreshInterval Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrvr:refreshInterval.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval")]
pub struct RichValueRefreshInterval {
  /// resourceIdInt
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :resourceIdInt
  #[sdk(attr(qname = ":resourceIdInt"))]
  pub resource_id_int: Option<crate::simple_type::Int32Value>,
  /// resourceIdStr
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :resourceIdStr
  #[sdk(attr(qname = ":resourceIdStr"))]
  pub resource_id_str: Option<crate::simple_type::StringValue>,
  /// interval
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :interval
  #[sdk(attr(qname = ":interval"))]
  pub interval: crate::simple_type::Int32Value,
}
