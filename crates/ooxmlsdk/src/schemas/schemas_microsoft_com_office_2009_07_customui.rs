//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GalleryShowInRibbonValues {
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "0")]
  Zero,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SizeValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "large")]
  Large,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ItemSizeValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "large")]
  Large,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BoxStyleValues {
  #[sdk(rename = "horizontal")]
  #[default]
  Horizontal,
  #[sdk(rename = "vertical")]
  Vertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TaskSizesValues {
  #[sdk(rename = "largeMediumSmall")]
  #[default]
  LargeMediumSmall,
  #[sdk(rename = "largeMedium")]
  LargeMedium,
  #[sdk(rename = "large")]
  Large,
  #[sdk(rename = "mediumSmall")]
  MediumSmall,
  #[sdk(rename = "medium")]
  Medium,
  #[sdk(rename = "small")]
  Small,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExpandValues {
  #[sdk(rename = "topLeft")]
  #[default]
  TopLeft,
  #[sdk(rename = "top")]
  Top,
  #[sdk(rename = "topRight")]
  TopRight,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "bottomLeft")]
  BottomLeft,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "bottomRight")]
  BottomRight,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "warning")]
  Warning,
  #[sdk(rename = "error")]
  Error,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum Style2Values {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "borderless")]
  Borderless,
  #[sdk(rename = "large")]
  Large,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LayoutChildrenValues {
  #[sdk(rename = "horizontal")]
  #[default]
  Horizontal,
  #[sdk(rename = "vertical")]
  Vertical,
}
/// Defines the ControlCloneRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ControlCloneRegular/mso14:control")]
pub struct ControlCloneRegular {
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
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ButtonRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ButtonRegular/mso14:button")]
pub struct ButtonRegular {
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
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the CheckBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_CheckBox/mso14:checkBox")]
pub struct CheckBox {
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
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the GalleryRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:gallery.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_GalleryRegular/mso14:gallery")]
pub struct GalleryRegular {
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
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :columns
  #[sdk(attr(qname = ":columns"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :rows
  #[sdk(attr(qname = ":rows"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemWidth
  #[sdk(attr(qname = ":itemWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "4096",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemHeight
  #[sdk(attr(qname = ":itemHeight"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "4096",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemWidth
  #[sdk(attr(qname = ":getItemWidth"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemHeight
  #[sdk(attr(qname = ":getItemHeight"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showInRibbon
  #[sdk(attr(qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:CT_Item/mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the ToggleButtonRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton")]
pub struct ToggleButtonRegular {
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
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the MenuSeparator Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menuSeparator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_MenuSeparator/mso14:menuSeparator")]
pub struct MenuSeparator {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// title
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton")]
pub struct SplitButtonRegular {
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
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_regular_choice: Option<SplitButtonRegularChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the MenuRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_MenuRegular/mso14:menu")]
pub struct MenuRegular {
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
  /// itemSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_MenuSeparator/mso14:menuSeparator",
    qname = "mso14:CT_SplitButtonRegular/mso14:splitButton",
    qname = "mso14:CT_MenuRegular/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"
  ))]
  pub menu_regular_choice: Vec<MenuRegularChoice>,
}
/// Defines the DynamicMenuRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:dynamicMenu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu")]
pub struct DynamicMenuRegular {
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
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getContent
  #[sdk(attr(qname = ":getContent"))]
  #[sdk(string_length(source = 1u32, min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonWithTitle Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_SplitButtonWithTitle/mso14:splitButton")]
pub struct SplitButtonWithTitle {
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
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_with_title_choice: Option<SplitButtonWithTitleChoice>,
  /// Defines the MenuWithTitle Class.
  #[sdk(child(qname = "mso14:CT_MenuWithTitle/mso14:menu"))]
  pub menu_with_title: Option<MenuWithTitle>,
}
/// Defines the MenuWithTitle Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_MenuWithTitle/mso14:menu")]
pub struct MenuWithTitle {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// itemSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// title
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_MenuSeparator/mso14:menuSeparator",
    qname = "mso14:CT_SplitButtonWithTitle/mso14:splitButton",
    qname = "mso14:CT_MenuWithTitle/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"
  ))]
  pub menu_with_title_choice: Vec<MenuWithTitleChoice>,
}
/// Defines the MenuSeparatorNoTitle Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menuSeparator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator")]
pub struct MenuSeparatorNoTitle {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ControlClone Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ControlClone/mso14:control")]
pub struct ControlClone {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the LabelControl Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:labelControl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_LabelControl/mso14:labelControl")]
pub struct LabelControl {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
}
/// Defines the Button Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Button/mso14:button")]
pub struct Button {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ToggleButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ToggleButton/mso14:toggleButton")]
pub struct ToggleButton {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the EditBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:editBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_EditBox/mso14:editBox")]
pub struct EditBox {
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
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ComboBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:comboBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ComboBox/mso14:comboBox")]
pub struct ComboBox {
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
  /// showItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:CT_Item/mso14:item"))]
  pub item: Vec<Item>,
}
/// Defines the DropDownRegular Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:dropDown.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_DropDownRegular/mso14:dropDown")]
pub struct DropDownRegular {
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
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:CT_Item/mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Gallery Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:gallery.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Gallery/mso14:gallery")]
pub struct Gallery {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :columns
  #[sdk(attr(qname = ":columns"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :rows
  #[sdk(attr(qname = ":rows"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemWidth
  #[sdk(attr(qname = ":itemWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "4096",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemHeight
  #[sdk(attr(qname = ":itemHeight"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "4096",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemWidth
  #[sdk(attr(qname = ":getItemWidth"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemHeight
  #[sdk(attr(qname = ":getItemHeight"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showInRibbon
  #[sdk(attr(qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:CT_Item/mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Menu Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Menu/mso14:menu")]
pub struct Menu {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// itemSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_MenuSeparator/mso14:menuSeparator",
    qname = "mso14:CT_SplitButtonRegular/mso14:splitButton",
    qname = "mso14:CT_MenuRegular/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"
  ))]
  pub menu_choice: Vec<MenuChoice>,
}
/// Defines the DynamicMenu Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:dynamicMenu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu")]
pub struct DynamicMenu {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getContent
  #[sdk(attr(qname = ":getContent"))]
  #[sdk(string_length(source = 1u32, min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:splitButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_SplitButton/mso14:splitButton")]
pub struct SplitButton {
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
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_choice: Option<SplitButtonChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the Box Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:box.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Box/mso14:box")]
pub struct Box {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// boxStyle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :boxStyle
  #[sdk(attr(qname = ":boxStyle"))]
  pub box_style: Option<BoxStyleValues>,
  #[sdk(choice(
    qname = "mso14:CT_ControlClone/mso14:control",
    qname = "mso14:CT_LabelControl/mso14:labelControl",
    qname = "mso14:CT_Button/mso14:button",
    qname = "mso14:CT_ToggleButton/mso14:toggleButton",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_EditBox/mso14:editBox",
    qname = "mso14:CT_ComboBox/mso14:comboBox",
    qname = "mso14:CT_DropDownRegular/mso14:dropDown",
    qname = "mso14:CT_Gallery/mso14:gallery",
    qname = "mso14:CT_Menu/mso14:menu",
    qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu",
    qname = "mso14:CT_SplitButton/mso14:splitButton",
    qname = "mso14:CT_Box/mso14:box",
    qname = "mso14:CT_ButtonGroup/mso14:buttonGroup"
  ))]
  pub xml_children: Vec<BoxChoice>,
}
/// Defines the ButtonGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:buttonGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ButtonGroup/mso14:buttonGroup")]
pub struct ButtonGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_MenuRegular/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu",
    qname = "mso14:CT_SplitButtonRegular/mso14:splitButton",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub button_group_choice: Vec<ButtonGroupChoice>,
}
/// Defines the BackstageMenuButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageMenuButton/mso14:button")]
pub struct BackstageMenuButton {
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
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageMenuCheckBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox")]
pub struct BackstageMenuCheckBox {
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
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageSubMenu Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageSubMenu/mso14:menu")]
pub struct BackstageSubMenu {
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
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup"))]
  pub mso14_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuToggleButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton")]
pub struct BackstageMenuToggleButton {
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
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroupButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageGroupButton/mso14:button")]
pub struct BackstageGroupButton {
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
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// style
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<Style2Values>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageCheckBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:checkBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox")]
pub struct BackstageCheckBox {
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
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageEditBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:editBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageEditBox/mso14:editBox")]
pub struct BackstageEditBox {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// maxLength
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "1024",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageDropDown Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:dropDown.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageDropDown/mso14:dropDown")]
pub struct BackstageDropDown {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageItem/mso14:item"))]
  pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the RadioGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:radioGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_RadioGroup/mso14:radioGroup")]
pub struct RadioGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageItem/mso14:radioButton"))]
  pub mso14_radio_button: Vec<RadioButtonBackstageItem>,
}
/// Defines the BackstageComboBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:comboBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageComboBox/mso14:comboBox")]
pub struct BackstageComboBox {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageItem/mso14:item"))]
  pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the Hyperlink Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Hyperlink/mso14:hyperlink")]
pub struct Hyperlink {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// target
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :target
  #[sdk(attr(qname = ":target"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub target: Option<crate::simple_type::StringValue>,
  /// getTarget
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getTarget
  #[sdk(attr(qname = ":getTarget"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_target: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageLabelControl Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:labelControl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl")]
pub struct BackstageLabelControl {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// noWrap
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :noWrap
  #[sdk(attr(qname = ":noWrap"))]
  pub no_wrap: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupBox Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:groupBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_GroupBox/mso14:groupBox")]
pub struct GroupBox {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroupButton/mso14:button",
    qname = "mso14:CT_BackstageCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageEditBox/mso14:editBox",
    qname = "mso14:CT_BackstageDropDown/mso14:dropDown",
    qname = "mso14:CT_RadioGroup/mso14:radioGroup",
    qname = "mso14:CT_BackstageComboBox/mso14:comboBox",
    qname = "mso14:CT_Hyperlink/mso14:hyperlink",
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl",
    qname = "mso14:CT_GroupBox/mso14:groupBox",
    qname = "mso14:CT_LayoutContainer/mso14:layoutContainer",
    qname = "mso14:CT_ImageControl/mso14:imageControl"
  ))]
  pub group_box_choice: Vec<GroupBoxChoice>,
}
/// Defines the LayoutContainer Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:layoutContainer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer")]
pub struct LayoutContainer {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// align
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<ExpandValues>,
  /// expand
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// layoutChildren
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :layoutChildren
  #[sdk(attr(qname = ":layoutChildren"))]
  pub layout_children: Option<LayoutChildrenValues>,
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroupButton/mso14:button",
    qname = "mso14:CT_BackstageCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageEditBox/mso14:editBox",
    qname = "mso14:CT_BackstageDropDown/mso14:dropDown",
    qname = "mso14:CT_RadioGroup/mso14:radioGroup",
    qname = "mso14:CT_BackstageComboBox/mso14:comboBox",
    qname = "mso14:CT_Hyperlink/mso14:hyperlink",
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl",
    qname = "mso14:CT_GroupBox/mso14:groupBox",
    qname = "mso14:CT_LayoutContainer/mso14:layoutContainer",
    qname = "mso14:CT_ImageControl/mso14:imageControl"
  ))]
  pub layout_container_choice: Vec<LayoutContainerChoice>,
}
/// Defines the ImageControl Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:imageControl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ImageControl/mso14:imageControl")]
pub struct ImageControl {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// altText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// getAltText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getAltText
  #[sdk(attr(qname = ":getAltText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_alt_text: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageGroup/mso14:group")]
pub struct BackstageGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// style
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<StyleValues>,
  /// getStyle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getStyle
  #[sdk(attr(qname = ":getStyle"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_style: Option<crate::simple_type::StringValue>,
  /// helperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_PrimaryItem/mso14:primaryItem"))]
  pub mso14_primary_item: Option<std::boxed::Box<PrimaryItem>>,
  /// _
  #[sdk(child(qname = "mso14:CT_GroupControls/mso14:topItems"))]
  pub mso14_top_items: Option<TopItemsGroupControls>,
  /// _
  #[sdk(child(qname = "mso14:CT_GroupControls/mso14:bottomItems"))]
  pub mso14_bottom_items: Option<BottomItemsGroupControls>,
}
/// Defines the TaskGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:taskGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskGroup/mso14:taskGroup")]
pub struct TaskGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :allowedTaskSizes
  #[sdk(attr(qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// _
  #[sdk(child(qname = "mso14:CT_TaskGroupCategory/mso14:category"))]
  pub mso14_category: Vec<TaskGroupCategory>,
}
/// Defines the MenuRoot Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_MenuRoot/mso14:menu")]
pub struct MenuRoot {
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
  /// title
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// itemSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_MenuSeparator/mso14:menuSeparator",
    qname = "mso14:CT_SplitButtonRegular/mso14:splitButton",
    qname = "mso14:CT_MenuRegular/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"
  ))]
  pub menu_root_choice: Vec<MenuRootChoice>,
}
/// Defines the CustomUI Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:customUI.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_CustomUI/mso14:customUI")]
pub struct CustomUi {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  /// onLoad
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onLoad
  #[sdk(attr(qname = ":onLoad"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_load: Option<crate::simple_type::StringValue>,
  /// loadImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :loadImage
  #[sdk(attr(qname = ":loadImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub load_image: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_Commands/mso14:commands"))]
  pub commands: Option<Commands>,
  /// _
  #[sdk(child(qname = "mso14:CT_Ribbon/mso14:ribbon"))]
  pub ribbon: Option<std::boxed::Box<Ribbon>>,
  /// _
  #[sdk(child(qname = "mso14:CT_Backstage/mso14:backstage"))]
  pub backstage: Option<Backstage>,
  /// _
  #[sdk(child(qname = "mso14:CT_ContextMenus/mso14:contextMenus"))]
  pub context_menus: Option<ContextMenus>,
}
/// Defines the Item Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Item/mso14:item")]
pub struct Item {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_VisibleButton/mso14:button")]
pub struct VisibleButton {
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
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleToggleButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:toggleButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton")]
pub struct VisibleToggleButton {
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
  /// getPressed
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Separator Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:separator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Separator/mso14:separator")]
pub struct Separator {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DialogBoxLauncher Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:dialogBoxLauncher.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_DialogLauncher/mso14:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
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
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub button_regular: std::boxed::Box<ButtonRegular>,
}
/// Defines the Group Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Group/mso14:group")]
pub struct Group {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// autoScale
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :autoScale
  #[sdk(attr(qname = ":autoScale"))]
  pub auto_scale: Option<crate::simple_type::BooleanValue>,
  /// centerVertically
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :centerVertically
  #[sdk(attr(qname = ":centerVertically"))]
  pub center_vertically: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlClone/mso14:control",
    qname = "mso14:CT_LabelControl/mso14:labelControl",
    qname = "mso14:CT_Button/mso14:button",
    qname = "mso14:CT_ToggleButton/mso14:toggleButton",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_EditBox/mso14:editBox",
    qname = "mso14:CT_ComboBox/mso14:comboBox",
    qname = "mso14:CT_DropDownRegular/mso14:dropDown",
    qname = "mso14:CT_Gallery/mso14:gallery",
    qname = "mso14:CT_Menu/mso14:menu",
    qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu",
    qname = "mso14:CT_SplitButton/mso14:splitButton",
    qname = "mso14:CT_Box/mso14:box",
    qname = "mso14:CT_ButtonGroup/mso14:buttonGroup",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub group_choice: Vec<GroupChoice>,
  /// _
  #[sdk(child(qname = "mso14:CT_DialogLauncher/mso14:dialogBoxLauncher"))]
  pub mso14_dialog_box_launcher: Option<std::boxed::Box<DialogBoxLauncher>>,
}
/// Defines the ControlCloneQat Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ControlCloneQat/mso14:control")]
pub struct ControlCloneQat {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// size
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SharedControlsQatItems Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:sharedControls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_QatItems/mso14:sharedControls")]
pub struct SharedControlsQatItems {
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
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneQat/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub shared_controls_qat_items_choice: Vec<SharedControlsQatItemsChoice>,
}
/// Defines the DocumentControlsQatItems Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:documentControls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_QatItems/mso14:documentControls")]
pub struct DocumentControlsQatItems {
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
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneQat/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub document_controls_qat_items_choice: Vec<DocumentControlsQatItemsChoice>,
}
/// Defines the QatItemsType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_QatItems/")]
pub struct QatItemsType {
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
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneQat/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub xml_children: Vec<QatItemsTypeChoice>,
}
/// Defines the Tab Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:tab.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Tab/mso14:tab")]
pub struct Tab {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_Group/mso14:group"))]
  pub mso14_group: Vec<Group>,
}
/// Defines the TabSet Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:tabSet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TabSet/mso14:tabSet")]
pub struct TabSet {
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
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 1u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  #[sdk(string_format(source = 1u32, kind = "ncname"))]
  pub id_mso: crate::simple_type::StringValue,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_Tab/mso14:tab"))]
  pub mso14_tab: Vec<Tab>,
}
/// Defines the Command Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:command.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Command/mso14:command")]
pub struct Command {
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
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
}
/// Defines the QuickAccessToolbar Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:qat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Qat/mso14:qat")]
pub struct QuickAccessToolbar {
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
  #[sdk(child(qname = "mso14:CT_QatItems/mso14:sharedControls"))]
  pub shared_controls_qat_items: Option<SharedControlsQatItems>,
  /// _
  #[sdk(child(qname = "mso14:CT_QatItems/mso14:documentControls"))]
  pub document_controls_qat_items: Option<DocumentControlsQatItems>,
}
/// Defines the Tabs Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:tabs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Tabs/mso14:tabs")]
pub struct Tabs {
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
  #[sdk(child(qname = "mso14:CT_Tab/mso14:tab"))]
  pub mso14_tab: Vec<Tab>,
}
/// Defines the ContextualTabs Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:contextualTabs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ContextualTabs/mso14:contextualTabs")]
pub struct ContextualTabs {
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
  #[sdk(child(qname = "mso14:CT_TabSet/mso14:tabSet"))]
  pub mso14_tab_set: Vec<TabSet>,
}
/// Defines the ContextMenu Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:contextMenu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ContextMenu/mso14:contextMenu")]
pub struct ContextMenu {
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
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_ControlCloneRegular/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_CheckBox/mso14:checkBox",
    qname = "mso14:CT_GalleryRegular/mso14:gallery",
    qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton",
    qname = "mso14:CT_SplitButtonRegular/mso14:splitButton",
    qname = "mso14:CT_MenuRegular/mso14:menu",
    qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu",
    qname = "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator"
  ))]
  pub context_menu_choice: Vec<ContextMenuChoice>,
}
/// Defines the ItemBackstageItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageItem/mso14:item")]
pub struct ItemBackstageItem {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the RadioButtonBackstageItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:radioButton.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageItem/mso14:radioButton")]
pub struct RadioButtonBackstageItem {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageItemType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageItem/")]
pub struct BackstageItemType {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageRegularButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageRegularButton/mso14:button")]
pub struct BackstageRegularButton {
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
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstagePrimaryMenu Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu")]
pub struct BackstagePrimaryMenu {
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
  /// screentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup"))]
  pub mso14_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:menuGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup")]
pub struct BackstageMenuGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// itemSize
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(choice(
    qname = "mso14:CT_BackstageMenuButton/mso14:button",
    qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageSubMenu/mso14:menu",
    qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton"
  ))]
  pub backstage_menu_group_choice: Vec<BackstageMenuGroupChoice>,
}
/// Defines the PrimaryItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:primaryItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_PrimaryItem/mso14:primaryItem")]
pub struct PrimaryItem {
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
  #[sdk(choice(
    qname = "mso14:CT_BackstageRegularButton/mso14:button",
    qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu"
  ))]
  pub xml_children: Option<PrimaryItemChoice>,
}
/// Defines the TopItemsGroupControls Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:topItems.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_GroupControls/mso14:topItems")]
pub struct TopItemsGroupControls {
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
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroupButton/mso14:button",
    qname = "mso14:CT_BackstageCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageEditBox/mso14:editBox",
    qname = "mso14:CT_BackstageDropDown/mso14:dropDown",
    qname = "mso14:CT_RadioGroup/mso14:radioGroup",
    qname = "mso14:CT_BackstageComboBox/mso14:comboBox",
    qname = "mso14:CT_Hyperlink/mso14:hyperlink",
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl",
    qname = "mso14:CT_GroupBox/mso14:groupBox",
    qname = "mso14:CT_LayoutContainer/mso14:layoutContainer",
    qname = "mso14:CT_ImageControl/mso14:imageControl"
  ))]
  pub xml_children: Vec<TopItemsGroupControlsChoice>,
}
/// Defines the BottomItemsGroupControls Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:bottomItems.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_GroupControls/mso14:bottomItems")]
pub struct BottomItemsGroupControls {
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
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroupButton/mso14:button",
    qname = "mso14:CT_BackstageCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageEditBox/mso14:editBox",
    qname = "mso14:CT_BackstageDropDown/mso14:dropDown",
    qname = "mso14:CT_RadioGroup/mso14:radioGroup",
    qname = "mso14:CT_BackstageComboBox/mso14:comboBox",
    qname = "mso14:CT_Hyperlink/mso14:hyperlink",
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl",
    qname = "mso14:CT_GroupBox/mso14:groupBox",
    qname = "mso14:CT_LayoutContainer/mso14:layoutContainer",
    qname = "mso14:CT_ImageControl/mso14:imageControl"
  ))]
  pub xml_children: Vec<BottomItemsGroupControlsChoice>,
}
/// Defines the GroupControlsType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_GroupControls/")]
pub struct GroupControlsType {
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
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroupButton/mso14:button",
    qname = "mso14:CT_BackstageCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageEditBox/mso14:editBox",
    qname = "mso14:CT_BackstageDropDown/mso14:dropDown",
    qname = "mso14:CT_RadioGroup/mso14:radioGroup",
    qname = "mso14:CT_BackstageComboBox/mso14:comboBox",
    qname = "mso14:CT_Hyperlink/mso14:hyperlink",
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl",
    qname = "mso14:CT_GroupBox/mso14:groupBox",
    qname = "mso14:CT_LayoutContainer/mso14:layoutContainer",
    qname = "mso14:CT_ImageControl/mso14:imageControl"
  ))]
  pub xml_children: Vec<GroupControlsTypeChoice>,
}
/// Defines the TaskGroupCategory Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:category.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskGroupCategory/mso14:category")]
pub struct TaskGroupCategory {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_TaskGroupTask/mso14:task"))]
  pub mso14_task: Vec<TaskGroupTask>,
}
/// Defines the TaskGroupTask Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:task.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskGroupTask/mso14:task")]
pub struct TaskGroupTask {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the TaskFormGroupCategory Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:category.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskFormGroupCategory/mso14:category")]
pub struct TaskFormGroupCategory {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_TaskFormGroupTask/mso14:task"))]
  pub mso14_task: Vec<TaskFormGroupTask>,
}
/// Defines the TaskFormGroupTask Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:task.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskFormGroupTask/mso14:task")]
pub struct TaskFormGroupTask {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageGroup/mso14:group"))]
  pub mso14_group: Vec<BackstageGroup>,
}
/// Defines the TaskFormGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:taskFormGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup")]
pub struct TaskFormGroup {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :allowedTaskSizes
  #[sdk(attr(qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// _
  #[sdk(child(qname = "mso14:CT_TaskFormGroupCategory/mso14:category"))]
  pub mso14_category: Vec<TaskFormGroupCategory>,
}
/// Defines the BackstageGroups Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:firstColumn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageGroups/mso14:firstColumn")]
pub struct BackstageGroups {
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
  #[sdk(choice(
    qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup",
    qname = "mso14:CT_BackstageGroup/mso14:group",
    qname = "mso14:CT_TaskGroup/mso14:taskGroup"
  ))]
  pub xml_children: Option<BackstageGroupsChoice>,
}
/// Defines the SimpleGroups Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:secondColumn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_SimpleGroups/mso14:secondColumn")]
pub struct SimpleGroups {
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
  #[sdk(choice(
    qname = "mso14:CT_BackstageGroup/mso14:group",
    qname = "mso14:CT_TaskGroup/mso14:taskGroup"
  ))]
  pub xml_children: Vec<SimpleGroupsChoice>,
}
/// Defines the BackstageTab Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:tab.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageTab/mso14:tab")]
pub struct BackstageTab {
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
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// title
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// columnWidthPercent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :columnWidthPercent
  #[sdk(attr(qname = ":columnWidthPercent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "99",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub column_width_percent: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMinWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :firstColumnMinWidth
  #[sdk(attr(qname = ":firstColumnMinWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "10000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub first_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMaxWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :firstColumnMaxWidth
  #[sdk(attr(qname = ":firstColumnMaxWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "10000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub first_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMinWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :secondColumnMinWidth
  #[sdk(attr(qname = ":secondColumnMinWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "10000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub second_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMaxWidth
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :secondColumnMaxWidth
  #[sdk(attr(qname = ":secondColumnMaxWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "10000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(number_sign(source = 0u32, kind = "positive"))]
  pub second_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_BackstageGroups/mso14:firstColumn"))]
  pub backstage_groups: Option<std::boxed::Box<BackstageGroups>>,
  /// _
  #[sdk(child(qname = "mso14:CT_SimpleGroups/mso14:secondColumn"))]
  pub simple_groups: Option<SimpleGroups>,
}
/// Defines the BackstageFastCommandButton Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:button.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_BackstageFastCommandButton/mso14:button")]
pub struct BackstageFastCommandButton {
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
  /// idMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  #[sdk(string_format(source = 0u32, kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 3u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  #[sdk(string_format(source = 0u32, kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Commands Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:commands.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Commands/mso14:commands")]
pub struct Commands {
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
  #[sdk(child(qname = "mso14:CT_Command/mso14:command"))]
  pub mso14_command: Vec<Command>,
}
/// Defines the Ribbon Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:ribbon.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Ribbon/mso14:ribbon")]
pub struct Ribbon {
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
  /// startFromScratch
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :startFromScratch
  #[sdk(attr(qname = ":startFromScratch"))]
  pub start_from_scratch: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "mso14:CT_Qat/mso14:qat"))]
  pub quick_access_toolbar: Option<std::boxed::Box<QuickAccessToolbar>>,
  /// _
  #[sdk(child(qname = "mso14:CT_Tabs/mso14:tabs"))]
  pub tabs: Option<Tabs>,
  /// _
  #[sdk(child(qname = "mso14:CT_ContextualTabs/mso14:contextualTabs"))]
  pub contextual_tabs: Option<ContextualTabs>,
}
/// Defines the Backstage Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:backstage.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_Backstage/mso14:backstage")]
pub struct Backstage {
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
  /// onShow
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onShow
  #[sdk(attr(qname = ":onShow"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_show: Option<crate::simple_type::StringValue>,
  /// onHide
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :onHide
  #[sdk(attr(qname = ":onHide"))]
  #[sdk(string_length(source = 0u32, min = 1u32, max = 1024u32))]
  pub on_hide: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "mso14:CT_BackstageTab/mso14:tab",
    qname = "mso14:CT_BackstageFastCommandButton/mso14:button"
  ))]
  pub backstage_choice: Vec<BackstageChoice>,
}
/// Defines the ContextMenus Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is mso14:contextMenus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:CT_ContextMenus/mso14:contextMenus")]
pub struct ContextMenus {
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
  #[sdk(child(qname = "mso14:CT_ContextMenu/mso14:contextMenu"))]
  pub mso14_context_menu: Vec<ContextMenu>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonRegularChoice {
  #[sdk(child(qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuRegularChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonWithTitleChoice {
  #[sdk(child(qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuWithTitleChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonWithTitle Class.
  #[sdk(child(qname = "mso14:CT_SplitButtonWithTitle/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonWithTitle>),
  /// Defines the MenuWithTitle Class.
  #[sdk(child(qname = "mso14:CT_MenuWithTitle/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuWithTitle>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonChoice {
  #[sdk(child(qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BoxChoice {
  /// Defines the ControlClone Class.
  #[sdk(child(qname = "mso14:CT_ControlClone/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  #[sdk(child(qname = "mso14:CT_LabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  #[sdk(child(qname = "mso14:CT_Button/mso14:button"))]
  Mso14Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  #[sdk(child(qname = "mso14:CT_ToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  #[sdk(child(qname = "mso14:CT_EditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  #[sdk(child(qname = "mso14:CT_ComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  #[sdk(child(qname = "mso14:CT_DropDownRegular/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  #[sdk(child(qname = "mso14:CT_Gallery/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  #[sdk(child(qname = "mso14:CT_Menu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  #[sdk(child(qname = "mso14:CT_SplitButton/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  #[sdk(child(qname = "mso14:CT_Box/mso14:box"))]
  Mso14Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  #[sdk(child(qname = "mso14:CT_ButtonGroup/mso14:buttonGroup"))]
  Mso14ButtonGroup(std::boxed::Box<ButtonGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ButtonGroupChoice {
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
  #[sdk(child(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  #[sdk(child(qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupBoxChoice {
  #[sdk(child(qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LayoutContainerChoice {
  #[sdk(child(qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuRootChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupChoice {
  /// Defines the ControlClone Class.
  #[sdk(child(qname = "mso14:CT_ControlClone/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  #[sdk(child(qname = "mso14:CT_LabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  #[sdk(child(qname = "mso14:CT_Button/mso14:button"))]
  Mso14Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  #[sdk(child(qname = "mso14:CT_ToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  #[sdk(child(qname = "mso14:CT_EditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  #[sdk(child(qname = "mso14:CT_ComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  #[sdk(child(qname = "mso14:CT_DropDownRegular/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  #[sdk(child(qname = "mso14:CT_Gallery/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  #[sdk(child(qname = "mso14:CT_Menu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  #[sdk(child(qname = "mso14:CT_SplitButton/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  #[sdk(child(qname = "mso14:CT_Box/mso14:box"))]
  Mso14Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  #[sdk(child(qname = "mso14:CT_ButtonGroup/mso14:buttonGroup"))]
  Mso14ButtonGroup(std::boxed::Box<ButtonGroup>),
  /// Defines the Separator Class.
  #[sdk(child(qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SharedControlsQatItemsChoice {
  #[sdk(child(qname = "mso14:CT_ControlCloneQat/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneQat>),
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  #[sdk(child(qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DocumentControlsQatItemsChoice {
  #[sdk(child(qname = "mso14:CT_ControlCloneQat/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneQat>),
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  #[sdk(child(qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum QatItemsTypeChoice {
  /// Defines the ControlCloneQat Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneQat/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  #[sdk(child(qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ContextMenuChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
  /// Defines the MenuSeparatorNoTitle Class.
  #[sdk(child(qname = "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparatorNoTitle>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageMenuGroupChoice {
  /// Defines the BackstageMenuButton Class.
  #[sdk(child(qname = "mso14:CT_BackstageMenuButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageMenuButton>),
  /// Defines the BackstageMenuCheckBox Class.
  #[sdk(child(qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageMenuCheckBox>),
  /// Defines the BackstageSubMenu Class.
  #[sdk(child(qname = "mso14:CT_BackstageSubMenu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<BackstageSubMenu>),
  /// Defines the BackstageMenuToggleButton Class.
  #[sdk(child(qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<BackstageMenuToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PrimaryItemChoice {
  /// Defines the BackstageRegularButton Class.
  #[sdk(child(qname = "mso14:CT_BackstageRegularButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageRegularButton>),
  /// Defines the BackstagePrimaryMenu Class.
  #[sdk(child(qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<BackstagePrimaryMenu>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopItemsGroupControlsChoice {
  #[sdk(child(qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomItemsGroupControlsChoice {
  #[sdk(child(qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupControlsTypeChoice {
  /// Defines the BackstageGroupButton Class.
  #[sdk(child(qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  #[sdk(child(qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  #[sdk(child(qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  #[sdk(child(qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  #[sdk(child(qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  #[sdk(child(qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  #[sdk(child(qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  #[sdk(child(qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  #[sdk(child(qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  #[sdk(child(qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageGroupsChoice {
  #[sdk(child(qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup"))]
  Mso14TaskFormGroup(std::boxed::Box<TaskFormGroup>),
  #[sdk(child(qname = "mso14:CT_BackstageGroup/mso14:group"))]
  Mso14Group(std::boxed::Box<BackstageGroup>),
  #[sdk(child(qname = "mso14:CT_TaskGroup/mso14:taskGroup"))]
  Mso14TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SimpleGroupsChoice {
  #[sdk(child(qname = "mso14:CT_BackstageGroup/mso14:group"))]
  Mso14Group(std::boxed::Box<BackstageGroup>),
  #[sdk(child(qname = "mso14:CT_TaskGroup/mso14:taskGroup"))]
  Mso14TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageChoice {
  #[sdk(child(qname = "mso14:CT_BackstageTab/mso14:tab"))]
  Mso14Tab(std::boxed::Box<BackstageTab>),
  #[sdk(child(qname = "mso14:CT_BackstageFastCommandButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageFastCommandButton>),
}
