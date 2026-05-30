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
#[sdk(office2010, qname = "mso14:control")]
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
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:checkBox")]
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
#[sdk(office2010, qname = "mso14:gallery")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the ToggleButtonRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:toggleButton")]
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
#[sdk(office2010, qname = "mso14:menuSeparator")]
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
#[sdk(office2010, qname = "mso14:splitButton")]
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
  #[sdk(
        choice(
            child(variant = VisibleButton, qname = "mso14:button"),
            child(variant = VisibleToggleButton, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_regular_choice: Option<SplitButtonRegularChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the MenuRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, qname = "mso14:splitButton"),
            child(variant = MenuRegular, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_regular_choice: Vec<MenuRegularChoice>,
}
/// Defines the DynamicMenuRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:dynamicMenu")]
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
#[sdk(office2010, qname = "mso14:splitButton")]
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
  #[sdk(
        choice(
            child(variant = VisibleButton, qname = "mso14:button"),
            child(variant = VisibleToggleButton, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_with_title_choice: Option<SplitButtonWithTitleChoice>,
  /// Defines the MenuWithTitle Class.
  #[sdk(child(office2010, qname = "mso14:menu"))]
  pub menu_with_title: Option<MenuWithTitle>,
}
/// Defines the MenuWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonWithTitle, qname = "mso14:splitButton"),
            child(variant = MenuWithTitle, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_with_title_choice: Vec<MenuWithTitleChoice>,
}
/// Defines the MenuSeparatorNoTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menuSeparator")]
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
#[sdk(office2010, qname = "mso14:control")]
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
#[sdk(office2010, qname = "mso14:labelControl")]
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
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:toggleButton")]
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
#[sdk(office2010, qname = "mso14:editBox")]
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
#[sdk(office2010, qname = "mso14:comboBox")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item: Vec<Item>,
}
/// Defines the DropDownRegular Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:dropDown")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Gallery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:gallery")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item: Vec<Item>,
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:button"))]
  pub button_regular: Vec<ButtonRegular>,
}
/// Defines the Menu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, qname = "mso14:splitButton"),
            child(variant = MenuRegular, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_choice: Vec<MenuChoice>,
}
/// Defines the DynamicMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:dynamicMenu")]
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
#[sdk(office2010, qname = "mso14:splitButton")]
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
  #[sdk(
        choice(
            child(variant = VisibleButton, qname = "mso14:button"),
            child(variant = VisibleToggleButton, qname = "mso14:toggleButton")
        )
    )]
  pub split_button_choice: Option<SplitButtonChoice>,
  /// Defines the MenuRegular Class.
  #[sdk(child(office2010, qname = "mso14:menu"))]
  pub menu_regular: Option<MenuRegular>,
}
/// Defines the Box Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:box")]
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
  #[sdk(
        choice(
            child(variant = ControlClone, qname = "mso14:control"),
            child(variant = LabelControl, qname = "mso14:labelControl"),
            child(variant = Button, qname = "mso14:button"),
            child(variant = ToggleButton, qname = "mso14:toggleButton"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = EditBox, qname = "mso14:editBox"),
            child(variant = ComboBox, qname = "mso14:comboBox"),
            child(variant = DropDownRegular, qname = "mso14:dropDown"),
            child(variant = Gallery, qname = "mso14:gallery"),
            child(variant = Menu, qname = "mso14:menu"),
            child(variant = DynamicMenu, qname = "mso14:dynamicMenu"),
            child(variant = SplitButton, qname = "mso14:splitButton"),
            child(variant = Box, qname = "mso14:box"),
            child(variant = ButtonGroup, qname = "mso14:buttonGroup")
        )
    )]
  pub box_choice: Vec<BoxChoice>,
}
/// Defines the ButtonGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:buttonGroup")]
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
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = MenuRegular, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu"),
            child(variant = SplitButtonRegular, qname = "mso14:splitButton"),
            child(variant = Separator, qname = "mso14:separator")
        )
    )]
  pub button_group_choice: Vec<ButtonGroupChoice>,
}
/// Defines the BackstageMenuButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:checkBox")]
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
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(child(office2010, qname = "mso14:menuGroup"))]
  pub backstage_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:toggleButton")]
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
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:checkBox")]
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
#[sdk(office2010, qname = "mso14:editBox")]
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
#[sdk(office2010, qname = "mso14:dropDown")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item_backstage_item: Vec<ItemBackstageItem>,
}
/// Defines the RadioGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:radioGroup")]
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
  #[sdk(child(office2010, qname = "mso14:radioButton"))]
  pub radio_button_backstage_item: Vec<RadioButtonBackstageItem>,
}
/// Defines the BackstageComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:comboBox")]
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
  #[sdk(child(office2010, qname = "mso14:item"))]
  pub item_backstage_item: Vec<ItemBackstageItem>,
}
/// Defines the Hyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:hyperlink")]
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
#[sdk(office2010, qname = "mso14:labelControl")]
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
#[sdk(office2010, qname = "mso14:groupBox")]
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
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, qname = "mso14:button"),
            child(variant = BackstageCheckBox, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, qname = "mso14:dropDown"),
            child(variant = RadioGroup, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, qname = "mso14:comboBox"),
            child(variant = Hyperlink, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, qname = "mso14:labelControl"),
            child(variant = GroupBox, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, qname = "mso14:imageControl")
        )
    )]
  pub group_box_choice: Vec<GroupBoxChoice>,
}
/// Defines the LayoutContainer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:layoutContainer")]
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
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, qname = "mso14:button"),
            child(variant = BackstageCheckBox, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, qname = "mso14:dropDown"),
            child(variant = RadioGroup, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, qname = "mso14:comboBox"),
            child(variant = Hyperlink, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, qname = "mso14:labelControl"),
            child(variant = GroupBox, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, qname = "mso14:imageControl")
        )
    )]
  pub layout_container_choice: Vec<LayoutContainerChoice>,
}
/// Defines the ImageControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:imageControl")]
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
#[sdk(office2010, qname = "mso14:group")]
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
  #[sdk(choice(child(variant = PrimaryItem, qname = "mso14:primaryItem")))]
  pub backstage_group_choice: Option<BackstageGroupChoice>,
  /// Defines the TopItemsGroupControls Class.
  #[sdk(child(office2010, qname = "mso14:topItems"))]
  pub top_items_group_controls: Option<TopItemsGroupControls>,
  /// Defines the BottomItemsGroupControls Class.
  #[sdk(child(office2010, qname = "mso14:bottomItems"))]
  pub bottom_items_group_controls: Option<BottomItemsGroupControls>,
}
/// Defines the TaskGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:taskGroup")]
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
  #[sdk(child(office2010, qname = "mso14:category"))]
  pub task_group_category: Vec<TaskGroupCategory>,
}
/// Defines the MenuRoot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = MenuSeparator, qname = "mso14:menuSeparator"),
            child(variant = SplitButtonRegular, qname = "mso14:splitButton"),
            child(variant = MenuRegular, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu")
        )
    )]
  pub menu_root_choice: Vec<MenuRootChoice>,
}
/// Defines the CustomUI Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:customUI")]
pub struct CustomUi {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// onLoad
  #[sdk(attr(office2010, qname = ":onLoad"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_load: Option<crate::simple_type::StringValue>,
  /// loadImage
  #[sdk(attr(office2010, qname = ":loadImage"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub load_image: Option<crate::simple_type::StringValue>,
  /// Defines the Commands Class.
  #[sdk(child(office2010, qname = "mso14:commands"))]
  pub commands: Option<Commands>,
  /// Defines the Ribbon Class.
  #[sdk(child(office2010, qname = "mso14:ribbon"))]
  pub ribbon: Option<std::boxed::Box<Ribbon>>,
  /// Defines the Backstage Class.
  #[sdk(child(office2010, qname = "mso14:backstage"))]
  pub backstage: Option<Backstage>,
  /// Defines the ContextMenus Class.
  #[sdk(child(office2010, qname = "mso14:contextMenus"))]
  pub context_menus: Option<ContextMenus>,
}
/// Defines the Item Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:item")]
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
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:toggleButton")]
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
#[sdk(office2010, qname = "mso14:separator")]
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
#[sdk(office2010, qname = "mso14:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
  /// Defines the ButtonRegular Class.
  #[sdk(child(office2010, qname = "mso14:button"))]
  pub button_regular: std::boxed::Box<ButtonRegular>,
}
/// Defines the Group Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:group")]
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
  #[sdk(
        choice(
            child(variant = ControlClone, qname = "mso14:control"),
            child(variant = LabelControl, qname = "mso14:labelControl"),
            child(variant = Button, qname = "mso14:button"),
            child(variant = ToggleButton, qname = "mso14:toggleButton"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = EditBox, qname = "mso14:editBox"),
            child(variant = ComboBox, qname = "mso14:comboBox"),
            child(variant = DropDownRegular, qname = "mso14:dropDown"),
            child(variant = Gallery, qname = "mso14:gallery"),
            child(variant = Menu, qname = "mso14:menu"),
            child(variant = DynamicMenu, qname = "mso14:dynamicMenu"),
            child(variant = SplitButton, qname = "mso14:splitButton"),
            child(variant = Box, qname = "mso14:box"),
            child(variant = ButtonGroup, qname = "mso14:buttonGroup"),
            child(variant = Separator, qname = "mso14:separator")
        )
    )]
  pub group_choice: Vec<GroupChoice>,
  /// Defines the DialogBoxLauncher Class.
  #[sdk(child(office2010, qname = "mso14:dialogBoxLauncher"))]
  pub dialog_box_launcher: Option<std::boxed::Box<DialogBoxLauncher>>,
}
/// Defines the ControlCloneQat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:control")]
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
#[sdk(office2010, qname = "mso14:sharedControls")]
pub struct SharedControlsQatItems {
  #[sdk(
        choice(
            child(variant = ControlCloneQat, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = Separator, qname = "mso14:separator")
        )
    )]
  pub shared_controls_qat_items_choice: Vec<SharedControlsQatItemsChoice>,
}
/// Defines the DocumentControlsQatItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:documentControls")]
pub struct DocumentControlsQatItems {
  #[sdk(
        choice(
            child(variant = ControlCloneQat, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = Separator, qname = "mso14:separator")
        )
    )]
  pub document_controls_qat_items_choice: Vec<DocumentControlsQatItemsChoice>,
}
/// Defines the Tab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:tab")]
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
  #[sdk(child(office2010, qname = "mso14:group"))]
  pub group: Vec<Group>,
}
/// Defines the TabSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:tabSet")]
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
  #[sdk(child(office2010, qname = "mso14:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the Command Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:command")]
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
#[sdk(office2010, qname = "mso14:qat")]
pub struct QuickAccessToolbar {
  /// Defines the SharedControlsQatItems Class.
  #[sdk(child(office2010, qname = "mso14:sharedControls"))]
  pub shared_controls_qat_items: Option<SharedControlsQatItems>,
  /// Defines the DocumentControlsQatItems Class.
  #[sdk(child(office2010, qname = "mso14:documentControls"))]
  pub document_controls_qat_items: Option<DocumentControlsQatItems>,
}
/// Defines the Tabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:tabs")]
pub struct Tabs {
  /// Defines the Tab Class.
  #[sdk(child(office2010, qname = "mso14:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the ContextualTabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:contextualTabs")]
pub struct ContextualTabs {
  /// Defines the TabSet Class.
  #[sdk(child(office2010, qname = "mso14:tabSet"))]
  pub tab_set: Vec<TabSet>,
}
/// Defines the ContextMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:contextMenu")]
pub struct ContextMenu {
  /// idMso
  #[sdk(attr(office2010, qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ControlCloneRegular, qname = "mso14:control"),
            child(variant = ButtonRegular, qname = "mso14:button"),
            child(variant = CheckBox, qname = "mso14:checkBox"),
            child(variant = GalleryRegular, qname = "mso14:gallery"),
            child(variant = ToggleButtonRegular, qname = "mso14:toggleButton"),
            child(variant = SplitButtonRegular, qname = "mso14:splitButton"),
            child(variant = MenuRegular, qname = "mso14:menu"),
            child(variant = DynamicMenuRegular, qname = "mso14:dynamicMenu"),
            child(variant = MenuSeparatorNoTitle, qname = "mso14:menuSeparator")
        )
    )]
  pub context_menu_choice: Vec<ContextMenuChoice>,
}
/// Defines the ItemBackstageItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:item")]
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
#[sdk(office2010, qname = "mso14:radioButton")]
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
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:menu")]
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
  #[sdk(child(office2010, qname = "mso14:menuGroup"))]
  pub backstage_menu_group: Vec<BackstageMenuGroup>,
}
/// Defines the BackstageMenuGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:menuGroup")]
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
  #[sdk(
        choice(
            child(variant = BackstageMenuButton, qname = "mso14:button"),
            child(variant = BackstageMenuCheckBox, qname = "mso14:checkBox"),
            child(variant = BackstageSubMenu, qname = "mso14:menu"),
            child(variant = BackstageMenuToggleButton, qname = "mso14:toggleButton")
        )
    )]
  pub backstage_menu_group_choice: Vec<BackstageMenuGroupChoice>,
}
/// Defines the PrimaryItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:primaryItem")]
pub struct PrimaryItem {
  #[sdk(
        choice(
            child(variant = BackstageRegularButton, qname = "mso14:button"),
            child(variant = BackstagePrimaryMenu, qname = "mso14:menu")
        )
    )]
  pub primary_item_choice: Option<PrimaryItemChoice>,
}
/// Defines the TopItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:topItems")]
pub struct TopItemsGroupControls {
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, qname = "mso14:button"),
            child(variant = BackstageCheckBox, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, qname = "mso14:dropDown"),
            child(variant = RadioGroup, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, qname = "mso14:comboBox"),
            child(variant = Hyperlink, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, qname = "mso14:labelControl"),
            child(variant = GroupBox, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, qname = "mso14:imageControl")
        )
    )]
  pub top_items_group_controls_choice: Vec<TopItemsGroupControlsChoice>,
}
/// Defines the BottomItemsGroupControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:bottomItems")]
pub struct BottomItemsGroupControls {
  #[sdk(
        choice(
            child(variant = BackstageGroupButton, qname = "mso14:button"),
            child(variant = BackstageCheckBox, qname = "mso14:checkBox"),
            child(variant = BackstageEditBox, qname = "mso14:editBox"),
            child(variant = BackstageDropDown, qname = "mso14:dropDown"),
            child(variant = RadioGroup, qname = "mso14:radioGroup"),
            child(variant = BackstageComboBox, qname = "mso14:comboBox"),
            child(variant = Hyperlink, qname = "mso14:hyperlink"),
            child(variant = BackstageLabelControl, qname = "mso14:labelControl"),
            child(variant = GroupBox, qname = "mso14:groupBox"),
            child(variant = LayoutContainer, qname = "mso14:layoutContainer"),
            child(variant = ImageControl, qname = "mso14:imageControl")
        )
    )]
  pub bottom_items_group_controls_choice: Vec<BottomItemsGroupControlsChoice>,
}
/// Defines the TaskGroupCategory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:category")]
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
  #[sdk(child(office2010, qname = "mso14:task"))]
  pub task_group_task: Vec<TaskGroupTask>,
}
/// Defines the TaskGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:task")]
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
#[sdk(office2010, qname = "mso14:category")]
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
  #[sdk(child(office2010, qname = "mso14:task"))]
  pub task_form_group_task: Vec<TaskFormGroupTask>,
}
/// Defines the TaskFormGroupTask Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:task")]
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
  #[sdk(child(office2010, qname = "mso14:group"))]
  pub backstage_group: Vec<BackstageGroup>,
}
/// Defines the TaskFormGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:taskFormGroup")]
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
  #[sdk(child(office2010, qname = "mso14:category"))]
  pub task_form_group_category: Vec<TaskFormGroupCategory>,
}
/// Defines the BackstageGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:firstColumn")]
pub struct BackstageGroups {
  #[sdk(
        choice(
            child(variant = TaskFormGroup, qname = "mso14:taskFormGroup"),
            child(variant = BackstageGroup, qname = "mso14:group"),
            child(variant = TaskGroup, qname = "mso14:taskGroup")
        )
    )]
  pub backstage_groups_choice: Option<BackstageGroupsChoice>,
}
/// Defines the SimpleGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:secondColumn")]
pub struct SimpleGroups {
  #[sdk(
        choice(
            child(variant = BackstageGroup, qname = "mso14:group"),
            child(variant = TaskGroup, qname = "mso14:taskGroup")
        )
    )]
  pub simple_groups_choice: Vec<SimpleGroupsChoice>,
}
/// Defines the BackstageTab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:tab")]
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
  #[sdk(child(office2010, qname = "mso14:firstColumn"))]
  pub backstage_groups: Option<std::boxed::Box<BackstageGroups>>,
  /// Defines the SimpleGroups Class.
  #[sdk(child(office2010, qname = "mso14:secondColumn"))]
  pub simple_groups: Option<SimpleGroups>,
}
/// Defines the BackstageFastCommandButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:button")]
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
#[sdk(office2010, qname = "mso14:commands")]
pub struct Commands {
  /// Defines the Command Class.
  #[sdk(child(office2010, qname = "mso14:command"))]
  pub command: Vec<Command>,
}
/// Defines the Ribbon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:ribbon")]
pub struct Ribbon {
  /// startFromScratch
  #[sdk(attr(office2010, qname = ":startFromScratch"))]
  pub start_from_scratch: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "mso14:qat"))]
  pub quick_access_toolbar: Option<std::boxed::Box<QuickAccessToolbar>>,
  /// _
  #[sdk(child(office2010, qname = "mso14:tabs"))]
  pub tabs: Option<Tabs>,
  /// _
  #[sdk(child(office2010, qname = "mso14:contextualTabs"))]
  pub contextual_tabs: Option<ContextualTabs>,
}
/// Defines the Backstage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:backstage")]
pub struct Backstage {
  /// onShow
  #[sdk(attr(office2010, qname = ":onShow"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_show: Option<crate::simple_type::StringValue>,
  /// onHide
  #[sdk(attr(office2010, qname = ":onHide"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub on_hide: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = BackstageTab, qname = "mso14:tab"),
            child(variant = BackstageFastCommandButton, qname = "mso14:button")
        )
    )]
  pub backstage_choice: Vec<BackstageChoice>,
}
/// Defines the ContextMenus Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "mso14:contextMenus")]
pub struct ContextMenus {
  /// Defines the ContextMenu Class.
  #[sdk(child(office2010, qname = "mso14:contextMenu"))]
  pub context_menu: Vec<ContextMenu>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonRegularChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonWithTitleChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SplitButtonChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageGroupChoice {
  /// Defines the PrimaryItem Class.
  PrimaryItem(std::boxed::Box<PrimaryItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SharedControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  ControlCloneQat(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DocumentControlsQatItemsChoice {
  /// Defines the ControlCloneQat Class.
  ControlCloneQat(std::boxed::Box<ControlCloneQat>),
  /// Defines the ButtonRegular Class.
  ButtonRegular(std::boxed::Box<ButtonRegular>),
  /// Defines the Separator Class.
  Separator(std::boxed::Box<Separator>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PrimaryItemChoice {
  /// Defines the BackstageRegularButton Class.
  BackstageRegularButton(std::boxed::Box<BackstageRegularButton>),
  /// Defines the BackstagePrimaryMenu Class.
  BackstagePrimaryMenu(std::boxed::Box<BackstagePrimaryMenu>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageGroupsChoice {
  /// Defines the TaskFormGroup Class.
  TaskFormGroup(std::boxed::Box<TaskFormGroup>),
  /// Defines the BackstageGroup Class.
  BackstageGroup(std::boxed::Box<BackstageGroup>),
  /// Defines the TaskGroup Class.
  TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SimpleGroupsChoice {
  /// Defines the BackstageGroup Class.
  BackstageGroup(std::boxed::Box<BackstageGroup>),
  /// Defines the TaskGroup Class.
  TaskGroup(std::boxed::Box<TaskGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackstageChoice {
  /// Defines the BackstageTab Class.
  BackstageTab(std::boxed::Box<BackstageTab>),
  /// Defines the BackstageFastCommandButton Class.
  BackstageFastCommandButton(std::boxed::Box<BackstageFastCommandButton>),
}
