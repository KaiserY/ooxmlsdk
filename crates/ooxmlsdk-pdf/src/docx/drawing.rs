use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

use super::{HyperlinkCatalog, ImageCatalog, InlineImage, InlineItem, StylesCatalog, TextStyle};

pub(super) fn inline_image(
  drawing: &w::Drawing,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Option<InlineImage> {
  super::inline_image_impl(drawing, images, hyperlinks)
}

pub(super) fn push_drawing_textboxes(
  drawing: &w::Drawing,
  inlines: &mut Vec<InlineItem>,
  style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  super::push_drawing_textboxes_impl(drawing, inlines, style, styles, images);
}

pub(super) fn pict_image(picture: &w::Picture, images: &ImageCatalog) -> Option<InlineImage> {
  super::pict_image_impl(picture, images)
}

pub(super) fn push_pict_textboxes(
  picture: &w::Picture,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  super::push_pict_textboxes_impl(picture, inlines, base_style, styles, images);
}
