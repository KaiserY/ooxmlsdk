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
#[sdk(qname = "mso14:control")]
pub struct ControlCloneRegular {
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct ButtonRegular {
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the CheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:checkBox")]
pub struct CheckBox {
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the GalleryRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:gallery")]
pub struct GalleryRegular {
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  #[sdk(attr(qname = ":columns"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  #[sdk(attr(qname = ":rows"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  #[sdk(attr(qname = ":itemWidth"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  #[sdk(attr(qname = ":itemHeight"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  #[sdk(attr(qname = ":getItemWidth"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  #[sdk(attr(qname = ":getItemHeight"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  #[sdk(attr(qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the ToggleButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:toggleButton")]
pub struct ToggleButtonRegular {
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the MenuSeparator Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menuSeparator")]
pub struct MenuSeparator {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:splitButton")]
pub struct SplitButtonRegular {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = VisibleButton, boxed, qname = "mso14:button"),
            child(variant = VisibleToggleButton, boxed, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_regular_choice: Option<SplitButtonRegularChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the MenuRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct MenuRegular {
  /// itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, boxed, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, boxed, qname = "mso14:splitButton"),
            child(variant = MenuRegular, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_regular_choice: Vec<MenuRegularChoice>,
}
/// Defines the DynamicMenuRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:dynamicMenu")]
pub struct DynamicMenuRegular {
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  #[sdk(attr(qname = ":getContent"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButtonWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:splitButton")]
pub struct SplitButtonWithTitle {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = VisibleButton, boxed, qname = "mso14:button"),
            child(variant = VisibleToggleButton, boxed, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_with_title_choice: Option<SplitButtonWithTitleChoice>,
  /// Defines the MenuWithTitle Class.
  #[sdk(child(qname = "mso14:menu"))]
  pub menu_with_title: Option<MenuWithTitle>,
}
/// Defines the MenuWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct MenuWithTitle {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, boxed, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonWithTitle, boxed, qname = "mso14:splitButton"),
            child(variant = MenuWithTitle, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_with_title_choice: Vec<MenuWithTitleChoice>,
}
/// Defines the MenuSeparatorNoTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menuSeparator")]
pub struct MenuSeparatorNoTitle {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the ControlClone Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:control")]
pub struct ControlClone {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the LabelControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:labelControl")]
pub struct LabelControl {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
}
/// Defines the Button Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct Button {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:toggleButton")]
pub struct ToggleButton {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the EditBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:editBox")]
pub struct EditBox {
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the ComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:comboBox")]
pub struct ComboBox {
  /// showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item: Vec<Item>,
}
/// Defines the DropDownRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:dropDown")]
pub struct DropDownRegular {
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Gallery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:gallery")]
pub struct Gallery {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// columns
  #[sdk(attr(qname = ":columns"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub columns: Option<crate::simple_type::IntegerValue>,
  /// rows
  #[sdk(attr(qname = ":rows"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub rows: Option<crate::simple_type::IntegerValue>,
  /// itemWidth
  #[sdk(attr(qname = ":itemWidth"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_width: Option<crate::simple_type::IntegerValue>,
  /// itemHeight
  #[sdk(attr(qname = ":itemHeight"))]
  #[sdk(number_range(range = 1..= 4096))]
  #[sdk(number_sign(kind = "positive"))]
  pub item_height: Option<crate::simple_type::IntegerValue>,
  /// getItemWidth
  #[sdk(attr(qname = ":getItemWidth"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_width: Option<crate::simple_type::StringValue>,
  /// getItemHeight
  #[sdk(attr(qname = ":getItemHeight"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_height: Option<crate::simple_type::StringValue>,
  /// showItemLabel
  #[sdk(attr(qname = ":showItemLabel"))]
  pub show_item_label: Option<crate::simple_type::BooleanValue>,
  /// showInRibbon
  #[sdk(attr(qname = ":showInRibbon"))]
  pub show_in_ribbon: Option<GalleryShowInRibbonValues>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// showItemImage
  #[sdk(attr(qname = ":showItemImage"))]
  pub show_item_image: Option<crate::simple_type::BooleanValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemScreentip
  #[sdk(attr(qname = ":getItemScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_screentip: Option<crate::simple_type::StringValue>,
  /// getItemSupertip
  #[sdk(attr(qname = ":getItemSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_supertip: Option<crate::simple_type::StringValue>,
  /// getItemImage
  #[sdk(attr(qname = ":getItemImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_image: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getSelectedItemID
  #[sdk(attr(qname = ":getSelectedItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_id: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  /// Defines the Item Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Menu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct Menu {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, boxed, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, boxed, qname = "mso14:splitButton"),
            child(variant = MenuRegular, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_choice: Vec<MenuChoice>,
}
/// Defines the DynamicMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:dynamicMenu")]
pub struct DynamicMenu {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// getContent
  #[sdk(attr(qname = ":getContent"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_content: crate::simple_type::StringValue,
  /// invalidateContentOnDrop
  #[sdk(attr(qname = ":invalidateContentOnDrop"))]
  pub invalidate_content_on_drop: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SplitButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:splitButton")]
pub struct SplitButton {
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = VisibleButton, boxed, qname = "mso14:button"),
            child(variant = VisibleToggleButton, boxed, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_choice: Option<SplitButtonChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(qname = "mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the Box Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:box")]
pub struct Box {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// boxStyle
  #[sdk(attr(qname = ":boxStyle"))]
  pub box_style: Option<BoxStyleValues>,
  #[sdk(
        choice(
            child(variant = ControlClone, boxed, qname = "mso14:control"),
            child(variant = LabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = Button, boxed, qname = "mso14:button"),
            child(variant = ToggleButton, boxed, qname = "mso14:toggleButton"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = EditBox, boxed, qname = "mso14:editBox"),
            child(variant = ComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = DropDownRegular, boxed, qname = "mso14:dropDown"),
            child(variant = Gallery, boxed, qname = "mso14:gallery"),
            child(variant = Menu, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenu, boxed, qname = "mso14:dynamicMenu"),
            child(variant = SplitButton, boxed, qname = "mso14:splitButton"),
            child(variant = Box, boxed, qname = "mso14:box"),
            child(variant = ButtonGroup, boxed, qname = "mso14:buttonGroup")
        )
    )]
  pub box_choice: Vec<BoxChoice>,
}
/// Defines the ButtonGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:buttonGroup")]
pub struct ButtonGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = MenuRegular, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu"),
            child(variant = SplitButtonRegular, boxed, qname = "mso14:splitButton"),
            child(variant = Separator, boxed, qname = "mso14:separator")
        )
    )]
  pub button_group_choice: Vec<ButtonGroupChoice>,
}
/// Defines the BackstageMenuButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct BackstageMenuButton {
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageMenuCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:checkBox")]
pub struct BackstageMenuCheckBox {
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageSubMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct BackstageSubMenu {
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageMenuGroup Class.
  #[sdk(child(qname = "mso14:menuGroup"))]
  pub backstage_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:toggleButton")]
pub struct BackstageMenuToggleButton {
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroupButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct BackstageGroupButton {
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<Style2Values>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:checkBox")]
pub struct BackstageCheckBox {
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageEditBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:editBox")]
pub struct BackstageEditBox {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// maxLength
  #[sdk(attr(qname = ":maxLength"))]
  #[sdk(number_range(range = 1..= 1024))]
  #[sdk(number_sign(kind = "positive"))]
  pub max_length: Option<crate::simple_type::IntegerValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageDropDown Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:dropDown")]
pub struct BackstageDropDown {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the ItemBackstageItem Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item_backstage_item: Vec<ItemBackstageItem>,
}
/// Defines the RadioGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:radioGroup")]
pub struct RadioGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getSelectedItemIndex
  #[sdk(attr(qname = ":getSelectedItemIndex"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_selected_item_index: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the RadioButtonBackstageItem Class.
  #[sdk(child(qname = "mso14:radioButton"))]
  pub radio_button_backstage_item: Vec<RadioButtonBackstageItem>,
}
/// Defines the BackstageComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:comboBox")]
pub struct BackstageComboBox {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// getText
  #[sdk(attr(qname = ":getText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_text: Option<crate::simple_type::StringValue>,
  /// onChange
  #[sdk(attr(qname = ":onChange"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_change: Option<crate::simple_type::StringValue>,
  /// sizeString
  #[sdk(attr(qname = ":sizeString"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub size_string: Option<crate::simple_type::StringValue>,
  /// getItemCount
  #[sdk(attr(qname = ":getItemCount"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_count: Option<crate::simple_type::StringValue>,
  /// getItemLabel
  #[sdk(attr(qname = ":getItemLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_label: Option<crate::simple_type::StringValue>,
  /// getItemID
  #[sdk(attr(qname = ":getItemID"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_item_id: Option<crate::simple_type::StringValue>,
  /// Defines the ItemBackstageItem Class.
  #[sdk(child(qname = "mso14:item"))]
  pub item_backstage_item: Vec<ItemBackstageItem>,
}
/// Defines the Hyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:hyperlink")]
pub struct Hyperlink {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// target
  #[sdk(attr(qname = ":target"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub target: Option<crate::simple_type::StringValue>,
  /// getTarget
  #[sdk(attr(qname = ":getTarget"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_target: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageLabelControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:labelControl")]
pub struct BackstageLabelControl {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// alignLabel
  #[sdk(attr(qname = ":alignLabel"))]
  pub align_label: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// noWrap
  #[sdk(attr(qname = ":noWrap"))]
  pub no_wrap: Option<crate::simple_type::BooleanValue>,
}
/// Defines the GroupBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:groupBox")]
pub struct GroupBox {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, boxed, qname = "mso14:button"),
            child(variant = BackstageCheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, boxed, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, boxed, qname = "mso14:dropDown"),
            child(variant = RadioGroup, boxed, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = Hyperlink, boxed, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = GroupBox, boxed, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, boxed, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, boxed, qname = "mso14:imageControl")
        )
    )]
  pub group_box_choice: Vec<GroupBoxChoice>,
}
/// Defines the LayoutContainer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:layoutContainer")]
pub struct LayoutContainer {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<ExpandValues>,
  /// expand
  #[sdk(attr(qname = ":expand"))]
  pub expand: Option<ExpandValues>,
  /// layoutChildren
  #[sdk(attr(qname = ":layoutChildren"))]
  pub layout_children: Option<LayoutChildrenValues>,
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, boxed, qname = "mso14:button"),
            child(variant = BackstageCheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, boxed, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, boxed, qname = "mso14:dropDown"),
            child(variant = RadioGroup, boxed, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = Hyperlink, boxed, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = GroupBox, boxed, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, boxed, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, boxed, qname = "mso14:imageControl")
        )
    )]
  pub layout_container_choice: Vec<LayoutContainerChoice>,
}
/// Defines the ImageControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:imageControl")]
pub struct ImageControl {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// altText
  #[sdk(attr(qname = ":altText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// getAltText
  #[sdk(attr(qname = ":getAltText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_alt_text: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:group")]
pub struct BackstageGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<StyleValues>,
  /// getStyle
  #[sdk(attr(qname = ":getStyle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_style: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  #[sdk(choice(child(variant = PrimaryItem, boxed, qname = "mso14:primaryItem")))]
  pub backstage_group_choice: Option<BackstageGroupChoice>,
  /// Defines the TopItemsGroupControls Class.
  #[sdk(child(qname = "mso14:topItems"))]
  pub top_items_group_controls: Option<TopItemsGroupControls>,
  /// Defines the BottomItemsGroupControls Class.
  #[sdk(child(qname = "mso14:bottomItems"))]
  pub bottom_items_group_controls: Option<BottomItemsGroupControls>,
}
/// Defines the TaskGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:taskGroup")]
pub struct TaskGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  #[sdk(attr(qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// Defines the TaskGroupCategory Class.
  #[sdk(child(qname = "mso14:category"))]
  pub task_group_category: Vec<TaskGroupCategory>,
}
/// Defines the MenuRoot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct MenuRoot {
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, boxed, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, boxed, qname = "mso14:splitButton"),
            child(variant = MenuRegular, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_root_choice: Vec<MenuRootChoice>,
}
/// Defines the CustomUI Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "mso14:customUI")]
pub struct CustomUi {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// onLoad
  #[sdk(attr(qname = ":onLoad"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_load: Option<crate::simple_type::StringValue>,
  /// loadImage
  #[sdk(attr(qname = ":loadImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub load_image: Option<crate::simple_type::StringValue>,
  /// Defines the Commands Class.
  #[sdk(child(qname = "mso14:commands"))]
  pub commands: Option<Commands>,
  /// Defines the Ribbon Class.
  #[sdk(child(qname = "mso14:ribbon"))]
  pub ribbon: Option<std::boxed::Box<Ribbon>>,
  /// Defines the Backstage Class.
  #[sdk(child(qname = "mso14:backstage"))]
  pub backstage: Option<Backstage>,
  /// Defines the ContextMenus Class.
  #[sdk(child(qname = "mso14:contextMenus"))]
  pub context_menus: Option<ContextMenus>,
}
/// Defines the Item Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:item")]
pub struct Item {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct VisibleButton {
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the VisibleToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:toggleButton")]
pub struct VisibleToggleButton {
  /// getPressed
  #[sdk(attr(qname = ":getPressed"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_pressed: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Separator Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:separator")]
pub struct Separator {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
}
/// Defines the DialogBoxLauncher Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
  /// Defines the ButtonRegular Class.
  #[sdk(child(qname = "mso14:button"))]
  pub button_regular: std::boxed::Box<ButtonRegular>,
}
/// Defines the Group Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:group")]
pub struct Group {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// autoScale
  #[sdk(attr(qname = ":autoScale"))]
  pub auto_scale: Option<crate::simple_type::BooleanValue>,
  /// centerVertically
  #[sdk(attr(qname = ":centerVertically"))]
  pub center_vertically: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = ControlClone, boxed, qname = "mso14:control"),
            child(variant = LabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = Button, boxed, qname = "mso14:button"),
            child(variant = ToggleButton, boxed, qname = "mso14:toggleButton"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = EditBox, boxed, qname = "mso14:editBox"),
            child(variant = ComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = DropDownRegular, boxed, qname = "mso14:dropDown"),
            child(variant = Gallery, boxed, qname = "mso14:gallery"),
            child(variant = Menu, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenu, boxed, qname = "mso14:dynamicMenu"),
            child(variant = SplitButton, boxed, qname = "mso14:splitButton"),
            child(variant = Box, boxed, qname = "mso14:box"),
            child(variant = ButtonGroup, boxed, qname = "mso14:buttonGroup"),
            child(variant = Separator, boxed, qname = "mso14:separator")
        )
    )]
  pub group_choice: Vec<GroupChoice>,
  /// Defines the DialogBoxLauncher Class.
  #[sdk(child(qname = "mso14:dialogBoxLauncher"))]
  pub dialog_box_launcher: Option<std::boxed::Box<DialogBoxLauncher>>,
}
/// Defines the ControlCloneQat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:control")]
pub struct ControlCloneQat {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<SizeValues>,
  /// getSize
  #[sdk(attr(qname = ":getSize"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_size: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// showImage
  #[sdk(attr(qname = ":showImage"))]
  pub show_image: Option<crate::simple_type::BooleanValue>,
  /// getShowImage
  #[sdk(attr(qname = ":getShowImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_image: Option<crate::simple_type::StringValue>,
}
/// Defines the SharedControlsQatItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:sharedControls")]
pub struct SharedControlsQatItems {
  #[sdk(
        choice(
            child(variant = ControlCloneQat, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = Separator, boxed, qname = "mso14:separator")
        )
    )]
  pub shared_controls_qat_items_choice: Vec<SharedControlsQatItemsChoice>,
}
/// Defines the DocumentControlsQatItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:documentControls")]
pub struct DocumentControlsQatItems {
  #[sdk(
        choice(
            child(variant = ControlCloneQat, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = Separator, boxed, qname = "mso14:separator")
        )
    )]
  pub document_controls_qat_items_choice: Vec<DocumentControlsQatItemsChoice>,
}
/// Defines the Tab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:tab")]
pub struct Tab {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the Group Class.
  #[sdk(child(qname = "mso14:group"))]
  pub group: Vec<Group>,
}
/// Defines the TabSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:tabSet")]
pub struct TabSet {
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: crate::simple_type::StringValue,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// Defines the Tab Class.
  #[sdk(child(qname = "mso14:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the Command Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:command")]
pub struct Command {
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
}
/// Defines the QuickAccessToolbar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:qat")]
pub struct QuickAccessToolbar {
  /// Defines the SharedControlsQatItems Class.
  #[sdk(child(qname = "mso14:sharedControls"))]
  pub shared_controls_qat_items: Option<SharedControlsQatItems>,
  /// Defines the DocumentControlsQatItems Class.
  #[sdk(child(qname = "mso14:documentControls"))]
  pub document_controls_qat_items: Option<DocumentControlsQatItems>,
}
/// Defines the Tabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:tabs")]
pub struct Tabs {
  /// Defines the Tab Class.
  #[sdk(child(qname = "mso14:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the ContextualTabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:contextualTabs")]
pub struct ContextualTabs {
  /// Defines the TabSet Class.
  #[sdk(child(qname = "mso14:tabSet"))]
  pub tab_set: Vec<TabSet>,
}
/// Defines the ContextMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:contextMenu")]
pub struct ContextMenu {
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, boxed, qname = "mso14:control"),
            child(variant = ButtonRegular, boxed, qname = "mso14:button"),
            child(variant = CheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, boxed, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, boxed, qname = "mso14:toggleButton"),
            child(variant = SplitButtonRegular, boxed, qname = "mso14:splitButton"),
            child(variant = MenuRegular, boxed, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, boxed, qname = "mso14:dynamicMenu"),
            child(variant = MenuSeparatorNoTitle, boxed, qname = "mso14:menuSeparator")
        )
    )]
  pub context_menu_choice: Vec<ContextMenuChoice>,
}
/// Defines the ItemBackstageItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:item")]
pub struct ItemBackstageItem {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the RadioButtonBackstageItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:radioButton")]
pub struct RadioButtonBackstageItem {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstageRegularButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct BackstageRegularButton {
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the BackstagePrimaryMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menu")]
pub struct BackstagePrimaryMenu {
  /// screentip
  #[sdk(attr(qname = ":screentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub screentip: Option<crate::simple_type::StringValue>,
  /// getScreentip
  #[sdk(attr(qname = ":getScreentip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_screentip: Option<crate::simple_type::StringValue>,
  /// supertip
  #[sdk(attr(qname = ":supertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub supertip: Option<crate::simple_type::StringValue>,
  /// getSupertip
  #[sdk(attr(qname = ":getSupertip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_supertip: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageMenuGroup Class.
  #[sdk(child(qname = "mso14:menuGroup"))]
  pub backstage_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:menuGroup")]
pub struct BackstageMenuGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// itemSize
  #[sdk(attr(qname = ":itemSize"))]
  pub item_size: Option<ItemSizeValues>,
  #[sdk(
        choice(
            child(variant = BackstageMenuButton, boxed, qname = "mso14:button"),
            child(variant = BackstageMenuCheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = BackstageSubMenu, boxed, qname = "mso14:menu"),
            child(
                variant = BackstageMenuToggleButton,
                boxed,
                qname = "mso14:toggleButton"
            )
        )
    )]
  pub backstage_menu_group_choice: Vec<BackstageMenuGroupChoice>,
}
/// Defines the PrimaryItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:primaryItem")]
pub struct PrimaryItem {
  #[sdk(
        choice(
            child(variant = BackstageRegularButton, boxed, qname = "mso14:button"),
            child(variant = BackstagePrimaryMenu, boxed, qname = "mso14:menu")
        )
    )]
  pub primary_item_choice: Option<PrimaryItemChoice>,
}
/// Defines the TopItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:topItems")]
pub struct TopItemsGroupControls {
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, boxed, qname = "mso14:button"),
            child(variant = BackstageCheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, boxed, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, boxed, qname = "mso14:dropDown"),
            child(variant = RadioGroup, boxed, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = Hyperlink, boxed, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = GroupBox, boxed, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, boxed, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, boxed, qname = "mso14:imageControl")
        )
    )]
  pub top_items_group_controls_choice: Vec<TopItemsGroupControlsChoice>,
}
/// Defines the BottomItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:bottomItems")]
pub struct BottomItemsGroupControls {
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, boxed, qname = "mso14:button"),
            child(variant = BackstageCheckBox, boxed, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, boxed, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, boxed, qname = "mso14:dropDown"),
            child(variant = RadioGroup, boxed, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, boxed, qname = "mso14:comboBox"),
            child(variant = Hyperlink, boxed, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, boxed, qname = "mso14:labelControl"),
            child(variant = GroupBox, boxed, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, boxed, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, boxed, qname = "mso14:imageControl")
        )
    )]
  pub bottom_items_group_controls_choice: Vec<BottomItemsGroupControlsChoice>,
}
/// Defines the TaskGroupCategory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:category")]
pub struct TaskGroupCategory {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// Defines the TaskGroupTask Class.
  #[sdk(child(qname = "mso14:task"))]
  pub task_group_task: Vec<TaskGroupTask>,
}
/// Defines the TaskGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:task")]
pub struct TaskGroupTask {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
}
/// Defines the TaskFormGroupCategory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:category")]
pub struct TaskFormGroupCategory {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// Defines the TaskFormGroupTask Class.
  #[sdk(child(qname = "mso14:task"))]
  pub task_form_group_task: Vec<TaskFormGroupTask>,
}
/// Defines the TaskFormGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:task")]
pub struct TaskFormGroupTask {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub description: Option<crate::simple_type::StringValue>,
  /// getDescription
  #[sdk(attr(qname = ":getDescription"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_description: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// Defines the BackstageGroup Class.
  #[sdk(child(qname = "mso14:group"))]
  pub backstage_group: Vec<BackstageGroup>,
}
/// Defines the TaskFormGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:taskFormGroup")]
pub struct TaskFormGroup {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// helperText
  #[sdk(attr(qname = ":helperText"))]
  #[sdk(string_length(min = 1u32, max = 4096u32))]
  pub helper_text: Option<crate::simple_type::StringValue>,
  /// getHelperText
  #[sdk(attr(qname = ":getHelperText"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_helper_text: Option<crate::simple_type::StringValue>,
  /// showLabel
  #[sdk(attr(qname = ":showLabel"))]
  pub show_label: Option<crate::simple_type::BooleanValue>,
  /// getShowLabel
  #[sdk(attr(qname = ":getShowLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_show_label: Option<crate::simple_type::StringValue>,
  /// allowedTaskSizes
  #[sdk(attr(qname = ":allowedTaskSizes"))]
  pub allowed_task_sizes: Option<TaskSizesValues>,
  /// Defines the TaskFormGroupCategory Class.
  #[sdk(child(qname = "mso14:category"))]
  pub task_form_group_category: Vec<TaskFormGroupCategory>,
}
/// Defines the BackstageGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:firstColumn")]
pub struct BackstageGroups {
  #[sdk(
        choice(
            child(variant = TaskFormGroup, boxed, qname = "mso14:taskFormGroup"),
            child(variant = BackstageGroup, boxed, qname = "mso14:group"),
            child(variant = TaskGroup, boxed, qname = "mso14:taskGroup")
        )
    )]
  pub backstage_groups_choice: Option<BackstageGroupsChoice>,
}
/// Defines the SimpleGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:secondColumn")]
pub struct SimpleGroups {
  #[sdk(
        choice(
            child(variant = BackstageGroup, boxed, qname = "mso14:group"),
            child(variant = TaskGroup, boxed, qname = "mso14:taskGroup")
        )
    )]
  pub simple_groups_choice: Vec<SimpleGroupsChoice>,
}
/// Defines the BackstageTab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:tab")]
pub struct BackstageTab {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
  /// columnWidthPercent
  #[sdk(attr(qname = ":columnWidthPercent"))]
  #[sdk(number_range(range = 1..= 99))]
  #[sdk(number_sign(kind = "positive"))]
  pub column_width_percent: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMinWidth
  #[sdk(attr(qname = ":firstColumnMinWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub first_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// firstColumnMaxWidth
  #[sdk(attr(qname = ":firstColumnMaxWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub first_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMinWidth
  #[sdk(attr(qname = ":secondColumnMinWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub second_column_min_width: Option<crate::simple_type::IntegerValue>,
  /// secondColumnMaxWidth
  #[sdk(attr(qname = ":secondColumnMaxWidth"))]
  #[sdk(number_range(range = 1..= 10000))]
  #[sdk(number_sign(kind = "positive"))]
  pub second_column_max_width: Option<crate::simple_type::IntegerValue>,
  /// Defines the BackstageGroups Class.
  #[sdk(child(qname = "mso14:firstColumn"))]
  pub backstage_groups: Option<std::boxed::Box<BackstageGroups>>,
  /// Defines the SimpleGroups Class.
  #[sdk(child(qname = "mso14:secondColumn"))]
  pub simple_groups: Option<SimpleGroups>,
}
/// Defines the BackstageFastCommandButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:button")]
pub struct BackstageFastCommandButton {
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterMso
  #[sdk(attr(qname = ":insertAfterMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_after_mso: Option<crate::simple_type::StringValue>,
  /// insertBeforeMso
  #[sdk(attr(qname = ":insertBeforeMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub insert_before_mso: Option<crate::simple_type::StringValue>,
  /// insertAfterQ
  #[sdk(attr(qname = ":insertAfterQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_after_qulified_id: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_qulified_id: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  #[sdk(string_format(kind = "id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// idQ
  #[sdk(attr(qname = ":idQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub qualified_id: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// onAction
  #[sdk(attr(qname = ":onAction"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_action: Option<crate::simple_type::StringValue>,
  /// isDefinitive
  #[sdk(attr(qname = ":isDefinitive"))]
  pub is_definitive: Option<crate::simple_type::BooleanValue>,
  /// enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// getEnabled
  #[sdk(attr(qname = ":getEnabled"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_enabled: Option<crate::simple_type::StringValue>,
  /// label
  #[sdk(attr(qname = ":label"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub label: Option<crate::simple_type::StringValue>,
  /// getLabel
  #[sdk(attr(qname = ":getLabel"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_label: Option<crate::simple_type::StringValue>,
  /// visible
  #[sdk(attr(qname = ":visible"))]
  pub visible: Option<crate::simple_type::BooleanValue>,
  /// getVisible
  #[sdk(attr(qname = ":getVisible"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_visible: Option<crate::simple_type::StringValue>,
  /// keytip
  #[sdk(attr(qname = ":keytip"))]
  #[sdk(string_length(min = 1u32, max = 3u32))]
  #[sdk(string_format(kind = "token"))]
  pub keytip: Option<crate::simple_type::StringValue>,
  /// getKeytip
  #[sdk(attr(qname = ":getKeytip"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_keytip: Option<crate::simple_type::StringValue>,
  /// image
  #[sdk(attr(qname = ":image"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub image: Option<crate::simple_type::StringValue>,
  /// imageMso
  #[sdk(attr(qname = ":imageMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub image_mso: Option<crate::simple_type::StringValue>,
  /// getImage
  #[sdk(attr(qname = ":getImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_image: Option<crate::simple_type::StringValue>,
}
/// Defines the Commands Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:commands")]
pub struct Commands {
  /// Defines the Command Class.
  #[sdk(child(qname = "mso14:command"))]
  pub command: Vec<Command>,
}
/// Defines the Ribbon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:ribbon")]
pub struct Ribbon {
  /// startFromScratch
  #[sdk(attr(qname = ":startFromScratch"))]
  pub start_from_scratch: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "mso14:qat"))]
  pub quick_access_toolbar: Option<std::boxed::Box<QuickAccessToolbar>>,
  /// _
  #[sdk(child(qname = "mso14:tabs"))]
  pub tabs: Option<Tabs>,
  /// _
  #[sdk(child(qname = "mso14:contextualTabs"))]
  pub contextual_tabs: Option<ContextualTabs>,
}
/// Defines the Backstage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:backstage")]
pub struct Backstage {
  /// onShow
  #[sdk(attr(qname = ":onShow"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_show: Option<crate::simple_type::StringValue>,
  /// onHide
  #[sdk(attr(qname = ":onHide"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_hide: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = BackstageTab, boxed, qname = "mso14:tab"),
            child(variant = BackstageFastCommandButton, boxed, qname = "mso14:button")
        )
    )]
  pub backstage_choice: Vec<BackstageChoice>,
}
/// Defines the ContextMenus Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso14:contextMenus")]
pub struct ContextMenus {
  /// Defines the ContextMenu Class.
  #[sdk(child(qname = "mso14:contextMenu"))]
  pub context_menu: Vec<ContextMenu>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SplitButtonRegularChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuRegularChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  SplitButtonRegular(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  MenuRegular(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SplitButtonWithTitleChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuWithTitleChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonWithTitle Class.
  SplitButtonWithTitle(std::boxed::Box<SplitButtonWithTitle>),
  /// Defines the MenuWithTitle Class.
  MenuWithTitle(std::boxed::Box<MenuWithTitle>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  SplitButtonRegular(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  MenuRegular(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SplitButtonChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BoxChoice {
  /// Defines the ControlClone Class.
  ControlClone(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  DropDownRegular(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  ButtonGroup(std::boxed::Box<ButtonGroup>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonGroupChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the MenuRegular Class.
  MenuRegular(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
  /// Defines the SplitButtonRegular Class.
  SplitButtonRegular(std::boxed::Box<SplitButtonRegular>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupBoxChoice {
  /// Defines the BackstageGroupButton Class.
  BackstageGroupButton(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  BackstageCheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  BackstageEditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  BackstageDropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  BackstageComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  BackstageLabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutContainerChoice {
  /// Defines the BackstageGroupButton Class.
  BackstageGroupButton(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  BackstageCheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  BackstageEditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  BackstageDropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  BackstageComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  BackstageLabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackstageGroupChoice {
  /// Defines the PrimaryItem Class.
  PrimaryItem(std::boxed::Box<PrimaryItem>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuRootChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonRegular Class.
  SplitButtonRegular(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  MenuRegular(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupChoice {
  /// Defines the ControlClone Class.
  ControlClone(std::boxed::Box<ControlClone>),
  /// Defines the LabelControl Class.
  LabelControl(std::boxed::Box<LabelControl>),
  /// Defines the Button Class.
  Button(std::boxed::Box<Button>),
  /// Defines the ToggleButton Class.
  ToggleButton(std::boxed::Box<ToggleButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the EditBox Class.
  EditBox(std::boxed::Box<EditBox>),
  /// Defines the ComboBox Class.
  ComboBox(std::boxed::Box<ComboBox>),
  /// Defines the DropDownRegular Class.
  DropDownRegular(std::boxed::Box<DropDownRegular>),
  /// Defines the Gallery Class.
  Gallery(std::boxed::Box<Gallery>),
  /// Defines the Menu Class.
  Menu(std::boxed::Box<Menu>),
  /// Defines the DynamicMenu Class.
  DynamicMenu(std::boxed::Box<DynamicMenu>),
  /// Defines the SplitButton Class.
  SplitButton(std::boxed::Box<SplitButton>),
  /// Defines the Box Class.
  Box(std::boxed::Box<Box>),
  /// Defines the ButtonGroup Class.
  ButtonGroup(std::boxed::Box<ButtonGroup>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SharedControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  ControlCloneQat(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  ControlCloneQat(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ContextMenuChoice {
  /// Defines the ControlCloneRegular Class.
  ControlCloneRegular(std::boxed::Box<ControlCloneRegular>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the GalleryRegular Class.
  GalleryRegular(std::boxed::Box<GalleryRegular>),
  /// Defines the ToggleButtonRegular Class.
  ToggleButtonRegular(std::boxed::Box<ToggleButtonRegular>),
  /// Defines the SplitButtonRegular Class.
  SplitButtonRegular(std::boxed::Box<SplitButtonRegular>),
  /// Defines the MenuRegular Class.
  MenuRegular(std::boxed::Box<MenuRegular>),
  /// Defines the DynamicMenuRegular Class.
  DynamicMenuRegular(std::boxed::Box<DynamicMenuRegular>),
  /// Defines the MenuSeparatorNoTitle Class.
  MenuSeparatorNoTitle(std::boxed::Box<MenuSeparatorNoTitle>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackstageMenuGroupChoice {
  /// Defines the BackstageMenuButton Class.
  BackstageMenuButton(std::boxed::Box<BackstageMenuButton>),
  /// Defines the BackstageMenuCheckBox Class.
  BackstageMenuCheckBox(std::boxed::Box<BackstageMenuCheckBox>),
  /// Defines the BackstageSubMenu Class.
  BackstageSubMenu(std::boxed::Box<BackstageSubMenu>),
  /// Defines the BackstageMenuToggleButton Class.
  BackstageMenuToggleButton(std::boxed::Box<BackstageMenuToggleButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryItemChoice {
  /// Defines the BackstageRegularButton Class.
  BackstageRegularButton(std::boxed::Box<BackstageRegularButton>),
  /// Defines the BackstagePrimaryMenu Class.
  BackstagePrimaryMenu(std::boxed::Box<BackstagePrimaryMenu>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopItemsGroupControlsChoice {
  /// Defines the BackstageGroupButton Class.
  BackstageGroupButton(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  BackstageCheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  BackstageEditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  BackstageDropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  BackstageComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  BackstageLabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomItemsGroupControlsChoice {
  /// Defines the BackstageGroupButton Class.
  BackstageGroupButton(std::boxed::Box<BackstageGroupButton>),
  /// Defines the BackstageCheckBox Class.
  BackstageCheckBox(std::boxed::Box<BackstageCheckBox>),
  /// Defines the BackstageEditBox Class.
  BackstageEditBox(std::boxed::Box<BackstageEditBox>),
  /// Defines the BackstageDropDown Class.
  BackstageDropDown(std::boxed::Box<BackstageDropDown>),
  /// Defines the RadioGroup Class.
  RadioGroup(std::boxed::Box<RadioGroup>),
  /// Defines the BackstageComboBox Class.
  BackstageComboBox(std::boxed::Box<BackstageComboBox>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the BackstageLabelControl Class.
  BackstageLabelControl(std::boxed::Box<BackstageLabelControl>),
  /// Defines the GroupBox Class.
  GroupBox(std::boxed::Box<GroupBox>),
  /// Defines the LayoutContainer Class.
  LayoutContainer(std::boxed::Box<LayoutContainer>),
  /// Defines the ImageControl Class.
  ImageControl(std::boxed::Box<ImageControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackstageGroupsChoice {
  /// Defines the TaskFormGroup Class.
  TaskFormGroup(std::boxed::Box<TaskFormGroup>),
  /// Defines the BackstageGroup Class.
  BackstageGroup(std::boxed::Box<BackstageGroup>),
  /// Defines the TaskGroup Class.
  TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleGroupsChoice {
  /// Defines the BackstageGroup Class.
  BackstageGroup(std::boxed::Box<BackstageGroup>),
  /// Defines the TaskGroup Class.
  TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackstageChoice {
  /// Defines the BackstageTab Class.
  BackstageTab(std::boxed::Box<BackstageTab>),
  /// Defines the BackstageFastCommandButton Class.
  BackstageFastCommandButton(std::boxed::Box<BackstageFastCommandButton>),
}
