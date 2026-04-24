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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Color/w15:color")]
pub struct Color {
  /// Run Content Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:themeColor
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color:
    Option<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ThemeColorValues>,
  /// Run Content Theme Color Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:themeTint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(source = 0u32, regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Run Content Theme Color Shade
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:themeShade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(source = 0u32, regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
}
/// Defines the DataBinding Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:dataBinding.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DataBinding/w15:dataBinding")]
pub struct DataBinding {
  /// XML Namespace Prefix Mappings
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:prefixMappings
  #[sdk(attr(qname = "w:prefixMappings"))]
  pub prefix_mappings: Option<crate::simple_type::StringValue>,
  /// XPath
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:xpath
  #[sdk(attr(qname = "w:xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Custom XML Data Storage ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:storeItemID
  #[sdk(attr(qname = "w:storeItemID"))]
  pub store_item_id: crate::simple_type::StringValue,
}
/// Defines the Appearance Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:appearance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_SdtAppearance/w15:appearance")]
pub struct Appearance {
  /// val
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:val
  #[sdk(attr(qname = "w15:val"))]
  pub val: Option<SdtAppearance>,
}
/// Defines the CommentsEx Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:commentsEx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_CommentsEx/w15:commentsEx")]
pub struct CommentsEx {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "w15:CT_CommentEx/w15:commentEx"))]
  pub w15_comment_ex: Vec<CommentEx>,
}
/// Defines the People Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:people.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_People/w15:people")]
pub struct People {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "w15:CT_Person/w15:person"))]
  pub w15_person: Vec<Person>,
}
/// Defines the SdtRepeatedSection Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:repeatingSection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_SdtRepeatedSection/w15:repeatingSection")]
pub struct SdtRepeatedSection {
  /// _
  #[sdk(child(qname = "w:CT_String/w15:sectionTitle"))]
  pub section_title: Option<SectionTitle>,
  /// _
  #[sdk(child(qname = "w:CT_OnOff/w15:doNotAllowInsertDeleteSection"))]
  pub do_not_allow_insert_delete_section: Option<DoNotAllowInsertDeleteSection>,
}
/// Defines the SdtRepeatedSectionItem Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:repeatingSectionItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Empty/w15:repeatingSectionItem")]
pub struct SdtRepeatedSectionItem {}
/// Defines the ChartTrackingRefBased Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:chartTrackingRefBased.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w15:chartTrackingRefBased")]
pub struct ChartTrackingRefBased {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DefaultCollapsed Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:collapsed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w15:collapsed")]
pub struct DefaultCollapsed {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebExtensionLinked Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:webExtensionLinked.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w15:webExtensionLinked")]
pub struct WebExtensionLinked {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebExtensionCreated Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:webExtensionCreated.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w15:webExtensionCreated")]
pub struct WebExtensionCreated {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotAllowInsertDeleteSection Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:doNotAllowInsertDeleteSection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w15:doNotAllowInsertDeleteSection")]
pub struct DoNotAllowInsertDeleteSection {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OnOffType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/")]
pub struct OnOffType {
  /// On/Off Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the PersistentDocumentId Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:docId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_Guid/w15:docId")]
pub struct PersistentDocumentId {
  /// val
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:val
  #[sdk(attr(qname = "w15:val"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub w15_val: Option<crate::simple_type::StringValue>,
}
/// Defines the FootnoteColumns Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:footnoteColumns.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w15:footnoteColumns")]
pub struct FootnoteColumns {
  /// Decimal Number Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CommentEx Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:commentEx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_CommentEx/w15:commentEx")]
pub struct CommentEx {
  /// paraId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:paraId
  #[sdk(attr(qname = "w15:paraId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub para_id: crate::simple_type::HexBinaryValue,
  /// paraIdParent
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:paraIdParent
  #[sdk(attr(qname = "w15:paraIdParent"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub para_id_parent: Option<crate::simple_type::HexBinaryValue>,
  /// done
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:done
  #[sdk(attr(qname = "w15:done"))]
  pub done: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Person Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:person.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_Person/w15:person")]
pub struct Person {
  /// author
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:author
  #[sdk(attr(qname = "w15:author"))]
  pub author: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "w15:CT_PresenceInfo/w15:presenceInfo"))]
  pub presence_info: Option<PresenceInfo>,
}
/// Defines the PresenceInfo Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:presenceInfo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w15:CT_PresenceInfo/w15:presenceInfo")]
pub struct PresenceInfo {
  /// providerId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:providerId
  #[sdk(attr(qname = "w15:providerId"))]
  pub w15_provider_id: crate::simple_type::StringValue,
  /// userId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: w15:userId
  #[sdk(attr(qname = "w15:userId"))]
  pub w15_user_id: crate::simple_type::StringValue,
}
/// Defines the SectionTitle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is w15:sectionTitle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w15:sectionTitle")]
pub struct SectionTitle {
  /// String Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w:val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
