//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CacheVersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xxpvi:CT_CacheVersionInfo/xxpvi:cacheVersionInfo"
)]
pub struct CacheVersionInfo {
  /// Defines the RequiredFeatureXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxpvi:requiredFeature"))]
  pub xxpvi_required_feature: Vec<RequiredFeatureXsdstring>,
  /// Defines the LastRefreshFeatureXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxpvi:lastRefreshFeature"))]
  pub xxpvi_last_refresh_feature: Vec<LastRefreshFeatureXsdstring>,
}
/// Defines the PivotVersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xxpvi:CT_PivotVersionInfo/xxpvi:pivotVersionInfo"
)]
pub struct PivotVersionInfo {
  /// Defines the RequiredFeatureXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxpvi:requiredFeature"))]
  pub xxpvi_required_feature: Vec<RequiredFeatureXsdstring>,
  /// Defines the LastUpdateFeatureXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxpvi:lastUpdateFeature"))]
  pub xxpvi_last_update_feature: Vec<LastUpdateFeatureXsdstring>,
}
/// Defines the RequiredFeatureXsdstring Class.
pub type RequiredFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastRefreshFeatureXsdstring Class.
pub type LastRefreshFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastUpdateFeatureXsdstring Class.
pub type LastUpdateFeatureXsdstring = crate::simple_type::StringValue;
