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
#[sdk(office2019, qname = "aanim:CT_AnimationProperties/aanim:animPr")]
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
  /// _
  #[sdk(child(office2019, qname = "a:CT_OfficeArtExtensionList/aanim:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "a:CT_OfficeArtExtensionList/aanim:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
