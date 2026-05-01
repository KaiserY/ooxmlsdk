//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CalcFeatures Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xcalcf:CT_CalcFeatures/xcalcf:calcFeatures")]
pub struct CalcFeatures {
  /// Defines the CalcFeature Class.
  #[sdk(child(office2019, qname = "xcalcf:CT_CalcFeature/xcalcf:feature"))]
  pub xcalcf_feature: Vec<CalcFeature>,
}
/// Defines the CalcFeature Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xcalcf:CT_CalcFeature/xcalcf:feature")]
pub struct CalcFeature {
  /// name
  #[sdk(attr(office2019, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
