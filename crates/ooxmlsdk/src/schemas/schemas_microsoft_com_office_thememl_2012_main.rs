//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ThemeFamily Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is thm15:themeFamily.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:CT_ThemeFamily/thm15:themeFamily")]
pub struct ThemeFamily {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// vid
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :vid
  #[sdk(attr(qname = ":vid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub vid: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/thm15:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is thm15:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/thm15:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the ThemeVariant Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is thm15:themeVariant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:CT_ThemeVariant/thm15:themeVariant")]
pub struct ThemeVariant {
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// vid
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :vid
  #[sdk(attr(qname = ":vid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub vid: crate::simple_type::StringValue,
  /// cx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// cy
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/thm15:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ThemeVariantList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is thm15:themeVariantLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "thm15:CT_ThemeVariantList/thm15:themeVariantLst")]
pub struct ThemeVariantList {
  /// _
  #[sdk(child(qname = "thm15:CT_ThemeVariant/thm15:themeVariant"))]
  pub thm15_theme_variant: Vec<ThemeVariant>,
}
