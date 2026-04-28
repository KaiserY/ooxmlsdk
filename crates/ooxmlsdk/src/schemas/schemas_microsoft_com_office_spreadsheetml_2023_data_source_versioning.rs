//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the VersionInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxdsv:versionInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxdsv:CT_VersionInfo/xxdsv:versionInfo")]
pub struct VersionInfo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/xxdsv:requiredFeature"))]
  pub xxdsv_required_feature: Vec<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/xxdsv:lastRefreshFeature"))]
  pub xxdsv_last_refresh_feature: Vec<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/xxdsv:lastEditFeature"))]
  pub xxdsv_last_edit_feature: Vec<crate::simple_type::StringValue>,
}
/// Defines the RequiredFeatureXsdstring Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxdsv:requiredFeature.
pub type RequiredFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastRefreshFeatureXsdstring Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxdsv:lastRefreshFeature.
pub type LastRefreshFeatureXsdstring = crate::simple_type::StringValue;
/// Defines the LastEditFeatureXsdstring Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxdsv:lastEditFeature.
pub type LastEditFeatureXsdstring = crate::simple_type::StringValue;
