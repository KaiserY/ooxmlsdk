//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ThemeFamily Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:themeFamily")]
pub struct ThemeFamily {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// vid
  #[sdk(attr(qname = ":vid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub vid: crate::simple_type::StringValue,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "thm15:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the ThemeVariant Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:themeVariant")]
pub struct ThemeVariant {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// vid
  #[sdk(attr(qname = ":vid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub vid: crate::simple_type::StringValue,
  /// cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "thm15:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ThemeVariantList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:themeVariantLst")]
pub struct ThemeVariantList {
  /// Defines the ThemeVariant Class.
  #[sdk(child(qname = "thm15:themeVariant"))]
  pub theme_variant: Vec<ThemeVariant>,
}
