//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SdtAppearance {
  #[sdk(rename = "boundingBox")]
  #[default]
  BoundingBox,
  #[sdk(rename = "tags")]
  Tags,
  #[sdk(rename = "hidden")]
  Hidden,
}
/// Defines the Color Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_Color/w15:color")]
pub struct Color {
  /// Run Content Color
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 2u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub val: crate::simple_type::StringValue,
  /// Run Content Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<crate::schemas::w::ThemeColorValues>,
  /// Run Content Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Run Content Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
}
/// Defines the DataBinding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_DataBinding/w15:dataBinding")]
pub struct DataBinding {
  /// XML Namespace Prefix Mappings
  #[sdk(attr(qname = "w:prefixMappings"))]
  pub prefix_mappings: Option<crate::simple_type::StringValue>,
  /// XPath
  #[sdk(attr(qname = "w:xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Custom XML Data Storage ID
  #[sdk(attr(qname = "w:storeItemID"))]
  pub store_item_id: crate::simple_type::StringValue,
}
/// Defines the Appearance Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_SdtAppearance/w15:appearance")]
pub struct Appearance {
  /// val
  #[sdk(attr(office2013, qname = "w15:val"))]
  pub val: Option<SdtAppearance>,
}
/// Defines the CommentsEx Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_CommentsEx/w15:commentsEx")]
pub struct CommentsEx {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the CommentEx Class.
  #[sdk(child(office2013, qname = "w15:CT_CommentEx/w15:commentEx"))]
  pub w15_comment_ex: Vec<CommentEx>,
}
/// Defines the People Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_People/w15:people")]
pub struct People {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Person Class.
  #[sdk(child(office2013, qname = "w15:CT_Person/w15:person"))]
  pub w15_person: Vec<Person>,
}
/// Defines the SdtRepeatedSection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_SdtRepeatedSection/w15:repeatingSection")]
pub struct SdtRepeatedSection {
  /// Defines the SectionTitle Class.
  #[sdk(child(office2013, qname = "w:CT_String/w15:sectionTitle"))]
  pub section_title: Option<SectionTitle>,
  /// Defines the DoNotAllowInsertDeleteSection Class.
  #[sdk(child(office2013, qname = "w:CT_OnOff/w15:doNotAllowInsertDeleteSection"))]
  pub do_not_allow_insert_delete_section: Option<DoNotAllowInsertDeleteSection>,
}
/// Defines the ChartTrackingRefBased Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_OnOff/w15:chartTrackingRefBased")]
pub struct ChartTrackingRefBased {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DefaultCollapsed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_OnOff/w15:collapsed")]
pub struct DefaultCollapsed {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebExtensionLinked Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_OnOff/w15:webExtensionLinked")]
pub struct WebExtensionLinked {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebExtensionCreated Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_OnOff/w15:webExtensionCreated")]
pub struct WebExtensionCreated {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotAllowInsertDeleteSection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_OnOff/w15:doNotAllowInsertDeleteSection")]
pub struct DoNotAllowInsertDeleteSection {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the PersistentDocumentId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_Guid/w15:docId")]
pub struct PersistentDocumentId {
  /// val
  #[sdk(attr(office2013, qname = "w15:val"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub w15_val: Option<crate::simple_type::StringValue>,
}
/// Defines the FootnoteColumns Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_DecimalNumber/w15:footnoteColumns")]
pub struct FootnoteColumns {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CommentEx Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_CommentEx/w15:commentEx")]
pub struct CommentEx {
  /// paraId
  #[sdk(attr(office2013, qname = "w15:paraId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub para_id: crate::simple_type::HexBinaryValue,
  /// paraIdParent
  #[sdk(attr(office2013, qname = "w15:paraIdParent"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub para_id_parent: Option<crate::simple_type::HexBinaryValue>,
  /// done
  #[sdk(attr(office2013, qname = "w15:done"))]
  pub done: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Person Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_Person/w15:person")]
pub struct Person {
  /// author
  #[sdk(attr(office2013, qname = "w15:author"))]
  pub author: crate::simple_type::StringValue,
  /// Defines the PresenceInfo Class.
  #[sdk(child(office2013, qname = "w15:CT_PresenceInfo/w15:presenceInfo"))]
  pub presence_info: Option<PresenceInfo>,
}
/// Defines the PresenceInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w15:CT_PresenceInfo/w15:presenceInfo")]
pub struct PresenceInfo {
  /// providerId
  #[sdk(attr(office2013, qname = "w15:providerId"))]
  pub w15_provider_id: crate::simple_type::StringValue,
  /// userId
  #[sdk(attr(office2013, qname = "w15:userId"))]
  pub w15_user_id: crate::simple_type::StringValue,
}
/// Defines the SectionTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "w:CT_String/w15:sectionTitle")]
pub struct SectionTitle {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
