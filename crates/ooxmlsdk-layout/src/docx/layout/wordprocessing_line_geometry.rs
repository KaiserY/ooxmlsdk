//! Preserve a completed Word line's source geometry until fixed-output paint.
use super::*;

pub(super) struct Placement {
  pub frame_width_pt: f64,
  pub y: f32,
  pub height: f32,
  pub before: f32,
  pub after: f32,
}

/// Recover an authored twip box only when its f32 representation is exact.
/// An already resolved 1/4096pt box and non-twip geometry keep their owner.
pub(super) fn frame_width(width: f32) -> f64 {
  let points = f64::from(width);
  let twip_points = (points * 20.0).round() / 20.0;
  if (points * 4096.0).fract() != 0.0 && twip_points as f32 == width {
    twip_points
  } else {
    points
  }
}

pub(super) fn mixed_math_height(
  items: &[PageItem],
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  current_height: f32,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if !matches!(paragraph.format.line_height_rule, LineHeightRule::Auto)
    // Compressed formula lines have a separate clipping/spacing contract.
    || paragraph
      .format
      .line_height_pt
      .is_some_and(|multiple| !multiple.is_finite() || multiple < 1.0)
    || !paragraph_has_mixed_office_math(paragraph)
    || document_grid_line_metrics(
      current_height,
      paragraph,
      flow.setup,
      flow.text_segmentation,
    )
    .is_some()
  {
    return current_height;
  }
  let mut ascent = 0.0_f32;
  let mut descent = 0.0_f32;
  let mut font_height = 0.0_f32;
  let mut has_math = false;
  let mut has_text = false;
  for item in items {
    if let PageItem::Text(text) = item
      && text.line_metrics_participant
      && text.wordprocessing_effect_host.is_none()
    {
      let metrics = text_metrics.line_vertical_metrics_for_text(&text.text, &text.style);
      let shift = if text.style.automatic_escapement_font_size_pt.is_some() {
        0.0
      } else {
        text.style.baseline_shift_pt
      };
      ascent = ascent.max(metrics.directwrite_baseline_offset_pt + shift);
      descent =
        descent.max(metrics.line_height_pt() - metrics.directwrite_baseline_offset_pt - shift);
      font_height = font_height.max(metrics.line_height_pt());
      has_text = true;
    }
    if is_office_math_alignment_item(item)
      && let Some(image) = inline_alignment_image(item)
    {
      // The SVG carrier includes paint-only protection on both sides. Native
      // line metrics own the formula bounds inside that canvas, not its inset.
      let padding = crate::docx::math::MATH_CANVAS_PADDING_PT;
      ascent = ascent.max(image.height_pt + image.inline_baseline_gap_pt - padding);
      descent = descent.max(-image.inline_baseline_gap_pt - padding);
      has_math = true;
    }
  }
  if !has_math || !has_text {
    return current_height;
  }
  // Native Word mixed text/math controls take independent ascent/descent
  // maxima. Proportional leading is additional flow space below that union,
  // and the largest text font contributes even when it follows the formula.
  // Do not feed this trailing space back into the already aligned paint box.
  let multiple = paragraph.format.line_height_pt.unwrap_or(1.0).max(1.0);
  current_height.max(ascent + descent + font_height * (multiple - 1.0))
}

pub(super) fn bind(
  items: &mut [PageItem],
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  placement: Placement,
  text_metrics: &mut TextMetrics,
) {
  // Native Line Services receives the complete line box for both natural
  // and minimum-height text. Exact lines, grids and objects have separate
  // contracts; preserve their existing owners.
  let minimum = matches!(paragraph.format.line_height_rule, LineHeightRule::AtLeast);
  let text_owned = if minimum {
    paragraph.list_label_image.is_none()
      && paragraph
        .inlines
        .iter()
        .all(InlineItem::leaves_host_line_metrics_text_owned)
  } else {
    paragraph_line_spacing_base_covers_line_height(paragraph)
  };
  if !matches!(paragraph.format.line_height_rule, LineHeightRule::Auto | LineHeightRule::AtLeast)
    || paragraph.format.wordprocessing_shape_story
    || paragraph.format.word_text_frame_story
    // Shape text borrows TableCell segmentation and ancestor cell bounds.
    // Its own text-frame formatter owns the baseline; bounds alone do not
    // establish the ordinary table's already-resolved-baseline contract.
    || (flow.text_segmentation == TextSegmentation::TableCell
      && (!flow.horizontal_table_cell || flow.table_cell_cursor_is_line_top))
    // Inline shapes may flatten their own text story directly into these
    // items. Their host paragraph must not replace that story's baseline.
    || !text_owned
    || paragraph_has_mixed_office_math(paragraph)
    || document_grid_line_metrics(
      placement.height,
      paragraph,
      flow.setup,
      flow.text_segmentation,
    )
    .is_some()
    || items
      .iter()
      .any(|item| inline_alignment_image(item).is_some())
    // Modern effect lines carry additional surface outsets and are realized
    // while materializing their effect geometry, before ordinary PDF paint.
    || items.iter().any(|item| {
      matches!(item, PageItem::Text(text)
        if text.style.text_glow.is_some()
          || text.style.text_shadow.is_some()
          || text.style.text_reflection.is_some()
          || text.style.wordprocessing_text_3d_parts.is_some()
          || text.style.drawingml_text_effects.is_some()
          || text.style.pdf_glyph_outline_options.is_some())
    })
  {
    return;
  }
  let Some(metrics) = items
    .iter()
    .filter_map(|item| match item {
      PageItem::Text(text)
        if text.line_metrics_participant && text.wordprocessing_effect_host.is_none() =>
      {
        wordprocessing_line_metrics(item, text_metrics)
      }
      _ => None,
    })
    .reduce(WordprocessingLineMetrics::include)
  else {
    return;
  };
  let natural = metrics.content_height_pt();
  if natural <= 0.0 || !natural.is_finite() {
    return;
  }
  let line_units = wordprocessing_auto_line_spacing_units(paragraph)
    .unwrap_or(240)
    .min(240);
  let scaled = |pt| {
    let ideal = word_fixed_output_reference_units(pt).unwrap_or(0);
    word_fixed_output_positive_mul_div_round(ideal, i64::from(line_units), 240) as f32 / 4096.0
  };
  let compressed_legacy_reference = !minimum
    && flow.compatibility_mode < 15
    && line_units < 240
    && items.iter().any(|item| {
      matches!(item, PageItem::Text(text)
        if text.line_metrics_participant
          && text.hyperlink_url.as_deref().and_then(page_note_link_parts)
            .is_some_and(|(_, backlink, _)| !backlink))
    });
  // Native Word's old-mode compressed reference lines retain the scaled
  // natural descent, but apply the ordinary-text spacing adjustment to the
  // complete height. Their ascent is the remainder, not the independently
  // scaled original ascent. This gives no positive gap below the line.
  let natural = if compressed_legacy_reference {
    placement.height
  } else {
    scaled(natural)
  };
  let extra = (placement.height - natural).max(0.0);
  // Word's minimum-line controls retain the natural descent and place all
  // excess above the font box. Auto multiples instead place it below.
  let leading_above = if minimum { extra } else { 0.0 };
  let leading_below = if minimum { 0.0 } else { extra };
  let ascent = if compressed_legacy_reference {
    natural - scaled(metrics.descent_pt)
  } else {
    scaled(metrics.ascent_pt) + leading_above
  };
  let cursor_offset =
    if flow.text_segmentation == TextSegmentation::TableCell && flow.layout_cell_bounds.is_some() {
      // A table cursor is already the current line's resolved baseline.
      // A following line can change fonts after a soft break; the first
      // paragraph font's ascent no longer owns this cursor-to-top conversion.
      if minimum {
        // Minimum cell cursors retain their initial Windows-metrics owner.
        // Its first-line delta removes the difference between the natural
        // initial box and the paragraph's authored minimum box.
        let style = paragraph_base_line_style(paragraph);
        let height = paragraph_line_height_for_setup(
          paragraph,
          &style,
          flow.setup,
          flow.text_segmentation,
          text_metrics,
        );
        table_cell_initial_baseline_offset(&style, height, text_metrics)
          + flow.table_cell_baseline_delta_pt
      } else {
        ascent
      }
    } else {
      0.0
    };
  let bottom = placement.y - cursor_offset + placement.height + placement.after;
  for item in items {
    let PageItem::Text(text) = item else { continue };
    if text.wordprocessing_effect_host.is_some() {
      // Flattening a floating WPS story does not transfer its baseline to
      // the enclosing paragraph. Its host retains the native paint origin.
      continue;
    }
    text.wordprocessing_line_metrics = Some(common::WordprocessingLineMetrics {
      frame_origin_offset_x_pt: f64::from(flow.content_left_pt) - f64::from(text.x_pt),
      frame_width_pt: placement.frame_width_pt,
      alignment: None,
      bottom_offset_pt: bottom - text.y_pt,
      height_pt: placement.before + placement.height + placement.after,
      baseline_from_bottom_pt: placement.height - ascent + placement.after,
      spacing_before_pt: placement.before + leading_above,
      spacing_after_pt: placement.after + leading_below,
    });
  }
}

/// Move line content within its frame without transferring the frame's origin.
pub(super) fn align(
  item: &mut PageItem,
  alignment: ParagraphAlignment,
  logical_offset: f32,
  bounds: (f32, f32, f32),
) {
  match item {
    PageItem::Text(text) => {
      let Some(metrics) = &mut text.wordprocessing_line_metrics else {
        return;
      };
      let frame = f64::from(text.x_pt) + metrics.frame_origin_offset_x_pt;
      let (left, right, width) = bounds;
      let device = wordprocessing_table_device::source_coordinate_precise;
      // Native line-input/convert/draw controls distinguish the complete
      // frame width, both paragraph indents and text width. Center uses the
      // remaining whole printer dots, truncating an odd remainder.
      let start = device(f64::from(left) - frame);
      let end =
        device(metrics.frame_width_pt) - device(frame + metrics.frame_width_pt - f64::from(right));
      let remaining = (end - start - device(f64::from(width))).max(0);
      let offset = match alignment {
        ParagraphAlignment::Center => remaining / 2,
        ParagraphAlignment::Right => remaining,
        ParagraphAlignment::Left | ParagraphAlignment::Justify => 0,
      };
      // shift_item_x writes an f32 position. Subtract that actual movement,
      // rather than its requested delta, to keep the original frame exactly.
      let actual_offset = f64::from(text.x_pt + logical_offset) - f64::from(text.x_pt);
      metrics.alignment =
        i32::try_from(offset)
          .ok()
          .map(|device_offset_px| common::WordprocessingLineAlignment {
            logical_offset_pt: actual_offset,
            device_offset_px,
          });
      metrics.frame_origin_offset_x_pt -= actual_offset;
    }
    PageItem::Group(items)
    | PageItem::InlineObjectGroup(items)
    | PageItem::OpacityGroup { items, .. } => {
      for item in items {
        align(item, alignment, logical_offset, bounds);
      }
    }
    // A separately positioned text story moves its own frame with the host.
    _ => {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_full_frame_width_keeps_its_source_precision() {
    // Native WWLIB supplies 3100262 ideal units for the 756.9pt frame.
    // Its f32 encoding is 756.9000244140625; rounding that encoding instead
    // would manufacture 3100263 units and add a printer dot.
    assert_eq!((frame_width(756.9) * 4096.0).round(), 3100262.0);
    let resolved = 3100262.0_f32 / 4096.0;
    assert_eq!(frame_width(resolved), f64::from(resolved));
    assert_eq!(frame_width(117.225), f64::from(117.225_f32));
  }
}
