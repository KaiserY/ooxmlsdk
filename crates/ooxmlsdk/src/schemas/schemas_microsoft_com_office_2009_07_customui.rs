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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control")]
pub struct ControlCloneRegular {
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ButtonRegular/mso14:button")]
pub struct ButtonRegular {
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the CheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox")]
pub struct CheckBox {
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the GalleryRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery")]
pub struct GalleryRegular {
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(office2010, qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  #[sdk(attr(office2010, qname = ":columns"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  #[sdk(attr(office2010, qname = ":rows"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  #[sdk(attr(office2010, qname = ":itemWidth"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  #[sdk(attr(office2010, qname = ":itemHeight"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  #[sdk(attr(office2010, qname = ":getItemWidth"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  #[sdk(attr(office2010, qname = ":getItemHeight"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(office2010, qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  #[sdk(attr(office2010, qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(office2010, qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(office2010, qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(office2010, qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(office2010, qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(office2010, qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(office2010, qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(office2010, qname = "mso14:CT_Item/mso14:item"))]
  pub mso14_item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub mso14_button: Vec<ButtonRegular>,
}
/// Defines the ToggleButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton")]
pub struct ToggleButtonRegular {
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the MenuSeparator Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_MenuSeparator/mso14:menuSeparator")]
pub struct MenuSeparator {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(office2010, qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(office2010, qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton")]
pub struct SplitButtonRegular {
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_regular_choice: Option<SplitButtonRegularChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  pub mso14_menu: Option<MenuRegular>,
}
/// Defines the MenuRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_MenuRegular/mso14:menu")]
pub struct MenuRegular {
  /// itemSize
  #[sdk(attr(office2010, qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu")]
pub struct DynamicMenuRegular {
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  #[sdk(attr(office2010, qname = ":getContent"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  #[sdk(attr(office2010, qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_SplitButtonWithTitle/mso14:splitButton")]
pub struct SplitButtonWithTitle {
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_with_title_choice: Option<SplitButtonWithTitleChoice>,
  /// Defines the MenuWithTitle Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuWithTitle/mso14:menu"))]
  pub mso14_menu: Option<MenuWithTitle>,
}
/// Defines the MenuWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_MenuWithTitle/mso14:menu")]
pub struct MenuWithTitle {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(office2010, qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// title
  #[sdk(attr(office2010, qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(office2010, qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator"
)]
pub struct MenuSeparatorNoTitle {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ControlClone Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ControlClone/mso14:control")]
pub struct ControlClone {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the LabelControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_LabelControl/mso14:labelControl")]
pub struct LabelControl {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
}
/// Defines the Button Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Button/mso14:button")]
pub struct Button {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ToggleButton/mso14:toggleButton")]
pub struct ToggleButton {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the EditBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_EditBox/mso14:editBox")]
pub struct EditBox {
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(office2010, qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  #[sdk(attr(office2010, qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(office2010, qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ComboBox/mso14:comboBox")]
pub struct ComboBox {
  /// showItemImage
  #[sdk(attr(office2010, qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(office2010, qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(office2010, qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(office2010, qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(office2010, qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(office2010, qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  #[sdk(attr(office2010, qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(office2010, qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(office2010, qname = "mso14:CT_Item/mso14:item"))]
  pub mso14_item: Vec<Item>,
}
/// Defines the DropDownRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_DropDownRegular/mso14:dropDown")]
pub struct DropDownRegular {
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(office2010, qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(office2010, qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(office2010, qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(office2010, qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(office2010, qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(office2010, qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(office2010, qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(office2010, qname = "mso14:CT_Item/mso14:item"))]
  pub mso14_item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub mso14_button: Vec<ButtonRegular>,
}
/// Defines the Gallery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Gallery/mso14:gallery")]
pub struct Gallery {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(office2010, qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  #[sdk(attr(office2010, qname = ":columns"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  #[sdk(attr(office2010, qname = ":rows"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  #[sdk(attr(office2010, qname = ":itemWidth"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  #[sdk(attr(office2010, qname = ":itemHeight"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  #[sdk(attr(office2010, qname = ":getItemWidth"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  #[sdk(attr(office2010, qname = ":getItemHeight"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(office2010, qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  #[sdk(attr(office2010, qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(office2010, qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(office2010, qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(office2010, qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(office2010, qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(office2010, qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(office2010, qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(office2010, qname = "mso14:CT_Item/mso14:item"))]
  pub mso14_item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub mso14_button: Vec<ButtonRegular>,
}
/// Defines the Menu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Menu/mso14:menu")]
pub struct Menu {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(office2010, qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu")]
pub struct DynamicMenu {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  #[sdk(attr(office2010, qname = ":getContent"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  #[sdk(attr(office2010, qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_SplitButton/mso14:splitButton")]
pub struct SplitButton {
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_VisibleButton/mso14:button",
    qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"
  ))]
  pub split_button_choice: Option<SplitButtonChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  pub mso14_menu: Option<MenuRegular>,
}
/// Defines the Box Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Box/mso14:box")]
pub struct Box {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// boxStyle
  #[sdk(attr(office2010, qname = ":boxStyle"))]
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
  pub box_choice: Vec<BoxChoice>,
}
/// Defines the ButtonGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ButtonGroup/mso14:buttonGroup")]
pub struct ButtonGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageMenuButton/mso14:button")]
pub struct BackstageMenuButton {
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(office2010, qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageMenuCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox")]
pub struct BackstageMenuCheckBox {
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageSubMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageSubMenu/mso14:menu")]
pub struct BackstageSubMenu {
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageMenuGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup"))]
  pub mso14_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton"
)]
pub struct BackstageMenuToggleButton {
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroupButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageGroupButton/mso14:button")]
pub struct BackstageGroupButton {
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// style
  #[sdk(attr(office2010, qname = ":style"))]
  pub style: Option<Style2Values>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(office2010, qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageCheckBox/mso14:checkBox")]
pub struct BackstageCheckBox {
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageEditBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageEditBox/mso14:editBox")]
pub struct BackstageEditBox {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  #[sdk(attr(office2010, qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(office2010, qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(office2010, qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageDropDown Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageDropDown/mso14:dropDown")]
pub struct BackstageDropDown {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(office2010, qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the ItemBackstageItem Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageItem/mso14:item"))]
  pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the RadioGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_RadioGroup/mso14:radioGroup")]
pub struct RadioGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(office2010, qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the RadioButtonBackstageItem Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageItem/mso14:radioButton"))]
  pub mso14_radio_button: Vec<RadioButtonBackstageItem>,
}
/// Defines the BackstageComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageComboBox/mso14:comboBox")]
pub struct BackstageComboBox {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  #[sdk(attr(office2010, qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(office2010, qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(office2010, qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(office2010, qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(office2010, qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(office2010, qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the ItemBackstageItem Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageItem/mso14:item"))]
  pub mso14_item: Vec<ItemBackstageItem>,
}
/// Defines the Hyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Hyperlink/mso14:hyperlink")]
pub struct Hyperlink {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// target
  #[sdk(attr(office2010, qname = ":target"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub target: Option<crate::simple_type::StringValue>,
  /// getTarget
  #[sdk(attr(office2010, qname = ":getTarget"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_target: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageLabelControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"
)]
pub struct BackstageLabelControl {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(office2010, qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// noWrap
  #[sdk(attr(office2010, qname = ":noWrap"))]
  pub no_wrap: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_GroupBox/mso14:groupBox")]
pub struct GroupBox {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_LayoutContainer/mso14:layoutContainer")]
pub struct LayoutContainer {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// align
  #[sdk(attr(office2010, qname = ":align"))]
  pub align: Option<ExpandValues>,
  /// expand
  #[sdk(attr(office2010, qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// layoutChildren
  #[sdk(attr(office2010, qname = ":layoutChildren"))]
  pub layout_children: Option<LayoutChildrenValues>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ImageControl/mso14:imageControl")]
pub struct ImageControl {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// altText
  #[sdk(attr(office2010, qname = ":altText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// getAltText
  #[sdk(attr(office2010, qname = ":getAltText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_alt_text: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageGroup/mso14:group")]
pub struct BackstageGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// style
  #[sdk(attr(office2010, qname = ":style"))]
  pub style: Option<StyleValues>,
  /// getStyle
  #[sdk(attr(office2010, qname = ":getStyle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_style: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(office2010, qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(office2010, qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(microsoft365, qname = "mso14:CT_PrimaryItem/mso14:primaryItem"))]
  pub backstage_group_choice: Option<BackstageGroupChoice>,
  /// Defines the TopItemsGroupControls Class.
  #[sdk(child(office2010, qname = "mso14:CT_GroupControls/mso14:topItems"))]
  pub mso14_top_items: Option<TopItemsGroupControls>,
  /// Defines the BottomItemsGroupControls Class.
  #[sdk(child(office2010, qname = "mso14:CT_GroupControls/mso14:bottomItems"))]
  pub mso14_bottom_items: Option<BottomItemsGroupControls>,
}
/// Defines the TaskGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskGroup/mso14:taskGroup")]
pub struct TaskGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(office2010, qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(office2010, qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  #[sdk(attr(office2010, qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// Defines the TaskGroupCategory Class.
  #[sdk(child(office2010, qname = "mso14:CT_TaskGroupCategory/mso14:category"))]
  pub mso14_category: Vec<TaskGroupCategory>,
}
/// Defines the MenuRoot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_MenuRoot/mso14:menu")]
pub struct MenuRoot {
  /// title
  #[sdk(attr(office2010, qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(office2010, qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(office2010, qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_CustomUI/mso14:customUI")]
pub struct CustomUi {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// onLoad
  #[sdk(attr(office2010, qname = ":onLoad"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_load: Option<crate::simple_type::StringValue>,
  /// loadImage
  #[sdk(attr(office2010, qname = ":loadImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub load_image: Option<crate::simple_type::StringValue>,
  /// Defines the Commands Class.
  #[sdk(child(office2010, qname = "mso14:CT_Commands/mso14:commands"))]
  pub commands: Option<Commands>,
  /// Defines the Ribbon Class.
  #[sdk(child(office2010, qname = "mso14:CT_Ribbon/mso14:ribbon"))]
  pub ribbon: Option<std::boxed::Box<Ribbon>>,
  /// Defines the Backstage Class.
  #[sdk(child(office2010, qname = "mso14:CT_Backstage/mso14:backstage"))]
  pub backstage: Option<Backstage>,
  /// Defines the ContextMenus Class.
  #[sdk(child(office2010, qname = "mso14:CT_ContextMenus/mso14:contextMenus"))]
  pub context_menus: Option<ContextMenus>,
}
/// Defines the Item Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Item/mso14:item")]
pub struct Item {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_VisibleButton/mso14:button")]
pub struct VisibleButton {
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton")]
pub struct VisibleToggleButton {
  /// getPressed
  #[sdk(attr(office2010, qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Separator Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Separator/mso14:separator")]
pub struct Separator {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DialogBoxLauncher Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_DialogLauncher/mso14:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  pub button_regular: std::boxed::Box<ButtonRegular>,
}
/// Defines the Group Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Group/mso14:group")]
pub struct Group {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// autoScale
  #[sdk(attr(office2010, qname = ":autoScale"))]
  pub auto_scale: Option<crate::simple_type::BooleanValue>,
  /// centerVertically
  #[sdk(attr(office2010, qname = ":centerVertically"))]
  pub center_vertically: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    microsoft365,
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
  /// Defines the DialogBoxLauncher Class.
  #[sdk(child(office2010, qname = "mso14:CT_DialogLauncher/mso14:dialogBoxLauncher"))]
  pub mso14_dialog_box_launcher: Option<std::boxed::Box<DialogBoxLauncher>>,
}
/// Defines the ControlCloneQat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ControlCloneQat/mso14:control")]
pub struct ControlCloneQat {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// size
  #[sdk(attr(office2010, qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(office2010, qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(office2010, qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(office2010, qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SharedControlsQatItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_QatItems/mso14:sharedControls")]
pub struct SharedControlsQatItems {
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_ControlCloneQat/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub shared_controls_qat_items_choice: Vec<SharedControlsQatItemsChoice>,
}
/// Defines the DocumentControlsQatItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_QatItems/mso14:documentControls")]
pub struct DocumentControlsQatItems {
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_ControlCloneQat/mso14:control",
    qname = "mso14:CT_ButtonRegular/mso14:button",
    qname = "mso14:CT_Separator/mso14:separator"
  ))]
  pub document_controls_qat_items_choice: Vec<DocumentControlsQatItemsChoice>,
}
/// Defines the Tab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Tab/mso14:tab")]
pub struct Tab {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the Group Class.
  #[sdk(child(office2010, qname = "mso14:CT_Group/mso14:group"))]
  pub mso14_group: Vec<Group>,
}
/// Defines the TabSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TabSet/mso14:tabSet")]
pub struct TabSet {
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: crate::simple_type::StringValue,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// Defines the Tab Class.
  #[sdk(child(office2010, qname = "mso14:CT_Tab/mso14:tab"))]
  pub mso14_tab: Vec<Tab>,
}
/// Defines the Command Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Command/mso14:command")]
pub struct Command {
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
}
/// Defines the QuickAccessToolbar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Qat/mso14:qat")]
pub struct QuickAccessToolbar {
  /// Defines the SharedControlsQatItems Class.
  #[sdk(child(office2010, qname = "mso14:CT_QatItems/mso14:sharedControls"))]
  pub shared_controls_qat_items: Option<SharedControlsQatItems>,
  /// Defines the DocumentControlsQatItems Class.
  #[sdk(child(office2010, qname = "mso14:CT_QatItems/mso14:documentControls"))]
  pub document_controls_qat_items: Option<DocumentControlsQatItems>,
}
/// Defines the Tabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Tabs/mso14:tabs")]
pub struct Tabs {
  /// Defines the Tab Class.
  #[sdk(child(office2010, qname = "mso14:CT_Tab/mso14:tab"))]
  pub mso14_tab: Vec<Tab>,
}
/// Defines the ContextualTabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ContextualTabs/mso14:contextualTabs")]
pub struct ContextualTabs {
  /// Defines the TabSet Class.
  #[sdk(child(office2010, qname = "mso14:CT_TabSet/mso14:tabSet"))]
  pub mso14_tab_set: Vec<TabSet>,
}
/// Defines the ContextMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ContextMenu/mso14:contextMenu")]
pub struct ContextMenu {
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageItem/mso14:item")]
pub struct ItemBackstageItem {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the RadioButtonBackstageItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageItem/mso14:radioButton")]
pub struct RadioButtonBackstageItem {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageRegularButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageRegularButton/mso14:button")]
pub struct BackstageRegularButton {
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(office2010, qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstagePrimaryMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu")]
pub struct BackstagePrimaryMenu {
  /// screentip
  #[sdk(attr(office2010, qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(office2010, qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(office2010, qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(office2010, qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageMenuGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup"))]
  pub mso14_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageMenuGroup/mso14:menuGroup")]
pub struct BackstageMenuGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(office2010, qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_BackstageMenuButton/mso14:button",
    qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox",
    qname = "mso14:CT_BackstageSubMenu/mso14:menu",
    qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton"
  ))]
  pub backstage_menu_group_choice: Vec<BackstageMenuGroupChoice>,
}
/// Defines the PrimaryItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_PrimaryItem/mso14:primaryItem")]
pub struct PrimaryItem {
  #[sdk(choice(
    qname = "mso14:CT_BackstageRegularButton/mso14:button",
    qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu"
  ))]
  pub primary_item_choice: Option<PrimaryItemChoice>,
}
/// Defines the TopItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_GroupControls/mso14:topItems")]
pub struct TopItemsGroupControls {
  #[sdk(choice(
    microsoft365,
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
  pub top_items_group_controls_choice: Vec<TopItemsGroupControlsChoice>,
}
/// Defines the BottomItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_GroupControls/mso14:bottomItems")]
pub struct BottomItemsGroupControls {
  #[sdk(choice(
    microsoft365,
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
  pub bottom_items_group_controls_choice: Vec<BottomItemsGroupControlsChoice>,
}
/// Defines the TaskGroupCategory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskGroupCategory/mso14:category")]
pub struct TaskGroupCategory {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// Defines the TaskGroupTask Class.
  #[sdk(child(office2010, qname = "mso14:CT_TaskGroupTask/mso14:task"))]
  pub mso14_task: Vec<TaskGroupTask>,
}
/// Defines the TaskGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskGroupTask/mso14:task")]
pub struct TaskGroupTask {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(office2010, qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the TaskFormGroupCategory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskFormGroupCategory/mso14:category")]
pub struct TaskFormGroupCategory {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// Defines the TaskFormGroupTask Class.
  #[sdk(child(office2010, qname = "mso14:CT_TaskFormGroupTask/mso14:task"))]
  pub mso14_task: Vec<TaskFormGroupTask>,
}
/// Defines the TaskFormGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskFormGroupTask/mso14:task")]
pub struct TaskFormGroupTask {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2010, qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(office2010, qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroup/mso14:group"))]
  pub mso14_group: Vec<BackstageGroup>,
}
/// Defines the TaskFormGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup")]
pub struct TaskFormGroup {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(office2010, qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(office2010, qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(office2010, qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(office2010, qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  #[sdk(attr(office2010, qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// Defines the TaskFormGroupCategory Class.
  #[sdk(child(office2010, qname = "mso14:CT_TaskFormGroupCategory/mso14:category"))]
  pub mso14_category: Vec<TaskFormGroupCategory>,
}
/// Defines the BackstageGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageGroups/mso14:firstColumn")]
pub struct BackstageGroups {
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup",
    qname = "mso14:CT_BackstageGroup/mso14:group",
    qname = "mso14:CT_TaskGroup/mso14:taskGroup"
  ))]
  pub backstage_groups_choice: Option<BackstageGroupsChoice>,
}
/// Defines the SimpleGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_SimpleGroups/mso14:secondColumn")]
pub struct SimpleGroups {
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_BackstageGroup/mso14:group",
    qname = "mso14:CT_TaskGroup/mso14:taskGroup"
  ))]
  pub simple_groups_choice: Vec<SimpleGroupsChoice>,
}
/// Defines the BackstageTab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageTab/mso14:tab")]
pub struct BackstageTab {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(office2010, qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(office2010, qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// columnWidthPercent
  #[sdk(attr(office2010, qname = ":columnWidthPercent"))]
  #[sdk(number_range(range = 1..= 99))]
  #[sdk(number_sign(kind = "positive"))]
  pub column_width_percent: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMinWidth
  #[sdk(attr(office2010, qname = ":firstColumnMinWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub first_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMaxWidth
  #[sdk(attr(office2010, qname = ":firstColumnMaxWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub first_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMinWidth
  #[sdk(attr(office2010, qname = ":secondColumnMinWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub second_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMaxWidth
  #[sdk(attr(office2010, qname = ":secondColumnMaxWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub second_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// Defines the BackstageGroups Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroups/mso14:firstColumn"))]
  pub backstage_groups: Option<std::boxed::Box<BackstageGroups>>,
  /// Defines the SimpleGroups Class.
  #[sdk(child(office2010, qname = "mso14:CT_SimpleGroups/mso14:secondColumn"))]
  pub simple_groups: Option<SimpleGroups>,
}
/// Defines the BackstageFastCommandButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_BackstageFastCommandButton/mso14:button")]
pub struct BackstageFastCommandButton {
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(office2010, qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(office2010, qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(office2010, qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(office2010, qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(office2010, qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(office2010, qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(office2010, qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(office2010, qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(office2010, qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(office2010, qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(office2010, qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(office2010, qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(office2010, qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(office2010, qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(office2010, qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(office2010, qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(office2010, qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(office2010, qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(office2010, qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Commands Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Commands/mso14:commands")]
pub struct Commands {
  /// Defines the Command Class.
  #[sdk(child(office2010, qname = "mso14:CT_Command/mso14:command"))]
  pub mso14_command: Vec<Command>,
}
/// Defines the Ribbon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Ribbon/mso14:ribbon")]
pub struct Ribbon {
  /// startFromScratch
  #[sdk(attr(office2010, qname = ":startFromScratch"))]
  pub start_from_scratch: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "mso14:CT_Qat/mso14:qat"))]
  pub quick_access_toolbar: Option<std::boxed::Box<QuickAccessToolbar>>,
  /// _
  #[sdk(child(office2010, qname = "mso14:CT_Tabs/mso14:tabs"))]
  pub tabs: Option<Tabs>,
  /// _
  #[sdk(child(office2010, qname = "mso14:CT_ContextualTabs/mso14:contextualTabs"))]
  pub contextual_tabs: Option<ContextualTabs>,
}
/// Defines the Backstage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_Backstage/mso14:backstage")]
pub struct Backstage {
  /// onShow
  #[sdk(attr(office2010, qname = ":onShow"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_show: Option<crate::simple_type::StringValue>,
  /// onHide
  #[sdk(attr(office2010, qname = ":onHide"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_hide: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    microsoft365,
    qname = "mso14:CT_BackstageTab/mso14:tab",
    qname = "mso14:CT_BackstageFastCommandButton/mso14:button"
  ))]
  pub backstage_choice: Vec<BackstageChoice>,
}
/// Defines the ContextMenus Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:CT_ContextMenus/mso14:contextMenus")]
pub struct ContextMenus {
  /// Defines the ContextMenu Class.
  #[sdk(child(office2010, qname = "mso14:CT_ContextMenu/mso14:contextMenu"))]
  pub mso14_context_menu: Vec<ContextMenu>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonRegularChoice {
  #[sdk(child(office2010, qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(office2010, qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuRegularChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonWithTitleChoice {
  #[sdk(child(office2010, qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(office2010, qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuWithTitleChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonWithTitle Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonWithTitle/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonWithTitle>),
  /// Defines the MenuWithTitle Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuWithTitle/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuWithTitle>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonChoice {
  #[sdk(child(office2010, qname = "mso14:CT_VisibleButton/mso14:button"))]
  Mso14Button(std::boxed::Box<VisibleButton>),
  #[sdk(child(office2010, qname = "mso14:CT_VisibleToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BoxChoice {
  /// Defines the ControlClone Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlClone/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  #[sdk(child(office2010, qname = "mso14:CT_LabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  #[sdk(child(office2010, qname = "mso14:CT_Button/mso14:button"))]
  Mso14Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_EditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_ComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DropDownRegular/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  #[sdk(child(office2010, qname = "mso14:CT_Gallery/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  #[sdk(child(office2010, qname = "mso14:CT_Menu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButton/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  #[sdk(child(office2010, qname = "mso14:CT_Box/mso14:box"))]
  Mso14Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonGroup/mso14:buttonGroup"))]
  Mso14ButtonGroup(std::boxed::Box<ButtonGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ButtonGroupChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the Separator Class.
  #[sdk(child(office2010, qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupBoxChoice {
  /// Defines the BackstageGroupButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  #[sdk(child(office2010, qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  #[sdk(child(
    office2010,
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"
  ))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  #[sdk(child(office2010, qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  #[sdk(child(office2010, qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LayoutContainerChoice {
  /// Defines the BackstageGroupButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  #[sdk(child(office2010, qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  #[sdk(child(
    office2010,
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"
  ))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  #[sdk(child(office2010, qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  #[sdk(child(office2010, qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageGroupChoice {
  #[sdk(child(office2010, qname = "mso14:CT_PrimaryItem/mso14:primaryItem"))]
  Mso14PrimaryItem(std::boxed::Box<PrimaryItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MenuRootChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuSeparator/mso14:menuSeparator"))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupChoice {
  /// Defines the ControlClone Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlClone/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  #[sdk(child(office2010, qname = "mso14:CT_LabelControl/mso14:labelControl"))]
  Mso14LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  #[sdk(child(office2010, qname = "mso14:CT_Button/mso14:button"))]
  Mso14Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButton/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_EditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_ComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DropDownRegular/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  #[sdk(child(office2010, qname = "mso14:CT_Gallery/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  #[sdk(child(office2010, qname = "mso14:CT_Menu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenu/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButton/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  #[sdk(child(office2010, qname = "mso14:CT_Box/mso14:box"))]
  Mso14Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonGroup/mso14:buttonGroup"))]
  Mso14ButtonGroup(std::boxed::Box<ButtonGroup>),
  /// Defines the Separator Class.
  #[sdk(child(office2010, qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SharedControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneQat/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  #[sdk(child(office2010, qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DocumentControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneQat/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  #[sdk(child(office2010, qname = "mso14:CT_Separator/mso14:separator"))]
  Mso14Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ContextMenuChoice {
  /// Defines the ControlCloneRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ControlCloneRegular/mso14:control"))]
  Mso14Control(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ButtonRegular/mso14:button"))]
  Mso14Button(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_CheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_GalleryRegular/mso14:gallery"))]
  Mso14Gallery(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_ToggleButtonRegular/mso14:toggleButton"))]
  Mso14ToggleButton(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the SplitButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_SplitButtonRegular/mso14:splitButton"))]
  Mso14SplitButton(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_MenuRegular/mso14:menu"))]
  Mso14Menu(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu"))]
  Mso14DynamicMenu(std::boxed::Box<DynamicMenuRegular>),
  /// Defines the MenuSeparatorNoTitle Class.
  #[sdk(child(
    office2010,
    qname = "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator"
  ))]
  Mso14MenuSeparator(std::boxed::Box<MenuSeparatorNoTitle>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageMenuGroupChoice {
  /// Defines the BackstageMenuButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageMenuButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageMenuButton>),
  /// Defines the BackstageMenuCheckBox Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageMenuCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageMenuCheckBox>),
  /// Defines the BackstageSubMenu Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageSubMenu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<BackstageSubMenu>),
  /// Defines the BackstageMenuToggleButton Class.
  #[sdk(child(
    office2010,
    qname = "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton"
  ))]
  Mso14ToggleButton(std::boxed::Box<BackstageMenuToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PrimaryItemChoice {
  /// Defines the BackstageRegularButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageRegularButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageRegularButton>),
  /// Defines the BackstagePrimaryMenu Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstagePrimaryMenu/mso14:menu"))]
  Mso14Menu(std::boxed::Box<BackstagePrimaryMenu>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopItemsGroupControlsChoice {
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(office2010, qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(office2010, qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(
    office2010,
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"
  ))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(office2010, qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(office2010, qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(office2010, qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomItemsGroupControlsChoice {
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroupButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageGroupButton>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageCheckBox/mso14:checkBox"))]
  Mso14CheckBox(std::boxed::Box<BackstageCheckBox>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageEditBox/mso14:editBox"))]
  Mso14EditBox(std::boxed::Box<BackstageEditBox>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageDropDown/mso14:dropDown"))]
  Mso14DropDown(std::boxed::Box<BackstageDropDown>),
  #[sdk(child(office2010, qname = "mso14:CT_RadioGroup/mso14:radioGroup"))]
  Mso14RadioGroup(std::boxed::Box<RadioGroup>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageComboBox/mso14:comboBox"))]
  Mso14ComboBox(std::boxed::Box<BackstageComboBox>),
  #[sdk(child(office2010, qname = "mso14:CT_Hyperlink/mso14:hyperlink"))]
  Mso14Hyperlink(std::boxed::Box<Hyperlink>),
  #[sdk(child(
    office2010,
    qname = "mso14:CT_BackstageLabelControl/mso14:labelControl"
  ))]
  Mso14LabelControl(std::boxed::Box<BackstageLabelControl>),
  #[sdk(child(office2010, qname = "mso14:CT_GroupBox/mso14:groupBox"))]
  Mso14GroupBox(std::boxed::Box<GroupBox>),
  #[sdk(child(office2010, qname = "mso14:CT_LayoutContainer/mso14:layoutContainer"))]
  Mso14LayoutContainer(std::boxed::Box<LayoutContainer>),
  #[sdk(child(office2010, qname = "mso14:CT_ImageControl/mso14:imageControl"))]
  Mso14ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageGroupsChoice {
  #[sdk(child(office2010, qname = "mso14:CT_TaskFormGroup/mso14:taskFormGroup"))]
  Mso14TaskFormGroup(std::boxed::Box<TaskFormGroup>),
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroup/mso14:group"))]
  Mso14Group(std::boxed::Box<BackstageGroup>),
  #[sdk(child(office2010, qname = "mso14:CT_TaskGroup/mso14:taskGroup"))]
  Mso14TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SimpleGroupsChoice {
  #[sdk(child(office2010, qname = "mso14:CT_BackstageGroup/mso14:group"))]
  Mso14Group(std::boxed::Box<BackstageGroup>),
  #[sdk(child(office2010, qname = "mso14:CT_TaskGroup/mso14:taskGroup"))]
  Mso14TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageChoice {
  /// Defines the BackstageTab Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageTab/mso14:tab"))]
  Mso14Tab(std::boxed::Box<BackstageTab>),
  /// Defines the BackstageFastCommandButton Class.
  #[sdk(child(office2010, qname = "mso14:CT_BackstageFastCommandButton/mso14:button"))]
  Mso14Button(std::boxed::Box<BackstageFastCommandButton>),
}
