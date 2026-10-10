use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use std::sync::Arc;
use unicode_script::{Script, UnicodeScript};

use crate::{fonts::effective_font_size_pt, units};

use super::{
  ComplexFieldImportState, CustomXmlBindings, FormWidgetIdAllocator, HyperlinkCatalog,
  ImageCatalog, LineHeightRule, ListLabelImage, NumberingCatalog, NumberingFormatMergeContext,
  NumberingReference, Paragraph, ParagraphFormat, ParagraphInlineImport, ParagraphProps,
  RunStyleOverrides, StylesCatalog, TextRun, TextStyle, paragraph_field_events,
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

pub(super) struct ParagraphImportState<'a> {
  pub(super) form_widget_ids: &'a mut FormWidgetIdAllocator,
  pub(super) complex_fields: Option<&'a mut ComplexFieldImportState>,
  pub(super) table_depth: usize,
}

impl<'a> ParagraphImportState<'a> {
  #[cfg(test)]
  fn isolated(form_widget_ids: &'a mut FormWidgetIdAllocator) -> Self {
    Self {
      form_widget_ids,
      complex_fields: None,
      table_depth: 0,
    }
  }

  pub(super) fn in_story(
    form_widget_ids: &'a mut FormWidgetIdAllocator,
    complex_fields: &'a mut ComplexFieldImportState,
    table_depth: usize,
  ) -> Self {
    Self {
      form_widget_ids,
      complex_fields: Some(complex_fields),
      table_depth,
    }
  }
}

#[cfg(test)]
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

pub(super) fn paragraph_model_in_story(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  custom_xml_bindings: &CustomXmlBindings,
  state: ParagraphImportState<'_>,
) -> Paragraph {
  paragraph_model_with_base_in_story(
    paragraph,
    styles,
    numbering,
    images,
    hyperlinks,
    ParagraphImportBase {
      custom_xml_bindings: Some(custom_xml_bindings),
      ..Default::default()
    },
    state,
  )
}

#[cfg(test)]
pub(super) fn paragraph_model_with_base<'a>(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
  base: ParagraphImportBase<'a>,
) -> Paragraph {
  paragraph_model_with_base_impl(
    paragraph,
    styles,
    numbering,
    images,
    hyperlinks,
    base,
    ParagraphImportState::isolated(form_widget_ids),
  )
}

pub(super) fn paragraph_model_with_base_in_story<'a>(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  base: ParagraphImportBase<'a>,
  state: ParagraphImportState<'_>,
) -> Paragraph {
  paragraph_model_with_base_impl(
    paragraph, styles, numbering, images, hyperlinks, base, state,
  )
}

fn paragraph_model_with_base_impl<'a>(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
  base: ParagraphImportBase<'a>,
  state: ParagraphImportState<'_>,
) -> Paragraph {
  let ParagraphImportState {
    form_widget_ids,
    mut complex_fields,
    table_depth,
  } = state;
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
  let has_direct_line_height = direct_paragraph_properties
    .as_ref()
    .and_then(|properties| properties.spacing_between_lines())
    .is_some_and(|spacing| spacing.line.is_some());
  let has_direct_justification = direct_paragraph_properties
    .as_ref()
    .is_some_and(|properties| properties.justification().is_some());
  let style_outline_level = styles
    .paragraph_format_with_base(style_id, base.format.clone())
    .outline_level;
  let mut format =
    properties::paragraph_format(styles, style_id, base.format, direct_paragraph_properties);
  format.list_label_default_tab_stop_pt = styles.default_tab_stop_pt;
  format.additive_paragraph_spacing = styles.import_settings.fixed_html_paragraph_auto_spacing;
  // The Word-compatible proportional-gap path is driven by the paragraph's
  // directly authored line spacing. A line value inherited from a style still
  // determines total line height, but does not move the first-line baseline.
  format.line_height_set = has_direct_line_height;
  format.justification_set = has_direct_justification;
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
    // Word resolves leftChars/rightChars against the document run default,
    // independently of the effective paragraph/run style. In paraind.docx, Heading2 is 16pt
    // while rPrDefault is 10.5pt; Microsoft's fixed PDF uses the 10.5pt unit.
    // Writer also models FONT_CJK_ADVANCE as the bound CJK font height.
    format.character_indent_unit_pt = Some(effective_font_size_pt(&styles.doc_default_run, None));
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
  format.numbering_id = numbering_reference.map(NumberingReference::num_id);
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
  format.paragraph_mark_font_size_set = paragraph_mark_run_properties.is_some_and(|properties| {
    super::paragraph_mark_run_properties_font_size(properties).is_some()
      || super::paragraph_mark_run_properties_complex_script_font_size(properties).is_some()
  });
  format.numbered_paragraph_mark_background = format.numbering_id.is_some()
    && paragraph_mark_run_properties.is_some_and(|properties| {
      properties
        .paragraph_mark_run_properties_choice2
        .iter()
        .any(|property| {
          matches!(
            property,
            w::ParagraphMarkRunPropertiesChoice2::Highlight(_)
              | w::ParagraphMarkRunPropertiesChoice2::Shading(_)
          )
        })
    });
  let mut paragraph_mark_style =
    properties::paragraph_mark_run_style(paragraph_mark_run_properties, run_style.clone(), styles);
  if format.bidi
    && let Some(size) = paragraph_mark_style
      .automatic_escapement_complex_font_size_pt
      .or(paragraph_mark_style.complex_font_size_pt)
  {
    // The paragraph mark has the paragraph's base direction. Native Word
    // empty-paragraph controls use szCs for its size in a bidi paragraph,
    // even with explicit rtl=0/cs=0. Those run flags still own the font face:
    // a Latin mark keeps its Latin metrics at the complex-script size.
    // Keep this on the mark layer; ordinary runs retain their own sz/szCs.
    properties::set_font_size_preserving_automatic_escapement(&mut paragraph_mark_style, size);
  }
  // ECMA-376 §17.3.2.41 inherits vanish through the style hierarchy; the
  // paragraph mark is not limited to a directly authored w:pPr/w:rPr.
  format.hidden_separator = paragraph_mark_style.hidden;
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
    numbering_image_follow,
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
        label.image_follow,
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
  if has_numbering_label {
    format.list_label_direct_tabs_before_indent_pt = direct_paragraph_properties
      .as_ref()
      .and_then(|properties| properties.tabs())
      .map(|tabs| {
        tabs
          .tab_stop
          .iter()
          .filter(|tab| {
            matches!(
              tab.val,
              w::TabStopValues::Left | w::TabStopValues::Start | w::TabStopValues::Number
            )
          })
          .filter_map(|tab| super::signed_twips_measure_to_points(&tab.position))
          .filter(|stop| stop.is_finite() && *stop >= 0.0 && *stop < format.indent_left_pt)
          .collect()
      })
      .unwrap_or_default();
  }
  let blank_numbering_label = list_label
    .as_deref()
    .is_some_and(|label| label.chars().all(char::is_whitespace));
  let mut list_label_tab_stop_pt = has_numbering_label
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
      complex_fields: complex_fields.as_deref_mut(),
      table_depth,
      style_outline_level: format.style_outline_level,
    },
  );
  let mut field_events = paragraph_field_events(paragraph);
  if let Some(complex_fields) = complex_fields {
    complex_fields.finish_paragraph(&mut inlines, &mut field_events, styles);
  }
  if inlines.iter().any(|inline| {
    matches!(
      inline,
      super::InlineItem::Text(run) if run.style.wordprocessingml_index_field_diagnostic
    )
  }) {
    // Word places the generated empty-INDEX diagnostic one field-result
    // advance below the cached Index paragraph's top border (tdf166436).
    format.spacing_before_pt += 9.0;
    format.spacing_before_set = true;
  }
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
  let style_ref_text = paragraph_style_ref_text(&inlines);
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
  let use_windows_font_metrics = paragraph_uses_windows_font_metrics(
    &format,
    paragraph_text_owns_line_metrics(&inlines, has_numbering_label),
  );
  paragraph_mark_style.line_vertical_alignment = line_vertical_alignment;
  paragraph_mark_style.use_windows_font_metrics = use_windows_font_metrics;
  list_label_style.line_vertical_alignment = line_vertical_alignment;
  list_label_style.use_windows_font_metrics = use_windows_font_metrics;
  for inline in &mut inlines {
    match inline {
      super::InlineItem::Text(run) => {
        run.style.line_vertical_alignment = line_vertical_alignment;
        run.style.use_windows_font_metrics = use_windows_font_metrics;
        apply_wordprocessingml_cjk_text_metrics(&run.text, &mut run.style);
      }
      super::InlineItem::PositionalTab(tab) => {
        tab.style.line_vertical_alignment = line_vertical_alignment;
        tab.style.use_windows_font_metrics = use_windows_font_metrics;
      }
      super::InlineItem::Ruby(ruby) => {
        for run in ruby.base.iter_mut().chain(&mut ruby.guide) {
          run.style.line_vertical_alignment = line_vertical_alignment;
          run.style.use_windows_font_metrics = use_windows_font_metrics;
          apply_wordprocessingml_cjk_text_metrics(&run.text, &mut run.style);
        }
      }
      super::InlineItem::Overstrike(overstrike) => {
        for run in overstrike.operands.iter_mut().flatten() {
          run.style.line_vertical_alignment = line_vertical_alignment;
          run.style.use_windows_font_metrics = use_windows_font_metrics;
          apply_wordprocessingml_cjk_text_metrics(&run.text, &mut run.style);
        }
      }
      super::InlineItem::NoteReferenceMark(mark) => {
        mark.style.use_windows_font_metrics = use_windows_font_metrics;
      }
      super::InlineItem::NoteSeparatorMark(mark) => {
        mark.style.use_windows_font_metrics = use_windows_font_metrics;
      }
      super::InlineItem::LegacyFormCheckBox(check_box) => {
        check_box.style.line_vertical_alignment = line_vertical_alignment;
        check_box.style.use_windows_font_metrics = use_windows_font_metrics;
      }
      _ => {}
    }
  }
  let (footnote_reference_ids, endnote_reference_ids) = paragraph_note_reference_ids(paragraph);
  let mut list_label_image = numbering_image.and_then(|image| {
    let replacement_text = numbering_image_replacement_text?;
    let intrinsic_height_pt = super::picture_bullet::intrinsic_height_pt(&image.data);
    (!replacement_text.is_empty()).then_some(ListLabelImage {
      image,
      replacement_text,
      intrinsic_height_pt,
      follow: numbering_image_follow,
    })
  });
  let page_break_only_paragraph = inlines.iter().any(|inline| {
    matches!(
      inline,
      super::InlineItem::PageBreak | super::InlineItem::ColumnBreak
    )
  }) && inlines.iter().all(|inline| {
    matches!(
      inline,
      super::InlineItem::PageBreak
        | super::InlineItem::ColumnBreak
        | super::InlineItem::BookmarkStart(_)
        | super::InlineItem::LastRenderedPageBreak
    )
  });
  if page_break_only_paragraph {
    // A list paragraph whose only body content is an authored page/column
    // break carries the break, not a visible inherited numbering label. This
    // is the final empty list paragraph in the Open-XML-SDK style/1.docx
    // control; Word leaves its page empty instead of painting the level-1
    // label (`o`).
    list_label = None;
    list_label_image = None;
    list_label_tab_stop_pt = None;
  }
  let starts_after_last_rendered_page_break =
    super::paragraph_starts_after_last_rendered_page_break(&inlines);
  if let Some(background) = format
    .frame
    .and(format.shading)
    .and_then(super::ShadingPaint::solid_color)
  {
    // ECMA-376 Part 1 §17.3.2.6 makes automatic run color dependent on
    // its display background. Writer's fixed-output path asks the text frame
    // for its effective background brush and chooses white for a dark brush
    // (fntcache.cxx). Keep that lookup scoped to an actual w:framePr:
    // fdo76979's dark shaded header frame paints white text, while Office's
    // para-shading fixed output keeps omitted body text black. A direct run
    // color remains authoritative through the shared automatic-only helper.
    super::apply_automatic_text_color_to_paragraph_parts(
      &mut paragraph_mark_style,
      &mut list_label_style,
      &mut inlines,
      super::automatic_text_color_for_background(background),
    );
  }
  if !format.bidi
    && format
      .first_line_indent_character_units
      .is_some_and(|units| units != 0.0)
  {
    // Native Word uses the initial text's em for firstLineChars/hangingChars,
    // unlike leftChars/rightChars. A leading space owns this size too; the
    // paragraph mark and a later larger run do not replace it. Keep non-text
    // leading objects on their existing default-unit path.
    let first = inlines.iter().find(|inline| match inline {
      super::InlineItem::BookmarkStart(_) | super::InlineItem::LastRenderedPageBreak => false,
      super::InlineItem::Text(run) => !run.text.is_empty(),
      _ => true,
    });
    if let Some(super::InlineItem::Text(run)) = first {
      format.first_line_character_indent_unit_pt = Some(effective_font_size_pt(&run.style, None));
    }
  }
  super::fit_text::resolve(&mut inlines, &mut field_events);
  merge_adjacent_continuous_text_runs(&mut inlines, &mut field_events);
  resolve_wordprocessingml_rtl_font_portions(&mut inlines, &mut field_events);
  preserve_interior_direction_control_metrics(&mut inlines, &mut field_events);
  attach_arabic_leading_mark_context(&mut inlines);
  if let Some(background) = format.shading.and_then(super::ShadingPaint::solid_color) {
    super::apply_legacy_text_effect_background_to_paragraph_parts(
      &mut paragraph_mark_style,
      &mut list_label_style,
      &mut inlines,
      background,
    );
  }
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      super::InlineItem::Text(run) => Some(run.clone()),
      super::InlineItem::NoteReferenceMark(_) => None,
      super::InlineItem::NoteSeparatorMark(_) => None,
      super::InlineItem::PositionalTab(_) => None,
      super::InlineItem::Ruby(_) | super::InlineItem::Overstrike(_) => None,
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
    field_events,
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

fn attach_arabic_leading_mark_context(inlines: &mut [super::InlineItem]) {
  use super::InlineItem;

  for index in 1..inlines.len() {
    let (preceding, following) = inlines.split_at_mut(index);
    let (InlineItem::Text(left), InlineItem::Text(right)) =
      (&preceding[index - 1], &mut following[0])
    else {
      continue;
    };
    if !right
      .text
      .chars()
      .next()
      .is_some_and(crate::fonts::arabic_nonspacing_mark)
      || !crate::fonts::arabic_shaping_properties_match(&left.style, &right.style)
      || left.dynamic_field.is_some()
      || right.dynamic_field.is_some()
    {
      continue;
    }
    // Native Word keeps a colored leading mark attached to its preceding
    // base. A marks-only portion with a different bidi language is an
    // independent shaping boundary and receives a dotted circle instead.
    // A following base in the same portion preserves joining across a
    // language boundary, as ordinary Arabic text already does.
    if right.text.chars().all(crate::fonts::arabic_nonspacing_mark)
      && left.style.bidi_language != right.style.bidi_language
    {
      continue;
    }
    let base = left
      .text
      .chars()
      .rev()
      .find(|&character| !crate::fonts::arabic_nonspacing_mark(character));
    if base.is_none_or(|character| character.script() != Script::Arabic) {
      continue;
    }
    right.style.shaping_context = Some(Arc::new(crate::common::TextShapingContext {
      before: Arc::from(left.text.as_str()),
      after: Arc::from(""),
      leading_marks_only: true,
    }));
  }
}

fn preserve_interior_direction_control_metrics(
  inlines: &mut Vec<super::InlineItem>,
  field_events: &mut [super::ParagraphFieldEvent],
) {
  use super::{InlineItem, ParagraphFieldEvent};

  if !inlines
    .iter()
    .any(|item| matches!(item, InlineItem::Text(run) if run.text.contains('\u{202c}')))
  {
    return;
  }
  // Native Word's itemization measures an interior sequence of two or more
  // literal PDFs through the font's nominal glyphs. A single PDF and a
  // paragraph's leading/trailing PDFs remain zero-width. XML w:dir boundaries
  // are metadata and never participate. Preserve source bytes, and classify
  // across equivalent w:r boundaries before assigning the measurement owner.
  // Separate native Arabic/Latin, font, count and boundary controls establish
  // this distinction; the width is a font metric, not a synthetic space.
  let mut text = String::new();
  for item in inlines.iter() {
    if let InlineItem::Text(run) = item {
      text.push_str(&run.text);
    } else {
      text.push('\0');
    }
  }
  let control = |ch| matches!(ch, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}');
  let visible = |ch: char| !ch.is_whitespace() && !control(ch);
  let mut ranges = Vec::new();
  let mut chars = text.char_indices().peekable();
  while let Some((start, ch)) = chars.next() {
    if ch != '\u{202c}' {
      continue;
    }
    let mut end = start + ch.len_utf8();
    let mut count = 1;
    while let Some(&(index, '\u{202c}')) = chars.peek() {
      chars.next();
      end = index + ch.len_utf8();
      count += 1;
    }
    let before = text[..start].rsplit('\0').next().unwrap_or_default();
    let after = text[end..].split('\0').next().unwrap_or_default();
    if count >= 2
      && before.chars().any(visible)
      && after.chars().any(visible)
      && !before.chars().next_back().is_some_and(control)
      && !after.chars().next().is_some_and(control)
    {
      ranges.push(start..end);
    }
  }
  let scoped_controls = inlines.iter().any(|item| {
    matches!(item, InlineItem::Text(run) if run.style.wordprocessing_bidi_scopes.is_some()
      && run.text.chars().any(|ch| matches!(ch, '\u{202a}'..='\u{202e}')))
  });
  if ranges.is_empty() && !scoped_controls {
    return;
  }
  let mut output = Vec::with_capacity(inlines.len() + ranges.len() * 2);
  let mut offsets = Vec::with_capacity(inlines.len() + 1);
  let mut position = 0;
  for item in inlines.drain(..) {
    offsets.push(output.len());
    let InlineItem::Text(run) = &item else {
      position += 1;
      output.push(item);
      continue;
    };
    let end = position + run.text.len();
    let mut cuts = vec![position, end];
    if run.style.wordprocessing_bidi_scopes.is_some() {
      // Retain the consumed control prefix at a source boundary, so line
      // splitting cannot resurrect an embedding closed on an earlier line.
      let mut was_control = false;
      for (offset, ch) in run.text.char_indices() {
        let is_control = matches!(ch, '\u{202a}'..='\u{202e}');
        if is_control != was_control {
          cuts.push(position + offset);
        }
        was_control = is_control;
      }
    }
    for range in &ranges {
      if range.start < end && range.end > position {
        cuts.extend([range.start.max(position), range.end.min(end)]);
      }
    }
    cuts.sort_unstable();
    cuts.dedup();
    if cuts.len() == 1 {
      output.push(item);
    } else {
      for part in cuts.windows(2) {
        let mut portion = run.clone();
        portion.text = run.text[(part[0] - position)..(part[1] - position)].to_owned();
        portion.style.wordprocessing_nominal_control_metrics = run.style.right_to_left
          == Some(true)
          && ranges
            .iter()
            .any(|range| range.start <= part[0] && part[1] <= range.end);
        output.push(InlineItem::Text(portion));
      }
    }
    position = end;
  }
  offsets.push(output.len());
  for event in field_events {
    match event {
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
        *inline_offset = offsets[*inline_offset];
      }
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start,
        inline_end,
        ..
      } => {
        *inline_start = offsets[*inline_start];
        *inline_end = offsets[*inline_end];
      }
      _ => {}
    }
  }
  let mut region = None;
  let mut prefix = String::new();
  for item in &mut output {
    let InlineItem::Text(run) = item else {
      continue;
    };
    let current = (
      run.style.wordprocessing_bidi_scopes.clone(),
      run.style.right_to_left,
    );
    if region.as_ref() != Some(&current) {
      prefix.clear();
      region = Some(current);
    }
    if run.style.wordprocessing_bidi_scopes.is_some() {
      run.style.wordprocessing_bidi_prefix =
        (!prefix.is_empty()).then(|| Arc::from(prefix.as_str()));
      prefix.extend(
        run
          .text
          .chars()
          .filter(|ch| matches!(ch, '\u{202a}'..='\u{202e}')),
      );
    }
  }
  *inlines = output;
}

fn merge_adjacent_continuous_text_runs(
  inlines: &mut Vec<super::InlineItem>,
  field_events: &mut [super::ParagraphFieldEvent],
) {
  use super::{InlineItem, ParagraphFieldEvent};

  // Equivalent w:r boundaries inside a word are not shaping or word-break
  // boundaries. Word keeps a word (including ZWNJ and its suffix) together
  // when edits split it into otherwise identical RTL runs. It also fills
  // overlong underscore form lines across otherwise identical runs: splitting
  // those runs independently leaves premature breaks at their boundaries.
  // Shape and fit one continuous portion while preserving semantic and
  // formatting boundaries.
  let same_style =
    |left: &TextStyle, right: &TextStyle, allow_arabic_regions: bool, punctuation_pair: bool| {
      if left == right {
        return true;
      }
      if punctuation_pair {
        // This bit is inferred from the portion's characters, not authored
        // formatting. A punctuation-only portion lacks it while the adjacent
        // CJK text has it; joining that pair takes the union of their metrics.
        let mut right = right.clone();
        right.wordprocessingml_cjk_line_metrics = left.wordprocessingml_cjk_line_metrics;
        if *left == right {
          return true;
        }
      }
      // MS-OI29500 §2.1.88: Arabic w:rtl text selects the complex-script font
      // independently of rFonts@hint. A hint-only edit inside a word must therefore
      // not break its joining context. Other style differences remain real
      // boundaries, including the selected font families and run direction.
      if left.right_to_left != Some(true) || right.right_to_left != Some(true) {
        return false;
      }
      let mut right = right.clone();
      right.wordprocessingml_font_hint = left.wordprocessingml_font_hint;
      // CJK line metrics are inferred from the characters of each XML run.
      // They must not separate otherwise identical RTL source portions:
      // native split 中/1 controls select the same CS digit face as 中1.
      // The merged portion takes the union of these metrics below.
      right.wordprocessingml_cjk_line_metrics = left.wordprocessingml_cjk_line_metrics;
      if left.bidi_language != right.bidi_language {
        if !allow_arabic_regions {
          return false;
        }
        let arabic = |language: Option<&str>| {
          language
            .and_then(|language| language.split(['-', '_']).next())
            .is_some_and(|primary| primary.eq_ignore_ascii_case("ar"))
        };
        if !arabic(left.bidi_language.as_deref()) || !arabic(right.bidi_language.as_deref()) {
          return false;
        }
        // Native Word retains Arabic ligatures across ar-SA/ar-JO and other
        // Arabic region changes. They share the Arabic shaping language and
        // numeric policy; other language families remain separate owners.
        right.bidi_language = left.bidi_language.clone();
      }
      *left == right
    };
  // Compact identical-language portions first: a leading mark followed by
  // a base in the same portion has valid joining context. Only then can
  // ordinary text cross an Arabic region boundary without changing which
  // marks are independent. The native three-portion controls distinguish
  // this from merging greedily across the first region change.
  for allow_arabic_regions in [false, true] {
    let compatible = |left: &TextRun, right: &TextRun| {
      (left.style.right_to_left == Some(true)
      || (left.text.ends_with('_') && right.text.starts_with('_'))
      // Identical-format XML runs must not hide an adjacent punctuation pair
      // from shaping/line fit. Keep real style, field and semantic boundaries.
      || crate::fonts::wordprocessingml_punctuation_pair_tightens(
        left.text.chars().next_back(), right.text.chars().next(), &left.style)
      // Native F/o/cal controls retain automatic hyphenation and identical
      // painted glyphs when these otherwise identical Latin runs are joined.
      || (left.text.chars().next_back().is_some_and(|c| c.is_ascii_alphabetic())
        && right.text.chars().next().is_some_and(|c| c.is_ascii_alphabetic())))
      && !left.text.is_empty()
      && !right.text.is_empty()
      && !left.preserve_text_portion
      && !right.preserve_text_portion
      && !left.style.wordprocessingml_field_group
      && left.dynamic_field.is_none()
      && right.dynamic_field.is_none()
      // Unlike ordinary Arabic text, separately tagged marks and whitespace
      // retain their native shaping boundaries. A foreign-region blank must
      // not acquire cross-boundary GPOS adjustments from its neighboring text.
      // Keep both edges, including the existing dotted-circle mark owner.
      && (left.style.bidi_language == right.style.bidi_language
        || (!left.text.chars().all(crate::fonts::arabic_nonspacing_mark)
          && !right.text.chars().all(crate::fonts::arabic_nonspacing_mark)
          && !left.text.chars().all(char::is_whitespace)
          && !right.text.chars().all(char::is_whitespace)))
      && same_style(&left.style, &right.style, allow_arabic_regions, crate::fonts::wordprocessingml_punctuation_pair_tightens(left.text.chars().next_back(), right.text.chars().next(), &left.style))
      && left.hyperlink_url == right.hyperlink_url
      && left.style_ref_keys == right.style_ref_keys
      && left.style_ref_text == right.style_ref_text
      && left.style_ref_numbering_text == right.style_ref_numbering_text
    };
    if !inlines.windows(2).any(|pair| {
    matches!(&pair, [InlineItem::Text(left), InlineItem::Text(right)] if compatible(left, right))
  }) {
    continue;
  }
    let mut boundaries = vec![false; inlines.len() + 1];
    for event in field_events.iter() {
      match event {
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
        | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
          if let Some(boundary) = boundaries.get_mut(*inline_offset) {
            *boundary = true;
          }
        }
        ParagraphFieldEvent::ReferenceResultSpan {
          inline_start,
          inline_end,
          ..
        } => {
          for offset in [*inline_start, *inline_end] {
            if let Some(boundary) = boundaries.get_mut(offset) {
              *boundary = true;
            }
          }
        }
        _ => {}
      }
    }
    let mut merged = Vec::with_capacity(inlines.len());
    let mut offsets = Vec::with_capacity(inlines.len() + 1);
    for (index, item) in inlines.drain(..).enumerate() {
      offsets.push(merged.len());
      if !boundaries[index]
        && let InlineItem::Text(right) = &item
        && let Some(InlineItem::Text(left)) = merged.last_mut()
        && compatible(left, right)
      {
        left.text.push_str(&right.text);
        left.style.wordprocessingml_cjk_line_metrics |=
          right.style.wordprocessingml_cjk_line_metrics;
        // Arabic regions share the ordinary text behavior, but the next
        // leading-mark boundary belongs to this portion's final source run.
        left.style.bidi_language = right.style.bidi_language.clone();
      } else {
        merged.push(item);
      }
    }
    offsets.push(merged.len());
    for event in field_events.iter_mut() {
      match event {
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
        | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
          *inline_offset = offsets[*inline_offset];
        }
        ParagraphFieldEvent::ReferenceResultSpan {
          inline_start,
          inline_end,
          ..
        } => {
          *inline_start = offsets[*inline_start];
          *inline_end = offsets[*inline_end];
        }
        _ => {}
      }
    }
    *inlines = merged;
  }
}

fn resolve_wordprocessingml_rtl_font_portions(
  inlines: &mut Vec<super::InlineItem>,
  field_events: &mut [super::ParagraphFieldEvent],
) {
  use super::{InlineItem, ParagraphFieldEvent};
  use ooxmlsdk_fonts::{FontSize, script_direction_runs_with_options};

  // Native Word selects the font from the complete RTL source portion. A
  // later automatic-spacing or line-break boundary must not turn the `1` in
  // A1 or 中1 into an independent ASCII number. Keep the painted slot separate
  // from cs/rtl, which still owns the complex run properties and line metrics.
  if !inlines.iter().any(|item| {
    matches!(item, InlineItem::Text(run) if run.style.wordprocessingml_font_slots
      && run.style.right_to_left == Some(true) && run.dynamic_field.is_none())
  }) {
    return;
  }
  let mut output = Vec::with_capacity(inlines.len());
  let mut offsets = Vec::with_capacity(inlines.len() + 1);
  for item in inlines.drain(..) {
    offsets.push(output.len());
    let InlineItem::Text(run) = &item else {
      output.push(item);
      continue;
    };
    if !run.style.wordprocessingml_font_slots
      || run.style.right_to_left != Some(true)
      || run.dynamic_field.is_some()
      || run.text.is_empty()
    {
      output.push(item);
      continue;
    }
    let script_runs = script_direction_runs_with_options(
      &run.text,
      FontSize(run.style.font_size_pt),
      crate::fonts::script_scan_options(&run.style, false),
    );
    let mut portions: Vec<(
      std::ops::Range<usize>,
      ooxmlsdk_fonts::WordprocessingFontSlot,
    )> = Vec::with_capacity(script_runs.len());
    for script_run in script_runs {
      let Some(slot) = script_run.wordprocessingml_font_slot else {
        continue;
      };
      if let Some((range, previous_slot)) = portions.last_mut()
        && *previous_slot == slot
        && range.end == script_run.text_range.start
      {
        range.end = script_run.text_range.end;
      } else {
        portions.push((script_run.text_range, slot));
      }
    }
    for (range, slot) in portions {
      let mut portion = run.clone();
      portion.text = run.text[range].to_owned();
      portion.style.wordprocessingml_resolved_font_slot = Some(slot);
      output.push(InlineItem::Text(portion));
    }
  }
  offsets.push(output.len());
  for event in field_events {
    match event {
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
        *inline_offset = offsets[*inline_offset];
      }
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start,
        inline_end,
        ..
      } => {
        *inline_start = offsets[*inline_start];
        *inline_end = offsets[*inline_end];
      }
      _ => {}
    }
  }
  *inlines = output;
}

fn wordprocessingml_cjk_text_metrics(text: &str, style: &TextStyle) -> bool {
  // Word applies its CJK-capable font-height adjustment to visible East Asian
  // text even when `w:noLeading` is absent. Keep the document-level
  // compatibility flag for the independent Latin-text path, and opt ordinary
  // runs in from their statically classified Unicode script. A generated
  // Office UI resource with an explicit line box already owns these metrics;
  // applying the font adjustment again would move its baseline inside that
  // box. Numbering labels retain their independent ownership in
  // include_numbering_label_height.
  style.line_height_override_pt.is_none()
    && text.chars().any(|character| {
      matches!(
        character.script(),
        Script::Bopomofo | Script::Han | Script::Hangul | Script::Hiragana | Script::Katakana
      )
    })
}

pub(super) fn apply_wordprocessingml_cjk_text_metrics(text: &str, style: &mut TextStyle) {
  if wordprocessingml_cjk_text_metrics(text, style) {
    style.wordprocessingml_cjk_line_metrics = true;
  }
}

fn paragraph_text_owns_line_metrics(
  inlines: &[super::InlineItem],
  has_numbering_label: bool,
) -> bool {
  !has_numbering_label
    && inlines
      .iter()
      .all(super::InlineItem::leaves_host_line_metrics_text_owned)
}

fn paragraph_uses_windows_font_metrics(
  format: &ParagraphFormat,
  text_owns_line_metrics: bool,
) -> bool {
  const WORD_COMPACT_AUTO_LINE_MULTIPLE: f32 = 259.0 / units::WORD_LINE_HEIGHT_UNITS_PER_LINE;

  // A text-only Word line is positioned on its Windows alignment baseline;
  // proportional excess remains the independent TextFrame gap below it. The
  // font resolver still honors OS/2 USE_TYPO_METRICS. When numbering or an
  // inline object participates in the line, however, that shared owner sets
  // the common baseline. Preserve the established compact/physical-line
  // boundary instead of applying usWinAscent a second time to the text run.
  // Native mixed-font controls retain the same alignment baseline for left
  // and block justification, including automatic sub/superscript portions.
  // Justification changes horizontal advances, not the font boxes combined
  // into a text-owned line. Switching to centered hhea boxes here fabricated
  // extra height when Times New Roman and Calibri shared a proportional line.
  text_owns_line_metrics
    || !matches!(format.line_height_rule, LineHeightRule::Auto)
    || !format
      .line_height_pt
      .is_some_and(|multiple| multiple > WORD_COMPACT_AUTO_LINE_MULTIPLE)
}

pub(super) fn paragraph_style_ref_text(inlines: &[super::InlineItem]) -> Option<Arc<str>> {
  let mut text = String::new();
  // A plain STYLEREF field returns only the referenced paragraph text. Its
  // list number is a separate result selected by \n/\r/\t/\w. Keeping the
  // visible label here makes adjacent number-only and text-only fields repeat
  // the label ("Appendix A A. Comment-Based Help" in tdf95495.docx).
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{
    FloatingImagePlacement, FloatingPaintOrder, HorizontalImageReference, ImagePlacement,
    ImageWrapMode, ImageWrapSide, ParagraphAdjust, ParagraphJustification, VerticalImageReference,
  };

  #[test]
  fn rtl_font_portions_preserve_source_context_and_field_boundaries() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    use ooxmlsdk_fonts::WordprocessingFontSlot::{Ascii, ComplexScript};

    for (text, expected) in [
      ("A1", vec![("A1", ComplexScript)]),
      ("中1", vec![("中1", ComplexScript)]),
      ("1A", vec![("1A", Ascii)]),
      ("1 A", vec![("1", Ascii), (" A", ComplexScript)]),
    ] {
      let mut inlines = vec![InlineItem::Text(TextRun {
        text: text.to_owned(),
        style: TextStyle {
          font_family: Some(Arc::from("Calibri")),
          complex_font_family: Some(Arc::from("Times New Roman")),
          right_to_left: Some(true),
          wordprocessingml_font_slots: true,
          ..Default::default()
        },
        hyperlink_url: Some("https://example.com/".to_owned()),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })];
      let mut events = vec![
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 },
        ParagraphFieldEvent::ReferenceResultSpan {
          field_id: 7,
          bookmark_name: "target".to_owned(),
          inline_start: 0,
          inline_end: 1,
          merge_format: false,
        },
      ];
      resolve_wordprocessingml_rtl_font_portions(&mut inlines, &mut events);
      assert_eq!(inlines.len(), expected.len());
      for (item, (text, slot)) in inlines.iter().zip(&expected) {
        let InlineItem::Text(run) = item else {
          panic!("literal text")
        };
        assert_eq!(run.text, *text);
        assert_eq!(run.style.wordprocessingml_resolved_font_slot, Some(*slot));
        assert_eq!(
          run.style.complex_font_family.as_deref(),
          Some("Times New Roman")
        );
        assert_eq!(run.hyperlink_url.as_deref(), Some("https://example.com/"));
        // Formatting either side of a CJK/number boundary independently
        // retains the slot selected from the complete source portion.
        for character in run.text.chars() {
          let mut buffer = [0; 4];
          let shaped = ooxmlsdk_fonts::script_direction_runs_with_options(
            character.encode_utf8(&mut buffer),
            ooxmlsdk_fonts::FontSize(11.0),
            crate::fonts::script_scan_options(&run.style, false),
          );
          assert!(
            shaped
              .iter()
              .all(|part| part.wordprocessingml_font_slot == Some(*slot))
          );
        }
      }
      assert!(
        matches!(events[0], ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
        if inline_offset == expected.len())
      );
      assert!(
        matches!(events[1], ParagraphFieldEvent::ReferenceResultSpan { inline_start: 0, inline_end, .. }
        if inline_end == expected.len())
      );
    }
  }

  #[test]
  fn identical_rtl_runs_with_cjk_metrics_share_font_selection_context() {
    use crate::docx::InlineItem;
    use ooxmlsdk_fonts::WordprocessingFontSlot::ComplexScript;

    let run = |text: &str| {
      let mut style = TextStyle {
        font_family: Some(Arc::from("Calibri")),
        complex_font_family: Some(Arc::from("Times New Roman")),
        right_to_left: Some(true),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      apply_wordprocessingml_cjk_text_metrics(text, &mut style);
      InlineItem::Text(TextRun {
        text: text.to_owned(),
        style,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut inlines = vec![run("中"), run("1")];
    merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
    resolve_wordprocessingml_rtl_font_portions(&mut inlines, &mut []);
    assert_eq!(inlines.len(), 1);
    let InlineItem::Text(run) = &inlines[0] else {
      panic!("literal text")
    };
    assert_eq!(run.text, "中1");
    assert!(run.style.wordprocessingml_cjk_line_metrics);
    assert_eq!(
      run.style.wordprocessingml_resolved_font_slot,
      Some(ComplexScript)
    );
  }

  #[test]
  fn literal_pdf_sequences_keep_native_metrics_and_field_boundaries() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    let run = |text: &str| {
      InlineItem::Text(TextRun {
        text: text.to_owned(),
        style: TextStyle {
          complex_font_family: Some(Arc::from("Times New Roman")),
          font_size_pt: 15.0,
          complex_font_size_pt: Some(15.0),
          right_to_left: Some(true),
          wordprocessingml_font_slots: true,
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut resolver = crate::fonts::FontResolver::default();
    for count in [1, 2, 6] {
      let controls = "\u{202c}".repeat(count);
      for split in [false, true] {
        let source = format!("A{controls}B");
        let mut inlines = if split {
          std::iter::once(run("A"))
            .chain((0..count).map(|_| run("\u{202c}")))
            .chain([run("B")])
            .collect()
        } else {
          vec![run(&source)]
        };
        let mut events = vec![ParagraphFieldEvent::DeferredParagraphBreak {
          inline_offset: inlines.len(),
        }];
        preserve_interior_direction_control_metrics(&mut inlines, &mut events);
        let mut joined = String::new();
        let mut width = 0.0;
        for item in &inlines {
          let InlineItem::Text(run) = item else {
            unreachable!()
          };
          joined.push_str(&run.text);
          if run.style.wordprocessing_nominal_control_metrics {
            let shaped = resolver.shape_text_runs(&run.text, &run.style).unwrap();
            width += shaped
              .iter()
              .flat_map(|run| run.glyphs.iter())
              .map(|glyph| glyph.x_advance_pt)
              .sum::<f32>();
          }
        }
        assert_eq!(joined, source);
        // Actual Word GetGlyphPlacements: TNR U+202C=1536/2048 em.
        let expected = if count >= 2 {
          count as f32 * 11.25
        } else {
          0.0
        };
        assert!(
          (width - expected).abs() < 0.0001,
          "{count} {split}: {width}"
        );
        assert!(
          matches!(events[0], ParagraphFieldEvent::DeferredParagraphBreak {
          inline_offset
        } if inline_offset == inlines.len())
        );
      }
    }
    for count in [1, 2, 6] {
      let controls = "\u{202c}".repeat(count);
      let mut item = run(&format!("A{controls}B"));
      let InlineItem::Text(text) = &mut item else {
        unreachable!()
      };
      text.style.wordprocessing_bidi_scopes =
        Some(Arc::from(vec![crate::model::WordprocessingBidiScope {
          id: 1,
          right_to_left: true,
          override_direction: false,
        }]));
      let mut inlines = vec![item];
      preserve_interior_direction_control_metrics(&mut inlines, &mut []);
      let InlineItem::Text(last) = inlines.last().unwrap() else {
        unreachable!()
      };
      assert_eq!(last.text, "B");
      assert_eq!(
        last.style.wordprocessing_bidi_prefix.as_deref(),
        Some(controls.as_str())
      );
    }
    for surrounding in [false, true] {
      for control in [false, true] {
        let mut inlines = vec![run("AAA"), run("\u{202c}\u{202c}"), run("BBB")];
        for (index, item) in inlines.iter_mut().enumerate() {
          let InlineItem::Text(text) = item else {
            unreachable!()
          };
          text.style.right_to_left = Some(if index == 1 { control } else { surrounding });
        }
        preserve_interior_direction_control_metrics(&mut inlines, &mut []);
        let InlineItem::Text(text) = &inlines[1] else {
          unreachable!()
        };
        assert_eq!(text.style.wordprocessing_nominal_control_metrics, control);
      }
    }
    for (family, advance) in [
      ("Traditional Arabic", 45.0),
      ("Times New Roman", 67.5),
      ("Arial", 0.0),
    ] {
      let mut item = run("\u{202c}\u{202c}\u{202c}\u{202c}\u{202c}\u{202c}");
      let InlineItem::Text(text) = &mut item else {
        unreachable!()
      };
      text.style.complex_font_family = Some(Arc::from(family));
      text.style.wordprocessing_nominal_control_metrics = true;
      let shaped = resolver.shape_text_runs(&text.text, &text.style).unwrap();
      let width = shaped.iter().map(|run| run.advance_pt).sum::<f32>();
      assert!((width - advance).abs() < 0.0001, "{family}: {width}");
      // A covered zero-advance glyph is authoritative; fallback applies only
      // when the authored face has no cmap entry for the nominal control.
      text.style.wordprocessing_nominal_control_metrics = false;
      let hidden = resolver.shape_text_runs(&text.text, &text.style).unwrap();
      assert_eq!(hidden.iter().map(|run| run.advance_pt).sum::<f32>(), 0.0);
    }
    for source in [
      "\u{202c}\u{202c}AB",
      "AB\u{202c}\u{202c}",
      "A\u{202a}\u{202a}B",
      "A\u{202b}\u{202b}B",
      "A\u{202d}\u{202d}B",
      "A\u{202e}\u{202e}B",
      "A\u{200e}\u{200e}B",
      "A\u{202c}B",
    ] {
      let mut inlines = vec![run(source)];
      preserve_interior_direction_control_metrics(&mut inlines, &mut []);
      assert!(
        inlines
          .iter()
          .all(|item| matches!(item, InlineItem::Text(run)
        if !run.style.wordprocessing_nominal_control_metrics)),
        "{source:?}"
      );
    }
  }

  #[test]
  fn arabic_leading_marks_keep_color_context_and_language_boundaries() {
    use crate::docx::InlineItem;

    let run = |text: &str, language: &str, size: f32| {
      InlineItem::Text(TextRun {
        text: text.to_string(),
        style: TextStyle {
          complex_font_family: Some(Arc::from("Traditional Arabic")),
          complex_font_size_pt: Some(size),
          right_to_left: Some(true),
          bidi_language: Some(Arc::from(language)),
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    for (text, language, size, attached) in [
      ("َ", "ar-EG", 15.0, true),
      ("َ", "ar-JO", 15.0, false),
      ("َ", "ar-EG", 18.0, false),
      ("َي", "ar-JO", 15.0, true),
      ("َي", "ar-EG", 18.0, false),
    ] {
      let mut inlines = vec![run("هدف", "ar-EG", 15.0), run(text, language, size)];
      let InlineItem::Text(mark) = &mut inlines[1] else {
        unreachable!()
      };
      mark.style.color = crate::model::RgbColor { r: 255, g: 0, b: 0 };
      attach_arabic_leading_mark_context(&mut inlines);
      let InlineItem::Text(mark) = &inlines[1] else {
        unreachable!()
      };
      assert_eq!(
        mark.style.shaping_context.is_some(),
        attached,
        "{text} {language} {size}"
      );
      if let Some(context) = &mark.style.shaping_context {
        assert_eq!(&*context.before, "هدف");
        assert!(context.leading_marks_only);
      }
    }
    let mut inlines = vec![run("Latin", "ar-EG", 15.0), run("َ", "ar-EG", 15.0)];
    attach_arabic_leading_mark_context(&mut inlines);
    let InlineItem::Text(mark) = &inlines[1] else {
      unreachable!()
    };
    assert!(mark.style.shaping_context.is_none());
  }

  #[test]
  fn rtl_arabic_region_changes_keep_ligatures_and_independent_marks() {
    use crate::docx::InlineItem;

    let run = |text: &str, language: &str| {
      InlineItem::Text(TextRun {
        text: text.to_owned(),
        style: TextStyle {
          complex_font_family: Some(Arc::from("Traditional Arabic")),
          complex_font_size_pt: Some(15.0),
          right_to_left: Some(true),
          resolved_bidi_level: Some(1),
          wordprocessingml_font_slots: true,
          bidi_language: Some(Arc::from(language)),
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut resolver = crate::fonts::FontResolver::default();
    // Actual Office controls retain these ligatures under each Arabic
    // region, while a separately tagged marks-only run keeps two circles.
    for language in ["ar-JO", "ar-EG", "ar-MA"] {
      for (left, right, native) in [
        ("ب", "القضاء", [160, 418, 468, 492, 499, 682].as_slice()),
        ("ب", "َي", [200, 314].as_slice()),
      ] {
        let mut inlines = vec![run(left, "ar-SA"), run(right, language)];
        merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
        assert_eq!(inlines.len(), 1, "{language} {left} {right}");
        let InlineItem::Text(joined) = &inlines[0] else {
          unreachable!()
        };
        let shaped = resolver
          .shape_text_runs(&joined.text, &joined.style)
          .unwrap();
        let glyphs = shaped
          .iter()
          .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
          .collect::<Vec<_>>();
        assert_eq!(glyphs, native, "{language} {left} {right}");
      }
      let mut inlines = vec![run("ب", "ar-SA"), run("ِّ", language), run("م", "ar-SA")];
      merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
      attach_arabic_leading_mark_context(&mut inlines);
      assert_eq!(inlines.len(), 3);
      let InlineItem::Text(mark) = &inlines[1] else {
        unreachable!()
      };
      assert!(mark.style.shaping_context.is_none());
      let shaped = resolver.shape_text_runs(&mark.text, &mark.style).unwrap();
      assert_eq!(
        shaped
          .iter()
          .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
          .collect::<Vec<_>>(),
        [202, 588, 203, 588]
      );
    }
    for language in ["fa-IR", "en-GB"] {
      let mut inlines = vec![run("ب", "ar-SA"), run("القضاء 12/34", language)];
      merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
      assert_eq!(inlines.len(), 2, "{language}");
      let InlineItem::Text(right) = &inlines[1] else {
        unreachable!()
      };
      assert_eq!(right.style.bidi_language.as_deref(), Some(language));
    }
    // Native three-portion controls pin both the final language owner and
    // the distinction between a marks-only run and one followed by a base.
    for (parts, count, native) in [
      (
        [("ب", "ar-SA"), ("ا", "ar-JO"), ("َ", "ar-JO")],
        1,
        [200, 682].as_slice(),
      ),
      (
        [("ب", "ar-SA"), ("ا", "ar-JO"), ("َ", "ar-SA")],
        2,
        [682, 200, 588].as_slice(),
      ),
      (
        [("ب", "ar-SA"), ("ِّ", "ar-JO"), ("م", "ar-JO")],
        1,
        [202, 203, 312].as_slice(),
      ),
      (
        [("ب", "ar-SA"), ("ِّ", "ar-JO"), ("م", "ar-EG")],
        3,
        [167, 202, 588, 203, 588, 191].as_slice(),
      ),
    ] {
      let mut inlines = parts.map(|(text, language)| run(text, language)).to_vec();
      merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
      attach_arabic_leading_mark_context(&mut inlines);
      assert_eq!(inlines.len(), count, "{parts:?}");
      let mut glyphs = Vec::new();
      for inline in &inlines {
        let InlineItem::Text(run) = inline else {
          unreachable!()
        };
        glyphs.extend(
          resolver
            .shape_text_runs(&run.text, &run.style)
            .unwrap()
            .iter()
            .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id)),
        );
      }
      assert_eq!(glyphs, native, "{parts:?}");
    }
  }

  #[test]
  fn rtl_foreign_region_whitespace_keeps_native_space_advances() {
    use crate::docx::InlineItem;

    let run = |text: &str, language: &str| {
      InlineItem::Text(TextRun {
        text: text.to_owned(),
        style: TextStyle {
          complex_font_family: Some(Arc::from("Traditional Arabic")),
          complex_font_size_pt: Some(15.0),
          right_to_left: Some(true),
          resolved_bidi_level: Some(1),
          wordprocessingml_font_slots: true,
          bidi_language: Some(Arc::from(language)),
          horizontal_scale: Some(1.03),
          wordprocessing_font_width_percent: Some(103),
          wordprocessing_legacy_font_measurement: Some(true),
          wordprocessing_kashida: Some(Arc::new(crate::common::WordprocessingKashida {
            retain_trailing_blank: false,
            unshaped_blanks: false,
            font_width_percent: 103,
            space_expansions: Vec::new(),
          })),
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for language in ["ar-EG", "ar-JO", "ar-SA", "ar-MA", "en-GB"] {
      for blank in [" ", "  ", "\u{00a0}", "\u{202f}"] {
        let mut inlines = vec![
          run("هيمنةً", "ar-EG"),
          run(blank, language),
          run("كبيرةً", "ar-EG"),
        ];
        merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
        let count = if language == "ar-EG" { 1 } else { 3 };
        assert_eq!(inlines.len(), count, "{language} {blank:?}");
        if !matches!(blank, " " | "\u{202f}") || (blank == "\u{202f}" && count == 1) {
          continue;
        }
        let InlineItem::Text(text) = &inlines[if count == 1 { 0 } else { 1 }] else {
          unreachable!()
        };
        let shaped = metrics.shape_text(&text.text, &text.style).unwrap();
        let space = if blank == " " {
          shaped.glyphs.iter().find(|glyph| glyph.glyph_id == 3)
        } else {
          // The native XPS font is Times New Roman; its U+202F maps to
          // glyph3031 and keeps a twenty-six-pixel advance on this grid.
          shaped.glyphs.iter().find(|glyph| glyph.glyph_id == 3031)
        }
        .unwrap();
        // Native exact-source controls: the same-region marked-word context
        // realizes a twelve-pixel space, while the independent blank retains
        // its thirty-one-pixel natural width before justification.
        let expected = if blank == "\u{202f}" {
          26
        } else if count == 1 {
          12
        } else {
          31
        };
        assert_eq!(
          (space.x_advance_em * space.font_size_pt / 0.12).round() as i32,
          expected,
          "{language}"
        );
      }
    }
  }

  #[test]
  fn rtl_run_compaction_keeps_field_spans_and_deferred_breaks_at_their_text() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    let run = |text: &str| {
      InlineItem::Text(TextRun {
        text: text.to_string(),
        style: TextStyle {
          right_to_left: Some(true),
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut inlines = vec![run("کتاب‌های"), run("ی"), run("آن"), run("‌ها"), run("پس")];
    let mut events = vec![
      ParagraphFieldEvent::ReferenceResultSpan {
        field_id: 1,
        bookmark_name: "target".into(),
        inline_start: 2,
        inline_end: 4,
        merge_format: false,
      },
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 4 },
    ];
    merge_adjacent_continuous_text_runs(&mut inlines, &mut events);
    let texts = inlines
      .iter()
      .map(|inline| match inline {
        InlineItem::Text(run) => run.text.as_str(),
        _ => unreachable!(),
      })
      .collect::<Vec<_>>();
    assert_eq!(texts, ["کتاب‌هایی", "آن‌ها", "پس"]);
    assert!(matches!(
      events[0],
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start: 1,
        inline_end: 2,
        ..
      }
    ));
    assert_eq!(
      events[1],
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 2 }
    );

    let mut styled = vec![run("کتاب‌های"), run("ی")];
    let InlineItem::Text(second) = &mut styled[1] else {
      unreachable!()
    };
    second.style.bold = true;
    merge_adjacent_continuous_text_runs(&mut styled, &mut []);
    assert_eq!(styled.len(), 2);

    let mut hinted = vec![run("کر"), run("ی"), run("م")];
    for (inline, hint) in hinted.iter_mut().zip([
      ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia,
      ooxmlsdk_fonts::WordprocessingFontTypeHint::ComplexScript,
      ooxmlsdk_fonts::WordprocessingFontTypeHint::Default,
    ]) {
      let InlineItem::Text(run) = inline else {
        unreachable!()
      };
      run.style.wordprocessingml_font_hint = Some(hint);
    }
    merge_adjacent_continuous_text_runs(&mut hinted, &mut []);
    assert_eq!(hinted.len(), 1);
    let InlineItem::Text(joined) = &hinted[0] else {
      unreachable!()
    };
    assert_eq!(joined.text, "کریم");
  }

  #[test]
  fn adjacent_punctuation_compaction_preserves_field_and_formatting_boundaries() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    let run = |text: &str| {
      InlineItem::Text(TextRun {
        text: text.into(),
        style: TextStyle {
          font_family: Some("ＭＳ 明朝".into()),
          east_asia_font_family: Some("ＭＳ 明朝".into()),
          font_size_pt: 12.0,
          wordprocessingml_font_slots: true,
          wordprocessingml_punctuation_spacing: true,
          ..TextStyle::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut plain = vec![run("漢。"), run("（漢")];
    let InlineItem::Text(second) = &mut plain[1] else {
      unreachable!()
    };
    second.style.wordprocessingml_cjk_line_metrics = true;
    merge_adjacent_continuous_text_runs(&mut plain, &mut []);
    assert_eq!(plain.len(), 1);
    let InlineItem::Text(joined) = &plain[0] else {
      unreachable!()
    };
    assert_eq!(joined.text, "漢。（漢");
    assert!(joined.style.wordprocessingml_cjk_line_metrics);
    // Native split and unsplit12pt controls have the same42pt logical width.
    let mut metrics = crate::text_metrics::TextMetrics::new();
    assert!((metrics.measure_text(&joined.text, &joined.style) - 42.0).abs() < 0.001);
    let mut styled = vec![run("漢。"), run("（漢")];
    let InlineItem::Text(second) = &mut styled[1] else {
      unreachable!()
    };
    second.style.bold = true;
    merge_adjacent_continuous_text_runs(&mut styled, &mut []);
    assert_eq!(styled.len(), 2);
    let mut fields = vec![run("漢。"), run("（漢")];
    let mut events = [ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 }];
    merge_adjacent_continuous_text_runs(&mut fields, &mut events);
    assert_eq!(fields.len(), 2);
    assert!(matches!(
      events[0],
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 }
    ));
  }

  #[test]
  fn latin_word_compaction_preserves_field_and_formatting_boundaries() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    let run = |text: &str| {
      InlineItem::Text(TextRun {
        text: text.to_string(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let texts = |inlines: &[InlineItem]| {
      inlines
        .iter()
        .map(|inline| match inline {
          InlineItem::Text(run) => run.text.clone(),
          _ => unreachable!(),
        })
        .collect::<Vec<_>>()
    };
    let mut field_inlines = vec![run("Associate WID F"), run("o"), run("cal Point")];
    let mut events = vec![ParagraphFieldEvent::ReferenceResultSpan {
      field_id: 1,
      bookmark_name: "target".into(),
      inline_start: 2,
      inline_end: 3,
      merge_format: false,
    }];
    merge_adjacent_continuous_text_runs(&mut field_inlines, &mut events);
    assert_eq!(texts(&field_inlines), ["Associate WID Fo", "cal Point"]);
    assert!(matches!(
      events[0],
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start: 1,
        inline_end: 2,
        ..
      }
    ));

    for preserve_portion in [false, true] {
      let mut styled = vec![run("Associate WID F"), run("o"), run("cal Point")];
      let InlineItem::Text(second) = &mut styled[1] else {
        unreachable!()
      };
      if preserve_portion {
        second.preserve_text_portion = true;
      } else {
        second.style.bold = true;
      }
      merge_adjacent_continuous_text_runs(&mut styled, &mut []);
      assert_eq!(texts(&styled), ["Associate WID F", "o", "cal Point"]);
    }

    let mut separate_words = vec![run("Focal "), run("Point")];
    merge_adjacent_continuous_text_runs(&mut separate_words, &mut []);
    assert_eq!(texts(&separate_words), ["Focal ", "Point"]);
  }

  #[test]
  fn split_underscore_form_line_is_one_continuous_text_portion() {
    use crate::docx::{InlineItem, ParagraphFieldEvent};
    let run = |text: &str| {
      InlineItem::Text(TextRun {
        text: text.to_string(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut inlines = vec![
      run("label ____________________________"),
      run("_____"),
      run("_________________________________________________, next"),
    ];
    merge_adjacent_continuous_text_runs(&mut inlines, &mut []);
    assert_eq!(inlines.len(), 1);
    let InlineItem::Text(joined) = &inlines[0] else {
      unreachable!()
    };
    assert!(joined.text.contains(&"_".repeat(82)));

    let mut field_inlines = vec![run("____"), run("____")];
    let mut events = vec![ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 }];
    merge_adjacent_continuous_text_runs(&mut field_inlines, &mut events);
    assert_eq!(field_inlines.len(), 2);
    assert!(matches!(
      events[0],
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 }
    ));
  }

  #[test]
  fn cjk_text_metrics_require_a_visible_east_asian_script() {
    let style = TextStyle::default();
    for text in ["預期結果", "かな", "カナ", "결과", "ㄅㄆㄇ"] {
      assert!(wordprocessingml_cjk_text_metrics(text, &style), "{text}");
    }
    for text in ["Expected Result", "نتيجةمتوقعة", "（）"] {
      assert!(!wordprocessingml_cjk_text_metrics(text, &style), "{text}");
    }

    let generated_resource = TextStyle {
      line_height_override_pt: Some(16.32),
      ..Default::default()
    };
    assert!(!wordprocessingml_cjk_text_metrics(
      "错误!使用资源",
      &generated_resource
    ));
  }

  #[test]
  fn paragraph_style_ref_text_excludes_the_separate_list_label() {
    let inlines = [super::super::InlineItem::Text(TextRun {
      text: "Comment-Based Help".to_string(),
      style: TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    })];

    // The caller retains "A. " in Paragraph::list_label and exposes its
    // numbering-only STYLEREF value through style_ref_numbering_text.
    assert_eq!(
      paragraph_style_ref_text(&inlines).as_deref(),
      Some("Comment-Based Help")
    );
  }

  #[test]
  fn proportional_auto_line_spacing_keeps_windows_baseline_metrics() {
    for line_units in [276.0, 360.0] {
      let format = ParagraphFormat {
        line_height_rule: LineHeightRule::Auto,
        line_height_pt: Some(line_units / units::WORD_LINE_HEIGHT_UNITS_PER_LINE),
        ..Default::default()
      };

      assert!(paragraph_uses_windows_font_metrics(&format, true));
      assert!(!paragraph_uses_windows_font_metrics(&format, false));

      let justified = ParagraphFormat {
        justification: ParagraphJustification {
          adjust: ParagraphAdjust::Block,
          ..Default::default()
        },
        justification_set: true,
        ..format
      };
      assert!(paragraph_uses_windows_font_metrics(&justified, true));
      assert!(!paragraph_uses_windows_font_metrics(&justified, false));
    }
  }

  #[test]
  fn only_inline_drawings_participate_in_host_line_metrics() {
    assert!(ImagePlacement::Inline.participates_in_host_line_metrics());

    let floating = ImagePlacement::Floating(FloatingImagePlacement {
      horizontal_relative_to: HorizontalImageReference::Margin,
      vertical_relative_to: VerticalImageReference::Paragraph,
      horizontal_alignment: None,
      vertical_alignment: None,
      alignment_extent: None,
      group_child_offset_x_pt: 0.0,
      group_child_offset_y_pt: 0.0,
      horizontal_offset_pt: 0.0,
      vertical_offset_pt: 0.0,
      horizontal_offset_pct: None,
      vertical_offset_pct: None,
      wrap: ImageWrapMode::None,
      wrap_side: ImageWrapSide::BothSides,
      behind_text: true,
      layout_in_cell: true,
      layout_in_cell_forced: false,
      allow_overlap: true,
      paint_order: FloatingPaintOrder::Unspecified,
      relative_width_to: None,
      relative_width_pct: None,
      relative_height_to: None,
      relative_height_pct: None,
      margin_top_pt: 0.0,
      margin_right_pt: 0.0,
      margin_bottom_pt: 0.0,
      margin_left_pt: 0.0,
    });
    assert!(!floating.participates_in_host_line_metrics());
  }

  #[test]
  fn non_proportional_line_spacing_keeps_windows_baseline_metrics() {
    for (line_height_rule, line_height_pt) in [
      (LineHeightRule::Auto, None),
      (LineHeightRule::Auto, Some(0.9)),
      (LineHeightRule::Auto, Some(1.0)),
      (
        LineHeightRule::Auto,
        Some(259.0 / units::WORD_LINE_HEIGHT_UNITS_PER_LINE),
      ),
      (LineHeightRule::AtLeast, Some(18.0)),
      (LineHeightRule::Exact, Some(18.0)),
    ] {
      let format = ParagraphFormat {
        line_height_rule,
        line_height_pt,
        ..Default::default()
      };

      assert!(paragraph_uses_windows_font_metrics(&format, false));
    }
  }
}
