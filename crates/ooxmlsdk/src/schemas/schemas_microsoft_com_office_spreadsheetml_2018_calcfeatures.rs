//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CalcFeatures Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xcalcf:calcFeatures.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xcalcf:CT_CalcFeatures/xcalcf:calcFeatures")]
pub struct CalcFeatures {
  /// _
  #[sdk(child(qname = "xcalcf:CT_CalcFeature/xcalcf:feature"))]
  pub xcalcf_feature: Vec<CalcFeature>,
}
/// Defines the CalcFeature Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xcalcf:feature.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xcalcf:CT_CalcFeature/xcalcf:feature")]
pub struct CalcFeature {
  /// name
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
