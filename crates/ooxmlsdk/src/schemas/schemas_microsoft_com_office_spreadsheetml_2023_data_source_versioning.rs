//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the VersionInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xxdsv:CT_VersionInfo/xxdsv:versionInfo")]
pub struct VersionInfo {
  /// _
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxdsv:requiredFeature"))]
  pub xxdsv_required_feature: Vec<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxdsv:lastRefreshFeature"))]
  pub xxdsv_last_refresh_feature: Vec<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(microsoft365, qname = "xsd:string/xxdsv:lastEditFeature"))]
  pub xxdsv_last_edit_feature: Vec<crate::simple_type::StringValue>,
}
/// Defines the RequiredFeatureXsdstring Class.
pub type RequiredFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastRefreshFeatureXsdstring Class.
pub type LastRefreshFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastEditFeatureXsdstring Class.
pub type LastEditFeatureXsdstring = crate::simple_type::StringValue;
