//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the RichValueRefreshIntervals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrvr:CT_RichValueRefreshIntervals/xlrvr:refreshIntervals"
)]
pub struct RichValueRefreshIntervals {
  /// Defines the RichValueRefreshInterval Class.
  #[sdk(child(
    office2021,
    qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval"
  ))]
  pub xlrvr_refresh_interval: Vec<RichValueRefreshInterval>,
}
/// Defines the RichValueRefreshInterval Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval"
)]
pub struct RichValueRefreshInterval {
  /// resourceIdInt
  #[sdk(attr(office2021, qname = ":resourceIdInt"))]
  pub resource_id_int: Option<crate::simple_type::Int32Value>,
  /// resourceIdStr
  #[sdk(attr(office2021, qname = ":resourceIdStr"))]
  pub resource_id_str: Option<crate::simple_type::StringValue>,
  /// interval
  #[sdk(attr(office2021, qname = ":interval"))]
  pub interval: crate::simple_type::Int32Value,
}
