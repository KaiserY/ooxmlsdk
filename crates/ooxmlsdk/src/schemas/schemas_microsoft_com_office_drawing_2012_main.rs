//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TargetScreenSize {
  #[sdk(rename = "544x376")]
  #[default]
  Sz544x376,
  #[sdk(rename = "640x480")]
  Sz640x480,
  #[sdk(rename = "720x512")]
  Sz720x512,
  #[sdk(rename = "800x600")]
  Sz800x600,
  #[sdk(rename = "1024x768")]
  Sz1024x768,
  #[sdk(rename = "1152x882")]
  Sz1152x882,
  #[sdk(rename = "1152x900")]
  Sz1152x900,
  #[sdk(rename = "1280x1024")]
  Sz1280x1024,
  #[sdk(rename = "1600x1200")]
  Sz1600x1200,
  #[sdk(rename = "1800x1440")]
  Sz1800x1440,
  #[sdk(rename = "1920x1200")]
  Sz1920x1200,
}
/// Defines the BackgroundProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a15:backgroundPr")]
pub struct BackgroundProperties {
  /// bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// bwPure
  #[sdk(attr(qname = ":bwPure"))]
  #[sdk(string_format(kind = "token"))]
  pub pure: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// bwNormal
  #[sdk(attr(qname = ":bwNormal"))]
  #[sdk(string_format(kind = "token"))]
  pub normal: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// targetScreenSize
  #[sdk(attr(qname = ":targetScreenSize"))]
  #[sdk(string_format(kind = "token"))]
  pub target_screen_size: Option<TargetScreenSize>,
}
/// Defines the NonVisualGroupProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a15:nonVisualGroupProps")]
pub struct NonVisualGroupProperties {
  /// isLegacyGroup
  #[sdk(attr(qname = ":isLegacyGroup"))]
  pub is_legacy_group: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ObjectProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a15:objectPr")]
pub struct ObjectProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// objectId
  #[sdk(attr(qname = ":objectId"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// isActiveX
  #[sdk(attr(qname = ":isActiveX"))]
  pub is_active_x: Option<crate::simple_type::BooleanValue>,
  /// linkType
  #[sdk(attr(qname = ":linkType"))]
  pub link_type: Option<crate::simple_type::StringValue>,
}
/// Defines the SignatureLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a15:signatureLine")]
pub struct SignatureLine {
  /// isSignatureLine
  #[sdk(attr(qname = ":isSignatureLine"))]
  pub is_signature_line: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// provId
  #[sdk(attr(qname = ":provId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// signingInstructionsSet
  #[sdk(attr(qname = ":signingInstructionsSet"))]
  pub signing_instructions_set: Option<crate::simple_type::BooleanValue>,
  /// allowComments
  #[sdk(attr(qname = ":allowComments"))]
  pub allow_comments: Option<crate::simple_type::BooleanValue>,
  /// showSignDate
  #[sdk(attr(qname = ":showSignDate"))]
  pub show_sign_date: Option<crate::simple_type::BooleanValue>,
  /// suggestedSigner
  #[sdk(attr(qname = ":suggestedSigner"))]
  pub suggested_signer: Option<crate::simple_type::StringValue>,
  /// suggestedSigner2
  #[sdk(attr(qname = ":suggestedSigner2"))]
  pub suggested_signer2: Option<crate::simple_type::StringValue>,
  /// suggestedSignerEmail
  #[sdk(attr(qname = ":suggestedSignerEmail"))]
  pub suggested_signer_email: Option<crate::simple_type::StringValue>,
  /// signingInstructions
  #[sdk(attr(qname = ":signingInstructions"))]
  pub signing_instructions: Option<crate::simple_type::StringValue>,
  /// addlXml
  #[sdk(attr(qname = ":addlXml"))]
  pub additional_xml: Option<crate::simple_type::StringValue>,
  /// sigProvUrl
  #[sdk(attr(qname = ":sigProvUrl"))]
  pub signature_provider_url: Option<crate::simple_type::StringValue>,
}
