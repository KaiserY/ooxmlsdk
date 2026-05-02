//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the EmbeddedAnimation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim")]
pub struct EmbeddedAnimation {
  /// animId
  #[sdk(attr(office2019, qname = ":animId"))]
  pub anim_id: crate::simple_type::UInt32Value,
  /// Defines the AnimationProperties Class.
  #[sdk(child(office2019, qname = "aanim:CT_AnimationProperties/a3danim:animPr"))]
  pub animation_properties: std::boxed::Box<AnimationProperties>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/a3danim:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PosterFrame Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a3danim:CT_PosterFrame/a3danim:posterFrame")]
pub struct PosterFrame {
  /// animId
  #[sdk(attr(office2019, qname = ":animId"))]
  pub anim_id: crate::simple_type::UInt32Value,
  /// frame
  #[sdk(attr(office2019, qname = ":frame"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub frame: Option<crate::simple_type::Int32Value>,
}
/// Defines the AnimationProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "aanim:CT_AnimationProperties/a3danim:animPr")]
pub struct AnimationProperties {
  /// name
  #[sdk(attr(office2019, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// length
  #[sdk(attr(office2019, qname = ":length"))]
  pub length: crate::simple_type::StringValue,
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  pub count: Option<crate::simple_type::StringValue>,
  /// auto
  #[sdk(attr(office2019, qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// offset
  #[sdk(attr(office2019, qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// st
  #[sdk(attr(office2019, qname = ":st"))]
  pub st: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(office2019, qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/aanim:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::aanim::OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_OfficeArtExtensionList/a3danim:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
