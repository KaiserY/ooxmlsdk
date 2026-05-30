//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CacheVersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xxpvi:cacheVersionInfo")]
pub struct CacheVersionInfo {
  /// Defines the RequiredFeatureXsdstring Class.
  #[sdk(text_child(
    microsoft365,
    simple_type = "StringValue",
    qname = "xxpvi:requiredFeature"
  ))]
  pub required_feature_xsdstring: Vec<RequiredFeatureXsdstring>,
  /// Defines the LastRefreshFeatureXsdstring Class.
  #[sdk(text_child(
    microsoft365,
    simple_type = "StringValue",
    qname = "xxpvi:lastRefreshFeature"
  ))]
  pub last_refresh_feature_xsdstring: Vec<LastRefreshFeatureXsdstring>,
}
/// Defines the PivotVersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xxpvi:pivotVersionInfo")]
pub struct PivotVersionInfo {
  /// Defines the RequiredFeatureXsdstring Class.
  #[sdk(text_child(
    microsoft365,
    simple_type = "StringValue",
    qname = "xxpvi:requiredFeature"
  ))]
  pub required_feature_xsdstring: Vec<RequiredFeatureXsdstring>,
  /// Defines the LastUpdateFeatureXsdstring Class.
  #[sdk(text_child(
    microsoft365,
    simple_type = "StringValue",
    qname = "xxpvi:lastUpdateFeature"
  ))]
  pub last_update_feature_xsdstring: Vec<LastUpdateFeatureXsdstring>,
}
/// Defines the RequiredFeatureXsdstring Class.
pub type RequiredFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastRefreshFeatureXsdstring Class.
pub type LastRefreshFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastUpdateFeatureXsdstring Class.
pub type LastUpdateFeatureXsdstring = crate::simple_type::StringValue;
