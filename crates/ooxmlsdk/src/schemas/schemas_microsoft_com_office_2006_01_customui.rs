//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

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
/// Defines the UnsizedControlClone Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:control")]
pub struct UnsizedControlClone {
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
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
/// Defines the UnsizedButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:button")]
pub struct UnsizedButton {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:checkBox")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
/// Defines the UnsizedGallery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:gallery")]
pub struct UnsizedGallery {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(child(qname = "mso:item"))]
  pub item: Vec<Item>,
  /// Defines the UnsizedButton Class.
  #[sdk(child(qname = "mso:button"))]
  pub unsized_button: Vec<UnsizedButton>,
}
/// Defines the UnsizedToggleButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:toggleButton")]
pub struct UnsizedToggleButton {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:menuSeparator")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub title: Option<crate::simple_type::StringValue>,
  /// getTitle
  #[sdk(attr(qname = ":getTitle"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub get_title: Option<crate::simple_type::StringValue>,
}
/// Defines the UnsizedSplitButton Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:splitButton")]
pub struct UnsizedSplitButton {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = VisibleButton, qname = "mso:button"),
            child(variant = VisibleToggleButton, qname = "mso:toggleButton")
        )
    )]
  pub unsized_split_button_choice: Option<UnsizedSplitButtonChoice>,
  /// Defines the UnsizedMenu Class.
  #[sdk(child(qname = "mso:menu"))]
  pub unsized_menu: Option<UnsizedMenu>,
}
/// Defines the UnsizedMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:menu")]
pub struct UnsizedMenu {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = MenuSeparator, qname = "mso:menuSeparator"),
            child(variant = UnsizedSplitButton, qname = "mso:splitButton"),
            child(variant = UnsizedMenu, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu")
        )
    )]
  pub unsized_menu_choice: Vec<UnsizedMenuChoice>,
}
/// Defines the UnsizedDynamicMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:dynamicMenu")]
pub struct UnsizedDynamicMenu {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:splitButton")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = VisibleButton, qname = "mso:button"),
            child(variant = VisibleToggleButton, qname = "mso:toggleButton")
        )
    )]
  pub split_button_with_title_choice: Option<SplitButtonWithTitleChoice>,
  /// Defines the MenuWithTitle Class.
  #[sdk(child(qname = "mso:menu"))]
  pub menu_with_title: Option<MenuWithTitle>,
}
/// Defines the MenuWithTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:menu")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = MenuSeparator, qname = "mso:menuSeparator"),
            child(variant = SplitButtonWithTitle, qname = "mso:splitButton"),
            child(variant = MenuWithTitle, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu")
        )
    )]
  pub menu_with_title_choice: Vec<MenuWithTitleChoice>,
}
/// Defines the ControlClone Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:control")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
/// Defines the TextLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:labelControl")]
pub struct TextLabel {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:button")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:toggleButton")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:editBox")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:comboBox")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(child(qname = "mso:item"))]
  pub item: Vec<Item>,
}
/// Defines the DropDown Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:dropDown")]
pub struct DropDown {
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(child(qname = "mso:item"))]
  pub item: Vec<Item>,
  /// Defines the UnsizedButton Class.
  #[sdk(child(qname = "mso:button"))]
  pub unsized_button: Vec<UnsizedButton>,
}
/// Defines the Gallery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:gallery")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(child(qname = "mso:item"))]
  pub item: Vec<Item>,
  /// Defines the UnsizedButton Class.
  #[sdk(child(qname = "mso:button"))]
  pub unsized_button: Vec<UnsizedButton>,
}
/// Defines the Menu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:menu")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = MenuSeparator, qname = "mso:menuSeparator"),
            child(variant = UnsizedSplitButton, qname = "mso:splitButton"),
            child(variant = UnsizedMenu, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu")
        )
    )]
  pub menu_choice: Vec<MenuChoice>,
}
/// Defines the DynamicMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:dynamicMenu")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:splitButton")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
            child(variant = VisibleButton, qname = "mso:button"),
            child(variant = VisibleToggleButton, qname = "mso:toggleButton")
        )
    )]
  pub split_button_choice: Option<SplitButtonChoice>,
  /// Defines the UnsizedMenu Class.
  #[sdk(child(qname = "mso:menu"))]
  pub unsized_menu: Option<UnsizedMenu>,
}
/// Defines the Box Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:box")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
  /// boxStyle
  #[sdk(attr(qname = ":boxStyle"))]
  pub box_style: Option<BoxStyleValues>,
  #[sdk(
        choice(
            child(variant = ControlClone, qname = "mso:control"),
            child(variant = TextLabel, qname = "mso:labelControl"),
            child(variant = Button, qname = "mso:button"),
            child(variant = ToggleButton, qname = "mso:toggleButton"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = EditBox, qname = "mso:editBox"),
            child(variant = ComboBox, qname = "mso:comboBox"),
            child(variant = DropDown, qname = "mso:dropDown"),
            child(variant = Gallery, qname = "mso:gallery"),
            child(variant = Menu, qname = "mso:menu"),
            child(variant = DynamicMenu, qname = "mso:dynamicMenu"),
            child(variant = SplitButton, qname = "mso:splitButton"),
            child(variant = Box, qname = "mso:box"),
            child(variant = ButtonGroup, qname = "mso:buttonGroup")
        )
    )]
  pub box_choice: Vec<BoxChoice>,
}
/// Defines the ButtonGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:buttonGroup")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedMenu, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu"),
            child(variant = UnsizedSplitButton, qname = "mso:splitButton")
        )
    )]
  pub button_group_choice: Vec<ButtonGroupChoice>,
}
/// Defines the MenuRoot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:menu")]
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
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = MenuSeparator, qname = "mso:menuSeparator"),
            child(variant = UnsizedSplitButton, qname = "mso:splitButton"),
            child(variant = UnsizedMenu, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu")
        )
    )]
  pub menu_root_choice: Vec<MenuRootChoice>,
}
/// Defines the CustomUI Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "mso:customUI")]
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
  /// Defines the RepurposedCommands Class.
  #[sdk(child(qname = "mso:commands"))]
  pub repurposed_commands: Option<RepurposedCommands>,
  /// Defines the Ribbon Class.
  #[sdk(child(qname = "mso:ribbon"))]
  pub ribbon: Option<std::boxed::Box<Ribbon>>,
}
/// Defines the Item Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:item")]
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
#[sdk(qname = "mso:button")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
#[sdk(qname = "mso:toggleButton")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  pub tag: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
/// Defines the VerticalSeparator Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:separator")]
pub struct VerticalSeparator {
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
  pub id_q: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
}
/// Defines the DialogBoxLauncher Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:dialogBoxLauncher")]
pub struct DialogBoxLauncher {
  /// Defines the UnsizedButton Class.
  #[sdk(child(qname = "mso:button"))]
  pub unsized_button: std::boxed::Box<UnsizedButton>,
}
/// Defines the Group Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:group")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(
        choice(
            child(variant = ControlClone, qname = "mso:control"),
            child(variant = TextLabel, qname = "mso:labelControl"),
            child(variant = Button, qname = "mso:button"),
            child(variant = ToggleButton, qname = "mso:toggleButton"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = EditBox, qname = "mso:editBox"),
            child(variant = ComboBox, qname = "mso:comboBox"),
            child(variant = DropDown, qname = "mso:dropDown"),
            child(variant = Gallery, qname = "mso:gallery"),
            child(variant = Menu, qname = "mso:menu"),
            child(variant = DynamicMenu, qname = "mso:dynamicMenu"),
            child(variant = SplitButton, qname = "mso:splitButton"),
            child(variant = Box, qname = "mso:box"),
            child(variant = ButtonGroup, qname = "mso:buttonGroup"),
            child(variant = VerticalSeparator, qname = "mso:separator")
        )
    )]
  pub group_choice: Vec<GroupChoice>,
  /// Defines the DialogBoxLauncher Class.
  #[sdk(child(qname = "mso:dialogBoxLauncher"))]
  pub dialog_box_launcher: Option<std::boxed::Box<DialogBoxLauncher>>,
}
/// Defines the QuickAccessToolbarControlClone Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:control")]
pub struct QuickAccessToolbarControlClone {
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
/// Defines the SharedQatControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:sharedControls")]
pub struct SharedQatControls {
  #[sdk(
        choice(
            child(variant = QuickAccessToolbarControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = VerticalSeparator, qname = "mso:separator")
        )
    )]
  pub shared_qat_controls_choice: Vec<SharedQatControlsChoice>,
}
/// Defines the DocumentSpecificQuickAccessToolbarControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:documentControls")]
pub struct DocumentSpecificQuickAccessToolbarControls {
  #[sdk(
        choice(
            child(variant = QuickAccessToolbarControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = VerticalSeparator, qname = "mso:separator")
        )
    )]
  pub document_specific_quick_access_toolbar_controls_choice:
    Vec<DocumentSpecificQuickAccessToolbarControlsChoice>,
}
/// Defines the Tab Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:tab")]
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
  pub id_q: Option<crate::simple_type::StringValue>,
  /// idMso
  #[sdk(attr(qname = ":idMso"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub id_mso: Option<crate::simple_type::StringValue>,
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
  pub insert_after_q: Option<crate::simple_type::StringValue>,
  /// insertBeforeQ
  #[sdk(attr(qname = ":insertBeforeQ"))]
  #[sdk(string_length(min = 1u32, max = 1024u32))]
  #[sdk(string_format(kind = "qname"))]
  pub insert_before_q: Option<crate::simple_type::StringValue>,
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
  #[sdk(child(qname = "mso:group"))]
  pub group: Vec<Group>,
}
/// Defines the ContextualTabSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:tabSet")]
pub struct ContextualTabSet {
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
  #[sdk(child(qname = "mso:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the RepurposedCommand Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:command")]
pub struct RepurposedCommand {
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
/// Defines the OfficeMenu Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:officeMenu")]
pub struct OfficeMenu {
  #[sdk(
        choice(
            child(variant = UnsizedControlClone, qname = "mso:control"),
            child(variant = UnsizedButton, qname = "mso:button"),
            child(variant = CheckBox, qname = "mso:checkBox"),
            child(variant = UnsizedGallery, qname = "mso:gallery"),
            child(variant = UnsizedToggleButton, qname = "mso:toggleButton"),
            child(variant = MenuSeparator, qname = "mso:menuSeparator"),
            child(variant = SplitButtonWithTitle, qname = "mso:splitButton"),
            child(variant = MenuWithTitle, qname = "mso:menu"),
            child(variant = UnsizedDynamicMenu, qname = "mso:dynamicMenu")
        )
    )]
  pub office_menu_choice: Vec<OfficeMenuChoice>,
}
/// Defines the QuickAccessToolbar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:qat")]
pub struct QuickAccessToolbar {
  /// Defines the SharedQatControls Class.
  #[sdk(child(qname = "mso:sharedControls"))]
  pub shared_qat_controls: Option<SharedQatControls>,
  /// Defines the DocumentSpecificQuickAccessToolbarControls Class.
  #[sdk(child(qname = "mso:documentControls"))]
  pub document_specific_quick_access_toolbar_controls:
    Option<DocumentSpecificQuickAccessToolbarControls>,
}
/// Defines the Tabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:tabs")]
pub struct Tabs {
  /// Defines the Tab Class.
  #[sdk(child(qname = "mso:tab"))]
  pub tab: Vec<Tab>,
}
/// Defines the ContextualTabSets Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:contextualTabs")]
pub struct ContextualTabSets {
  /// Defines the ContextualTabSet Class.
  #[sdk(child(qname = "mso:tabSet"))]
  pub contextual_tab_set: Vec<ContextualTabSet>,
}
/// Defines the RepurposedCommands Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:commands")]
pub struct RepurposedCommands {
  /// Defines the RepurposedCommand Class.
  #[sdk(child(qname = "mso:command"))]
  pub repurposed_command: Vec<RepurposedCommand>,
}
/// Defines the Ribbon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "mso:ribbon")]
pub struct Ribbon {
  /// startFromScratch
  #[sdk(attr(qname = ":startFromScratch"))]
  pub start_from_scratch: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "mso:officeMenu"))]
  pub office_menu: Option<OfficeMenu>,
  /// _
  #[sdk(child(qname = "mso:qat"))]
  pub quick_access_toolbar: Option<std::boxed::Box<QuickAccessToolbar>>,
  /// _
  #[sdk(child(qname = "mso:tabs"))]
  pub tabs: Option<Tabs>,
  /// _
  #[sdk(child(qname = "mso:contextualTabs"))]
  pub contextual_tab_sets: Option<ContextualTabSets>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnsizedSplitButtonChoice {
  /// Defines the VisibleButton Class.
  VisibleButton(std::boxed::Box<VisibleButton>),
  /// Defines the VisibleToggleButton Class.
  VisibleToggleButton(std::boxed::Box<VisibleToggleButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnsizedMenuChoice {
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the UnsizedSplitButton Class.
  UnsizedSplitButton(std::boxed::Box<UnsizedSplitButton>),
  /// Defines the UnsizedMenu Class.
  UnsizedMenu(std::boxed::Box<UnsizedMenu>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
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
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonWithTitle Class.
  SplitButtonWithTitle(std::boxed::Box<SplitButtonWithTitle>),
  /// Defines the MenuWithTitle Class.
  MenuWithTitle(std::boxed::Box<MenuWithTitle>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuChoice {
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the UnsizedSplitButton Class.
  UnsizedSplitButton(std::boxed::Box<UnsizedSplitButton>),
  /// Defines the UnsizedMenu Class.
  UnsizedMenu(std::boxed::Box<UnsizedMenu>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
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
  /// Defines the TextLabel Class.
  TextLabel(std::boxed::Box<TextLabel>),
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
  /// Defines the DropDown Class.
  DropDown(std::boxed::Box<DropDown>),
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
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedMenu Class.
  UnsizedMenu(std::boxed::Box<UnsizedMenu>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
  /// Defines the UnsizedSplitButton Class.
  UnsizedSplitButton(std::boxed::Box<UnsizedSplitButton>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuRootChoice {
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the UnsizedSplitButton Class.
  UnsizedSplitButton(std::boxed::Box<UnsizedSplitButton>),
  /// Defines the UnsizedMenu Class.
  UnsizedMenu(std::boxed::Box<UnsizedMenu>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupChoice {
  /// Defines the ControlClone Class.
  ControlClone(std::boxed::Box<ControlClone>),
  /// Defines the TextLabel Class.
  TextLabel(std::boxed::Box<TextLabel>),
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
  /// Defines the DropDown Class.
  DropDown(std::boxed::Box<DropDown>),
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
  /// Defines the VerticalSeparator Class.
  VerticalSeparator(std::boxed::Box<VerticalSeparator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SharedQatControlsChoice {
  /// Defines the QuickAccessToolbarControlClone Class.
  QuickAccessToolbarControlClone(std::boxed::Box<QuickAccessToolbarControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the VerticalSeparator Class.
  VerticalSeparator(std::boxed::Box<VerticalSeparator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentSpecificQuickAccessToolbarControlsChoice {
  /// Defines the QuickAccessToolbarControlClone Class.
  QuickAccessToolbarControlClone(std::boxed::Box<QuickAccessToolbarControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the VerticalSeparator Class.
  VerticalSeparator(std::boxed::Box<VerticalSeparator>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OfficeMenuChoice {
  /// Defines the UnsizedControlClone Class.
  UnsizedControlClone(std::boxed::Box<UnsizedControlClone>),
  /// Defines the UnsizedButton Class.
  UnsizedButton(std::boxed::Box<UnsizedButton>),
  /// Defines the CheckBox Class.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Defines the UnsizedGallery Class.
  UnsizedGallery(std::boxed::Box<UnsizedGallery>),
  /// Defines the UnsizedToggleButton Class.
  UnsizedToggleButton(std::boxed::Box<UnsizedToggleButton>),
  /// Defines the MenuSeparator Class.
  MenuSeparator(std::boxed::Box<MenuSeparator>),
  /// Defines the SplitButtonWithTitle Class.
  SplitButtonWithTitle(std::boxed::Box<SplitButtonWithTitle>),
  /// Defines the MenuWithTitle Class.
  MenuWithTitle(std::boxed::Box<MenuWithTitle>),
  /// Defines the UnsizedDynamicMenu Class.
  UnsizedDynamicMenu(std::boxed::Box<UnsizedDynamicMenu>),
}
