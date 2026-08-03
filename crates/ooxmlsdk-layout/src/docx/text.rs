use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use std::sync::Arc;

use crate::fonts::effective_font_size_pt;

use super::{
  CustomXmlBindings, FormWidgetIdAllocator, HyperlinkCatalog, ImageCatalog, ListLabelImage,
  NumberingCatalog, NumberingFormatMergeContext, NumberingReference, Paragraph, ParagraphAdjust,
  ParagraphAlignment, ParagraphFormat, ParagraphInlineImport, ParagraphProps, RunStyleOverrides,
  StylesCatalog, TextRun, TextStyle, math_paragraph_alignment, paragraph_field_events,
  paragraph_inlines_with_policy, paragraph_note_reference_ids, properties,
  select_paragraph_numbering,
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
    direct_tab_stops: direct_paragraph_properties
      .as_ref()
      .is_some_and(|properties| properties.tabs().is_some()),
    ..NumberingFormatMergeContext::from_direct_properties(direct_paragraph_properties)
  };
  let style_outline_level = styles
    .paragraph_format_with_base(style_id, base.format.clone())
    .outline_level;
  let mut format =
    properties::paragraph_format(styles, style_id, base.format, direct_paragraph_properties);
  format.style_id = style_id.map(Arc::<str>::from);
  format.style_outline_level = style_outline_level;
  if [
    format.indent_left_character_units,
    format.indent_right_character_units,
    format.first_line_indent_character_units,
  ]
  .into_iter()
  .flatten()
  .any(|value| value != 0.0)
  {
    // Word resolves w:*Chars against the document run default, independently
    // of the effective paragraph/run style. In paraind.docx, Heading2 is 16pt
    // while rPrDefault is 10.5pt; Microsoft's fixed PDF uses the 10.5pt unit.
    // Writer also models FONT_CJK_ADVANCE as the bound CJK font height.
    format.character_indent_unit_pt = Some(effective_font_size_pt(&styles.doc_default_run, None));
  }
  if let Some(alignment) = math_paragraph_alignment(paragraph, styles.display_math_alignment) {
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
  let (numbering_reference, style_numbering_applies, numbering_cancelled) =
    select_paragraph_numbering(direct_numbering, style_numbering);
  if numbering_cancelled {
    let (left, first_line) = styles.paragraph_indents_without_numbering(style_id);
    if !numbering_format_context.direct_indent_left {
      format.indent_left_pt = left.0;
      format.indent_left_character_units = left.1;
      format.indent_left_set = true;
    }
    if !numbering_format_context.direct_first_line_indent {
      format.first_line_indent_pt = first_line.0;
      format.first_line_indent_character_units = first_line.1;
      format.first_line_indent_set = true;
    }
  }
  let style_indent_overrides_numbering = style_numbering_applies && format.indent_left_set;
  let paragraph_mark_run_properties = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_mark_run_properties.as_deref());
  let mut paragraph_mark_style =
    properties::paragraph_mark_run_style(paragraph_mark_run_properties, run_style.clone(), styles);
  let has_direct_indentation = numbering_format_context.has_direct_indentation();
  // ECMA-376 Part 1 §17.9.24 makes w:lvl/w:rPr an overlay for numbering
  // text. Word/Writer start the number portion from the paragraph font, then
  // apply w:pPr/w:rPr separately to the synthesized number. Keep that
  // paragraph-mark layer unresolved here: NumberingCatalog applies it once
  // after the numbering-level overlay and restores explicit level properties.
  // Passing paragraph_mark_style would apply a referenced character style
  // twice, incorrectly reversing its toggle properties on the second pass.
  let numbering_base_style = run_style.clone();
  let style_tab_stop_pt = format.tab_stops.last().map(|stop| stop.position_pt);
  let numbering_label = numbering_reference.and_then(|reference| {
    let matched_style_indent_context = styles.numbering_matched_style_indent_context(style_id);
    numbering.next_label(
      reference,
      &mut format,
      styles,
      numbering_base_style,
      paragraph_mark_run_properties,
      NumberingFormatMergeContext {
        style_numbering: style_numbering_applies,
        matched_style_indent_left: matched_style_indent_context.matched_style_indent_left,
        matched_style_indent_right: matched_style_indent_context.matched_style_indent_right,
        matched_style_first_line_indent: matched_style_indent_context
          .matched_style_first_line_indent,
        ..numbering_format_context
      },
    )
  });
  let (
    mut list_label,
    style_ref_numbering_text,
    numbering_image,
    numbering_image_replacement_text,
    mut list_label_style,
    list_label_justification,
    numbering_list_tab_stop_pt,
    list_label_width_aware_tab,
  ) = numbering_label.map_or_else(
    || {
      (
        None,
        None,
        None,
        None,
        TextStyle::default(),
        w::LevelJustificationValues::Left,
        None,
        false,
      )
    },
    |label| {
      (
        label.text,
        label.suppressed_non_numerical_text,
        label.image,
        label.image_replacement_text,
        label.style,
        label.justification,
        label.list_tab_stop_pt,
        label.width_aware_tab,
      )
    },
  );
  format.list_label_width_aware_tab = list_label_width_aware_tab;
  format.list_label_uses_explicit_tab_stop =
    style_indent_overrides_numbering && numbering_list_tab_stop_pt.is_some();
  format.list_label_justification = list_label_justification;
  let has_numbering_label = list_label.is_some() || numbering_image.is_some();
  let blank_numbering_label = list_label
    .as_deref()
    .is_some_and(|label| label.chars().all(char::is_whitespace));
  let list_label_tab_stop_pt = has_numbering_label
    .then(|| {
      // A direct paragraph indent is Word's persisted result of opening and
      // accepting the paragraph dialog for pseudo-numbering. In that state
      // the numbering-level num tab remains authoritative and the paragraph
      // style's ordinary tab must not revive the old large pseudo-numbering
      // gap (tdf153042_noTab). Without direct indentation, the style tab is
      // the legacy large-tab behavior retained by Word 2019.
      (if has_direct_indentation {
        numbering_list_tab_stop_pt
      } else {
        style_tab_stop_pt.or(numbering_list_tab_stop_pt)
      })
      .or_else(|| {
        // An empty w:lvlText with the default tab suffix is a real TabLeft
        // portion, not visible pseudo-numbering. Writer's tdf#148360 layout
        // and Word fixed output both place the following text at the
        // numbering level's left indent.
        (blank_numbering_label && format.indent_left_pt > 0.0).then_some(format.indent_left_pt)
      })
      .or_else(|| {
        // The legacy large-tab fallback models text pseudo-numbering. A
        // picture bullet is a fixed numbering-margin portion; when the level
        // has no authored num tab its body starts at the ordinary left
        // indent, not four indents beyond it.
        (!has_direct_indentation && numbering_image.is_none() && format.indent_left_pt > 0.0)
          .then_some(
            format.indent_left_pt + format.first_line_indent_pt.max(format.indent_left_pt) * 4.0,
          )
      })
    })
    .flatten();
  if list_label.as_deref() == Some("\t") && style_tab_stop_pt.is_some() && !has_direct_indentation {
    list_label = Some(" \t".to_string());
  }
  let mut inlines = paragraph_inlines_with_policy(
    paragraph,
    run_style.clone(),
    styles,
    images,
    hyperlinks,
    ParagraphInlineImport {
      custom_xml_bindings,
      form_widget_ids,
      suppress_toc_hyperlink_style: styles.is_toc_entry_paragraph_style(style_id),
    },
  );
  if let Some(bold_override) = paragraph_mark_style.wordprocessingml_field_bold_override {
    for inline in &mut inlines {
      let super::InlineItem::Text(run) = inline else {
        continue;
      };
      if run.dynamic_field.is_some() && run.style.wordprocessingml_field_bold_override.is_none() {
        // Word applies direct paragraph-mark formatting to application-
        // generated field diagnostics, while ordinary persisted result text
        // continues to use its own run/style cascade.
        run.style.wordprocessingml_field_bold_override = Some(bold_override);
      }
    }
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
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }));
  }
  let line_vertical_alignment = format.line_vertical_alignment.unwrap_or_default();
  paragraph_mark_style.line_vertical_alignment = line_vertical_alignment;
  list_label_style.line_vertical_alignment = line_vertical_alignment;
  for inline in &mut inlines {
    match inline {
      super::InlineItem::Text(run) => {
        run.style.line_vertical_alignment = line_vertical_alignment;
      }
      super::InlineItem::PositionalTab(tab) => {
        tab.style.line_vertical_alignment = line_vertical_alignment;
      }
      super::InlineItem::Ruby(ruby) => {
        for run in ruby.base.iter_mut().chain(&mut ruby.guide) {
          run.style.line_vertical_alignment = line_vertical_alignment;
        }
      }
      super::InlineItem::LegacyFormCheckBox(check_box) => {
        check_box.style.line_vertical_alignment = line_vertical_alignment;
      }
      _ => {}
    }
  }
  let (footnote_reference_ids, endnote_reference_ids) = paragraph_note_reference_ids(paragraph);
  let mut list_label_image = numbering_image.and_then(|image| {
    let replacement_text = numbering_image_replacement_text?;
    (!replacement_text.is_empty()).then_some(ListLabelImage {
      image,
      replacement_text,
    })
  });
  if !has_direct_indentation && let Some(legacy_image) = list_label_image.take() {
    // A style-owned legacy list keeps its graphic in the number portion's
    // inline line box. Direct paragraph indentation switches Word to the
    // fixed label-alignment margin model handled by `list_label_image`.
    for _ in 0..legacy_image.replacement_text.chars().count() {
      inlines.insert(0, super::InlineItem::Image(legacy_image.image.clone()));
    }
  }
  let starts_after_last_rendered_page_break =
    super::paragraph_starts_after_last_rendered_page_break(&inlines);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      super::InlineItem::Text(run) => Some(run.clone()),
      super::InlineItem::PositionalTab(_) => None,
      super::InlineItem::Ruby(_) => None,
      super::InlineItem::LegacyFormCheckBox(_) => None,
      super::InlineItem::Image(_) => None,
      super::InlineItem::Shape(_) => None,
      super::InlineItem::BookmarkStart(_) => None,
      super::InlineItem::FormWidgetStart(_) | super::InlineItem::FormWidgetEnd(_) => None,
      super::InlineItem::DrawingGroupStart(_) | super::InlineItem::DrawingGroupEnd => None,
      super::InlineItem::LastRenderedPageBreak => None,
      super::InlineItem::ClearLineBreak(_) => None,
      super::InlineItem::PageBreak | super::InlineItem::ColumnBreak => None,
    })
    .collect();

  Paragraph {
    inlines,
    field_events: paragraph_field_events(paragraph),
    footnote_reference_ids,
    endnote_reference_ids,
    starts_after_last_rendered_page_break,
    base_style: paragraph_mark_style,
    #[cfg(test)]
    runs,
    format: Box::new(format),
    style_ref_keys,
    style_ref_text,
    style_ref_numbering_text: style_ref_numbering_text.map(Arc::<str>::from),
    list_label,
    list_label_image,
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

pub(super) fn paragraph_mark_is_deleted(paragraph: &w::Paragraph) -> bool {
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
