use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use std::sync::Arc;

use super::{
  CustomXmlBindings, FormWidgetIdAllocator, HyperlinkCatalog, ImageCatalog, NumberingCatalog,
  NumberingFormatMergeContext, NumberingReference, Paragraph, ParagraphAdjust, ParagraphAlignment,
  ParagraphFormat, ParagraphProps, RunStyleOverrides, StylesCatalog, TextRun, TextStyle,
  math_paragraph_alignment, paragraph_inlines, paragraph_note_reference_ids, properties,
};

#[derive(Clone, Debug, Default)]
pub(super) struct ParagraphImportBase<'a> {
  pub(super) format: ParagraphFormat,
  pub(super) run_style: TextStyle,
  pub(super) run_overrides: RunStyleOverrides,
  pub(super) custom_xml_bindings: Option<&'a CustomXmlBindings>,
}

pub(super) fn paragraph_model(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  form_widget_ids: &mut FormWidgetIdAllocator,
) -> Paragraph {
  paragraph_model_with_base(
    paragraph,
    styles,
    numbering,
    images,
    hyperlinks,
    form_widget_ids,
    ParagraphImportBase {
      custom_xml_bindings: Some(custom_xml_bindings),
      ..Default::default()
    },
  )
}

pub(super) fn paragraph_model_with_base<'a>(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
  base: ParagraphImportBase<'a>,
) -> Paragraph {
  let default_custom_xml_bindings;
  let custom_xml_bindings = if let Some(custom_xml_bindings) = base.custom_xml_bindings {
    custom_xml_bindings
  } else {
    default_custom_xml_bindings = CustomXmlBindings::default();
    &default_custom_xml_bindings
  };
  let paragraph_properties = paragraph.paragraph_properties.as_deref();
  let previous_paragraph_properties = paragraph_properties
    .and_then(|properties| properties.paragraph_properties_change.as_deref())
    .and_then(|change| change.paragraph_properties_extended.as_deref());
  let use_previous_paragraph_properties =
    paragraph_mark_is_deleted(paragraph) && previous_paragraph_properties.is_some();
  let effective_style_id = if use_previous_paragraph_properties {
    previous_paragraph_properties.and_then(|properties| properties.paragraph_style_id.as_ref())
  } else {
    paragraph_properties.and_then(|properties| properties.paragraph_style_id.as_ref())
  };
  let style_id = effective_style_id.map(|style| style.val.as_str());
  let direct_paragraph_properties = if use_previous_paragraph_properties {
    previous_paragraph_properties.map(ParagraphProps::Extended)
  } else {
    paragraph_properties.map(ParagraphProps::Direct)
  };
  let numbering_format_context = NumberingFormatMergeContext {
    direct_indentation: direct_paragraph_properties
      .as_ref()
      .is_some_and(|properties| properties.indentation().is_some()),
    direct_tab_stops: direct_paragraph_properties
      .as_ref()
      .is_some_and(|properties| properties.tabs().is_some()),
    style_numbering: false,
  };
  let mut format =
    properties::paragraph_format(styles, style_id, base.format, direct_paragraph_properties);
  format.style_id = style_id.map(Arc::<str>::from);
  if let Some(alignment) = math_paragraph_alignment(paragraph) {
    format.alignment = alignment;
    let adjust = match alignment {
      ParagraphAlignment::Center => ParagraphAdjust::Center,
      ParagraphAlignment::Right => ParagraphAdjust::Right,
      ParagraphAlignment::Justify => ParagraphAdjust::Block,
      ParagraphAlignment::Left => ParagraphAdjust::Left,
    };
    format.justification.adjust = adjust;
    format.justification.one_word_adjust = adjust;
    format.justification.last_line_adjust = adjust;
  }
  let run_style =
    properties::paragraph_run_style(styles, style_id, base.run_style.clone(), base.run_overrides);
  let direct_numbering = direct_paragraph_properties
    .as_ref()
    .and_then(|properties| properties.numbering_properties())
    .and_then(NumberingReference::from_properties);
  let style_numbering = styles.paragraph_numbering_reference(style_id);
  let style_numbering_applies = direct_numbering.is_none() && style_numbering.is_some();
  let style_indent_overrides_numbering = style_numbering_applies && format.indent_left_set;
  let paragraph_mark_run_properties = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref());
  let paragraph_mark_style =
    properties::paragraph_mark_run_style(paragraph_mark_run_properties, run_style.clone(), styles);
  let has_direct_indentation = numbering_format_context.direct_indentation;
  // ECMA-376 Part 1 §17.9.24 makes w:lvl/w:rPr an overlay for numbering
  // text. With no explicit overlay, Word/Writer build the number portion from
  // the current paragraph font; this is why a numbered Heading 1 carries its
  // Times New Roman bold style into "Article 1.". LibreOffice implements the
  // same precedence in SwTextFormatter::NewNumberPortion.
  let numbering_base_style = paragraph_mark_style.clone();
  let style_tab_stop_pt = format.tab_stops.last().map(|stop| stop.position_pt);
  let numbering_label = direct_numbering.or(style_numbering).and_then(|reference| {
    numbering.next_label(
      reference,
      &mut format,
      styles,
      numbering_base_style,
      paragraph_mark_run_properties,
      NumberingFormatMergeContext {
        style_numbering: style_numbering_applies,
        ..numbering_format_context
      },
    )
  });
  let (
    mut list_label,
    numbering_image,
    list_label_style,
    numbering_list_tab_stop_pt,
    list_label_width_aware_tab,
  ) = numbering_label.map_or_else(
    || (None, None, TextStyle::default(), None, false),
    |label| {
      (
        label.text,
        label.image,
        label.style,
        label.list_tab_stop_pt,
        label.width_aware_tab,
      )
    },
  );
  format.list_label_width_aware_tab = list_label_width_aware_tab;
  format.list_label_uses_explicit_tab_stop =
    style_indent_overrides_numbering && numbering_list_tab_stop_pt.is_some();
  let has_numbering_label = list_label.is_some() || numbering_image.is_some();
  let list_label_tab_stop_pt = has_numbering_label
    .then(|| {
      style_tab_stop_pt
        .or(numbering_list_tab_stop_pt)
        .or_else(|| {
          (!has_direct_indentation && format.indent_left_pt > 0.0).then_some(
            format.indent_left_pt + format.first_line_indent_pt.max(format.indent_left_pt) * 4.0,
          )
        })
    })
    .flatten();
  if list_label.as_deref() == Some("\t") && style_tab_stop_pt.is_some() && !has_direct_indentation {
    list_label = Some(" \t".to_string());
  }
  let mut inlines = paragraph_inlines(
    paragraph,
    run_style.clone(),
    styles,
    images,
    hyperlinks,
    custom_xml_bindings,
    form_widget_ids,
  );
  if let Some(image) = numbering_image {
    inlines.insert(0, super::InlineItem::Image(image));
  }
  fill_character_style_ref_texts(&mut inlines);
  let style_ref_keys = style_id
    .map(|style_id| styles.style_ref_keys(style_id))
    .unwrap_or_default();
  let style_ref_text = paragraph_style_ref_text(&inlines, list_label.as_deref());
  if inlines.is_empty() && paragraph_requires_placeholder_run(paragraph) {
    inlines.push(super::InlineItem::Text(TextRun {
      text: String::new(),
      style: paragraph_mark_style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      preserve_text_portion: false,
    }));
  }
  let (footnote_reference_ids, endnote_reference_ids) = paragraph_note_reference_ids(paragraph);
  let starts_after_last_rendered_page_break =
    super::paragraph_starts_after_last_rendered_page_break(&inlines);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      super::InlineItem::Text(run) => Some(run.clone()),
      super::InlineItem::Image(_) => None,
      super::InlineItem::Shape(_) => None,
      super::InlineItem::BookmarkStart(_) => None,
      super::InlineItem::FormWidgetStart(_) | super::InlineItem::FormWidgetEnd(_) => None,
      super::InlineItem::LastRenderedPageBreak => None,
      super::InlineItem::PageBreak | super::InlineItem::ColumnBreak => None,
    })
    .collect();

  Paragraph {
    inlines,
    footnote_reference_ids,
    endnote_reference_ids,
    starts_after_last_rendered_page_break,
    base_style: paragraph_mark_style,
    #[cfg(test)]
    runs,
    format: Box::new(format),
    style_ref_keys,
    style_ref_text,
    list_label,
    list_label_style,
    list_label_hyperlink_url: None,
    list_label_tab_stop_pt,
  }
}

fn paragraph_style_ref_text(
  inlines: &[super::InlineItem],
  list_label: Option<&str>,
) -> Option<Arc<str>> {
  let mut text = String::new();
  if let Some(label) = list_label
    && !label.chars().all(char::is_whitespace)
  {
    text.push_str(label);
  }
  for item in inlines {
    if let super::InlineItem::Text(run) = item
      && run.dynamic_field.is_none()
    {
      if let Some(style_ref_text) = &run.style_ref_text {
        text.push_str(style_ref_text);
      } else {
        text.push_str(&run.text);
      }
    }
  }
  let text = text.trim();
  (!text.is_empty()).then(|| Arc::<str>::from(text))
}

fn fill_character_style_ref_texts(inlines: &mut [super::InlineItem]) {
  let mut index = 0;
  while index < inlines.len() {
    let Some(keys) = text_run_style_ref_keys(&inlines[index]) else {
      index += 1;
      continue;
    };
    let start = index;
    let mut text = String::new();
    while index < inlines.len()
      && text_run_style_ref_keys(&inlines[index]).is_some_and(|run_keys| run_keys == keys)
    {
      if let super::InlineItem::Text(run) = &inlines[index] {
        if let Some(style_ref_text) = &run.style_ref_text {
          text.push_str(style_ref_text);
        } else {
          text.push_str(&run.text);
        }
      }
      index += 1;
    }
    let text = text.trim();
    if text.is_empty() {
      continue;
    }
    let text = Arc::<str>::from(text);
    for item in &mut inlines[start..index] {
      if let super::InlineItem::Text(run) = item {
        run.style_ref_text = Some(text.clone());
      }
    }
  }
}

fn text_run_style_ref_keys(item: &super::InlineItem) -> Option<&[Arc<str>]> {
  let super::InlineItem::Text(run) = item else {
    return None;
  };
  (!run.style_ref_keys.is_empty() && run.dynamic_field.is_none()).then_some(&run.style_ref_keys)
}

fn paragraph_mark_is_deleted(paragraph: &w::Paragraph) -> bool {
  paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref())
    .is_some_and(|properties| properties.deleted.is_some() || properties.move_from.is_some())
}

fn paragraph_requires_placeholder_run(paragraph: &w::Paragraph) -> bool {
  let Some(properties) = paragraph.paragraph_properties.as_deref() else {
    return false;
  };
  let Some(run_properties) = properties.paragraph_mark_run_properties.as_deref() else {
    return false;
  };

  super::paragraph_mark_run_properties_font_size(run_properties)
    .map(|size| size.val)
    .or_else(|| {
      super::paragraph_mark_run_properties_complex_script_font_size(run_properties)
        .map(|size| size.val)
    })
    .map(|size| size.to_half_points() <= 9)
    .unwrap_or(false)
}
