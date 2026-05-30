//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum Indefinite {
  #[sdk(rename = "indefinite")]
  #[default]
  Indefinite,
}
/// Defines the AnimationProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aanim:animPr")]
pub struct AnimationProperties {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// length
  #[sdk(attr(qname = ":length"))]
  pub length: crate::simple_type::StringValue,
  /// count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  pub count: Option<crate::simple_type::StringValue>,
  /// auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// st
  #[sdk(attr(qname = ":st"))]
  pub st: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "aanim:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aanim:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
