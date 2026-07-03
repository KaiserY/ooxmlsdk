//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the VersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxdsv:versionInfo")]
pub struct VersionInfo {
  /// Defines the RequiredFeatureXsdstring Class.
  #[sdk(text_child(qname = "xxdsv:requiredFeature"))]
  pub required_feature_xsdstring: Vec<RequiredFeatureXsdstring>,
  /// Defines the LastRefreshFeatureXsdstring Class.
  #[sdk(text_child(qname = "xxdsv:lastRefreshFeature"))]
  pub last_refresh_feature_xsdstring: Vec<LastRefreshFeatureXsdstring>,
  /// Defines the LastEditFeatureXsdstring Class.
  #[sdk(text_child(qname = "xxdsv:lastEditFeature"))]
  pub last_edit_feature_xsdstring: Vec<LastEditFeatureXsdstring>,
}
/// Defines the RequiredFeatureXsdstring Class.
pub type RequiredFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastRefreshFeatureXsdstring Class.
pub type LastRefreshFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastEditFeatureXsdstring Class.
pub type LastEditFeatureXsdstring = crate::simple_type::StringValue;
