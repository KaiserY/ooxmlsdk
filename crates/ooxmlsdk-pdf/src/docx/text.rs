use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

use super::{
  HyperlinkCatalog, ImageCatalog, NumberingCatalog, Paragraph, ParagraphFormat, ParagraphProps,
  RunStyleOverrides, StylesCatalog, TextStyle, paragraph_inlines, paragraph_note_reference_ids,
  properties,
};

#[derive(Clone, Debug, Default)]
pub(super) struct ParagraphImportBase {
  pub(super) format: ParagraphFormat,
  pub(super) run_style: TextStyle,
  pub(super) run_overrides: RunStyleOverrides,
}

pub(super) fn paragraph_model(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Paragraph {
  paragraph_model_with_base(
    paragraph,
    styles,
    numbering,
    images,
    hyperlinks,
    ParagraphImportBase::default(),
  )
}

pub(super) fn paragraph_model_with_base(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  base: ParagraphImportBase,
) -> Paragraph {
  let style_id = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_style_id.as_ref())
    .map(|style| style.val.as_str());
  let mut format = properties::paragraph_format(
    styles,
    style_id,
    base.format,
    paragraph
      .paragraph_properties
      .as_deref()
      .map(ParagraphProps::Direct),
  );
  let list_label = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.numbering_properties.as_deref())
    .and_then(|properties| numbering.next_label(properties, &mut format));

  let inlines = paragraph_inlines(
    paragraph,
    properties::paragraph_run_style(styles, style_id, base.run_style, base.run_overrides),
    styles,
    images,
    hyperlinks,
  );
  let (footnote_reference_ids, endnote_reference_ids) = paragraph_note_reference_ids(paragraph);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      super::InlineItem::Text(run) => Some(run.clone()),
      super::InlineItem::Image(_) => None,
      super::InlineItem::PageBreak | super::InlineItem::ColumnBreak => None,
    })
    .collect();

  Paragraph {
    inlines,
    footnote_reference_ids,
    endnote_reference_ids,
    #[cfg(test)]
    runs,
    format,
    list_label,
    list_label_hyperlink_url: None,
  }
}
